include!("./scrapper.rs");
include!("./characters/genshin/get_genshin_characters.rs");

// Struct pour stocker les informations de scrapping
struct ScrappingInfos{
    alias: String,
    urls: Vec<String>,
    selector_str: Vec<ScrapSelectors>,
}

#[derive(Clone)]
#[derive(Debug)]
struct ScrapSelectors{
    start_selector_str: String,
    end_selector_str: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let characters = get_genshin_characters::<()>().await?;

    // Définition des infos de scrapping
    let mut quick_guide_kgm: ScrappingInfos = {
        ScrappingInfos {
            alias: "quick_guide_kgm".to_string(),
            urls: Vec::new(),
            selector_str: vec![
                ScrapSelectors {
                    start_selector_str: "span.ez-toc-section[id='Infographic']".to_string(),
                    end_selector_str: "span.ez-toc-section[id='Character_Overview']".to_string()
                },
                ScrapSelectors {
                    start_selector_str: "span.ez-toc-section[id='Infographics']".to_string(),
                    end_selector_str: "span.ez-toc-section[id='Character_Overview']".to_string()
                }
            ],
        }
    };
    let mut extended_guide_kgm: ScrappingInfos = {
        ScrappingInfos {
            alias: "extended_guide_kgm".to_string(),
            urls: Vec::new(),
            selector_str: vec![
                ScrapSelectors {
                    start_selector_str: "span.ez-toc-section[id='TLDR']".to_string(),
                    end_selector_str: "span.ez-toc-section[id='Glossary']".to_string()
                }
            ],
        }
    };

    for character in characters {
        quick_guide_kgm.urls.push(format!("https://keqingmains.com/q/{}-quickguide/", character.formatedValue));
        extended_guide_kgm.urls.push(format!("https://keqingmains.com/{}/", character.formatedValue));
    }

    // Scrapping des infographics

    // Quick Guide KGM
    scrape_infographics(quick_guide_kgm).await.expect("TODO: panic message");
    // Extended Guide KGM
    scrape_infographics(extended_guide_kgm).await.expect("TODO: panic message");

    Ok(())
}
