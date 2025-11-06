use std::time::Instant;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Character {
    id: u32,
    formatedValue:String,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    data: Vec<Character>,
}
async fn get_genshin_characters<T>() -> Result<Vec<Character>, Box<dyn std::error::Error>> {
    let start = Instant::now();

    // Crée un client HTTP réutilisable 
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; FastScraper/1.0; +https://example.com)")
        .build()?;
    
    // Obtenir une liste de personnages via une api
    let client_api = Client::new();
    let url = "https://citlapi.antredesloutres.fr/api/genshin/characters";

    // Récupération du JSON et parsing de l'objet global
    let resp: ApiResponse = client_api
        .get(url)
        .send()
        .await?
        .json::<ApiResponse>()
        .await?;
    
    println!("Personnages récupéré en {:.2?} ", start.elapsed());
    Ok(resp.data)
}