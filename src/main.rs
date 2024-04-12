use dotenv::dotenv;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, Write};
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::time::sleep;

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
        print_response_character_by_character(&content.text).await;
        messages.push(Message {
            role: "assistant".to_string(),
            content: content.text.clone(),
        });
    } else {
        println!("No content found in the response");
    }
}

async fn animate_thinking(mut stop_signal: oneshot::Receiver<()>) {
    let mut dots = 0;
    loop {
        if stop_signal.try_recv().is_ok() {
            println!("\rThinking{} ", " ".repeat(6)); // Clear the line and add space for transition
            break;
        }
        if dots == 6 {
            print!("\rThinking{}", " ".repeat(6)); // Clear the dots visually
            dots = 0;
        } else {
            print!("\rThinking{}", ".".repeat(dots));
            dots += 1;
        }
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(100)).await;
    }
}

async fn print_response_character_by_character(response: &String) {
    print!("Bot: "); // Print the "Bot: " prefix before the response
    for c in response.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(10)).await;
    }
    println!(); // Ensure the output ends on a new line
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    println!("Welcome to the Rust Chatbot! Enter 'quit' to exit.");
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

        let (stop_sender, stop_receiver) = oneshot::channel();
        let thinking_task = tokio::spawn(animate_thinking(stop_receiver));

        let response = send_message(&client, &headers, &messages).await?;

        stop_sender.send(()).unwrap(); // Signal the thinking animation to stop
        thinking_task.await.unwrap();

        if response.content.is_empty() {
            println!("No content found in the response");
            continue;
        }

        process_response(response, &mut messages).await;
    }

    Ok(())
}