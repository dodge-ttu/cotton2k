use crate::de::{from_isoformat, from_isoformat_option};
use serde::Deserialize;
use chrono::NaiveDate;
use std::path::PathBuf;

#[inline]
fn zero() -> f64 {
    0.
}

#[derive(Deserialize, Debug)]
pub struct Profile {
    pub name: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
    #[serde(deserialize_with = "from_isoformat")]
    pub start_date: NaiveDate,
    #[serde(deserialize_with = "from_isoformat")]
    pub stop_date: NaiveDate,
    #[serde(default)]
    #[serde(deserialize_with = "from_isoformat_option")]
    pub emerge_date: Option<NaiveDate>,
    #[serde(default)]
    #[serde(deserialize_with = "from_isoformat_option")]
    pub plant_date: Option<NaiveDate>,
    pub co2_enrichment: Option<CO2Enrichment>,
    pub mulch: Option<Mulch>,
    pub weather_path: PathBuf,
    pub site: Site,
    pub cultivar_parameters: Vec<f64>,
    pub row_space: f64,
    #[serde(default = "zero")]
    pub skip_row_width: f64,
    pub plants_per_meter: f64,
}

#[derive(Deserialize, Debug)]
pub struct CO2Enrichment {
    pub factor: f64,
    #[serde(deserialize_with = "from_isoformat")]
    pub start_date: NaiveDate,
    #[serde(deserialize_with = "from_isoformat")]
    pub stop_date: NaiveDate,
}

#[derive(Deserialize, Debug)]
pub enum MulchType {
    NoMulch,
    All,                  // plastic layer on all soil surface
    OneColumnLeftAtSide, // plastic layer on all soil surface except one column at each side of the plant row.
    TwoColumnsLeftAtSide, // plastic layer on all soil surface except two columns at each side of the plant row.
}

#[derive(Deserialize, Debug)]
pub struct Mulch {
    pub indicator: MulchType,
    pub sw_trans: f64,
    pub lw_trans: f64,
    #[serde(deserialize_with = "from_isoformat")]
    pub start_date: NaiveDate,
    #[serde(default)]
    #[serde(deserialize_with = "from_isoformat_option")]
    pub stop_date: Option<NaiveDate>,
}

#[derive(Deserialize, Debug)]
pub struct Site {
    pub average_wind_speed: Option<f64>,
    pub estimate_dew_point: (f64, f64),
    pub wind_blow_after_sunrise: f64,
    pub wind_max_after_noon: f64,
    pub wind_stop_after_sunset: f64,
    pub night_time_wind_factor: f64,
    pub cloud_type_correction_factor: f64,
    pub max_temperature_after_noon: f64,
    pub deep_soil_temperature: (f64, f64, f64),
    pub dew_point_range: (f64, f64, f64),
    pub albedo_range: (f64, f64),
}

#[derive(Deserialize, Debug)]
pub struct WeatherRecord {
    #[serde(deserialize_with = "from_isoformat")]
    pub date: NaiveDate,
    pub irradiation: f64,
    pub tmax: f64,
    pub tmin: f64,
    pub rain: f64,
    pub wind: Option<f64>,
    pub tdew: Option<f64>,
}
