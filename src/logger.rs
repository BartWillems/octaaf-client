use simplelog::*;
use std::error::Error;

pub fn init(verbosity: u8) -> Result<(), Box<Error>> {
    let level = get_log_level(verbosity);
    TermLogger::init(level, Config::default())?;

    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_level() {
        assert_eq!(get_log_level(0), log::LevelFilter::Off);
        assert_eq!(get_log_level(5), log::LevelFilter::max());
    }
}
