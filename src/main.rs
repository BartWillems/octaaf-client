#[macro_use]
extern crate serde_derive;
extern crate structopt;

extern crate libc;
extern crate reqwest;
extern crate serde;

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
}

fn main() {
    // Don't panic on broken pipes when writing to STDOUT
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    let opt = Opt::from_args();

    let quote = fetch_quote(opt.filter);

    match quote {
        Ok(q) => println!("\"{}\"\n\t~{}", q.quote, q.from),
        Err(e) => println!("Quote fetch error: {}", e),
    }
}

fn fetch_quote(filter: Option<String>) -> Result<Quote, Box<std::error::Error>> {
    let client = reqwest::Client::new();

    let quote: Quote = client
        .get("http://188.166.33.109:8080/api/v1/kali/quote")
        .query(&[("filter", filter.unwrap_or("".to_string()))])
        .send()?
        .json()?;

    Ok(quote)
}
