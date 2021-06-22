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

#[derive(Deserialize)]
pub struct GeolocationReq {
    pub long: f32,
    pub lat: f32
}


