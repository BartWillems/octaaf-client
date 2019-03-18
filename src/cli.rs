use super::{quote, status};

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "octaaf-client")]
pub struct Opt {
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: u8,
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(name = "quote")]
    Quote {
        #[structopt(name = "type", long, short)]
        /// Quote Type, either text or presidential
        quote_type: quote::QuoteType,
        #[structopt(name = "filter", long, short)]
        filter: Option<String>,
    },
    #[structopt(name = "status")]
    Status,
}

pub fn handle_user_command(cmd: Command) {
    match cmd {
        Command::Quote { quote_type, filter } => match quote::get(quote_type, filter) {
            Ok(_) => (),
            Err(e) => {
                println!("Quote error: {}", e);
                std::process::exit(1);
            }
        },
        Command::Status => match status::Status::get() {
            Ok(s) => println!("{}", s),
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            }
        },
    }
}
