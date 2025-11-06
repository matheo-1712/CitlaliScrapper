use futures::stream::{self, StreamExt};
use reqwest::Client;
use scraper::{Html, Selector};

pub async fn scrape_infographics(
    info: ScrappingInfos,
) -> Result<Vec<Option<(String, Vec<String>)>>, Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; Citlali/1.0; +https://citlapi.antredesloutres.fr)")
        .build()?;

    let concurrency = 10;

    let results = stream::iter(info.urls.clone())
        .map(|url| {
            let client = &client;
            let selectors = info.selector_str.clone();

            async move {
                match client.get(&url).send().await {
                    Ok(resp) => match resp.text().await {
                        Ok(text) => {
                            let document = Html::parse_document(&text);
                            let full_html = document.root_element().html();

                            // 👉 Essaie chaque couple de sélecteurs jusqu’à trouver un match
                            for pair in selectors.iter() {
                                let start_selector = match Selector::parse(&pair.start_selector_str) {
                                    Ok(s) => s,
                                    Err(_) => {
                                        eprintln!(
                                            "⚠️ Erreur de parsing du sélecteur start: {}",
                                            pair.start_selector_str
                                        );
                                        continue;
                                    }
                                };
                                let end_selector = match Selector::parse(&pair.end_selector_str) {
                                    Ok(s) => s,
                                    Err(_) => {
                                        eprintln!(
                                            "⚠️ Erreur de parsing du sélecteur end: {}",
                                            pair.end_selector_str
                                        );
                                        continue;
                                    }
                                };

                                let start_elem = document.select(&start_selector).next();
                                let end_elem = document.select(&end_selector).next();

                                if let (Some(start_elem), Some(end_elem)) = (start_elem, end_elem) {
                                    if let (Some(start_idx), Some(end_idx)) = (
                                        full_html.find(&start_elem.html()),
                                        full_html.find(&end_elem.html()),
                                    ) {
                                        if start_idx < end_idx {
                                            let section = &full_html[start_idx..end_idx];
                                            let url_selector = Selector::parse("img").unwrap();
                                            let fragment = Html::parse_fragment(section);
                                            let images: Vec<String> = fragment
                                                .select(&url_selector)
                                                .filter_map(|e| e.value().attr("src"))
                                                .map(|src| src.to_string())
                                                .collect();

                                            if !images.is_empty() {
                                                return Some((url.to_string(), images));
                                            }
                                        } else {
                                            eprintln!(
                                                "⚠️ Indices incohérents ({} >= {}) pour {} avec sélecteur {:?}",
                                                start_idx, end_idx, url, pair
                                            );
                                        }
                                    }
                                }
                            }

                            // Si aucun des couples n’a fonctionné :
                            eprintln!("⚠️ Aucun sélecteur valide trouvé pour {}", url);
                            None
                        }
                        Err(e) => {
                            eprintln!("⚠️ Erreur lecture corps {} : {}", url, e);
                            None
                        }
                    },
                    Err(e) => {
                        eprintln!("⚠️ Erreur requête {} : {}", url, e);
                        None
                    }
                }
            }
        })
        .buffer_unordered(concurrency)
        .collect::<Vec<_>>()
        .await;

    // Compte total d’images extraites
    let infographics_count: usize = results
        .iter()
        .filter_map(|r| r.as_ref())
        .map(|(_, imgs)| imgs.len())
        .sum();

    println!(
        "{} infographies récupérées pour {} en {:.2?} ⏱️",
        infographics_count,
        info.alias,
        start.elapsed()
    );

    Ok(results)
}
