use reqwest::Url;

// Utility functions for Keqing Mains application
pub async fn extract_and_register_infographic(combined_url: &str, alias: &str, jeu: &str) {
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
            source: alias.to_string(),
        };

        register_infographics(&infographic, Box::from(jeu))
            .await
            .expect("TODO: panic message");
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

// Petit helper pour détecter les <meta http-equiv="refresh" content="0;url=...">
fn extract_meta_refresh(body: &str, base_url: &Url) -> Option<Url> {
    // Cherche "http-equiv" sans tenir compte de la casse
    if let Some(idx) = body.to_lowercase().find("http-equiv=\"refresh\"") {
        // On utilise la slice originale, pas la lowercase
        let slice = &body[idx..];
        if let Some(content_pos) = slice.to_lowercase().find("content=\"") {
            let sub = &slice[content_pos + 9..];
            if let Some(url_start) = sub.to_lowercase().find("url=") {
                let sub_url = &sub[url_start + 4..];
                // L’URL va jusqu’au prochain guillemet
                let end = sub_url.find('"').unwrap_or(sub_url.len());
                let url_candidate = &sub_url[..end].trim();

                if let Ok(u) = base_url.join(url_candidate) {
                    return Some(u);
                }
            }
        }
    }
    None
}
