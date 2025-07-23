# Cues CLI
**A tool for devs to manage your tasks at your home (terminal)**

## Project Overview

**Cues** is a multi-platform task and project management platform aimed at making task management for your projects as simple and easy as possible, with a CLI to access your tasks right from your terminal, while also providing a web UI for users who prefer a visual dashboard view.

### Problem
There are many existing platforms that provide excellent features to manage projects and tasks effectively (some including `Linear`, `Todoist`, etc.). However, one issue I personally faced, despite wanting to use such a platform, was that it would become a hassle to constantly switch between the terminal and the browser to develop software and creating/marking off tasks.

In order to solve this, I thought of developing a similar task and project management platform, except it comes with a **CLI** to enhance the productivity flow of developers who live in their terminal.

## Tech Stack
This CLI is completely written in **Rust**, with the following tools:
- `chrono`: for date and time handling
- `clap`: for argument parsing while running the CLI
- `colored`: for colored outputs
- `directories`: for fetching config directory based on user OS
- `keyring`: to securely store access and refresh tokens for authentication
- `reqwest`: to make API requests and communicate with the backend
- `serde` & `serde_json`: to handle and serialize/deserialize data from user and backend
- `tokio`: for async Rust and concurrency to make the CLI more efficient

## Installation

**Cues** can be directly installed with a single command (for *Windows* and *Linux*).

If you are on **Linux**, run the following command:
```bash
curl -fsSL https://cues-web.vercel.app/linux/install.sh | bash
```

For **Windows** users, run:
```bash
curl -L https://cues-web.vercel.app/windows/install.bat -o install.bat && install.bat
```

> **NOTE**: For Windows users. If it says `cues is not recognized...` after the installation, try restarting your system once.

### Manual Setup
**Cues** is currently not supported for single-command installation on **Mac** systems. However, the CLI can be manually set up in any OS.

#### Requirements:
Since the CLI is built in **Rust**, so you need to have **rustc** and **cargo** installed on your system.

#### Steps
To build the CLI manually, first clone the Git repository:
```bash
git clone https://github.com/aether-flux/cues-cli
cd cues-cli
```

Then, run the following command to directly run the CLI:
```bash
cargo run
```

Or the following, to build an executable binary for your OS:
```bash
cargo build --release
```

This will create the executable binary (eg, `cues.exe` (Windows) or `cues` (Linux)) in the `/target/release` directory.

## CLI Reference

The Cues CLI provides powerful command-line tools for managing your tasks and projects.

### Global Options

All commands support these global options:

- `--help, -h` - Show help information
- `--version, -V` - Show version number

### Authentication Commands

#### `cues login`
Log in to your *Cues* account in the CLI.

```bash
cues login
```

#### `cues logout`
Log out from any account logged in locally.

```bash
cues logout
```

#### `cues whoami`
Display information about currently logged in user.

```bash
cues whoami
```

## Project Commands

#### `cues projects`
List all projects of the logged in user, with *project id* and *name*.

```bash
cues projects
```

#### `cues use`
Selects a project to be used as default project locally.

```bash
cues use <id>
```

#### Example:
```bash
# Will set the currently active project as the project with id 2
cues use 2
```

#### `cues cwp` / `cues active` / `cues current`
Display the project currently set as active project.

> **CWP**: Current Working Project

```bash
cues cwp
```

If it's confusing to use `cwp`, you can also use:
```bash
cues active
```
Or
```bash
cues current
```

#### `cues new`
Create a new project.

```bash
cues new project <name>
```

#### Example:
```bash
cues new project "Learning JS"
```

### Task Commands

#### `cues tasks`
Lists all available tasks, with its *id*, *title*, *description*, *priority*, *due date* and *todo progress (done/not done)*.

By default, displays tasks in the **CWP** (Current Working Project).

#### Options
- `--all, -a` - Displays all tasks, grouped by projects

#### Example
To list tasks in CWP:
```bash
cues tasks
```

To list all tasks, grouped by project:
```bash
cues tasks --all
```

#### `cues add`
Add a new task.

#### Options
- `--desc, -d` - Task description (optional)
- `--priority, -p` - Task priority, high/medium/low (optional)
- `--due, -u` - Due date and time of completion (optional)

The `--due` flag can be passed in any of the following formats:
- `--due "today 18:00"` - Due by today, 6pm
- `--due "tomorrow 4:00"` - Due by tomorrow, 4am
- `--due "thursday 17:30"` - Due by upcoming Thursday, 5:30pm
- `--due "20/08/2025 9:15"` - Due by 20th August, 2025, 9:15am

#### Example
To create a task with no optional fields:
```bash
cues add "Task Name"
```

To add any field, specify the flag and then the corresponding value:
```bash
cues add "DOM Manipulation" -d "Learn about basics of DOM Manipulation" -p medium -u "today 18:00"
```

#### `cues done`
Mark any task as done.

```bash
cues done <id>
```

#### Example:
To mark task with id *14* as done:
```bash
cues done 14
```

> You can get the id of a task using the `cues tasks` command.

#### `cues edit`
Edit the contents of a task.

#### Options
- `--title, -t` - Task title (optional)
- `--desc, -d` - Task description (optional)
- `--priority, -p` - Task priority, high/medium/low (optional)
- `--due, -u` - Due date and time of completion (optional)

#### Example:
You may pass any combination of the flags that you may want to edit. For example, if you want to edit just the task title:
```bash
cues edit <id> -t "New title"
```

Or if you wish to edit the priority and due date:
```bash
cues edit <id> -p low -u "tomorrow 17:30"
```

#### `cues delete`
Delete a task.

```bash
cues delete <id>
```

#### Example:
To delete the task with id *14*:
```bash
cues delete 14
```

### Examples

#### Daily Workflow

```bash
# Morning: Check today's tasks
cues tasks

# Create a new task
cues add "Review PR #42" --priority medium --due "today 10:00"

# Mark completed tasks as done
cues done 123
```

#### Project Setup

```bash
# Create new project
cues new project "Mobile App Redesign"

# Suppose the new project created has id 54

# Switch to project
cues use 54

# Add initial tasks
cues add "Design wireframes" --priority high
cues add "Set up React Native project" --priority high
cues add "Configure CI/CD" --priority medium
```

## Interested?
- Visit the [web](https://cues-web.vercel.app) version of **Cues** to explore a familiar dashboard-based task and project management experience.
- Check out the [backend repository](https://github.com/aether-flux/cues-backend) if you want to take a look at the internals.

## Support
If you liked the project and it turned out actually useful for you, consider supporting me at:
[![buymeacoffee-badge](https://img.shields.io/badge/aetherflux-ffdd00?style=for-the-badge&logo=buymeacoffee&logoColor=1a1a1a)](https://buymeacoffee.com/aetherflux).

## License
This project is licensed under **MIT**.

