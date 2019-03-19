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

use structopt::StructOpt;

mod api;
mod cli;
mod logger;
mod quote;
mod status;

fn main() {
    // Don't panic on broken pipes when writing to STDOUT
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    let opt = cli::Opt::from_args();

    logger::init(opt.verbose).expect("Unable to init the logger");

    cli::handle_user_command(opt.cmd);
}
