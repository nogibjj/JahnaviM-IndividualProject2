use reqwest::blocking::get;
use std::fs::File;
use std::io::Write;
use std::error::Error;

pub fn extract(url: &str, path: &str) -> Result<String, Box<dyn Error>> {
    let response = get(url)?;
    let mut file = File::create(path)?;
    file.write_all(&response.bytes()?)?;
    Ok(path.to_string())
}