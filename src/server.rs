use super::hashmap::HashMap;
use super::hmp_parser::HmpCompiler;
use crate::token::Types;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_connection(
    hashmap: &mut HashMap<String, Types>,
    stream: &mut TcpStream,
) -> Result<(), String> {
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).unwrap();
    let source = String::from_utf8_lossy(&buf[..n]);

    if let Ok(input) = HmpCompiler::compile_hmp(source.as_bytes()) {
        // get the ownership of the vector containing the tokens
        for mut elem in input {
            hashmap.set(elem.take().unwrap().unwrap_key().unwrap(), elem.take().unwrap().unwrap_val())
        }
        Ok(())
    } else {
        Err("".to_string())
    }
}

pub fn start_server() -> std::io::Result<()> {
    let mut hashmap = HashMap::new();

    let listener = TcpListener::bind("127.0.0.1:34254")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => match handle_connection(&mut hashmap, &mut stream) {
                Ok(data) => {}
                Err(e) => {
                    println!("{}", e)
                }
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    Ok(())
}
