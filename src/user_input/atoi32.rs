// use core::fmt;
use crate::user_input::user_input;
use std::io::{self};
fn atoi32() -> Result<i32, io::Error> {
    let buffer = user_input::user_input("enter number")?;
    let buffer: i32 = buffer
        .trim()
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "nan".to_string()))?;

    Ok(buffer)
    // Ok(buffer)
}
