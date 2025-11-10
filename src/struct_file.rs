use serde::Serialize;

// Struct pour stocker les informations de scrapping
pub struct ScrappingInfos {
    alias: String,
    url: String,
    jeu: String,   
}

pub struct Infographic {
    url: String,
    build: String,
    character: String,
    source: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct InfographicPayload<'a> {
    url: &'a str,
    build: &'a str,
    formatedValue: &'a str,
    source: &'a str,
}

// Struct pour stocker les informations de l'API
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct UidInfos {
    id: u32,
    uid: String,
    nickname: String,
    worldLevel: u16,
    signature: String,
    finishAchievementNum: u32,
    towerFloor: String,
    affinityCount: u32,
    theaterAct: u32,
    theaterMode: String,
    playerIcon: String,
    stygianIndex: u32,
    stygianSeconds: u32,
}
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Character {
    id: u32,
    formatedValue:String,
}
