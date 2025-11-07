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

    main_keqing_mains().await;

    // Scrapping des infographics
    Ok(())
}
