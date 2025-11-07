use std::io::Write;
include!("./characters/genshin/get_genshin_characters.rs");
include!("./keqing_mains/scrapper.rs");
include!("./struct_file.rs");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Définition des infos de scrapping
    let kgm_genshin: ScrappingInfos = {
        ScrappingInfos {
            alias: "Keqing Mains".to_string(),
            url: "https://keqingmains.com/".to_string()
        }
    };
    let kgm_hsr: ScrappingInfos ={
        ScrappingInfos {
            alias: "Keqing Mains HSR".to_string(),
            url: "https://hsr.keqingmains.com/".to_string()
        }
    };
    let mut gazette_de_teyvat: ScrappingInfos = ScrappingInfos {
        alias: "gazette_de_teyvat".to_string(),
        url: "https://keqingmains.com/".to_string()
    };

    // Scrapping des infographics

    // Quick Guide KGM
    scrape_infographics_kqm(kgm_genshin).await.expect("Une erreur est survenue lors du scrapping de Keqing mains");
    scrape_infographics_kqm(kgm_hsr).await.expect("Une erreur est survenue lors du scrapping de Keqing mains HSR");
    Ok(())
}
