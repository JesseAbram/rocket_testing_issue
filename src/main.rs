#[macro_use]
extern crate rocket;
use rocket::routes;

#[cfg(test)]
mod tests;

#[get("/world")]
async fn world() -> &'static str {
    let mut route = "http://127.0.0.1/hello/world".to_owned();
    reqwest::get(route).await.unwrap().text().await.unwrap();
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/hello", routes![world])
}
