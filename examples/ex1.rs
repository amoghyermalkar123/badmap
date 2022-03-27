use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:34254") {
        println!("connected to the server");
        println!("{}", stream.write(b"<p><h>name\n<k>+1\n<v>:user1\n<H><P>")?);
    } else {
        println!("connection failed!");
    }
    Ok(())
}
