#[macro_use]
extern crate rocket;
use rocket::routes;

#[cfg(test)]
mod tests;

#[get("/world")]
async fn world() -> &'static str {
    let route = "http://127.0.0.1:3002/hello/test".to_owned();
    reqwest::get(route).await.unwrap().text().await.unwrap();
    "Hello, world!"
}

#[get("/test")]
async fn test() {

}


#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/hello", routes![world, test])
}
