mod server;
#[tokio::main]
async fn main() -> anyhow::Result<()>{
    println!("starting metriko server");
    let handler = tokio::spawn(server::collect_data()) ; 
    handler.await??; // Two question marks - we're unwrapping the task result, and the result from running the collector.
    Ok(())
}
