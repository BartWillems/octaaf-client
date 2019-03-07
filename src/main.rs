#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate simple_error;

#[macro_use]
extern crate log;

extern crate libc;
extern crate reqwest;
extern crate serde;
extern crate simplelog;
extern crate structopt;

use simplelog::*;
use std::error::Error;
use std::fs::File;
use std::io;
use std::process::Command;
use structopt::StructOpt;

#[derive(Deserialize)]
struct Quote {
    #[serde(rename = "Quote")]
    quote: String,
    from: String,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "f", long = "filter")]
    filter: Option<String>,
    #[structopt(raw(possible_values = "&QuoteType::variants()", case_insensitive = "true",))]
    quote_type: QuoteType,
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,
}

arg_enum! {
    #[derive(Debug)]
    enum QuoteType {
        Text,
        Presidential,
    }
}

fn main() {
    // Don't panic on broken pipes when writing to STDOUT
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    let opt = Opt::from_args();

    CombinedLogger::init(vec![TermLogger::new(
        get_log_level(opt.verbose),
        Config::default(),
    )
    .unwrap()])
    .unwrap();

    match opt.quote_type {
        QuoteType::Text => {
            let quote = fetch_quote(opt.filter);

            match quote {
                Ok(q) => println!("\"{}\"\n\t~{}", q.quote, q.from),
                Err(e) => {
                    println!("Quote fetch error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        QuoteType::Presidential => {
            let res = fetch_presidential_quote(opt.filter);
            match res {
                Ok(quote_file) => {
                    Command::new("xdg-open")
                        .arg(quote_file)
                        .output()
                        .expect("Failed to open presidential quote");
                }
                Err(e) => {
                    println!("Unable to fetch presidential quote: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

fn get_log_level(level: u8) -> log::LevelFilter {
    match level {
        0 => log::LevelFilter::Off,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        3 => log::LevelFilter::Trace,
        _ => log::LevelFilter::max(),
    }
}

fn fetch_quote(filter: Option<String>) -> Result<Quote, Box<Error>> {
    let params = [("filter", filter.unwrap_or_default())];

    let quote: Quote = reqwest::Client::new()
        .get("http://188.166.33.109:8080/api/v1/kali/quote")
        .query(&params)
        .send()?
        .json()?;

    Ok(quote)
}

fn fetch_presidential_quote(filter: Option<String>) -> Result<&'static str, Box<Error>> {
    let file_name = "/tmp/trumpie";
    let params = [("filter", filter.unwrap_or_default())];

    let mut resp = reqwest::Client::new()
        .get("http://188.166.33.109:8080/api/v1/kali/quote/presidential")
        .query(&params)
        .send()?;

    if !resp.status().is_success() {
        bail!("Invalid server response: {}", resp.status());
    }

    let mut out = File::create(file_name)?;
    io::copy(&mut resp, &mut out)?;

    info!("Saved the presidential quote as {}", file_name);

    Ok(file_name)
}
