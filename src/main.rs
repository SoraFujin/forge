#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::{menu::options, types::Priority, utils::read_input};
pub mod utils;
pub mod types;
pub mod menu;
pub mod projects;
pub mod tasks;

fn main() {
    loop {
        options(types::MenuContext::Main);
        let option: u8 = match read_input("") {
            Some(option) => option,
            None => {
                eprint!("[Error] from src/main.rs: reading and parsing line");
                continue
            }
        };

        if option > 5 || option == 0 {
            println!("only from 1-5");
            continue
        }

        if option == 5 {
            break
        }
    }
}
