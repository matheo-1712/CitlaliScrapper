use std::io::Write;
include!("./scrapper.rs");
include!("./characters/genshin/get_genshin_characters.rs");

// Struct pour stocker les informations de scrapping
pub struct ScrappingInfos {
    alias: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Définition des infos de scrapping
    let kgm_genshin: ScrappingInfos = {
        ScrappingInfos {
            alias: "Keqing Mains".to_string(),
            url: "https://keqingmains.com/".to_string()
        }
    };
    let mut gazette_de_teyvat: ScrappingInfos = ScrappingInfos {
        alias: "gazette_de_teyvat".to_string(),
        url: "https://keqingmains.com/".to_string()
    };

    // Scrapping des infographics

    // Quick Guide KGM
    scrape_infographics_kqm(kgm_genshin).await.expect("Une erreur est survenue lors du scrapping de Keqing mains");
    Ok(())
}
