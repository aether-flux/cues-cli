use reqwest::{header::{USER_AGENT, HeaderMap, HeaderValue}, Client};
use serde_json::Value;

// Base URL for backend API
const BASE_URL: &str = "http://localhost:5000/api";

// SECTION - Project Management

// GET: All projects
pub async fn get_projects (token: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let res = Client::new()
        .get(format!("{}/projects", BASE_URL))
        .bearer_auth(token)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(res)
}

// GET: One project
pub async fn get_uniq_proj (token: &str, id: &u32) -> Result<Value, Box<dyn std::error::Error>> {
    let res = Client::new()
        .get(format!("{}/projects/{}", BASE_URL, id))
        .bearer_auth(token)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(res)
}

// POST: New project
pub async fn post_project (token: &str, payload: &Value) -> Result<Value, Box<dyn std::error::Error>> {
    let res = Client::new()
        .post(format!("{}/projects/new", BASE_URL))
        .bearer_auth(token)
        .json(payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(res)
}

// PUT: Update project
pub async fn put_project (token: &str, id: &u32, payload: &Value) -> Result<Value, Box<dyn std::error::Error>> {
    let res = Client::new()
        .put(format!("{}/projects/{}", BASE_URL, id))
        .bearer_auth(token)
        .json(payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(res)
}

// DELETE: Delete project
pub async fn delete_project (token: &str, id: &u32) -> Result<Value, Box<dyn std::error::Error>> {
    let res = Client::new()
        .delete(format!("{}/projects/{}", BASE_URL, id))
        .bearer_auth(token)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(res)
}

// SECTION - Task Management

// GET: All tasks
pub async fn get_tasks (token: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let res = Client::new()
        .get(format!("{}/tasks", BASE_URL))
        .bearer_auth(token)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(res)
}

// POST: New task
pub async fn post_task (token: &str, payload: &Value) -> Result<Value, Box<dyn std::error::Error>> {
    let res = Client::new()
        .post(format!("{}/tasks/new", BASE_URL))
        .bearer_auth(token)
        .json(payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(res)
}

// PUT: Update task
pub async fn put_task (token: &str, id: &u32, payload: &Value) -> Result<Value, Box<dyn std::error::Error>> {
    let res = Client::new()
        .put(format!("{}/tasks/{}", BASE_URL, id))
        .bearer_auth(token)
        .json(payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(res)
}

// DELETE: Delete task
pub async fn delete_task (token: &str, id: &u32) -> Result<Value, Box<dyn std::error::Error>> {
    let res = Client::new()
        .delete(format!("{}/tasks/{}", BASE_URL, id))
        .bearer_auth(token)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(res)
}

// SECTION - Authentication

// GET: User details
pub async fn get_user (token: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let res = Client::new()
        .get(format!("{}/auth/user", BASE_URL))
        .bearer_auth(token)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(res)
}

// POST: Log in
pub async fn post_login (payload: &Value) -> Result<Value, Box<dyn std::error::Error>> {
    let res = Client::new()
        .post(format!("{}/auth/login", BASE_URL))
        .header(USER_AGENT, "Cues-CLI")
        .json(payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(res)
}

// POST: Refresh access token
pub async fn post_refresh (payload: &Value) -> Result<Value, Box<dyn std::error::Error>> {
    let res = Client::new()
        .post(format!("{}/auth/refresh", BASE_URL))
        .header(USER_AGENT, "Cues-CLI")
        .json(payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(res)
}
