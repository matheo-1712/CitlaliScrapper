include!("scrapper.rs");
include!("utils.rs");
async fn main_gazette() {
    scrape_all_characters()
        .await
        .expect("Une erreur est survenue lors du scrapping de Gazette de Teyvat");
}
