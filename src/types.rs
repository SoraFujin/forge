pub enum MenuContext {
    Main,
    NewProject,
    ProjectWithTasks,
    NewTask,
    FilterTask,
    EditTask
}
pub struct Project {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub tasks: Vec<Task>
}

pub struct Task {
    pub id: u32,
    pub title: String,
    pub priority: Priority,
    pub status: Status,
    pub due_date: Option<String>
}

pub enum Priority{
    HIGH,
    MEDIUM,
    LOW
}


impl Priority {
    fn to_str(&self) -> &str{
        match self {
            Priority::HIGH => "High",
                Priority::MEDIUM => "Medium",
            Priority::LOW => "Low"
        }
    }
}

pub enum Status {
    InProgress,
    TODO,
    Done,
}

impl Status {
    fn to_str(&self) -> &str {
        match self {
            Status::TODO => "TODO",
            Status::InProgress => "In Progress",
            Status::Done => "Done"
        }
    }
}

impl Project {
    pub fn new(id: u32, name: String, description: String, tasks: Vec<Task>) -> Self {
        Self {
            id,
            name,
            description,
            tasks
        }
    }
}

impl Task {
    pub fn new(id: u32, title: String, priority: Priority, status: Status, due_date: Option<String>) -> Self {
        Self {
            id,
            title,
            priority,
            status,
            due_date
        }
    }
}
