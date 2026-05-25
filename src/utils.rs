use std::{io, str::FromStr};

use crate::types::MenuContext;

pub fn read_input<T: FromStr>(message: &str) -> Option<T> {
    println!("{message}");
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<T>().ok()
}

