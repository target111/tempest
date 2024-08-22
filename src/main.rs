use clap::Parser;
use tempest::{Args, Commands, Unit};
use ureq::AgentBuilder;
use url::Url;

const BASE_URL: &str = "http://wttr.in";

fn main() {
    let args = Args::parse();

    match build_url(&args) {
        Ok(url) => match get_weather(&url) {
            Ok(response) => println!("{}", response),
            Err(e) => eprintln!("Error fetching weather data: {}", e),
        },
        Err(e) => eprintln!("Error building URL: {}", e),
    }
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

fn get_weather(url: &Url) -> Result<String, String> {
    let agent = AgentBuilder::new()
        .redirects(5) // Allow up to 5 redirects
        .build();

    match agent
        .request_url("GET", url)
        .set("User-Agent", "curl/8.9.1")
        .set("Accept-Language", "en")
        .set("Accept", "*/*")
        .call()
    {
        Ok(response) => response
            .into_string()
            .map_err(|e| format!("Failed to read response: {}", e)),
        // wttr.in is weird like that and returns 404 for aribtrary moon dates but still provides a response
        Err(ureq::Error::Status(_, response)) => response
            .into_string()
            .map_err(|e| format!("Failed to read error response: {}", e)),
        Err(ureq::Error::Transport(e)) => Err(format!("Transport error: {}", e)),
    }
}
