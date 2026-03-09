#[cfg(feature = "stdio-server")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    eprintln!("arbor-mcp starting");
    arbor_mcp::serve_stdio().await
}

#[cfg(not(feature = "stdio-server"))]
fn main() {
    eprintln!("arbor-mcp stdio server is disabled; build with --features stdio-server");
}
