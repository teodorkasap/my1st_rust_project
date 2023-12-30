use regex::Regex;
use reqwest::Error;


async fn get_request() -> Result<(), Error> {
    let response = reqwest::get("https://www.fruityvice.com/api/fruit/apple").await?;
    println!("Status: {}", response.status());

    let body = response.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}


#[tokio::main]

async fn main() -> Result<(), Error> {
    
    println!("Hello, world!");
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));

    get_request().await?;
    Ok(())
 

    
}
