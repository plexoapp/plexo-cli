use chrono::NaiveDate;
use clap::{arg, Args, ValueEnum};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Args, Clone, Debug, PartialEq, Deserialize)]
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

#[derive(ValueEnum, Copy, Clone, PartialEq, Debug, Deserialize)]
pub enum TaskStatus {
    // #[default]
    // None,
    Backlog,
    ToDo,
    InProgress,
    Done,
    Canceled,
}

#[derive(ValueEnum, Copy, Clone, PartialEq, Debug, Deserialize)]
pub enum TaskPriority {
    // #[default]
    // None,
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Args, Clone, Copy, Debug, PartialEq, Deserialize)]
pub struct TaskGet {
    #[arg(short = 't', long)]
    /// Retrieve tasks with teams.
    with_teams: bool,

    #[arg(short = 'm', long)]
    /// Retrieve tasks with members.
    with_members: bool,

    #[arg(short = 'l', long)]
    /// Retrieve tasks with labels.
    with_labels: bool,

    #[arg(short = 'p', long)]
    /// Retrieve tasks with projects.
    with_project: bool,

    #[arg(short = 'r', long)]
    /// Retrieve tasks with parent.
    with_parent: bool,

    #[arg(short = 'c', long)]
    /// Retrieve tasks with children.
    with_children: bool,
}
