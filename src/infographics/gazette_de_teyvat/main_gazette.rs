include!("scrapper.rs");
include!("utils.rs");

async fn main_gazette() {
    let gazette_genshin: ScrappingInfos = {
        ScrappingInfos {
            alias: "Gazette de Teyvat".to_string(),
            url: "https://lagazettedeteyvat.fr/personnages/".to_string(),
            jeu: "genshin".to_string(),
        }
    };

    scrape_all_characters(gazette_genshin)
        .await
        .expect("Une erreur est survenue lors du scrapping de Gazette de Teyvat");
}
