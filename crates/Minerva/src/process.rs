use std::net::TcpStream;
use std::process::{Child, Command};
use std::time::Duration;

use Minerva_Agent::Agent;

pub struct AgentProcess {
    pub agent: Agent,
}

impl AgentProcess {
    pub fn new() -> Self {
        Self {
            agent: Agent::new(),
        }
    }
}

// // 当 AgentProcess 离开作用域时自动清理
// impl Drop for AgentProcess {
//     fn drop(&mut self) {
//         if let Some(mut child) = self.0.take() {
//             println!("Stopping agent...");
//             let _ = child.kill();
//             let _ = child.wait();
//         }
//     }
// }
