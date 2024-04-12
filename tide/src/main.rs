use tide::Request;
//use tide::prelude::*;

#[derive(serde::Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
}

async fn hello(req: tide::Request<()>) -> tide::Result {
    println!("Request: {:?}", req);
    Ok("Hello, world!".into())
}

// create_user that accepts a JSON body of User type
// and returns the user as JSON
// curl -X POST -H "Content-Type: application/json" -d '{"name": "Alice", "age": 30}' http://localhost:8080/api/users

async fn create_user(mut req: Request<()>) -> tide::Result {
    let user: User = req.body_json().await?;

    if user.name.is_empty() {
        let mut res = tide::Response::new(tide::StatusCode::BadRequest);
        res.set_body("Name is required");
        return Ok(res);
    }


    Ok(format!("User created {:?}", user).into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/").get(hello);
    app.at("/api/users").post(create_user);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
