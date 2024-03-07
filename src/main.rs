mod config;
mod converter;
mod parser;

use anyhow::{anyhow, Error};
use config::Config;
use converter::{convert_temperature, to_titlecase};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "WeathRS",
    about = "A lightweight command-line weather application, written in rust"
)]
struct Application {
    #[structopt(subcommand)]
    cmd: Option<SubCommand>,
}

#[derive(StructOpt)]
enum SubCommand {
    #[structopt(about = "Sets the api key in your config.json file")]
    Key {
        #[structopt(help = "Your OpenWeatherMap API key")]
        api_key: String,
    },
    #[structopt(about = "Sets the location name in your config.json file")]
    Location {
        #[structopt(help = "The location name you want to use")]
        location: String,
    },
    #[structopt(about = "Sets the temperature format in your config.json file")]
    TemperatureFormat {
        #[structopt(
            help = "The temperature format you want to use\nOptions: celsius, fahrenheit, kelvin\nDefault: celsius"
        )]
        temperature_format: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Application::from_args();
    let mut config = Config::read_or_default()?;

    match args.cmd {
        Some(SubCommand::Key { api_key }) => {
            config.set_api_key(api_key)?;
        }
        Some(SubCommand::Location { location }) => {
            config.set_location_name(location)?;
        }
        Some(SubCommand::TemperatureFormat { temperature_format }) => {
            config.set_temperature_format(temperature_format)?;
        }
        None => {
            query(&config).await?;
        }
    }

    Ok(())
}

async fn query(config: &Config) -> Result<(), Error> {
    if config.api_key.is_empty() {
        return Err(anyhow!("Set the api key with \"weathrs key $key\""));
    }

    if config.location_name.is_empty() {
        return Err(anyhow!(
            "Set the location id with \"weathrs location $locationid\""
        ));
    }

    let query_url: String = format!(
        "https://api.openweathermap.org/data/2.5/forecast?q={}&appid={}",
        config.location_name, config.api_key,
    );

    let weather_info = parser::parse_weather_info(&query_url).await?;

    let icon_code = weather_info.list[0].weather[0].icon.clone();
    let desc_raw = weather_info.list[0].weather[0].description.clone();
    let temp_kelvin = weather_info.list[0].main.temp.clone();
    let temp_format = config.get_temp_format();

    let icon = config.get_icon(&icon_code);
    let (temp, unit) = convert_temperature(temp_kelvin, temp_format);
    let description = to_titlecase(&desc_raw);

    println!("{icon}{temp}{unit} {description}");

    Ok(())
}
