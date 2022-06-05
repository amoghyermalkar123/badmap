mod hashmap;
mod hmp_parser;
mod in_memory;
mod server;
mod token;

fn main() -> std::io::Result<()> {
    let _ = server::start_server();
    Ok(())
}
