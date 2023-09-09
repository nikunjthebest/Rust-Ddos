use reqwest;
use tokio;
use std::io;

#[tokio::main]
async fn main() {
    // Create a reqwest Client instance
    let client = reqwest::Client::new();

    println!("Enter a URL:");
    
    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("Failed to read line");
 
     // Remove leading/trailing whitespace and newline characters
    let url = url.trim();

    loop {
        // Clone the client and URL for each task
        let client_clone = client.clone();
        let url_clone = url.to_string();

        // Spawn an asynchronous task to send an HTTP GET request to the URL
        tokio::spawn(async move {
            send_request(&client_clone, &url_clone).await;
            // println!("Sent request to {}", url_clone)
        });
    }
}

async fn send_request(client: &reqwest::Client, url: &str) {
    let response = client.get(url).send();

    // Spawn a separate task to handle the response
    tokio::spawn(async move {
        match response.await {
            Ok(res) => {
                if res.status() == reqwest::StatusCode::OK {
                    println!("200");
                }
            }
            Err(err) => {
                println!("Error sending request: {}", err);
            }
        }
    });
}
