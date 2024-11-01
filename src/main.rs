/*
Rust script to use ETL and CRUD strategies on bad-drivers data 
using sqlite.
*/

use analyze::{extract, trans_load, query};
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Command needed after cargo run. Choose between ETL and Q <your-custom-query>.");
        return Ok(());
    }

    let command = &args[1];
    match command.as_str() {
        "ETL" => {
            const URL: &str = "https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/bad-drivers/bad-drivers.csv";
            const FILE_PATH: &str = "data/bad-drivers.csv";

            let path = extract(URL, FILE_PATH)?;
            println!("[EXTRACT] Data extracted to csv file at path {}", path);

            let db_path = trans_load(&path)?;
            println!("[TRANSFORM and LOAD] Data transformed and loaded to sqlite db at path {}", db_path);
        },
        "Q" => {
            if let Some(user_query) = args.get(2) {
                match query(user_query) {
                    Ok(result) => println!("Query Output: \n{}", result),
                    Err(err) => eprintln!("Error: {:?}", err),
                }
            } else {
                println!("Must follow command Q with custom query. cargo run Q <your-query> ");
            }
        }
        _ => {
            println!("Invalid command. Use 'ETL' or 'Q <your-custom-query>'.");
        }
    }


    Ok(())
}