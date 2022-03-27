mod hashmap;
mod hmp_parser;
mod in_memory;
mod server;
mod token;

use std::borrow::BorrowMut;
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    Ok(())
}

// fn main() {
//     let mut hm = HashMap::<u64, String>::new();

//     for i in 1..6 {
//         hm.set(i, format!("value is {value}", value = i).to_string());
//     }

//     for i in 1..6 {
//         let ans = hm.get(i);

//         println!("answer is {:?}", ans.unwrap());
//     }

//     hm.delete(2);

//     for i in 1..6 {
//         let ans = hm.get(i);

//         println!("answer is {:?}", ans.unwrap_or(&"none".to_string()));
//     }
// }
