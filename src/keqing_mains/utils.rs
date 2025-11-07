// Utility functions for Keqing Mains application

fn register_infographics(infographic: &Infographic) {
    // Ouvre le fichier en mode append (ajoute à la fin) ou crée s'il n'existe pas
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("infographics.txt")
        .expect("Impossible d'ouvrir/créer le fichier infographics.txt");

    // Écrit les informations séparées par des espaces et retourne à la ligne
    writeln!(
        file,
        "{} {} {}",
        infographic.url, infographic.build, infographic.character
    )
        .expect("Impossible d'écrire dans le fichier");
}

fn extract_and_register_infographic(combined_url: &str) {
    // On suppose que la chaîne est du type "page_url'image_url'"
    let parts: Vec<&str> = combined_url.split('\'').collect();

    if parts.len() >= 2 {
        let page_url = parts[0].trim();
        let image_url = parts[1].trim();

        let character_name = extract_character_name(page_url);
        let character_build = extract_character_build(page_url);

        let infographic = Infographic {
            url: image_url.to_string(),
            build: character_build,
            character: character_name,
        };

        register_infographics(&infographic)

    } else {
        println!("⚠️ Impossible de séparer les URLs : {}", combined_url);
    }
}

fn extract_character_details(url: &str) -> String {
    if let Ok(parsed_url) = Url::parse(url) {
        if let Some(segments) = parsed_url.path_segments() {
            // On prend le dernier segment non vide
            for segment in segments.rev() {
                if !segment.is_empty() && segment != "i" {
                    return segment.to_string();
                }
            }
        }
    }
    "unknown".to_string()
}

fn extract_character_name(url: &str) -> String {
    let character_name = extract_character_details(url);
    if let Some(pos) = character_name.find('-') {
        // Tout ce qui est avant le tiret
        character_name[..pos].to_string()
    } else {
        character_name
    }
}

fn extract_character_build(url: &str) -> String {
    let character_name = extract_character_details(url);
    if let Some(pos) = character_name.find('-') {
        // Tout ce qui est après le tiret
        character_name[pos + 1..].to_string()
    } else {
        "classique".to_string()
    }
}