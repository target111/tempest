use chrono::NaiveDate;
use clap::{ColorChoice, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Tempest")]
#[command(
    about = "Your friendly CLI weather buddy - fetches forecasts from wttr.in without the fuss."
)]
#[command(version, color = ColorChoice::Always)]
pub struct Args {
    /// Location for weather data (default: IP-based)
    ///
    /// Supported formats: city name, domain name, airport code, area code, GPS coords
    #[arg(short, long)]
    pub location: Option<String>,

    /// Unit system for weather data
    ///
    /// Choose between metric (m) or imperial (i) units.
    #[arg(short, long, value_enum, default_value = "m")]
    pub unit: Option<Unit>,

    /// Forecast duration in days
    ///
    /// Specify 0 for current weather, 1 for today, or 2 for today and tomorrow.
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(..=2))]
    pub days: Option<u8>,

    /// Enable detailed weather and astronomical data
    #[arg(long)]
    pub rich: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum Unit {
    /// Metric (SI)
    M,
    /// Imperial
    I,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Display moon phase information
    Moon {
        /// Date for moon phase (default: today)
        date: Option<NaiveDate>,
    },
}
