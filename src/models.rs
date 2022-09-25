use super::schema::geolocations;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Clone, Deserialize)]
pub struct Geolocation {
    pub id: i32,
    pub description: Option<String>,
    pub lat: f64,
    pub lon: f64,
    pub url: String,
    pub uuid: Vec<u8>,
    pub notes: Option<String>,
    pub geolocator: Option<String>,
    pub datetime: Option<NaiveDateTime>,
    pub provisional: Option<bool>,
    #[serde(skip)]
    pub cookie: Option<String>,
}

#[derive(Insertable, Clone, Serialize, Deserialize)]
#[table_name = "geolocations"]
pub struct NewGeolocation {
    pub description: Option<String>,
    pub lat: f64,
    pub lon: f64,
    pub url: String,
    pub uuid: Option<Vec<u8>>,
    pub notes: Option<String>,
    pub geolocator: Option<String>,
    pub datetime: Option<NaiveDateTime>,
    pub provisional: Option<bool>,
    #[serde(skip)]
    pub cookie: Option<String>,
}
