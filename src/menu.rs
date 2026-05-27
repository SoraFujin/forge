use crate::{projects::{delete_project, edit_description, edit_name, export_project, new_project, print_projects, print_stats}, tasks::{delete_task, edit_due_date, edit_priority, edit_status, edit_title, filter_priority, filter_status, new_task, print_tasks}, types::{MenuContext, Priority, Project, Status, Task}, utils::read_input};

pub fn options(menu_context: MenuContext) {
    match menu_context {
        MenuContext::Main =>{
            println!("1. Create a new Project\n2. Choose a Project\n3. List all Projects\n4. Edit a Project\n5. Stats\n6. Exit")
        },
        MenuContext::NewProject => {
            println!("1. Create a new Task\n2. Exit")
        },
        MenuContext::EditProject => {
            println!("1. Edit name\n2. Edit description\n3. Delete Project\n4. Exit")
        },
        MenuContext::ProjectWithTasks => {
            println!("1. Create a new Task\n2. List all Tasks\n3. Edit a Task\n4. Delete Task\n5. Filter Tasks\n6. Stats\n7. Export\n8. Exit")
        },
        MenuContext::DeleteProject => {
            println!("Enter Project ID to delete (or 0 to cancel)")
        },
        MenuContext::EditTask => {
            println!("1. Edit title\n2. Edit status\n3. Edit priority\n4. Edit due date\n5. Exit")
        },
        MenuContext::FilterTask => {
            println!("1. Filter by status\n2. Filter by priority\n3. Exit")
        },
        MenuContext::DeleteTask => {
            println!("Enter Task ID to delete (or 0 to cancel)")
        },
        _ => ()
    }
}

//======================= Handle Project Options =================================
pub fn handle_create_project(projects: &mut Vec<Project>) {
    let name: String = match read_input("Project name:") {
        Some(name) => name,
        None => {
            eprintln!("Failed to read input.");
            return
        }
    };

    let description: String = match read_input("Project description:") {
        Some(desc) => desc,
        None => {
            eprintln!("Failed to read input.");
            return
        }
    };

    new_project(projects, name, description)
}

pub fn handle_choose_project(projects: &mut [Project]) {
    print_projects(projects);
    let project_id: u32 = match read_input("Project ID:") {
        Some(id) => id,
        None => {
            eprintln!("Invalid ID. Must be a number.");
            return
        }
    };
    if let Some(project) = projects.iter_mut().find(|p| p.id == project_id) {
        handle_project_context(project);
    } else {
        eprintln!("No project found with that ID.");
    }
}

pub fn handle_project_context(project: &mut Project) {
    loop {
        if project.tasks.is_empty() {
            options(MenuContext::NewProject);
            let option: u8 = match read_input("") {
                Some(option) => option,
                None => {
                    eprintln!("Invalid option. Must be a number.");
                    continue
                }
            };

            if option == 2 {
                break;
            }
            if option == 1 {
                handle_create_task(&mut project.tasks);
            }

            if option > 2 || option == 0  {
                eprintln!("Invalid option. Choose 1 or 2.");
                continue
            }
        } else {
            options(MenuContext::ProjectWithTasks);
            let option: u8 = match read_input("") {
                Some(option) => option,
                None => {
                    eprintln!("Invalid option. Must be a number.");
                    continue
                }
            };
            match option {
                1 => handle_create_task(&mut project.tasks),
                2 => print_tasks(&project.tasks),
                3 => handle_edit_task(&mut project.tasks),
                4 => {
                    let task_id: u32 = match read_input("Task ID to delete:") {
                        Some(id) => id,
                        None => {
                            eprintln!("Invalid ID. Must be a number.");
                            continue;
                        }
                    };
                    delete_task(&mut project.tasks, task_id);
                },
                5 => handle_filter_tasks(&project.tasks),
                6 => print_stats(project),
                7 => export_project(project),
                8 => break,
                _ => eprintln!("Invalid option. Choose between 1 and 8.")
            }
        }
    }
}

pub fn handle_edit_project(projects: &mut Vec<Project>) {
    loop {
        options(MenuContext::EditProject);

        let option: u8 = match read_input("") {
            Some(option) => option,
            None => {
                eprintln!("Invalid option. Must be a number.");
                continue;
            }
        };

        match option {
            1 => {
                let project_id: u32 = match read_input("Project ID:") {
                    Some(id) => id,
                    None => {
                        eprintln!("Invalid ID. Must be a number.");
                        continue;
                    }
                };

                let new_name: String = match read_input("New name:") {
                    Some(name) => name,
                    None => {
                        eprintln!("Failed to read input.");
                        continue;
                    }
                };

                edit_name(projects, project_id, &new_name);
            }

            2 => {
                let project_id: u32 = match read_input("Project ID:") {
                    Some(id) => id,
                    None => {
                        eprintln!("Invalid ID. Must be a number.");
                        continue;
                    }
                };

                let new_description: String = match read_input("New description:") {
                    Some(desc) => desc,
                    None => {
                        eprintln!("Failed to read input.");
                        continue;
                    }
                };

                edit_description(projects, project_id, &new_description);
            },
            3 => {
                let project_id: u32 = match read_input("Project ID to delete:") {
                    Some(id) => id,
                    None => {
                        eprintln!("Invalid ID.");
                        continue;
                    }
                };

                delete_project(projects, project_id);
            }

            4 => return,

            _ => {
                eprintln!("Invalid option. Choose between 1 and 4.");
            }
        }
    }
}

