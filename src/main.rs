#[macro_use]
extern crate serde_derive;
extern crate structopt;

extern crate libc;
extern crate reqwest;
extern crate serde;

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

#[derive(StructOpt)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "f", long = "filter")]
    filter: Option<String>,
    /// Quote type, either text or presidential
    #[structopt(short = "t", long = "type", default_value = "text")]
    quote_type: String,
}

fn main() {
    // Don't panic on broken pipes when writing to STDOUT
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    let opt = Opt::from_args();

    match opt.quote_type.as_ref() {
        "text" => {
            let quote = fetch_quote(opt.filter);

            match quote {
                Ok(q) => println!("\"{}\"\n\t~{}", q.quote, q.from),
                Err(e) => println!("Quote fetch error: {}", e),
            }
        }
        "presidential" => {
            let res = fetch_presidential_quote(opt.filter);
            match res {
                Ok(quote_file) => {
                    Command::new("xdg-open")
                        .arg(quote_file)
                        .output()
                        .expect("Failed to open presidential quote");
                }
                Err(e) => println!("Unable to fetch presidential quote: {}", e),
            }
        }
        _ => {
            println!("Invalid quote type given!");
        }
    }
}

fn fetch_quote(filter: Option<String>) -> Result<Quote, Box<std::error::Error>> {
    let params = [("filter", filter.unwrap_or_default())];

    let quote: Quote = reqwest::Client::new()
        .get("http://188.166.33.109:8080/api/v1/kali/quote")
        .query(&params)
        .send()?
        .json()?;

    Ok(quote)
}

fn fetch_presidential_quote(filter: Option<String>) -> Result<String, Box<std::error::Error>> {
    let file_name = "/tmp/trumpie";
    let params = [("filter", filter.unwrap_or_default())];

    let mut resp = reqwest::Client::new()
        .get("http://188.166.33.109:8080/api/v1/kali/quote/presidential")
        .query(&params)
        .send()?;

    let mut out = File::create(file_name)?;
    io::copy(&mut resp, &mut out)?;

    Ok(file_name.to_string())
}
