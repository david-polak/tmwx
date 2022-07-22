use clap::Parser;
use reqwest;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;

use preferences::{AppInfo, Preferences, PreferencesMap};

const APP_INFO: AppInfo = AppInfo {
    name: "tmwx",
    author: "David Polák",
};

/*
 * CLI
 *
 * tmwx <icao>
 * tmwx --metar <icao>
 * tmwx --taf <icao>
 *
 */

#[derive(Parser)]
struct Cli {
    icao: String,

    #[clap(short, long, action)]
    metar: bool,
    #[clap(short, long, action)]
    taf: bool,
}

fn main() {
    let args = Cli::parse();

    println!("{}", args.icao);

    // let client = reqwest::blocking::Client::new();
    // let res = client.get("https://api.met.no/weatherapi/tafmetar/1.0/taf?extended=false&icao=LKPR")
    //     .header(USER_AGENT, "tmwx/dev github.com/david-polak/tmwx")
    //     .send().expect("failed to get request").text().expect("failed");

    // println!("{}", res);

    // let body = reqwest::blocking::get("https://www.rust-lang.org")?.text()?;

    /* let body = reqwest::get("https://www.rust-lang.org")
    .await?
    .text()
    .await?; */

    // println!("body = {:?}", body);
}

// curl --user-agent "tmwx/dev github.com/david-polak/tmwx" -X GET --header 'Accept: text/plain' 'https://api.met.no/weatherapi/tafmetar/1.0/metar?extended=false&icao=LKPR'

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
