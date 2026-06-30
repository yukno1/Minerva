mod agent;
mod server;

fn main() -> std::io::Result<()> {
    let server = server::AgentServer::listen("127.0.0.1:8080")?;
    server.serve()
}
