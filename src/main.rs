use clap::Parser;
use tempest::{Args, Commands, Unit};
use url::Url;

const BASE_URL: &str = "https://wttr.in";

fn main() {
    let args = Args::parse();

    match build_url(&args) {
        Ok(url) => match get_weather(&url) {
            Ok(_) => println!("Weather information retrieved successfully."),
            Err(e) => eprintln!("Error fetching weather data: {}", e),
        },
        Err(e) => eprintln!("Error building URL: {}", e),
    }
    /*
    // Handle global options
    let location = args.location.unwrap_or_else(|| "IP-based".to_string());
    let unit = args.unit.unwrap_or(tempest::Unit::M);
    let days = args.days.unwrap_or(0);
    let rich = args.rich;

    println!("Location: {}", location);
    println!("Unit: {:?}", unit);
    println!("Days: {}", days);
    println!("Rich: {}", rich);

    // Handle subcommands
    match args.command {
        Some(tempest::Commands::Moon { date }) => {
            let date = date.unwrap_or_else(|| chrono::Local::now().date_naive());
            println!("Showing moon phase for date: {}", date);
            // Implement moon phase logic here
        }
        None => {
            println!("Showing weather forecast");
            // Implement weather forecast logic here
        }
    }
    */

    //    println!("{:?}", args);
}

fn build_url(args: &Args) -> Result<Url, url::ParseError> {
    let mut base_url: Url = Url::parse(BASE_URL).unwrap();

    match args.command {
        Some(Commands::Moon { date }) => {
            let mut moon_url = base_url.join("moon")?;

            if let Some(date) = date {
                moon_url.set_path(&format!("{}@{}", moon_url.path(), date));
            }
            Ok(moon_url)
        }
        None => {
            if let Some(location) = &args.location {
                base_url.path_segments_mut().unwrap().push(location);
            }

            let mut pairs = base_url.query_pairs_mut();

            if args.rich {
                pairs.append_pair("format", "v2");
            }

            if let Some(Unit::M) = args.unit {
                pairs.append_key_only("m");
            } else if let Some(Unit::I) = args.unit {
                pairs.append_key_only("u");
            }

            if let Some(days) = &args.days {
                pairs.append_key_only(&days.to_string());
            }
            drop(pairs);

            Ok(base_url)
        }
    }
}

fn get_weather(url: &Url) -> Result<(), ureq::Error> {
    let response = ureq::request_url("GET", url)
        .set("User-Agent", "curl")
        .set("Accept-Language", "en")
        .call()?;

    println!("{}", response.into_string().unwrap());
    Ok(())
}
