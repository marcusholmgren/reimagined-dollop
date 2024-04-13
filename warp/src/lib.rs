use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;
use warp::Reply;

#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    id: u32,
    name: String,
}

// employee database substitute
pub type EmployeeMap = Arc<AsyncMutex<HashMap<u32, String>>>;

async fn fetch_employee_name(id: u32, employees: EmployeeMap) -> Option<String> {
    let guard = employees.lock().await;
    guard.get(&id).cloned()
}

pub async fn get_employee_name(
    id: u32,
    employees: EmployeeMap,
) -> Result<impl Reply, warp::Rejection> {
    match fetch_employee_name(id, employees).await {
        Some(name) => Ok(warp::reply::json(&Employee { id, name })),
        None => Ok(warp::reply::json(&format!(
            "Employee with ID {} not found",
            id
        ))),
    }
}
