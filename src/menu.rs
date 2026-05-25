use crate::{projects::new_project, tasks::new_task, types::{MenuContext, Priority, Project, Status, Task}, utils::read_input};

pub fn options(menu_context: MenuContext) {
    match menu_context {
        MenuContext::Main =>{
            println!("Enter an option [1-5]");
            println!("1. Create a new Project\n2. Choose a Project\n3. List all Projects\n4. Edit a Project\n5. Exit")
        },
        MenuContext::NewProject => {
            println!("Enter an option [1-2]");
            println!("1. Create a new Task\n2. Exit")
        },
        MenuContext::ProjectWithTasks => {
            println!("Enter an option [1-5]");
            println!("1. Create a new Task\n2. List all Tasks\n3.Edit a Task\n4. Filter through Tasks\n5. Exit")
        },
        MenuContext::EditTask => {
            println!("Enter an option [1-5]");
            println!("1. Edit title\n2. Edit Status\n3. Edit Priority\n4. Edit due date\n5. Exit")
        },
        MenuContext::FilterTask => {
            println!("Enter an option to filter [1-3]");
            println!("1. Status\n2. Priority\n3. Exit")
        }
        _ => ()
    }
}

//======================= Handle Project Options =================================
pub fn handle_create_project(projects: &mut Vec<Project>) {
    let name: String = match read_input("Enter the name of the project") {
        Some(name) => name,
        None => {
            eprintln!("[Error] from src/menu.rs: Cannot parse");
            return
        }
    };

    let description: String = match read_input("Enter the description of the project") {
        Some(desc) => desc,
        None => {
            eprintln!("[Error] from src/menu.rs: Cannot parse");
            return
        }
    };

    new_project(projects, name, description)
}

pub fn handle_choose_project(projects: &[Project]) {
}

// ====================== Handle Task Options ====================================
pub fn handle_create_task(tasks: &mut Vec<Task>) {
    let title: String = match read_input("Enter the description of the project") {
        Some(title) => title,
        None => {
            eprintln!("[Error] from src/menu.rs: Cannot parse");
            return
        }
    };

    let priority_input: String = match read_input("Enter the priority of the task") {
        Some(prio) => prio,
        None => {
            eprintln!("[Error] from src/menu.rs: Cannot parse");
            return
        }
    };
    let priority = match Priority::to_priority(&priority_input.to_lowercase()) {
        Some(priority) => priority,
        None => {
            eprintln!("[Error] from src/menu.rs: Cannot parse");
            return
        }
    };

    let status_input: String = match read_input("Enter the priority of the task") {
        Some(status)=> status,
        None => {
            eprintln!("[Error] from src/menu.rs: Cannot parse");
            return
        }
    };
    let status = match Status::to_status(&status_input.to_lowercase()) {
        Some(status) => status,
        None => {
            eprintln!("[Error] from src/menu.rs: Cannot parse");
            return
        }
    };

    let due_date_option: String = match read_input("Do you want to add due date? [y/n]") {
        Some(option) => option,
        None => {
            eprintln!("[Error] from src/menu.rs: Cannot parse");
            return
        }
    };
    let due_date: Option<String> = if due_date_option.trim() == "y" {
        read_input("Enter Due Date in the format [DD-MM-YYYY]")
    } else {
        None
    };

    new_task(tasks, title, priority, status, due_date);
}

pub fn handle_search_project(projects: &[Project]) {
}
