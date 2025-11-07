use std::fs;

#[derive(Debug, Serialize)]
struct CharacterTempo {
    name: String,
    url: String,
}

pub async fn scrape_build_url() -> Result<(), Box<dyn Error>> {
    // URL de la page des personnages
    let url = "https://lagazettedeteyvat.fr/personnages/";
    println!("🔍 Scraping : {}", url);

    // Téléchargement de la page
    let html = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&html);

    // Sélecteur principal des boîtes de personnages
    let selector_box = Selector::parse("a.entitybox").unwrap();

    // Sélecteur pour les noms
    let selector_name = Selector::parse("h5").unwrap();

    let mut results: Vec<CharacterTempo> = Vec::new();

    for box_elem in document.select(&selector_box) {
        // Récupération du lien
        let url = box_elem
            .value()
            .attr("href")
            .unwrap_or_default()
            .to_string();

        // Nom du personnage
        let name = box_elem
            .select(&selector_name)
            .next()
            .map(|n| n.text().collect::<Vec<_>>().join("").trim().to_string())
            .unwrap_or_default();

        results.push(CharacterTempo { name, url });
    }

    // Sauvegarde en JSON
    let json = serde_json::to_string_pretty(&results)?;
    fs::write("genshin_characters.json", &json)?;

    println!(
        "✅ {} personnages enregistrés dans genshin_characters.json",
        results.len()
    );

    Ok(())
}
