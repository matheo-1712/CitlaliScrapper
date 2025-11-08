use dotenv::dotenv;

include!("./characters/genshin/get_genshin_characters.rs");
include!("./struct_file.rs");
include!("./infographics/keqing_mains/main_kgm.rs");
include!("./infographics/gazette_de_teyvat/main_gazette.rs");
include!("register.rs");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Charge le .env
    dotenv().ok();

    main_keqing_mains().await;
    main_gazette().await;

    // Scrapping des infographics
    Ok(())
}
