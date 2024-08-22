# Tempest
**Tempest** is a nice little rust package that fetches forecasts from wttr.in without the fuss.

It provides a simple command-line interface to quickly check weather conditions for any location. With Tempest, you can:

- Get current weather information
- View short-term forecasts
- Check temperatures, humidity, wind speeds, moon phase and more

Tempest aims to make weather checking from the terminal both easy and enjoyable.

## Usage ##
```
Usage: tempest [OPTIONS] [COMMAND]

Commands:
  moon  Display moon phase information
  help  Print this message or the help of the given subcommand(s)

Options:
  -l, --location <LOCATION>  Location for weather data (default: IP-based)
  -u, --unit <UNIT>          Unit system for weather data [default: m] [possible values: m, i]
  -d, --days <DAYS>          Forecast duration in days
      --rich                 Enable detailed weather and astronomical data
  -h, --help                 Print help (see more with '--help')
  -V, --version              Print version
```
