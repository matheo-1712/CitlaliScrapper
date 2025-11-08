use futures::stream::{self, StreamExt};
use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;

async fn scrap_redirect_urls(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let start = std::time::Instant::now();

    // URL de la page principale contenant toutes les infographies
    let base_url = url;

    // Création du client HTTP
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; Citlali/3.0; +https://citlapi.antredesloutres.fr/)")
        .build()?;

    // Télécharge la page HTML
    let resp = client.get(base_url).send().await?;
    let body = resp.text().await?;
    let document = Html::parse_document(&body);

    // Sélecteurs pour extraire les cartes et leurs liens
    let card_selector = Selector::parse("div.card.character-card").unwrap();
    let link_selector = Selector::parse("a").unwrap();

    // Liste des URLs des infographies
    let mut infographic_urls = Vec::new();

    // Parcours des cartes de personnages
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
    println!(
        "✅ {} URLs récupérées en {:.2?}",
        infographic_urls.len(),
        start.elapsed()
    );
    Ok(infographic_urls)
}

pub async fn scrape_infographics_kqm(info: ScrappingInfos) -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let urls = scrap_redirect_urls(info.url.as_str());
    let alias: &str = info.alias.as_str();
    let jeu: &str = info.jeu.as_str();

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
                extract_and_register_infographic(meta_refresh_url.as_str(), &*alias, jeu).await;
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
    println!(
        "✅ Traitement terminé en {:.2?}, Nombre d'infographies trouvées : {}",
        start.elapsed(),
        meta_refresh_count.load(std::sync::atomic::Ordering::Relaxed)
    );

    Ok(())
}
