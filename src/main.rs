#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;

use std::env;

#[derive(Serialize, Deserialize)]
struct Quote {
    #[serde(rename = "Quote")]
    quote: String,
    from: String,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let client = reqwest::Client::new();

    let res: Quote = client
        .get("http://188.166.33.109:8080/api/v1/kali/quote")
        .query(&[("filter", get_filter())])
        .send()?
        .json()?;

    println!("{}\n\t~{}", res.quote, res.from);
    Ok(())
}

fn get_filter() -> String {
    let mut args: Vec<String> = env::args().collect();
    let filter;
    if args.len() > 1 {
        filter = args.split_off(1).join(" ");
    } else {
        filter = "".to_string();
    }
    filter
}
