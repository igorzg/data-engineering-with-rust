use chatbot::chatbot::run_chat_loop;
use reqwest::Client;
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let api_key = env::var("API_KEY").unwrap_or_else(|_| String::from("none"));
    let url = "https://code-smith-agent.llm.lan/v1/completions";
    run_chat_loop(&client, &api_key, url).await?;
    Ok(())
}
