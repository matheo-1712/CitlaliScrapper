use std::fmt::format;

include!("get_uid_infos.rs");
pub async fn player_icon_update() {
    // Récupération de tous les UidInfos
    match get_uid_infos().await {
        Ok(uid_infos) => {
            if uid_infos.is_empty() {
                println!("Aucun UidInfos récupéré !");
            } else {
                for info in uid_infos.iter() {
                    match get_player_icon(&info.uid).await {
                        Ok(icon_url) => println!("{} -> {}", info.nickname, icon_url),
                        Err(e) => eprintln!("Erreur pour {} : {}", info.nickname, e),
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Erreur lors de la récupération des UidInfos : {}", e);
        }
    }
}

async fn get_player_icon(uid: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("https://akasha.cv/profile/{}", uid);
    println!("{}", url);

    // Requête HTTP
    let resp = client.get(&url).send().await?.text().await?;
    println!("{}", resp);

    // Parse le HTML
    let document = Html::parse_document(&resp);
    let selector = Selector::parse("img.profile-picture").unwrap();

    // Cherche l'image
    if let Some(img) = document.select(&selector).next() {
        if let Some(src) = img.value().attr("src") {
            Ok(src.to_string()) // retourne l'URL complète
        } else {
            Err("L'attribut src de l'image n'a pas été trouvé".into())
        }
    } else {
        Err("Image de profil introuvable dans le HTML".into())
    }
}
