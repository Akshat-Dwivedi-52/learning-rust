use std::fmt::Debug;

use crate::{queue::Queue, stack::Stack, traits::Printable};

impl<T: Debug> Printable for Stack<T> {
    fn print(&self) {
        println!("{:#?}", self);
    }
}

impl<T: Debug> Printable for Queue<T> {
    fn print(&self) {
        println!("{:#?}", self);
    }
}
