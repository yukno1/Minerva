use std::net::TcpStream;
use std::process::{Child, Command};
use std::time::Duration;

pub struct AgentProcess(Option<Child>);

impl AgentProcess {
    /// 启动 agent（如果未运行）
    pub fn start(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // 检查 agent 是否已在运行
        if TcpStream::connect(addr).is_ok() {
            println!("Agent already running, using existing instance.");
            return Ok(AgentProcess(None));
        }

        // 启动 agent 子进程
        println!("Starting agent...");
        let child = Command::new("cargo")
            .args(["run", "-p", "Minerva-agent"])
            .spawn()?;

        // 等待 agent 启动（最多 5 秒）
        for i in 0..50 {
            if TcpStream::connect(addr).is_ok() {
                println!("Agent started successfully.");
                return Ok(AgentProcess(Some(child)));
            }
            if i == 49 {
                return Err("Agent failed to start in time".into());
            }
            std::thread::sleep(Duration::from_millis(100));
        }

        Err("Agent startup timeout".into())
    }
}

// 当 AgentProcess 离开作用域时自动清理
impl Drop for AgentProcess {
    fn drop(&mut self) {
        if let Some(mut child) = self.0.take() {
            println!("Stopping agent...");
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}
