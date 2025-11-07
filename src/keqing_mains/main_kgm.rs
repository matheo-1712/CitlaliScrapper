async fn main_keqing_mains() {

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

    // Appel des fonctions de scrapping
    scrape_infographics_kqm(kgm_genshin).await.expect("Une erreur est survenue lors du scrapping de Keqing mains");
    scrape_infographics_kqm(kgm_hsr).await.expect("Une erreur est survenue lors du scrapping de Keqing mains HSR");

}