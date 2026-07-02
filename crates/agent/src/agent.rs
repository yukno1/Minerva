use async_stream::try_stream;
use futures::{Stream, StreamExt};
use rig::{
    agent::MultiTurnStreamItem,
    completion::Prompt,
    prelude::*,
    providers::ollama::{Client, CompletionModel},
    streaming::{StreamedAssistantContent, StreamingPrompt},
};
use std::pin::Pin;

pub struct Agent {
    kernel: rig::agent::Agent<CompletionModel>,
}

impl Agent {
    pub fn new() -> Self {
        let ollama_client = Client::from_env()
            .map_err(|e| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to create Ollama client: {}", e),
                )
            })
            .unwrap();

        // agent
        let kernel = ollama_client
            .agent("qwen3:4b")
            .preamble("You are a helpful assistant.")
            .name("Bob")
            .build();
        // let prompt = "What is the Rust programming language?";
        // println!("{prompt}");
        Self { kernel }
    }

    pub async fn respond(&self, prompt: String) -> Result<std::string::String, std::io::Error> {
        let response = self.kernel.prompt(prompt).await.map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, format!("Agent error: {}", e))
        });
        // format!("Response: {response}")

        response
    }

    pub async fn stream_respond(
        &self,
        prompt: String,
    ) -> Pin<Box<dyn Stream<Item = Result<String, std::io::Error>> + Send>> {
        let stream = self.kernel.stream_prompt(prompt).await;

        Box::pin(stream.filter_map(|item| async move {
            match item {
                Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Text(
                    text,
                ))) => Some(Ok(text.text)),
                Ok(MultiTurnStreamItem::FinalResponse(_)) => Some(Ok("".to_string())),
                Ok(_) => None,
                Err(e) => Some(Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Stream error: {}", e),
                ))),
            }
        }))
    }
}
