mod process;
mod server;

use std::process::{Child, Command};

use protocol::DEFAULT_ADDR;

fn main() -> Result<(), std::io::Error> {
    // let agent_process = AgentProcess::start(DEFAULT_ADDR).map_err(|e| {
    //     eprintln!("Failed to start agent: {}", e);
    //     std::process::exit(1);
    // })?;
    let server = server::AgentServer::listen(DEFAULT_ADDR)?;
    let server_handle = std::thread::spawn(move || {
        if let Err(e) = server.serve() {
            eprintln!("Server error: {}", e);
        }
    });

    // 引入 Windows 特有的进程扩展 trait
    use std::os::windows::process::CommandExt;
    const CREATE_NEW_CONSOLE: u32 = 0x00000010;
    let mut tui = Command::new("cmd")
        .args(["/C", "chcp 65001 >nul && cargo run -p Minerva-tui"])
        .creation_flags(CREATE_NEW_CONSOLE)
        .spawn()
        .unwrap();
    let _ = tui.wait();

    println!("Shutting down...");
    Ok(())
}
