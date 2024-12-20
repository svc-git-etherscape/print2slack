use chrono::Local;
use reqwest::blocking::Client;
use serde_json::json;

fn main() {
    let now = Local::now();
    let message = format!("Current date and time: {}", now);

    let webhook_url = "https://hooks.slack.com/services/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";
    let payload = json!({
        "text": message
    });

    let client = Client::new();
    let res = client.post(webhook_url)
        .json(&payload)
        .send();

    match res {
        Ok(response) => println!("Message sent: {:?}", response),
        Err(e) => println!("Error sending message: {:?}", e),
    }
}