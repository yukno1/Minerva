use rig::{
    client::Nothing,
    completion::{CompletionModel, Prompt},
    prelude::*,
    providers::ollama::Client,
};

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ollama_client = Client::from_env()?;

    // chat completion
    // let ollama_completions_model = ollama_client.completion_model("qwen3:4b");
    // let req = ollama_completions_model
    //     .completion_request("What is the Rust programming language?")
    //     .preamble("You are a helpful assistant.".to_string())
    //     .build();
    // let response = ollama_completions_model.completion(req).await?;

    // agent
    let agent = ollama_client
        .agent("qwen3:4b")
        .preamble("You are a helpful assistant.")
        .name("Bob")
        .build();
    let prompt = "What is the Rust programming language?";
    println!("{prompt}");

    let response = agent.prompt(prompt).await?;
    println!("Response: {response}");
    Ok(())
}
