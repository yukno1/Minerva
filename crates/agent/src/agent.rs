use futures::StreamExt;
use rig::{
    agent::MultiTurnStreamItem,
    completion::{CompletionModel, Prompt},
    prelude::*,
    providers::ollama::Client,
    streaming::{StreamedAssistantContent, StreamingPrompt},
};

pub async fn process(prompt: String) -> Result<std::string::String, std::io::Error> {
    let ollama_client = Client::from_env().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to create Ollama client: {}", e),
        )
    })?;

    // agent
    let agent = ollama_client
        .agent("qwen3:4b")
        .preamble("You are a helpful assistant.")
        .name("Bob")
        .build();
    // let prompt = "What is the Rust programming language?";
    // println!("{prompt}");

    let response = agent
        .prompt(prompt)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Agent error: {}", e)));
    // format!("Response: {response}")

    // let mut stream = agent.stream_prompt(prompt).await;

    // while let Some(item) = stream.next().await {
    //     match item? {
    //         MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Text(text)) => {
    //             format!("{}", text.text);
    //         }
    //         MultiTurnStreamItem::FinalResponse(_) => format!(),
    //         _ => {}
    //     }
    // }

    response
}
