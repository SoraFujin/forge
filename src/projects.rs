use crate::types::Project;

pub fn new_project(projects: &mut Vec<Project>, name: String, description: String) {
    let next_id = projects.len() as u32 + 1;
    let project = Project::new(next_id, name, description, Vec::new());
    projects.push(project);
}
