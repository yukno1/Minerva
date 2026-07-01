mod agent;

use futures::StreamExt;
use rig::{
    completion::Prompt,
    prelude::*,
    providers::ollama::{Client, CompletionModel},
};

pub struct AgentProcess {
    agent: rig::agent::Agent<CompletionModel>,
}

impl AgentProcess {
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
        let agent = ollama_client
            .agent("qwen3:4b")
            .preamble("You are a helpful assistant.")
            .name("Bob")
            .build();
        // let prompt = "What is the Rust programming language?";
        // println!("{prompt}");
        Self { agent }
    }

    pub async fn respond(&self, prompt: String) -> Result<std::string::String, std::io::Error> {
        let response = self.agent.prompt(prompt).await.map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, format!("Agent error: {}", e))
        });
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
}
