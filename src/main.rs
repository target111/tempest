use clap::Parser;
use tempest::Args;

fn main() {
    let args = Args::parse();

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

    //    println!("{:?}", args);
}
