// On vérifie les variables d'environnement
fn check_env() -> Result<(), Box<dyn Error>> {
    let required_vars = ["API_TOKEN", "API_URL"];
    let mut missing_vars = Vec::new();

    for &var in required_vars.iter() {
        match env::var(var) {
            Ok(val) if !val.is_empty() => (), // tout va bien
            _ => missing_vars.push(var),
        }
    }

    if !missing_vars.is_empty() {
        return Err(format!(
            "Les variables d'environnement suivantes sont manquantes ou vides : {}",
            missing_vars.join(", ")
        )
            .into());
    }

    Ok(())
}
