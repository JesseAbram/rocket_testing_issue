use rocket::local::asynchronous::Client;

use crate::world;
use rocket::http::Status;

#[rocket::async_test]
async fn hello_world() {
    let client = Client::tracked(crate::rocket().await)
        .await
        .expect("valid `Rocket`");
    let _client_2 = create_clients(3002).await;
    let response = client.get("/hello/world").dispatch().await;
    // assert_eq!(response.into_string(), Some("Hello, world!".into()));
    assert_eq!(response.status(), Status::Ok);
}

async fn create_clients(port: i64) -> rocket::local::asynchronous::Client {
    let config = rocket::Config::figment().merge(("port", port));

    Client::tracked(rocket::custom(config).mount("/hello", routes![world]))
        .await
        .expect("valid `Rocket`")
}
