// #![no_std]
extern crate alloc;
use alloc::{
    rc::Rc,
    string::{String, ToString},
};

use core::fmt::{Display, Error, Formatter};

pub const S: Combinator = Combinator::S { x: None, y: None };
pub const K: Combinator = Combinator::K { x: None };
pub const I: Combinator = Combinator::I;
pub const G: Combinator = Combinator::Get;
pub const P: Combinator = Combinator::Print;

pub fn string(s: impl ToString) -> Combinator {
    Combinator::Str(s.to_string())
}

#[derive(Clone, Debug)]
pub enum Combinator {
    S {
        x: Option<Rc<Self>>,
        y: Option<Rc<Self>>,
    },
    K {
        x: Option<Rc<Self>>,
    },
    I,
    Print,
    Get,
    Str(String),
}

impl Combinator {
    pub fn app(&self, c: &Combinator) -> Self {
        match self {
            Self::K { x } => match x {
                None => Self::K {
                    x: Some(Rc::new(c.clone())),
                },
                Some(ret) => (**ret).clone(),
            },
            Self::S { x, y } => match (x, y) {
                (None, None) => Self::S {
                    x: Some(Rc::new(c.clone())),
                    y: None,
                },
                (Some(_), None) => Self::S {
                    x: x.clone(),
                    y: Some(Rc::new(c.clone())),
                },
                (Some(a), Some(b)) => (a.app(c)).app(&b.app(c)),
                _ => unreachable!(),
            },
            Self::I => c.clone(),
            Self::Print => {
                match c {
                    Self::Str(s) => print!("{}", s),
                    _ => {}
                }
                c.clone()
            }
            Self::Get => match c {
                Self::Str(s) => {
                    use std::io::{stdin, stdout, Write};
                    print!("{}", s);
                    let mut s = String::new();
                    let _ = stdout().flush();
                    stdin().read_line(&mut s).unwrap();
                    if let Some('\n') = s.chars().next_back() {
                        s.pop();
                    }
                    if let Some('\r') = s.chars().next_back() {
                        s.pop();
                    }
                    string(s)
                }
                _ => c.clone(),
            },
            Self::Str(_) => self.clone(),
        }
    }
}

impl Display for Combinator {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Self::S { x, y } => match (x, y) {
                (None, None) => write!(f, "S"),
                (Some(a), None) => write!(f, "S({})", a),
                (Some(a), Some(b)) => write!(f, "S({}{})", a, b),
                _ => unreachable!(),
            },
            Self::K { x } => match x {
                None => write!(f, "K"),
                Some(a) => write!(f, "K({})", a),
            },
            Self::I => write!(f, "I"),
            Self::Str(s) => write!(f, "{}", s),
            Self::Print => write!(f, "P"),
            Self::Get => write!(f, "G"),
        }
    }
}
