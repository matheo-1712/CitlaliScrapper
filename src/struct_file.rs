use serde::Serialize;

// Struct pour stocker les informations de scrapping
pub struct ScrappingInfos {
    alias: String,
    url: String,
}

pub struct Infographic {
    url: String,
    build: String,
    character: String,
    source: String,
}
#[derive(Serialize)]
pub struct InfographicPayload<'a> {
    url: &'a str,
    build: &'a str,
    formatedValue: &'a str,
    source: &'a str,
}