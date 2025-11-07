use std::env;

// Envoie des infographies vers l'API
pub async fn register_infographics(
    infographic: &Infographic,
    jeu: Box<str>,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let api_token = env::var("API_TOKEN")?;

    let payload = InfographicPayload {
        url: &infographic.url,
        build: &infographic.build,
        formatedValue: &infographic.character,
        source: &infographic.source,
    };

    let api_url = format!(
        "https://citlapi.antredesloutres.fr/api/infographics/{}/new",
        jeu
    );

    let resp = client
        .post(api_url)
        .bearer_auth(api_token)
        .json(&payload)
        .send()
        .await?;

    if !resp.status().is_success() {
        eprintln!("❌ Erreur API {} : {:?}", resp.status(), resp.text().await?);
    }

    Ok(())
}
