use serde::Deserialize;

use super::validator::{Validator, is_uuid};

#[derive(Deserialize)]
pub struct NestboxReq {
    pub uuid: String,

}

impl Validator for NestboxReq {
    fn is_valid(&self) -> bool {
        is_uuid(&self.uuid)
    }
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




