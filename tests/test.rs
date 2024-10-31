use analyze::*;

#[test]
fn test_extract() -> Result<(), Box<dyn std::error::Error>> {
    const URL: &str = "https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/bad-drivers/bad-drivers.csv";
    const FILE_PATH: &str = "data/bad-drivers.csv";

    let path = extract(URL, FILE_PATH)?;
    assert_eq!(path, "data/bad-drivers.csv");

    Ok(())
}