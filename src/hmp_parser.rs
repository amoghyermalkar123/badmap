use super::token::Token;
use std::{iter::Peekable, str::Chars};

pub struct ParseHmp;

impl ParseHmp {
    pub fn lex_hmp(data: &str) -> Result<Vec<Token>, String> {
        let mut tokens = vec![];
        let mut source: Peekable<Chars> = data.chars().peekable();

        while let Some(ch) = source.peek() {
            match ch {
                '<' => {
                    while let Some(ch) = source.peek() {
                        match ch {
                            'P' => source.next(),
                            _ => source.next(),
                        };
                    }
                }

                'p' => {
                    tokens.push(Token::String(make_hm_name(&mut source)?));
                }

                'h' => {
                    tokens.push(Token::String(make_hm_name(&mut source)?));
                }
                'k' => {
                    while let Some(ch) = source.peek() {
                        match ch {
                            'd' => {
                                // get datatype, create token from data type
                                // increment global peeker
                            }
                        }
                    }
                    tokens.push(Token::String(make_hm_name(&mut source)?));
                }
                'v' => {
                    tokens.push(Token::String(make_hm_name(&mut source)?));
                }
                '\n' => {
                    source.next();
                }
                // for reference
                // 'a'..='z' | 'A'..='Z' => {
                //     tokens.push(Token::Ident(make_var(&mut source)));
                // }
                // '=' | '+' | '-' | '*' | '/' | '%' | '!' | '>' | '<' | ':' | ';' | '(' | ')'
                // | '[' | ']' | '{' | '}' | '|' | '^' | '&' => {
                //     tokens.push(Token::Operator(make_operator(&mut source)));
                // }
                // '\'' => {
                //     tokens.push(Token::Char(make_char(&mut source)?));
                // }
                _ => {
                    return Err(format!("Unexpected token: `{}`", source.next().unwrap()));
                }
            }
        }
        Ok(tokens)
    }
}

fn make_hm_name(src : &mut Peekable<Chars>) {}

fn make_int(src : &mut Peekable<Chars>) {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::ParseHmp;

        let tokens = ParseHmp::lex_hmp("<><hm>myhashmap</hm>");

        println!("{:?}", tokens);
    }
}
