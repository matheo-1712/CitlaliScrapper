use futures::stream::{self, StreamExt};
use reqwest::{Client, header::CONTENT_TYPE, Url};
use std::error::Error;
use std::fs::OpenOptions;
use scraper::{Html, Selector};

async fn scrap_redirect_urls(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let start = std::time::Instant::now();
    // 🔹 URL de la page principale contenant toutes les infographies
    let base_url = url;

    // 🔹 Création du client HTTP
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; Citlali/3.0; +https://citlapi.antredesloutres.fr/)")
        .build()?;

    // 🔹 Télécharge la page HTML
    let resp = client.get(base_url).send().await?;
    let body = resp.text().await?;
    let document = Html::parse_document(&body);

    // 🔹 Sélecteurs pour extraire les cartes et leurs liens
    let card_selector = Selector::parse("div.card.character-card").unwrap();
    let link_selector = Selector::parse("a").unwrap();

    let mut infographic_urls = Vec::new();

    // 🔹 Parcours des cartes de personnages
    for card in document.select(&card_selector) {
        // Vérifie que la carte correspond à une infographie
        if let Some(category) = card.value().attr("data-category") {
            if category.contains("Infographic") {
                for a_tag in card.select(&link_selector) {
                    if let Some(href) = a_tag.value().attr("href") {
                        // On ne garde que les liens du type /i/
                        if href.contains("/i/") && !infographic_urls.contains(&href.to_string()) {
                            infographic_urls.push(href.to_string());
                        }
                    }
                }
            }
        }
    }


    println!("✅ {} URLs récupérées en {:.2?}", infographic_urls.len(), start.elapsed());
    Ok(infographic_urls)
}

pub async fn scrape_infographics_kqm(info: ScrappingInfos) -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let urls = scrap_redirect_urls(info.url.as_str());

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; Citlali/3.0; +https://citlapi.antredesloutres.fr/)")
        .build()?;

    let concurrency = 10;
    let meta_refresh_count = std::sync::atomic::AtomicUsize::new(0);

    let fetches = stream::iter(urls.await?.into_iter().map(|url| {
        let client = client.clone();
        let counter = &meta_refresh_count;

        async move {
            let resp = match client.get(&url).send().await {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("⚠️  Erreur requête {} : {}", url, e);
                    return Ok(());
                }
            };

            let final_url = resp.url().clone();

            let body = match resp.text().await {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("⚠️  Erreur lecture corps {} : {}", url, e);
                    return Ok(());
                }
            };

            if let Some(meta_refresh_url) = extract_meta_refresh(&body, &final_url) {
                // println!("🔄 Meta refresh trouvé : {}", meta_refresh_url);
                extract_and_register_infographic(meta_refresh_url.as_str());
                counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                return Ok(());
            }

            println!("⚠️  Pas de meta refresh trouvé sur {}", url);

            Ok::<(), Box<dyn Error>>(())
        }
    }))
        .buffer_unordered(concurrency);

    fetches
        .for_each(|res| async {
            if let Err(e) = res {
                eprintln!("❌ Erreur : {}", e);
            }
        })
        .await;

    println!("✅ Traitement terminé en {:.2?}, Nombre d'infographies trouvées : {}", start.elapsed(), meta_refresh_count.load(std::sync::atomic::Ordering::Relaxed));

    Ok(())
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

struct Infographic {
    url: String,
    build: String,
    character: String,
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

