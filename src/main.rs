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

mod cli;
mod quote;
mod status;

fn main() {
    // Don't panic on broken pipes when writing to STDOUT
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    let opt = cli::Opt::from_args();

    init_logger(opt.verbose);

    cli::handle_user_command(opt.cmd);
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
