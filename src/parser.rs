use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseRoot {
    pub cod: String,
    pub message: u32,
    pub cnt: u32,
    pub list: Vec<ListElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListElement {
    pub dt: u32,
    pub main: WeatherMain,
    pub weather: Vec<WeatherInfo>,
    pub clouds: CloudInfo,
    pub wind: WindInfo,
    pub visibility: u32,
    pub pop: f32,
    pub rain: Option<RainInfo>,
    pub sys: SysInfo,
    pub dt_txt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherMain {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: u16,
    pub sea_level: u16,
    pub grnd_level: u16,
    pub humidity: u16,
    pub temp_kf: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherInfo {
    pub id: u16,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudInfo {
    pub all: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindInfo {
    pub speed: f32,
    pub deg: u16,
    pub gust: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RainInfo {
    #[serde(rename = "3h")]
    pub three_hour: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SysInfo {
    pub pod: String,
}

pub async fn parse_weather_info(url: &str) -> Result<ResponseRoot, Error> {
    let response = reqwest::get(url)
        .await
        .map_err(|_| anyhow!("Connection error"))?;
    let response_text = response
        .text()
        .await
        .map_err(|_| anyhow!("Parsing error"))?;
    let response_object =
        serde_json::from_str(&response_text).map_err(|_| anyhow!("Serialization error"))?;
    Ok(response_object)
}
