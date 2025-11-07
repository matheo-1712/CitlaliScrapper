use std::env;
use dotenv::dotenv;

include!("./characters/genshin/get_genshin_characters.rs");
include!("./keqing_mains/scrapper.rs");
include!("./struct_file.rs");
include!("./keqing_mains/main_kgm.rs");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Charge le .env
    dotenv().ok();

    // Variable pour le scrapping des infographics
    let mut gazette_de_teyvat: ScrappingInfos = ScrappingInfos {
        alias: "gazette_de_teyvat".to_string(),
        url: "https://keqingmains.com/".to_string()
    };

    main_keqing_mains().await;

    // Scrapping des infographics
    Ok(())
}
