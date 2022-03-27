use std::panic::UnwindSafe;

#[derive(Debug)]
pub enum Token<T> {
    Hash(T),
    Key(T),
    Value(T),
    EOF,
}

pub enum Types {
    Str(String),
    // RefStr(&str),
    U64(u64),
}

impl<Types> Token<Types> {
    pub fn unwrap_key(&self) -> &Types {
        match self {
            self::Token::Key(k) => k,
            _ => panic!(),
        }
    }
    pub fn unwrap_val(&self) -> &Types {
        match self {
            self::Token::Value(v) => v,
            _ => panic!(),
        }
    }
}

pub trait UnWrappable {
    fn unwrap(var : Types) -> Self;
}

impl UnWrappable for String {
    fn unwrap(var : Types) -> String {
        match var {
            Types::Str(d) => d,
            _ => panic!()
        }
    }
}

impl UnWrappable for u64 {
    fn unwrap(var : Types) -> u64 {
        match var {
            Types::U64(d) => d,
            _ => panic!()
        }
    }
}

// impl UnWrappable for Types {
//     fn unwrap(var : Types) -> Types {
//         var
//     }
// }

impl Types {
    pub fn unwrap<T: UnWrappable>(self) -> T {
        T::unwrap(self)
    }
}
