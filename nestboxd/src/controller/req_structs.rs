use serde::Deserialize;
#[derive(Deserialize)]
pub struct NestboxReq {
    pub uuid: String,
}


#[derive(Deserialize)]
pub struct BirdReq {
    pub bird_uuid: String,
    pub bird: String
}
