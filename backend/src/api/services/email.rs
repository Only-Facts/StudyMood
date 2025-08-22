use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Sender {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipient {
    pub email: String,
}

pub async fn send_verification_email(
    recipient_email: &str,
    token: &str,
) -> Result<(), reqwest::Error> {
    let ip = std::env::var("IP").expect("Expected IP environment variable.");
    let port: u16 = std::env::var("PORT")
        .expect("Expected PORT environment variable.")
        .parse()
        .unwrap();

    let verification_link = format!("http://{ip}:{port}/auth/verify?token={token}");

    let api_key = std::env::var("API_TOKEN").expect("API_TOKEN not set");

    let client = Client::new();
    let api_url = "https://api.mailersend.com/v1/email";

    let from = Sender {
        name: "StudyMood".to_string(),
        email: "study.mood@test-p7kx4xwx6vmg9yjr.mlsender.net".to_string(),
    };

    let to = vec![Recipient {
        email: recipient_email.to_string(),
    }];

    let body = serde_json::json!({
        "from": {
            "name": from.name,
            "email": from.email,
        },
        "to": to,
        "subject": "Verify Your Email Address",
        "html": format!("Hello! Please verify your email by clicking <a href=\"{verification_link}\">this link</a>."),
    });

    let response = client
        .post(api_url)
        .header("Authorization", format!("Bearer {api_key}"))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Email sent successfully!");
    } else {
        println!("Failed to send email: {:?}", response.text().await?);
    }

    Ok(())
}
