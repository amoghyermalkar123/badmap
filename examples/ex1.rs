use nom::{bytes::complete::tag,bytes::complete::take_till, IResult};
use std::{iter::Peekable, str::Chars};

fn foo<'a> (s: &'a str, pt : &'a str) -> IResult<&'a str, &'a str> {
    tag(pt)(s)
}

fn bar(s: &str) -> IResult<&str, &str> {
    // println!("{}",s);
    tag("\n")(s)
}

fn till_colon(s: &str) -> IResult<&str, &str> {
    take_till(|c| c == '\n')(s)
  }

fn main() {
    let tokens: Vec<String> = vec![];
    let data = "<proto><hash>name\n<key>1\n<d>u64\n<v>user1\n<d>str\n<key>2\n<d>u64\n<v>user2\n<d>str\n</hash></proto>";
    
    let mut source: Peekable<Chars> = data.chars().peekable();

   
    let a = foo(, "<proto>");
    let b = a.unwrap();

    
    let b = foo(b.0, "<hash>").unwrap();

    let c = till_colon(b.0).unwrap();
    println!("{:?}", c);
}
