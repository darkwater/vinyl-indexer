use std::env;
use std::fmt::Debug;

pub trait Report {
    type Returns;

    fn report(self) -> Self::Returns;
}

impl<T, E: Debug> Report for Result<T, E> {
    type Returns = Result<T, E>;

    fn report(self) -> Self::Returns {
        match self {
            Ok(val)  => Ok(val),
            Err(err) => {
                if env::var("RUST_BACKTRACE").map(|s| s == "1").unwrap_or(false) {
                    panic!("Encountered a non-fatal error (but panicking as requested): {:#?}", err);
                // } else {
                //     eprintln!("Encountered a non-fatal error: {:#?}", err);
                }
                Err(err)
            },
        }
    }
}

impl<T> Report for Option<T> {
    type Returns = Option<T>;

    fn report(self) -> Self::Returns {
        match self {
            Some(val) => Some(val),
            None      => {
                if env::var("RUST_BACKTRACE").map(|s| s == "1").unwrap_or(false) {
                    panic!("Missing a value!");
                // } else {
                //     eprintln!("Missing a value somewhere! Run with RUST_BACKTRACE=1 to panic with a backtrace!");
                }
                None
            },
        }
    }
}
