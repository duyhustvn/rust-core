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

// The MyBox type is a tuple struct with one element of type T.
// The MyBox::new function takes one parameter of type T and returns a MyBox instance that holds the value passed in.
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list: {list}");

    let x = 5;
    let y = MyBox::new(5);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
