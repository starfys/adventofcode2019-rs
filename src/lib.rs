use reqwest::Client;

const AOC_URL: &str = "https://adventofcode.com/2019";

async fn download_input(session: &str, year: u16, day: u8) -> Result<String, Box<dyn std::error::Error>> {
    // Create a reqwest client 
    let client = Client::new();
    
    let url = format!("{}/{}/day/{}/input", AOC_URL, year, day);
    
    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session))
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}
