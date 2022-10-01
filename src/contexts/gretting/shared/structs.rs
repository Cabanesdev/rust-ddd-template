use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CityRequest {
    pub zip_code: i32,
    pub city: String,
}

#[derive(Serialize)]
pub struct CityResponse {
    pub zip_code: i32,
    pub city: String,
    pub gretting: String,
}
