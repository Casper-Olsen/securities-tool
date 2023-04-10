use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ArgsError;

impl Error for ArgsError {}

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No security specified")
    }
}

pub fn get_security(args: &Vec<String>) -> Result<String, ArgsError> {
    if args.len() < 2 {
        return Err(ArgsError {});
    }

    let security = &args[1];
    Ok(String::from(security))
}
