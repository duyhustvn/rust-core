enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cons(val, next) => write!(f, "{} -> {}", val, next),
            Nil => write!(f, "Nil"),
        }
    }
}

use core::fmt;

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list: {list}");
}
