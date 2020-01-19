use anyhow::Error;
use chrono::{DateTime, FixedOffset, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::Write;

use crate::temperature::Temperature;
use crate::timestamp;

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
    pub temp: Temperature,
    pub feels_like: Temperature,
    pub temp_min: Temperature,
    pub temp_max: Temperature,
    pub pressure: f64,
    pub humidity: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Wind {
    pub speed: f64,
    pub deg: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sys {
    pub country: Option<String>,
    #[serde(with = "timestamp")]
    pub sunrise: DateTime<Utc>,
    #[serde(with = "timestamp")]
    pub sunset: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WeatherData {
    pub coord: Coord,
    pub weather: Vec<WeatherCond>,
    pub base: String,
    pub main: WeatherMain,
    pub visibility: Option<f64>,
    pub wind: Wind,
    #[serde(with = "timestamp")]
    pub dt: DateTime<Utc>,
    pub sys: Sys,
    pub timezone: i32,
    pub name: String,
}

impl WeatherData {
    pub fn get_current_conditions<T: Write>(&self, buf: &mut T) -> Result<(), Error> {
        let fo = FixedOffset::east(self.timezone);
        let dt = self.dt.with_timezone(&fo);
        let sunrise = self.sys.sunrise.with_timezone(&fo);
        let sunset = self.sys.sunset.with_timezone(&fo);
        writeln!(
            buf,
            "Current conditions {} {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            if let Some(country) = &self.sys.country {
                format!("{} {}", self.name, country)
            } else {
                "".to_string()
            },
            format!("{}N {}E", self.coord.lat, self.coord.lon),
            format!("Last Updated {}", dt,),
            format!(
                "\tTemperature: {:0.2} F ({:0.2} C)",
                self.main.temp.fahr(),
                self.main.temp.celc(),
            ),
            format!("\tRelative Humidity: {}%", self.main.humidity),
            format!(
                "\tWind: {} degrees at {:0.2} mph",
                self.wind.deg.unwrap_or(0.0),
                (self.wind.speed * 3600. / 1609.344)
            ),
            format!("\tConditions: {}", self.weather[0].description),
            format!("\tSunrise: {}", sunrise),
            format!("\tSunset: {}", sunset)
        )
        .map(|_| ())
        .map_err(Into::into)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ForecastMain {
    pub temp: Temperature,
    pub feels_like: f64,
    pub temp_min: Temperature,
    pub temp_max: Temperature,
    pub pressure: i64,
    pub sea_level: i64,
    pub grnd_level: i64,
    pub humidity: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ForecastEntry {
    #[serde(with = "timestamp")]
    pub dt: DateTime<Utc>,
    pub main: ForecastMain,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CityEntry {
    pub timezone: i32,
    #[serde(with = "timestamp")]
    pub sunrise: DateTime<Utc>,
    #[serde(with = "timestamp")]
    pub sunset: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WeatherForecast {
    pub list: Vec<ForecastEntry>,
    pub city: CityEntry,
}

impl WeatherForecast {
    pub fn get_high_low(&self) -> BTreeMap<NaiveDate, (Temperature, Temperature)> {
        let fo = FixedOffset::east(self.city.timezone);
        self.list.iter().fold(BTreeMap::new(), |mut hmap, entry| {
            let date = entry.dt.with_timezone(&fo).date().naive_local();
            let high = entry.main.temp_max;
            let low = entry.main.temp_min;

            if let Some((h, l)) = hmap.get(&date) {
                let high = if high > *h { high } else { *h };
                let low = if low < *l { low } else { *l };

                if (high, low) != (*h, *l) {
                    hmap.insert(date, (high, low));
                }
            } else {
                hmap.insert(date, (high, low));
            }
            hmap
        })
    }

    pub fn get_forecast<T: Write>(&self, buf: &mut T) -> Result<(), Error> {
        writeln!(buf, "\nForecast:")?;
        self.get_high_low()
            .into_iter()
            .map(|(d, (h, l))| {
                writeln!(
                    buf,
                    "\t{} {:30} {:30}",
                    d,
                    format!("High: {:0.2} F / {:0.2} C", h.fahr(), h.celc(),),
                    format!("Low: {:0.2} F / {:0.2} C", l.fahr(), l.celc(),),
                )
                .map(|_| ())
                .map_err(Into::into)
            })
            .collect()
    }
}
