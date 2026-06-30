// crates/tui/src/agent_client.rs
use serde_json;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct AgentClient {
    stream: TcpStream,
}

impl AgentClient {
    pub fn connect(addr: &str) -> std::io::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Ok(Self { stream })
    }

    pub fn send_prompt(&mut self, prompt: &str) -> std::io::Result<String> {
        let request = serde_json::json!({ "prompt": prompt });
        let request_str = serde_json::to_string(&request)?;

        self.stream.write_all(request_str.as_bytes())?;
        self.stream.write_all(b"\n")?;

        let mut reader = BufReader::new(&self.stream);
        let mut response_str = String::new();
        reader.read_line(&mut response_str)?;

        let response: serde_json::Value = serde_json::from_str(&response_str)?;
        Ok(response["response"].as_str().unwrap_or("").to_string())
    }
}
