use crate::types::MenuContext;

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
