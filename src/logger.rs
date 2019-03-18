use simplelog::*;
use std::error::Error;

pub fn init(verbosity: u8) -> Result<(), Box<Error>> {
    let cfg: Vec<Box<SharedLogger>> =
        vec![TermLogger::new(get_log_level(verbosity), Config::default()).unwrap()];

    CombinedLogger::init(cfg)?;

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
    fn init_logger() {
        assert_eq!(init(1).is_ok(), true);
        // A second logger init should fail
        assert_eq!(init(0).is_ok(), false);
    }

    #[test]
    fn log_level() {
        assert_eq!(get_log_level(0), log::LevelFilter::Off);
        assert_eq!(get_log_level(5), log::LevelFilter::max());
    }
}
