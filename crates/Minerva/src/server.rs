use serde_json;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use protocol::{ChatRequest, ChatResponse, DEFAULT_ADDR};

use Minerva_agent::AgentProcess;

pub struct AgentServer {
    listener: TcpListener,
    agent_process: Arc<AgentProcess>,
}

impl AgentServer {
    pub fn listen(addr: &str) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(DEFAULT_ADDR)?;
        Ok(Self {
            listener,
            agent_process: Arc::new(AgentProcess::new()),
        })
    }

    pub fn serve(&self) -> std::io::Result<()> {
        println!("Agent server listening on {}", self.listener.local_addr()?);

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection from {:?}", stream.peer_addr());
                    let agent_process = self.agent_process.clone();
                    thread::spawn(move || {
                        if let Err(e) = handle_conn(stream, agent_process) {
                            eprintln!("Connection error: {}", e);
                        }
                    });
                }
                Err(e) => eprintln!("Failed to accept connection: {}", e),
            }
        }

        Ok(())
    }

    // fn send_response(&mut self, prompt: &str) -> std::io::Result<String> {
    //     let request = serde_json::json!({ "prompt": prompt });
    //     let request_str = serde_json::to_string(&request)?;

    //     self.stream.write_all(request_str.as_bytes())?;
    //     self.stream.write_all(b"\n")?;

    //     let mut reader = BufReader::new(&self.stream);
    //     let mut response_str = String::new();
    //     reader.read_line(&mut response_str)?;

    //     let response: serde_json::Value = serde_json::from_str(&response_str)?;
    //     Ok(response["response"].as_str().unwrap_or("").to_string())
    // }
}

fn handle_conn(mut stream: TcpStream, agent_process: Arc<AgentProcess>) -> std::io::Result<()> {
    // 克隆 stream 用于读取（BufReader 需要所有权）
    let reader_stream = stream.try_clone()?;
    let mut reader = BufReader::new(reader_stream);

    loop {
        // 读取请求
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;

        if bytes_read == 0 {
            // 客户端关闭连接
            println!("Connection closed by client");
            break;
        }

        // 解析 JSON 请求
        let request: ChatRequest = serde_json::from_str(&line.trim()).map_err(|e| {
            eprintln!("Failed to parse request: {}", e);
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;

        println!("Received prompt: {}", request.prompt);

        // 使用 agent 处理 prompt
        let response_text = process_prompt(agent_process.clone(), request.prompt)?;

        // 创建并发送响应
        let response = ChatResponse {
            response: response_text,
        };

        let response_str = serde_json::to_string(&response).map_err(|e| {
            eprintln!("Failed to serialize response: {}", e);
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;

        println!("Sending response ({} chars)", response_str.len());

        stream.write_all(response_str.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush()?;
    }

    Ok(())
}

fn process_prompt(agent_process: Arc<AgentProcess>, prompt: String) -> std::io::Result<String> {
    // 为这个连接创建单线程 tokio 运行时
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create runtime: {}", e),
            )
        })?;

    // 阻塞执行异步 agent 操作
    rt.block_on(agent_process.respond(prompt))
}
