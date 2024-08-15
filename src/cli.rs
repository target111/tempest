use chrono::NaiveDate;
use clap::ColorChoice;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, color = ColorChoice::Always)]
pub struct Args {
    /// Location for weather data (default: IP-based)
    ///
    /// Supported formats: city name, domain name, airport code, area code, GPS coords
    #[arg(short, long)]
    location: Option<String>,

    /// Unit system for weather data
    ///
    /// Choose between metric (m) or imperial (i) units.
    #[arg(short, long, value_enum, default_value = "m")]
    unit: Option<Unit>,

    /// Forecast duration in days
    ///
    /// Specify 0 for current weather, 1 for today, or 2 for today and tomorrow.
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(..=2))]
    days: Option<u8>,

    /// Enable detailed weather and astronomical data
    #[arg(long)]
    rich: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::ValueEnum, Debug, Clone)]
enum Unit {
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
