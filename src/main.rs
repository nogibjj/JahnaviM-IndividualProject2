/*
Rust script to use ETL and CRUD strategies on bad-drivers data 
using sqlite.
*/

use analyze::{extract};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    const URL: &str = "https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/bad-drivers/bad-drivers.csv";
    const FILE_PATH: &str = "data/bad-drivers.csv";

    let path = extract(URL, FILE_PATH)?;
    println!("[EXTRACT] Data extracted to csv file at path {}", path);
    

    Ok(())
}
