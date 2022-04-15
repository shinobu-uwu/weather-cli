#[macro_use]
extern crate structopt;

use crate::cli::Cli;
use crate::config::{Config, Units};
use crate::response::Response;
use reqwest::Client;
use structopt::StructOpt;

const APP_NAME: &str = "weather";
const CONFIG_NAME: &str = "config";
const BASE_URL: &str = "http://api.openweathermap.org/data/2.5/weather";

mod cli;
mod response;
mod config;

#[tokio::main]
async fn main() {
    let mut config: Config = confy::load(APP_NAME, CONFIG_NAME).unwrap();
    let args: Cli = Cli::from_args();

    if args.toggle_units {
        toggle_units(&mut config).unwrap();
    }

    get_weather(args, config).await;
}

async fn get_weather(args: Cli, config: Config) {
    let location = match args.location {
        Some(location) => location,
        None => config.location
    };

    let client = Client::new();

    let response = client
        .get(BASE_URL)
        .query(&[("q", location)])
        .query(&[("appid", &config.api_key)])
        .query(&[("units", &config.units)])
        .send()
        .await
        .unwrap()
        .json::<Response>()
        .await
        .unwrap();
    let output = response.main.temp;

    let sufix = match config.units {
        Units::Metric => "°C",
        Units::Imperial => "°F"
    };

    println!("{output:.0}{sufix}")
}

fn toggle_units(config: &mut Config) -> Result<(), confy::ConfyError> {
    match &config.units {
        Units::Metric => config.units = Units::Imperial,
        Units::Imperial => config.units = Units::Metric,
    }

    confy::store(APP_NAME, CONFIG_NAME, &config)
}
