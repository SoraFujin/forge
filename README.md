# forge

A CLI project and task tracker written in Rust. Manage your development work from the terminal — create projects, track tasks, filter by priority or status, and export reports.

## What it does

- Create and manage multiple projects
- Add tasks with title, priority, status, and optional due date
- Edit or delete projects and tasks
- Filter tasks by priority or status
- View per-project stats — total tasks, count per status, highest priority pending task
- Export a project report to a `.txt` file
- Auto-saves on exit and loads on startup

## Usage

```bash
cargo run
```

Navigate using the numbered menus:

**Main menu**
```
1. Create a new Project
2. Choose a Project
3. List all Projects
4. Edit a Project
5. Stats
6. Exit
```

**Inside a project**
```
1. Create a new Task
2. List all Tasks
3. Edit a Task
4. Delete Task
5. Filter Tasks
6. Stats
7. Export
8. Exit
```

## Data

Saved automatically to `forge_data.txt` in the current directory on exit, loaded on startup.

## Project structure

```
src/
├── main.rs       # Entry point and main loop
├── menu.rs       # Menu display and input handlers
├── projects.rs   # Project logic (create, edit, delete, stats, export)
├── tasks.rs      # Task logic (create, edit, delete, filter)
├── types.rs      # Structs and enums
├── storage.rs    # File persistence (save/load)
└── utils.rs      # Input helper
```

## Built with

Rust — standard library only, no external crates.
