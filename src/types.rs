pub enum MenuContext {
    Main,
    NewProject,
    ProjectWithTasks,
    EditProject,
    FilterTask,
    EditTask,
}

#[derive(Debug)]
pub struct Project {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub tasks: Vec<Task>
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub priority: Priority,
    pub status: Status,
    pub due_date: Option<String>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Priority{
    HIGH,
    MEDIUM,
    LOW
}

impl Priority {
   pub fn to_str(&self) -> &str{
        match self {
            Priority::HIGH => "High",
                Priority::MEDIUM => "Medium",
            Priority::LOW => "Low"
        }
    }
   pub fn to_priority(priority: &str) -> Option<Priority> {
       match priority.to_lowercase().as_str() {
           "high" => Some(Priority::HIGH),
           "medium" => Some(Priority::MEDIUM),
           "low" => Some(Priority::LOW),
           _ => None
       }
   }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    InProgress,
    TODO,
    Done,
}

impl Status {
    pub fn to_str(&self) -> &str {
        match self {
            Status::TODO => "TODO",
            Status::InProgress => "In Progress",
            Status::Done => "Done"
        }
    }
    pub fn to_status(status: &str) -> Option<Self> {
        match status.to_lowercase().as_str() {
            "todo" => Some(Status::TODO),
            "inprogress" | "in progress" | "in_progress" => Some(Status::InProgress),
            "done" => Some(Status::Done),
            _ => None
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
