#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::{menu::{handle_choose_project, handle_create_project, handle_edit_project, options}, projects::print_projects, types::{Priority, Project}, utils::read_input};
pub mod utils;
pub mod types;
pub mod menu;
pub mod projects;
pub mod tasks;

fn main() {
    let mut projects: Vec<Project> = Vec::new();
    loop {
        options(types::MenuContext::Main);
        let option: u8 = match read_input("") {
            Some(option) => option,
            None => {
                eprintln!("Invalid option. Must be a number.");
                continue
            }
        };

        match option {
            1 => {
                handle_create_project(&mut projects);
            },
            2 => {
                handle_choose_project(&mut projects);
            },
            3 => {
                print_projects(&projects);
            },
            4 => {
                handle_edit_project(&mut projects);
            },
            5 => break,
            _ => ()
        }
    }
}
