use std::io::{self};

#[allow(dead_code)]
pub fn user_input(prompt: &str) -> Result<String, io::Error> {
    println!(":{}", prompt);
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input)?;
    if user_input.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "nan"));
    }
    let user_input = user_input.trim();

    Ok(user_input.to_string())
}
