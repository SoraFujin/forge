# forge

A CLI project and task tracker written in Rust. Manage your development work from the terminal — create projects, track tasks, filter by priority or status, and export reports.

> ⚠️ Work in progress — actively being developed

## What it does

- Create and manage multiple projects
- Add tasks to projects with title, priority, status, and optional due date
- View all projects and their tasks
- Filter tasks by priority or status
- View per-project stats — total tasks, count per status, highest priority pending task
- Export project reports to a text file
- Auto-saves on exit and loads on startup — no data lost between runs

## Usage

```bash
cargo run
```

Navigate using the numbered menu:

```
1. Create a new Project
2. Choose a Project
3. List All Projects
4. Edit a Project
5. Exit
```

### Creating a project

Enter a name and description. You'll be dropped into the project menu where you can start adding tasks.

### Adding a task

```
Enter task title:
Enter priority [high/medium/low]:
Enter status [todo/inprogress/done]:
Enter due date (optional, press enter to skip):
```

### Filtering tasks

Filter by priority or status to see only what's relevant.

## Data

All data is saved automatically to `forge_data.txt` in the current directory on exit and loaded on startup.

## Project structure

```
src/
├── main.rs       # Entry point and main loop
├── menu.rs       # Menu display and navigation
├── projects.rs   # Project management logic
├── tasks.rs      # Task management logic
├── types.rs      # Structs and enums
└── utils.rs      # Input helpers and file I/O
```

## Built with

Rust — standard library only, no external crates.
