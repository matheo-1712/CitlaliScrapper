#[derive(Deserialize, Debug)]
struct ApiResponseUidInfos {
    data: Vec<UidInfos>,
}

async fn get_uid_infos() -> Result<Vec<UidInfos>, Box<dyn std::error::Error>> {
    let start = Instant::now();

    let client_api = Client::new();
    let url = format!("{}/api/uid-infos", env::var("API_URL")?);

    // Récupération du JSON avec gestion d'erreur
    let resp = match client_api.get(&url).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Erreur lors de la requête HTTP : {}", e);
            return Ok(vec![]); // stop la fonction, retourne un vecteur vide
        }
    };

    // Parsing JSON avec gestion d'erreur
    let api_data: ApiResponseUidInfos = match resp.json::<ApiResponseUidInfos>().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Erreur lors de la désérialisation JSON : {}", e);
            return Ok(vec![]); // stop la fonction, retourne un vecteur vide
        }
    };

    println!("Uid Infos récupéré en {:.2?}", start.elapsed());

    Ok(api_data.data)
}
