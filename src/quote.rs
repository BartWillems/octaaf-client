extern crate serde;

use std::error::Error;
use std::fs::File;
use std::io;
use std::process::Command;
use std::str::FromStr;

static API_URL: &'static str = "http://188.166.33.109:8080/api/v1/kali/quote";

#[derive(Deserialize)]
pub struct Quote {
    #[serde(rename = "Quote")]
    quote: String,
    from: String,
}

#[derive(StructOpt, Debug)]
pub enum QuoteType {
    #[structopt(name = "text")]
    Text,
    #[structopt(name = "presidential")]
    Presidential,
}

impl FromStr for QuoteType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_ref() {
            "text" => Ok(QuoteType::Text),
            "presidential" => Ok(QuoteType::Presidential),
            &_ => Err(String::from("Invalid quote type found")),
        }
    }
}

pub fn get(quote_type: QuoteType, filter: Option<String>) -> Result<(), Box<Error>> {
    match quote_type {
        QuoteType::Text => {
            let quote = fetch_text_quote(filter)?;
            println!("\"{}\"\n\t~{}", quote.quote, quote.from);
            Ok(())
        }
        QuoteType::Presidential => {
            let image = fetch_presidential_quote(filter)?;
            Command::new("xdg-open")
                        .arg(image)
                        .output()?;
            Ok(())
        }
    }
}

fn fetch_text_quote(filter: Option<String>) -> Result<Quote, Box<Error>> {
    let params = [("filter", filter.unwrap_or_default())];

    let quote: Quote = reqwest::Client::new()
        .get(API_URL)
        .query(&params)
        .send()?
        .json()?;

    Ok(quote)
}

fn fetch_presidential_quote(filter: Option<String>) -> Result<&'static str, Box<Error>> {
    let file_name = "/tmp/trumpie";
    let params = [("filter", filter.unwrap_or_default())];
    let uri: String = format!("{}/presidential", API_URL).to_string();

    let mut resp = reqwest::Client::new().get(&uri).query(&params).send()?;

    if !resp.status().is_success() {
        bail!("Invalid server response: {}", resp.status());
    }

    let mut out = File::create(file_name)?;
    io::copy(&mut resp, &mut out)?;

    info!("Saved the presidential quote as {}", file_name);

    Ok(file_name)
}