// ====================== Handle Task Options ====================================
pub fn handle_create_task(tasks: &mut Vec<Task>) {
    let title: String = match read_input("Task title:") {
        Some(title) => title,
        None => {
            eprintln!("Failed to read input.");
            return
        }
    };

    let priority_input: String = match read_input("Priority (high / medium / low):") {
        Some(prio) => prio,
        None => {
            eprintln!("Failed to read input.");
            return
        }
    };
    let priority = match Priority::to_priority(&priority_input.to_lowercase()) {
        Some(priority) => priority,
        None => {
            eprintln!("Invalid priority. Expected: high, medium, or low.");
            return
        }
    };

    let status_input: String = match read_input("Status (todo / in progress / done):") {
        Some(status)=> status,
        None => {
            eprintln!("Failed to read input.");
            return
        }
    };
    let status = match Status::to_status(&status_input.to_lowercase()) {
        Some(status) => status,
        None => {
            eprintln!("Invalid status. Expected: todo, in progress, or done.");
            return
        }
    };

    let due_date_option: String = match read_input("Add due date? (y/n):") {
        Some(option) => option,
        None => {
            eprintln!("Failed to read input.");
            return
        }
    };
    let due_date: Option<String> = if due_date_option.trim() == "y" {
        read_input("Due date (DD-MM-YYYY):")
    } else {
        None
    };

    new_task(tasks, title, priority, status, due_date);
}

pub fn handle_edit_task(tasks: &mut [Task]) {
    'outer: loop {
        options(MenuContext::EditTask);
        let option: u8 = match read_input("") {
            Some(option) => option,
            None => {
                eprintln!("Invalid option. Must be a number.");
                continue;
            }
        };

        'inner: loop{
            match option {
                1 => {
                    let task_id: u32 = match read_input("Task ID:") {
                        Some(id) => id,
                        None => {
                            eprintln!("Invalid ID. Must be a number.");
                            continue 'inner
                        }
                    };
                    let new_title: String = match read_input("New title:") {
                        Some(title) => title,
                        None => {
                            eprintln!("Failed to read input.");
                            continue 'inner
                        }
                    };
                    edit_title(tasks, task_id, new_title);
                    break 'outer
                },
                2 => {
                    let task_id: u32 = match read_input("Task ID:") {
                        Some(id) => id,
                        None => {
                            eprintln!("Invalid ID. Must be a number.");
                            continue 'inner
                        }
                    };

                    let status: String = match read_input("New status (todo / in progress / done):") {
                        Some(status) => status,
                        None => {
                            eprintln!("Failed to read input.");
                            continue 'inner
                        }
                    };

                    let new_status = match Status::to_status(&status.to_lowercase()) {
                        Some(status) => status,
                        None => {
                            eprintln!("Invalid status. Expected: todo, in progress, or done.");
                            continue 'inner
                        }
                    };
                    edit_status(tasks, task_id, new_status);
                    break 'outer
                },
                3 => {
                    let task_id: u32 = match read_input("Task ID:") {
                        Some(id) => id,
                        None => {
                            eprintln!("Invalid ID. Must be a number.");
                            continue 'inner
                        }
                    };

                    let priority: String = match read_input("New priority (high / medium / low):") {
                        Some(prio) => prio,
                        None => {
                            eprintln!("Failed to read input.");
                            continue 'inner
                        }
                    };
                    let new_priority = match Priority::to_priority(&priority.to_lowercase()) {
                        Some(prio) => prio,
                        None =>  {
                            eprintln!("Invalid priority. Expected: high, medium, or low.");
                            continue 'inner
                        }
                    };

                    edit_priority(tasks, task_id, new_priority);
                    break 'outer
                },
                4 => {
                    let task_id: u32 = match read_input("Task ID:") {
                        Some(id) => id,
                        None => {
                            eprintln!("Invalid ID. Must be a number.");
                            continue 'inner
                        }
                    };
                    let new_date: String = match read_input("New due date (DD-MM-YYYY):") {
                        Some(date) => date,
                        None => {
                            eprintln!("Failed to read input.");
                            continue 'inner
                        }
                    };
                    edit_due_date(tasks, task_id, new_date);
                    break 'outer
                },
                5 => break 'outer,
                _ => {
                    eprintln!("Invalid option. Choose between 1 and 5.");
                    continue 'outer
                }
            }
        }
    }
}

pub fn handle_filter_tasks(tasks: &[Task]) {
    'outer: loop {
        options(MenuContext::FilterTask);
        let option: u8 = match read_input("") {
            Some(option) => option,
            None => {
                eprintln!("Invalid option. Must be a number.");
                continue;
            }
        };

        'inner: loop{
            match option {
                1 => {
                    let status_filter: String = match read_input("Status (todo / in progress / done):") {
                        Some(status) => status,
                        None => {
                            eprintln!("Failed to read input.");
                            continue 'inner
                        }
                    };
                    filter_status(tasks, status_filter.as_str());
                    break 'outer
                },
                2 => {
                    let priority_filter: String = match read_input("Priority (high / medium / low):") {
                        Some(prio) => prio,
                        None => {
                            eprintln!("Failed to read input.");
                            continue 'inner
                        }
                    };
                    filter_priority(tasks, &priority_filter);
                    break 'outer
                },
                3 => break 'outer,
                _ => {
                    eprintln!("Invalid option. Choose between 1 and 3.");
                    continue 'outer
                }
            }
        }
    }
}
