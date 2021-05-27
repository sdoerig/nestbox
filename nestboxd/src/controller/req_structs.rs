use serde::Deserialize;
#[derive(Deserialize)]
pub struct NestboxReq {
    pub uuid: String,
}


#[derive(Deserialize)]
pub struct BirdReq {
    pub uuid: String,
    pub bird: String
}
