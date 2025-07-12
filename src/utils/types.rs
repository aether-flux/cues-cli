use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,

    #[serde(rename="createdAt")]
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub id: u32,
    pub name: String,

    #[serde(rename="userId")]
    pub user_id: u32,

    #[serde(rename="createdAt")]
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PriorityType {
    High,
    Medium,
    Low
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub due: Option<String>,
    pub priority: Option<PriorityType>,

    #[serde(rename="projectId")]
    pub project_id: u32,

    #[serde(rename="isDone")]
    pub is_done: bool,

    #[serde(rename="createdAt")]
    pub created_at: String,
}
