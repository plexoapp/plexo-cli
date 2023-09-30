use chrono::NaiveDate;
use clap::{arg, command, Args, Parser, Subcommand, ValueEnum};
use uuid::Uuid;

/// Plexo CLI tool.
#[derive(Parser)]
#[command(name = "plexo-cli")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommands,
}

#[derive(Subcommand)]

pub enum CliCommands {
    /// Login to your Plexo instance.
    Login,
    /// List one or more resources.
    Get { resource: Resource },
    /// Create one or more resources from a file or stdin.
    Create {
        #[command(subcommand)]
        resource: ResourceCreate,
        // #[arg(long, short)]
        // interactive: bool,
    },
    /// Set resources either from a file, stdin, or specifying label selectors, names, resource selectors, or resources.
    Set,
    /// Delete resources either from a file, stdin, or specifying label selectors, names, resource selectors, or resources.
    Delete,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq)]
pub enum Resource {
    Tasks,
    Projects,
    Teams,
    Members,
    Labels,
}

#[derive(Clone, Debug, PartialEq, Subcommand)]
pub enum ResourceCreate {
    // #[command(name = "tasks")]
    Task(TaskCreate),
}

#[derive(Args, Clone, Debug, PartialEq)]
pub struct TaskCreate {
    // #[arg(short, long)]
    /// The title of the task.
    pub title: String,
    /// The description of the task.
    #[arg(short, long)]
    pub description: Option<String>,
    /// The priority of the task.
    #[arg(short, long)]
    pub priority: Option<TaskPriority>,
    /// The status of the task.
    #[arg(short, long)]
    pub status: Option<TaskStatus>,
    /// The due date of the task.
    /// Format: YYYY-MM-DD
    #[arg(short = 'D', long)]
    pub due_date: Option<NaiveDate>,
    /// The project ID of the task.
    #[arg(short = 'P', long)]
    pub project_id: Option<Uuid>,
    /// The parent ID of the task.
    #[arg(short = 'r', long)]
    pub parent_id: Option<Uuid>,
    /// The Leader ID of the task.
    #[arg(short, long)]
    pub lead_id: Option<Uuid>,
    /// Labels of the task.
    pub labels: Option<Vec<String>>,

    // / The assignee IDs of the task.
    // pub assignee_ids: Option<Vec<Uuid>>,
    /// Create the task interactively.
    #[arg(short, long, default_value = "false")]
    pub interactive: bool,
}

#[derive(ValueEnum, Copy, Clone, PartialEq, Debug)]
pub enum TaskStatus {
    // #[default]
    // None,
    Backlog,
    ToDo,
    InProgress,
    Done,
    Canceled,
}

#[derive(ValueEnum, Copy, Clone, PartialEq, Debug)]
pub enum TaskPriority {
    // #[default]
    // None,
    Low,
    Medium,
    High,
    Urgent,
}
