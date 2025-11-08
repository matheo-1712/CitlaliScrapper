use reqwest;

#[derive(Debug, Serialize)]
pub struct CharacterTempo {
    pub name: String,
    pub url: String,
}
pub async fn scrape_infographics(character_url: &str) -> Result<Vec<Infographic>, Box<dyn Error>> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; Citlali/3.0; +https://citlapi.antredesloutres.fr/)")
        .build()?;

    let html = client.get(character_url).send().await?.text().await?;
    let document = Html::parse_document(&html);

    let selector_title = Selector::parse(".elementor-shortcode h3").unwrap();
    let selector_image = Selector::parse(".elementor-shortcode a[href$='.webp']").unwrap();

    let mut infographics: Vec<Infographic> = Vec::new();

    let titles: Vec<String> = document
        .select(&selector_title)
        .map(|el| el.text().collect::<Vec<_>>().join("").trim().to_string())
        .collect();

    let image_urls: Vec<String> = document
        .select(&selector_image)
        .filter_map(|el| el.value().attr("href"))
        .map(|s| s.to_string())
        .take(1)
        .collect();

    for (i, img) in image_urls.iter().enumerate() {
        let title = titles
            .get(i)
            .cloned()
            .unwrap_or_else(|| "Build inconnu".to_string());

        let infographic = Infographic {
            url: img.to_string(),
            build: title,
            character: extract_character_name_gazette(character_url),
            source: "La Gazette de Teyvat".to_string(),
        };

        // Enregistre chaque infographie directement
        register_infographics(&infographic, Box::from("genshin")).await?;

        infographics.push(infographic);
    }
    Ok(infographics)
}

pub async fn scrape_all_characters() -> Result<(), Box<dyn Error>> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; Citlali/3.0; +https://citlapi.antredesloutres.fr/)")
        .build()?;

    let start_total = Instant::now();
    let url = "https://lagazettedeteyvat.fr/personnages/";
    let html = client.get(url).send().await?.text().await?;
    let document = Html::parse_document(&html);

    let selector_box = Selector::parse("a.entitybox").unwrap();
    let selector_name = Selector::parse("h5").unwrap();

    let mut characters: Vec<CharacterTempo> = Vec::new();

    for box_elem in document.select(&selector_box) {
        let url = box_elem
            .value()
            .attr("href")
            .unwrap_or_default()
            .to_string();

        let name = box_elem
            .select(&selector_name)
            .next()
            .map(|n| n.text().collect::<Vec<_>>().join("").trim().to_string())
            .unwrap_or_default();

        characters.push(CharacterTempo { name, url });
    }

    println!(
        "✅ {} URLs récupérées en {:?}",
        characters.len(),
        start_total.elapsed()
    );

    let concurrency = 10;
    let total_infographics = stream::iter(characters)
        .map(|character| async move {
            match scrape_infographics(&character.url).await {
                Ok(infos) => infos.len(),
                Err(e) => {
                    eprintln!("❌ Erreur sur {} : {}", character.name, e);
                    0
                }
            }
        })
        .buffer_unordered(concurrency)
        .fold(0, |acc, x| async move { acc + x })
        .await;

    println!(
        "✅ Traitement terminé en {:?}, Nombre d'infographies trouvées : {}",
        start_total.elapsed(),
        total_infographics
    );

    Ok(())
}
