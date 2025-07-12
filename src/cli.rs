use clap::{Parser, Subcommand};

// Defining the CLI Struct
#[derive(Parser)]
#[command(name = "cues", version, about = "A todo list cli")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

// Sub commands here
#[derive(Subcommand)]
pub enum Commands {

    // COMMANDS: Projects
    Projects,
    Use {
        pid: u32,
    },
    #[command(aliases=["current", "active"])]
    Cwp,
    New {
        #[command(subcommand)]
        kind: NewProject
    },

    // COMMANDS: Tasks
    Add {
        title: String,

        #[arg(short, long, help="Task priority", value_enum)]
        priority: Option<Priority>,

        #[arg(short, long, help="Task description")]
        desc: Option<String>,

        #[arg(short='u', long, help="Task due date & time")]
        due: Option<String>,
    },
    Tasks {
        #[arg(short, long, help="List tasks in all projects")]
        all: bool,
    },
    Done {
        task_id: u32,
    },
    Edit {
        task_id: u32,

        #[arg(short, long, help="Task title")]
        title: Option<String>,

        #[arg(short, long, help="Task priority", value_enum)]
        priority: Option<Priority>,

        #[arg(short, long, help="Task description")]
        desc: Option<String>,

        #[arg(short='u', long, help="Task due date & time")]
        due: Option<String>,

        #[arg(short='D', long, help="Task done status")]
        done: Option<bool>,
    },
    Delete {
        task_id: u32,
    },

    // COMMANDS: Authentication
    Login,
    Logout,
    Whoami,
}

#[derive(Subcommand)]
pub enum NewProject {
    Project {
        name: String,
    }
}

#[derive(clap::ValueEnum, Clone)]
pub enum Priority {
    High,
    Medium,
    Low,
}
