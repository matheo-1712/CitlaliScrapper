include!("./scrapper.rs");
include!("./characters/genshin/get_genshin_characters.rs");

// Struct pour stocker les informations de scrapping
pub struct ScrappingInfos {
    alias: String,
    urls: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let characters = get_genshin_characters::<()>().await?;

    // Définition des infos de scrapping
    let mut quick_guide_kgm: ScrappingInfos = {
        ScrappingInfos {
            alias: "quick_guide_kgm".to_string(),
            urls: Vec::new(),
        }
    };
    let mut extended_guide_kgm: ScrappingInfos = {
        ScrappingInfos {
            alias: "extended_guide_kgm".to_string(),
            urls: Vec::new(),
        }
    };
    let mut gazette_de_teyvat: ScrappingInfos = ScrappingInfos {
        alias: "gazette_de_teyvat".to_string(),
        urls: Vec::new(),
    };


    for character in characters {
        quick_guide_kgm.urls.push(format!("https://keqingmains.com/i/{}/", character.formatedValue));
        extended_guide_kgm.urls.push(format!("https://keqingmains.com/{}/", character.formatedValue));
        gazette_de_teyvat.urls.push(format!("https://lagazettedeteyvat.fr/personnages/{}/", character.formatedValue));
    }

    // Scrapping des infographics

    // Quick Guide KGM
    scrape_infographics_kqm(quick_guide_kgm).await.expect("TODO: panic message");
    // Extended Guide KGM
    // scrape_infographics(extended_guide_kgm).await.expect("TODO: panic message");
    // Gazette de Teyvat
    // scrape_infographics(gazette_de_teyvat).await.expect("TODO: panic message");

    Ok(())
}
