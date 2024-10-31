use reqwest::blocking::get;
use std::fs::File;
use std::io::Write;
use std::error::Error;
use std::io::{BufReader, BufRead};
use rusqlite::{params, Connection};

pub fn extract(url: &str, path: &str) -> Result<String, Box<dyn Error>> {
    let response = get(url)?;
    let mut file = File::create(path)?;
    file.write_all(&response.bytes()?)?;
    Ok(path.to_string())
}

pub fn trans_load(path: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let connection = Connection::open("data/badDrivers.db")?;
    connection.execute("DROP TABLE IF EXISTS badDrivers", [])?;
    connection.execute(
        "CREATE TABLE badDrivers (
            state TEXT,
            drivers_count INTEGER,
            speeding_percent REAL,
            alc_percent REAL,
            no_distraction_percent REAL,
            no_prev_percent REAL,
            car_insurance REAL,
            insurance_losses REAL
        )",
        [],
    )?;

    let mut stmt = connection.prepare("INSERT INTO badDrivers VALUES (?, ?, ?, ?, ?, ?, ?, ?)")?;
    for line in reader.lines().skip(1) {
        let line = line?;
        let fields: Vec<&str> = line.split(',').collect();
        stmt.execute(params![
            fields[0],
            fields[1].parse::<f64>()?,
            fields[2].parse::<f64>()?,
            fields[3].parse::<f64>()?,
            fields[4].parse::<f64>()?,
            fields[5].parse::<f64>()?,
            fields[6].parse::<f64>()?,
            fields[7].parse::<f64>()?
        ])?;
    }

    Ok("data/badDrivers.db".to_string())
}