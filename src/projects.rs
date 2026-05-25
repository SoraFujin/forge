use crate::{tasks::print_tasks, types::Project};

pub fn new_project(projects: &mut Vec<Project>, name: String, description: String) {
    let next_id = projects.len() as u32 + 1;
    let project = Project::new(next_id, name, description, Vec::new());
    projects.push(project);
}

pub fn find_project(projects: &[Project], project_id: u32) {
    for project in projects {
        if project.id == project_id {
            println!("Name: {}, Desciption: {}, Tasks:", project.name, project.description);
            for task in &project.tasks {
                match &task.due_date {
                    Some(date) => {
                        println!("Title: {}, Priority: {:?}, Status: {:?}, Due date: {}", task.title, task.priority, task.status, date)
                    },
                    None => println!("Title: {}, Priority: {:?}, Status: {:?}", task.title, task.priority, task.status)
                }
            }
        }
    }
}

pub fn edit_name(projects: &mut [Project], project_id: u32, new_name: &str) {
    for project in projects {
        if project.id == project_id {
            project.name = new_name.to_string();
            break
        }
    }
}

pub fn edit_desciption(projects: &mut [Project], project_id: u32, new_desciption: &str) {
    for project in projects {
        if project.id == project_id {
            project.description = new_desciption.to_string();
            break
        }
    }
}

pub fn print_projects(projects: &[Project]) {
    for project in projects {
        println!("id: {}, Name: {}, Desciption: {}, Tasks: ", project.id, project.name, project.description);
        print_tasks(&project.tasks);
    }
}
