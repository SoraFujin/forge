use crate::types::{Priority, Status, Task};

pub fn new_task(tasks: &mut Vec<Task>, title: String, priority: Priority, status: Status, due_date: Option<String>) {
    let next_id = tasks.len() as u32 + 1;
    let task = Task::new(next_id, title, priority, status, due_date);
    tasks.push(task);
}

pub fn find_task(tasks: &[Task], task_id: u32) {
    for task in tasks {
        if task.id == task_id {
            match &task.due_date {
                Some(date) => {
                    println!("Title: {}, Priority: {:?}, Status: {:?}, Due Date: {}", task.title, task.priority, task.status, date);
                    break
                },
                None => {
                    println!("Title: {}, Priority: {:?}, Status: {:?}", task.title, task.priority, task.status);
                    break
                }
            }
        }
    }
}

pub fn edit_title(tasks: &mut [Task], task_id: u32, new_title: String) {
    for task in tasks.iter_mut() {
        if task.id == task_id {
            task.title = new_title.to_string();
            break;
        }
    }
}

pub fn edit_status(tasks: &mut [Task], task_id: u32, new_status: Status) {
    for task in tasks.iter_mut() {
        if task.id == task_id {
            task.status = new_status;
            break
        }
    }
}

pub fn edit_priority(tasks: &mut [Task], task_id: u32, new_priority: Priority) {
    for task in tasks.iter_mut() {
        if task.id == task_id {
            task.priority = new_priority;
            break
        }
    }
}

pub fn edit_due_date(tasks: &mut [Task], task_id: u32, new_date: String) {
    for task in tasks.iter_mut() {
        if task.id == task_id {
            task.due_date = Some(new_date);
            break;
        }
    }
}

pub fn filter_status(tasks: &[Task], status_filter: &str) {
    let status = match Status::to_status(status_filter.to_lowercase().as_str()) {
        Some(status) => status,
        None =>{
            eprint!("[Error] from utils/tasks.rs: Cannot parse to status enum");
            return
        }
    };
    for task in tasks {
        if task.status == status {
            match &task.due_date {
                Some(date) => println!("id: {}, Title: {}, Priority: {:?}, Status: {:?}, Due date: {}", task.id, task.title, task.priority, task.status, date),
                None => println!("id: {}, Title: {}, Priority: {:?}, Status: {:?}", task.id, task.title, task.priority, task.status)
            }
        } 
    }
}

pub fn filter_priority(tasks: &[Task], priority_filter: &str) {
    let priority = match Priority::to_priority(priority_filter.to_lowercase().as_str()) {
        Some(priority) => priority,
        None => {
            eprintln!("[Error] from utils/tasks.rs: Cannot parse to Priority enum");
            return
        }
    };
    for task in tasks {
        if task.priority == priority {
            match &task.due_date {
                Some(date) => println!("id: {}, Title: {}, Priority: {:?}, Status: {:?}, Due date: {}", task.id, task.title, task.priority, task.status, date),
                None => println!("id: {}, Title: {}, Priority: {:?}, Status: {:?}", task.id, task.title, task.priority, task.status)
            }
        }
    }
}

 pub fn print_tasks(tasks: &[Task]) {
    for task in tasks {
        match &task.due_date {
            Some(date) => println!("id: {}, Title: {}, Priority: {:?}, Status: {:?}, Due date: {}", task.id, task.title, task.priority, task.status, date),
            None => println!("id: {}, Title: {}, Priority: {:?}, Status: {:?}", task.id, task.title, task.priority, task.status)
        }
    }
}
