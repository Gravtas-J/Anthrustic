use dotenv::dotenv;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, Write};

#[derive(Serialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct RequestBody {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Deserialize, Debug)]
struct ResponseContent {
    text: String,
}

#[derive(Deserialize, Debug)]
struct Response {
    content: Vec<ResponseContent>,
}

async fn send_message(
    client: &reqwest::Client,
    headers: &HeaderMap,
    messages: &[Message],
) -> Result<Response, reqwest::Error> {
    let request_body = RequestBody {
        model: "claude-3-haiku-20240307".to_string(),
        max_tokens: 1024,
        messages: messages.to_vec(),
    };

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .headers(headers.clone())
        .json(&request_body)
        .send()
        .await?;

    let response_body: Response = response.json().await?;
    Ok(response_body)
}

async fn get_user_input() -> Result<String, io::Error> {
    print!("User: ");
    io::stdout().flush()?;

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    Ok(user_input.trim().to_string())
}

async fn process_response(response: Response, messages: &mut Vec<Message>) {
    if let Some(content) = response.content.first() {
        println!("Assistant: {}", content.text);
        messages.push(Message {
            role: "assistant".to_string(),
            content: content.text.clone(),
        });
    } else {
        println!("No content found in the response");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set");

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("x-api-key", api_key.parse().unwrap());
    headers.insert("anthropic-version", "2023-06-01".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let mut messages = Vec::new();

    loop {
        let user_input = get_user_input().await?;

        if user_input.eq_ignore_ascii_case("quit") {
            println!("Conversation ended.");
            break;
        }

        messages.push(Message {
            role: "user".to_string(),
            content: user_input,
        });

        let response = send_message(&client, &headers, &messages).await?;

        if response.content.is_empty() {
            println!("No content found in the response");
            continue;
        }

        process_response(response, &mut messages).await;
    }

    Ok(())
}