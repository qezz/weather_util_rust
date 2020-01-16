use chrono::{FixedOffset, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherCond {
    pub main: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherMain {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: f64,
    pub humidity: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Wind {
    pub speed: f64,
    pub deg: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sys {
    pub country: Option<String>,
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WeatherData {
    pub coord: Coord,
    pub weather: Vec<WeatherCond>,
    pub base: String,
    pub main: WeatherMain,
    pub visibility: Option<f64>,
    pub wind: Wind,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i32,
    pub name: String,
}

impl WeatherData {
    pub fn get_current_conditions(&self) -> String {
        let fo = FixedOffset::east(self.timezone);
        let dt = fo.timestamp(self.dt, 0);
        let sunrise = fo.timestamp(self.sys.sunrise, 0);
        let sunset = fo.timestamp(self.sys.sunset, 0);
        format!(
            "Current conditions {} {}\n{}\n{}\n{}\n{}\n{}",
            if let Some(country) = &self.sys.country {
                format!("{} {}", self.name, country)
            } else {
                "".to_string()
            },
            format!("{}N {}E", self.coord.lat, self.coord.lon),
            format!("Last Updated {}", dt,),
            format!(
                "\tTemperature: {:0.2} F ({:0.2} C)",
                fahr(self.main.temp),
                celc(self.main.temp),
            ),
            format!("\tRelative Humidity: {}%", self.main.humidity),
            format!(
                "\tWind: {} degrees at {:0.2} mph",
                self.wind.deg,
                (self.wind.speed * 3600. / 1609.344)
            ),
            format!("\tConditions: {}", self.weather[0].description),
        )
    }
}

fn celc(temp: f64) -> f64 {
    temp - 273.15
}

fn fahr(temp: f64) -> f64 {
    temp * 1.8 - 459.67
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ForecastMain {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i64,
    pub sea_level: i64,
    pub grnd_level: i64,
    pub humidity: i64,
    pub temp_kf: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ForecastEntry {
    pub dt: i64,
    pub main: ForecastMain,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CityEntry {
    pub name: String,
    pub coord: Coord,
    pub country: String,
    pub timezone: i32,
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WeatherForecast {
    pub list: Vec<ForecastEntry>,
    pub city: CityEntry,
}

impl ForecastEntry {
    pub fn get_forecast_entry(&self, timezone: i32) -> String {
        let fo = FixedOffset::east(timezone);
        let dt = fo.timestamp(self.dt, 0);
        format!(
            "Forecast: {}, {:0.2} F / {:0.2} C, max: {:0.2} F / {:0.2} C, min: {:0.2} F / {:0.2} C ",
            dt,
            fahr(self.main.temp),
            celc(self.main.temp),
            fahr(self.main.temp_max),
            celc(self.main.temp_max),
            fahr(self.main.temp_min),
            celc(self.main.temp_min),
        )
    }
}