include!("scrapper.rs");
async fn main_gazette() {
    scrape_build_url()
        .await
        .expect("Une erreur est survenue lors du scrapping de Gazette de Teyvat");
}
