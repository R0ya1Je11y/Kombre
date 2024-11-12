use reqwest::header;
use reqwest::Client;
use serde_json::Value;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
	let client = reqwest::Client::builder()
	.user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36")
	.build()?;
	
	//let request = client.get("https://api.mangadex.org")
		//.header(header::USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36")
		//.build()?;
	
	let search_query = search_cli();
	
	let search_result = search(client, search_query).await?;
	
	println!("{:?}", search_result);
	
	// let response = client.execute(request).await?;

	// let body = response.text().await?;
	// println!("body = {:#?}", body);
	
    	Ok(())
}

fn search_cli() -> String {
		use std::io::{self, Write};
		
		print!("Enter the title to search: ");
		io::stdout().flush().unwrap();
		
		let mut search_query = String::new();
		io::stdin().read_line(&mut search_query).expect("Failed to read line");
		
		search_query.trim().to_string()
}

async fn search(client: Client, title: String) -> Result<Vec<String>, reqwest::Error> {
	
	let base_url = "https://api.mangadex.org";
	
	let response = client
		.get(&format!("{}/manga", base_url))
		.query(&[("title", title)])
		.send()
		.await?;
		
	let response_body = response.json::<Value>().await?;
	let manga_ids = response_body["data"].as_array()
		.map(|array| array.to_vec())
		.unwrap_or_default();
	
	let ids: Vec<String> = manga_ids
	.iter()
	.filter_map(|manga| manga["id"].as_str().map(|s| s.to_string()))
	.collect();
	
	Ok(ids)
}
