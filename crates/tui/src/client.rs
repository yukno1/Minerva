// crates/tui/src/agent_client.rs
use serde_json;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn connect(remote_addr: &str) -> std::io::Result<Self> {
        let stream = TcpStream::connect(remote_addr)?;
        Ok(Self { stream })
    }

    pub fn send_prompt(&mut self, prompt: String) -> std::io::Result<()> {
        let request = serde_json::json!({ "prompt": prompt });
        let request_str = serde_json::to_string(&request)?;

        self.stream.write_all(request_str.as_bytes())?;
        self.stream.write_all(b"\n")?;
        Ok(())
    }

    pub fn _receive_response(&mut self) -> std::io::Result<String> {
        let mut reader = BufReader::new(&self.stream);
        let mut response_str = String::new();
        reader.read_line(&mut response_str)?;

        let response: serde_json::Value = serde_json::from_str(&response_str)?;
        Ok(response["response"].as_str().unwrap_or("").to_string())
    }

    pub fn receive_stream<F>(&mut self, mut callback: F) -> std::io::Result<()>
    where
        F: FnMut(String),
    {
        let mut reader = BufReader::new(&self.stream);
        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line)?;

            if bytes_read == 0 {
                break;
            }

            if let Ok(chunk) = serde_json::from_str::<protocol::ChatResponseChunk>(&line.trim()) {
                // 调用回调函数处理内容
                if !chunk.content.is_empty() {
                    callback(chunk.content);
                }

                if chunk.done {
                    break;
                }
            }
        }

        Ok(())
    }
}
