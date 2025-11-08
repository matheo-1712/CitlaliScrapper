use reqwest;

#[derive(Debug, Serialize)]
pub struct CharacterTempo {
    pub name: String,
    pub url: String,
}
pub async fn scrape_infographics(
    character_urls: Vec<String>,
) -> Result<Vec<Infographic>, Box<dyn Error>> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; Citlali/3.0; +https://citlapi.antredesloutres.fr/)")
        .build()?;

    let concurrency = 10;

    let infographics: Vec<Infographic> = stream::iter(character_urls)
        .map(|url| {
            let client = client.clone();
            async move {
                let html = match client.get(&url).send().await {
                    Ok(resp) => resp.text().await.unwrap_or_default(),
                    Err(e) => {
                        eprintln!("❌ Erreur téléchargement {} : {}", url, e);
                        return Vec::new();
                    }
                };
                let document = Html::parse_document(&html);

                let selector_title = Selector::parse(".elementor-shortcode h3").unwrap();
                let selector_image =
                    Selector::parse(".elementor-shortcode a[href$='.webp']").unwrap();

                let titles: Vec<String> = document
                    .select(&selector_title)
                    .map(|el| el.text().collect::<Vec<_>>().join("").trim().to_string())
                    .collect();

                // Filtrer les images : http(s) et contenant "build" dans src ou alt
                let image_urls: Vec<String> = document
                    .select(&selector_image)
                    .filter_map(|el| {
                        let href = el.value().attr("href").unwrap_or_default();
                        let alt = el
                            .select(&Selector::parse("img").unwrap())
                            .next()
                            .and_then(|img| img.value().attr("alt"))
                            .unwrap_or_default();

                        // Garde seulement les URLs contenant "build" dans le href ou alt
                        if href.to_lowercase().contains("build")
                            || alt.to_lowercase().contains("build")
                        {
                            Some(href.to_string())
                        } else {
                            None
                        }
                    })
                    .collect();

                let character_name = extract_character_name_gazette(&url);

                let mut infos = Vec::new();
                for (i, img) in image_urls.iter().enumerate() {
                    let title = titles
                        .get(i)
                        .cloned()
                        .unwrap_or_else(|| "Build inconnu".to_string());

                    let infographic = Infographic {
                        url: img.to_string(),
                        build: title,
                        character: format_character_name(&*character_name.clone()),
                        source: "La Gazette de Teyvat".to_string(),
                    };

                    // Envoie chaque infographie immédiatement
                    if let Err(e) = register_infographics(&infographic, Box::from("genshin")).await
                    {
                        eprintln!("❌ Erreur API pour {} : {}", character_name, e);
                    }

                    infos.push(infographic);
                }

                infos
            }
        })
        .buffer_unordered(concurrency)
        .flat_map(|vec_inf| stream::iter(vec_inf))
        .collect()
        .await;

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

    let character_urls: Vec<String> = characters.iter().map(|c| c.url.clone()).collect();

    // Scraping parallèle avec limite de 10
    let total_infographics = scrape_infographics(character_urls).await?;

    println!(
        "✅ Traitement terminé en {:?}, Nombre d'infographies trouvées : {}",
        start_total.elapsed(),
        total_infographics.len()
    );

    Ok(())
}
