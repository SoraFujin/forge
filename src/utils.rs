use std::{io, str::FromStr};

pub fn read_input<T: FromStr>(message: &str) -> Option<T> {
    println!("{message}");
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<T>().ok()
}

