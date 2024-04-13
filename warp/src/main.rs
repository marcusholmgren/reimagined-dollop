use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;
use warp::Filter;
use warp_web::{get_employee_name, EmployeeMap};

#[tokio::main]
async fn main() {
    let employees: EmployeeMap = Arc::new(AsyncMutex::new(HashMap::new()));

    {
        let mut guard = employees.lock().await;
        guard.insert(1, "Alice".to_string());
        guard.insert(2, "Bob".to_string());
        guard.insert(3, "Charlie".to_string());
    }

    let get_employee_name_route = warp::path!("employees" / u32)
        .and(warp::get())
        .and(warp::any().map(move || employees.clone()))
        .and_then(get_employee_name);

    warp::serve(get_employee_name_route)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
