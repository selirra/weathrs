use crate::converter::TemperatureFormat;
use anyhow::{anyhow, Error};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{collections::HashMap, fs};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub location_name: String,
    pub temperature_format: String,
    pub condition_icons: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        let mut condition_icon_map: HashMap<String, String> = HashMap::new();
        condition_icon_map.insert("01d".to_string(), "󰖙 ".to_string()); // Clear Sky -- day
        condition_icon_map.insert("01n".to_string(), "󰖔 ".to_string()); // Clear Sky -- night
        condition_icon_map.insert("02d".to_string(), "󰖕 ".to_string()); // Few Clouds -- day
        condition_icon_map.insert("02n".to_string(), "󰼱 ".to_string()); // Few Clouds -- night
        condition_icon_map.insert("03d".to_string(), " ".to_string()); // Scattered Clouds -- day
        condition_icon_map.insert("03n".to_string(), " ".to_string()); // Scattered Clouds -- night
        condition_icon_map.insert("04d".to_string(), " ".to_string()); // Broken Clouds -- day
        condition_icon_map.insert("04n".to_string(), " ".to_string()); // Broken Clouds -- night
        condition_icon_map.insert("09d".to_string(), "󰖗 ".to_string()); // Shower Rain -- day
        condition_icon_map.insert("09n".to_string(), "󰖗 ".to_string()); // Shower Rain -- night
        condition_icon_map.insert("10d".to_string(), "󰖖 ".to_string()); // Rain -- day
        condition_icon_map.insert("10n".to_string(), "󰖖 ".to_string()); // Rain -- night
        condition_icon_map.insert("11d".to_string(), "󰙾 ".to_string()); // Thunderstorm -- day
        condition_icon_map.insert("11n".to_string(), "󰙾 ".to_string()); // Thunderstorm -- night
        condition_icon_map.insert("13d".to_string(), "󰖘 ".to_string()); // Snowy -- day
        condition_icon_map.insert("13n".to_string(), "󰖘 ".to_string()); // Snowy -- night
        condition_icon_map.insert("50d".to_string(), "󰖑 ".to_string()); // Mist -- day
        condition_icon_map.insert("50n".to_string(), "󰖑 ".to_string()); // Mist -- night

        Self {
            api_key: String::new(),
            location_name: String::new(),
            temperature_format: "celsius".to_string(),
            condition_icons: condition_icon_map,
        }
    }
}

impl Config {
    pub fn get_icon(&self, code: &str) -> String {
        self.condition_icons
            .get(code)
            .unwrap_or(&String::new())
            .to_string()
    }

    pub fn set_api_key(&mut self, api_key: String) -> Result<(), Error> {
        self.api_key = api_key;
        self.write()?;
        Ok(())
    }

    pub fn set_location_name(&mut self, location_name: String) -> Result<(), Error> {
        self.location_name = location_name;
        self.write()?;
        Ok(())
    }

    pub fn set_temperature_format(&mut self, temperature_format: String) -> Result<(), Error> {
        self.temperature_format = temperature_format;
        self.write()?;
        Ok(())
    }

    pub fn get_temp_format(&self) -> TemperatureFormat {
        match self.temperature_format.as_str() {
            "celsius" => TemperatureFormat::Celsius,
            "fahrenheit" => TemperatureFormat::Fahrenheit,
            "kelvin" => TemperatureFormat::Kelvin,
            _ => TemperatureFormat::Celsius,
        }
    }

    pub fn read_or_default() -> Result<Self, Error> {
        let path = Config::config_path()?;
        if path.exists() {
            Config::read(&path)
        } else {
            let default = Config::default();
            default.write()?;
            Ok(default)
        }
    }

    pub fn read(path: &std::path::PathBuf) -> Result<Self, Error> {
        let file_content =
            fs::read_to_string(&path).map_err(|_| anyhow!("Failed to read config"))?;
        let config: Self =
            serde_json::from_str(&file_content).map_err(|_| anyhow!("Failed to parse config"))?;
        Ok(config)
    }

    pub fn write(&self) -> Result<(), Error> {
        let path = Config::config_path()?;
        let serialized_config = serde_json::to_string_pretty(self)
            .map_err(|_| anyhow!("Failed to serialize config"))?;
        fs::write(&path, serialized_config).map_err(|_| anyhow!("Failed to write file"))?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf, Error> {
        let mut path = config_dir().ok_or_else(|| anyhow!("Failed to find config dir"))?;
        path.push("weathrs");
        fs::create_dir_all(&path).map_err(|_| anyhow!("Failed to create config directory"))?;
        path.push("config.json");
        Ok(path)
    }
}
