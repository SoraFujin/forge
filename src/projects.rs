use std::fs;
use crate::{tasks::print_tasks, types::{Priority, Project, Status}};

pub fn new_project(projects: &mut Vec<Project>, name: String, description: String) {
    let next_id = projects.iter().map(|p| p.id).max().unwrap_or(0) + 1;
    let project = Project::new(next_id, name, description, Vec::new());
    projects.push(project);
}


pub fn edit_name(projects: &mut [Project], project_id: u32, new_name: &str) {
    for project in projects.iter_mut() {
        if project.id == project_id {
            project.name = new_name.to_string();
            return;
        }
    }

    println!("Project with id {} not found", project_id);
}

pub fn edit_description(
    projects: &mut [Project],
    project_id: u32,
    new_description: &str,
) {
    for project in projects.iter_mut() {
        if project.id == project_id {
            project.description = new_description.to_string();
            return;
        }
    }

    println!("Project with id {} not found", project_id);
}

pub fn print_projects(projects: &[Project]) {
    for project in projects {
        println!(
            "id: {}, Name: {}, Description: {}\nTasks:",
            project.id, project.name, project.description
        );

        print_tasks(&project.tasks);
    }
}

pub fn print_stats(project: &Project) {
    let total = project.tasks.len();
    let mut todo = 0;
    let mut in_progress = 0;
    let mut done = 0;
    let mut highest: Option<&str> = None;

    for task in &project.tasks {
        match task.status {
            Status::TODO => todo += 1,
            Status::InProgress => in_progress += 1,
            Status::Done => done += 1,
        }
        if task.status != Status::Done {
            match task.priority {
                Priority::HIGH => highest = Some(&task.title),
                Priority::MEDIUM => {
                    if highest.is_none() {
                        highest = Some(&task.title);
                    }
                }
                Priority::LOW => {
                    if highest.is_none() {
                        highest = Some(&task.title);
                    }
                }
            }
        }
    }

    println!("Project: {}", project.name);
    println!("  Total tasks:  {}", total);
    println!("  TODO:         {}", todo);
    println!("  In Progress:  {}", in_progress);
    println!("  Done:         {}", done);
    match highest {
        Some(title) => println!("  Top pending:  {}", title),
        None => println!("  Top pending:  none"),
    }
}

pub fn export_project(project: &Project) {
    let mut content = String::new();

    content.push_str(&format!("Project: {}\n", project.name));
    content.push_str(&format!("Description: {}\n", project.description));
    content.push_str(&format!("Total tasks: {}\n\n", project.tasks.len()));

    for task in &project.tasks {
        content.push_str(&format!("  - {}\n", task.title));
        content.push_str(&format!("    Priority: {}\n", task.priority.to_str()));
        content.push_str(&format!("    Status:   {}\n", task.status.to_str()));
        match &task.due_date {
            Some(date) => content.push_str(&format!("    Due:      {}\n", date)),
            None => content.push_str("    Due:      none\n"),
        }
        content.push('\n');
    }

    let file_name = format!("{}.txt", project.name.to_lowercase().replace(' ', "_"));

    match fs::write(&file_name, content) {
        Ok(_) => println!("Exported to {}", file_name),
        Err(e) => eprintln!("Export failed: {}", e),
    }
}

pub fn delete_project(projects: &mut Vec<Project>, project_id: u32) {
    let initial_len = projects.len();

    projects.retain(|project| project.id != project_id);

    if projects.len() == initial_len {
        println!("Project with id {} not found", project_id);
    } else {
        println!("Project {} deleted", project_id);
    }
}
