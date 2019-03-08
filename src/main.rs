#[macro_use]
extern crate structopt;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate simple_error;

#[macro_use]
extern crate log;

extern crate libc;
extern crate reqwest;
extern crate serde;
extern crate simplelog;

use simplelog::*;
use structopt::StructOpt;

mod quote;
mod status;

#[derive(StructOpt)]
#[structopt(name = "octaaf-client")]
struct Opt {
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
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

fn main() {
    // Don't panic on broken pipes when writing to STDOUT
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    let opt = Opt::from_args();

    init_logger(opt.verbose);

    handle_user_command(opt.cmd);
}

fn handle_user_command(cmd: Command) {
    match cmd {
        Command::Quote { quote_type, filter } => match quote::get(quote_type, filter) {
            Ok(_) => (),
            Err(e) => {
                println!("Quote error: {}", e);
                std::process::exit(1);
            }
        },
        Command::Status => {
            let status = status::get_status();

            println!("{}", serde_json::to_string_pretty(&status).unwrap());
            if !status.healthy {
                std::process::exit(1);
            }
        }
    }
}

fn init_logger(verbosity: u8) {
    let cfg: Vec<Box<SharedLogger>> =
        vec![TermLogger::new(get_log_level(verbosity), Config::default()).unwrap()];

    CombinedLogger::init(cfg).unwrap();
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
