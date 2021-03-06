use crate::token::Types;
use super::token::Token;
use std::{iter::Peekable, str::Chars, io::Read};

pub struct HmpCompiler;

impl HmpCompiler {
    pub fn compile_hmp(mut data: &[u8]) -> Result<Vec<Option<Token<Types>>>, String> {
        let mut tokens = vec![];
        let mut source = String::new();

        let src = data.read_to_string(&mut source);
        match src {
            Err(_) => {return Err("parsing failed, wrong format".to_string())}
            _ => {},
        }

        let mut source: Peekable<Chars> = source.chars().peekable();

        while let Some(ch) = source.peek() {
            match ch {
                '<' | '>' => {
                    source.next();
                }

                'h' => {
                    while let Some(ch) = source.next() {
                        match ch {
                            '>' => {
                                tokens.push(Some(Token::Hash(Types::Str(make_string(&mut source)?))));
                                break;
                            }

                            _ => {}
                        }
                    }
                }

                'k' => {
                    while let Some(ch) = source.next() {
                        match ch {
                            ':' => {
                                tokens.push(Some(Token::Key(Types::Str(make_string(&mut source)?))));
                            }

                            '+' => {
                                tokens.push(Some(Token::Key(Types::U64(make_u64(&mut source)?))));
                            }

                            '\n' => break,

                            _ => {}
                        }
                    }
                }

                'v' => {
                    while let Some(ch) = source.next() {
                        match ch {
                            ':' => {
                                tokens.push(Some(Token::Value(Types::Str(make_string(&mut source)?))));
                            }

                            '+' => {
                                tokens.push(Some(Token::Value(Types::U64(make_u64(&mut source)?))));
                            }

                            '\n' => break,

                            _ => {}
                        }
                    }
                }

                'P' => {
                    break;
                }

                _ => {
                    source.next();
                }
            }
        }
        Ok(tokens)
    }

}

fn make_string(src: &mut Peekable<Chars>) -> Result<String, String> {
    let mut name = src.next().unwrap().to_string();
    while let Some(&ch) = src.peek() {
        match ch {
            '\n' => {
                break;
            }

            _ => {
                name.push(ch);
                src.next();
            }
        };
    }
    Ok(name)
}

fn make_u64(src: &mut Peekable<Chars>) -> Result<u64, String> {
    let mut number = src.next().unwrap().to_string();
    while let Some(&ch) = src.peek() {
        match ch {
            '0'..='9' => {
                number.push(ch);
                src.next();
            }

            '\n' => {
                break;
            }

            _ => break,
        };
    }
    number.parse::<u64>().map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;

        let data = "<p><h>myhashmap\n<k>+100\n<v>:amogh\n<H><P>".as_bytes();

        let tokens = HmpCompiler::compile_hmp(data);
        let vect = tokens.unwrap();
        println!("{:?}", vect)
        // TODO: don't know how to test this, figure out.
        // for elem in vect.iter() {
        //     assert_eq!(elem.unwrap_key().unwrap::<u64>(), Types::U64(100).unwrap::<u64>())
        // }
    }
}
