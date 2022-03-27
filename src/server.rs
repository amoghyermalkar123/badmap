use std::hash::Hash;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use crate::token::{Token, Types};
use super::hmp_parser::HmpCompiler;
use super::hashmap::HashMap;
use super::token;

pub fn handle_connection(hashmap : &mut HashMap<String, &Types>, stream : &mut TcpStream) -> Result<(), String>{
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).unwrap();
    // println!("recv : {:?}", buf);
    // println!("The bytes: {:?}", &buf[..n]);
    // println!("Request: {}", String::from_utf8_lossy(&buf[..n]));
    let source = String::from_utf8_lossy(&buf[..n]);

    if let Ok(ref input) = HmpCompiler::compile_hmp(source) {
        let key = input[1].unwrap_key().unwrap();
        let value = input[2].unwrap_val();
        hashmap.set(key, value);
        // hashmap.set(input[1].unwrap_key().unwrap(), input[2].unwrap_val());
        Ok(())
    } else {
        Err("".to_string())
    }
}

pub fn start_server() -> std::io::Result<()> {
    let mut hashmap  = HashMap::new();

    let listener = TcpListener::bind("127.0.0.1:34254")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                match handle_connection(&mut hashmap, &mut stream) {
                    Ok(data) => {},
                    Err(e) => {println!("{}", e)},
                }
            }
            Err(e) => { println!("{}", e);}
        }
    }
    Ok(())
}
