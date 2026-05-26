use std::fs;
use crate::types::{Project, Task, Priority, Status};

pub fn write_to_file(projects: &[Project]) {
    if projects.is_empty() {
        println!("No projects to save");
        return;
    }

    let mut data = String::new();

    for project in projects {
        data.push_str("PROJECT\n");

        data.push_str(&format!("id={}\n", project.id));
        data.push_str(&format!("name={}\n", project.name));
        data.push_str(&format!("description={}\n", project.description));
        data.push_str(&format!("tasks={}\n\n", project.tasks.len()));

        for task in &project.tasks {
            data.push_str("TASK\n");

            data.push_str(&format!("id={}\n", task.id));
            data.push_str(&format!("title={}\n", task.title));
            data.push_str(&format!(
                "priority={}\n",
                task.priority.to_str()
            ));
            data.push_str(&format!(
                "status={}\n",
                task.status.to_str()
            ));

            let due_date = match &task.due_date {
                Some(date) => date.as_str(),
                None => "",
            };

            data.push_str(&format!("due_date={}\n\n", due_date));
        }

        data.push_str("END_PROJECT\n\n");
    }

    match fs::write("forge_data.txt", data) {
        Ok(_) => println!("Saved successfully"),
        Err(error) => println!("Error: {}", error),
    }
}

pub fn read_from_file(file_name: &str) -> Vec<Project> {
    let content = match fs::read_to_string(file_name) {
        Ok(data) => data,
        Err(error) => {
            println!("Error reading file: {}", error);
            return Vec::new();
        }
    };

    let mut projects: Vec<Project> = Vec::new();

    let mut current_project_id: u32 = 0;
    let mut current_project_name = String::new();
    let mut current_project_description = String::new();
    let mut current_tasks: Vec<Task> = Vec::new();

    let mut current_task_id: u32 = 0;
    let mut current_task_title = String::new();
    let mut current_task_priority = Priority::LOW;
    let mut current_task_status = Status::TODO;

    let mut inside_task = false;

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        match line {
            "PROJECT" => {
                current_tasks.clear();
            }

            "TASK" => {
                inside_task = true;

                current_task_id = 0;
                current_task_title.clear();
                current_task_priority = Priority::LOW;
                current_task_status = Status::TODO;
            }

            "END_PROJECT" => {
                let project = Project::new(
                    current_project_id,
                    current_project_name.clone(),
                    current_project_description.clone(),
                    current_tasks.clone(),
                );

                projects.push(project);

                current_project_name.clear();
                current_project_description.clear();
            }

            _ => {
                let (key, value) = match line.split_once('=') {
                    Some(pair) => pair,
                    None => continue,
                };

                match key {
                    // PROJECT FIELDS
                    "id" if !inside_task => {
                        current_project_id =
                            value.parse().unwrap_or(0);
                    }

                    "name" => {
                        current_project_name =
                            value.to_string();
                    }

                    "description" => {
                        current_project_description =
                            value.to_string();
                    }

                    // TASK FIELDS
                    "id" if inside_task => {
                        current_task_id =
                            value.parse().unwrap_or(0);
                    }

                    "title" => {
                        current_task_title =
                            value.to_string();
                    }

                    "priority" => {
                        current_task_priority =
                            Priority::to_priority(value)
                                .unwrap_or(Priority::LOW);
                    }

                    "status" => {
                        current_task_status =
                            Status::to_status(value)
                                .unwrap_or(Status::TODO);
                    }

                    "due_date" => {
                        let due_date = if value.is_empty() {
                            None
                        } else {
                            Some(value.to_string())
                        };

                        let task = Task::new(
                            current_task_id,
                            current_task_title.clone(),
                            current_task_priority.clone(),
                            current_task_status.clone(),
                            due_date,
                        );

                        current_tasks.push(task);

                        inside_task = false;
                    }

                    _ => {}
                }
            }
        }
    }

    projects
}
