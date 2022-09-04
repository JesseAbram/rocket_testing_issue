// use rocket::local::asynchronous::Client;
use std::thread;
use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};
use crate::{world, test};
// use rocket::http::Status;
// use std::process;
use reqwest;

#[rocket::async_test]
async fn hello_world() {
	tokio::spawn(async { create_clients(3001).await; });
	tokio::time::sleep(Duration::from_secs(1)).await;
	tokio::spawn(async { create_clients(3002).await; });
	tokio::time::sleep(Duration::from_secs(1)).await;

	dbg!("here");
	let route = "http://127.0.0.1:3001/hello/world".to_owned();

    let response = reqwest::get(route).await.unwrap().text().await.unwrap();
    // assert_eq!(response.into_string(), Some("Hello, world!".into()));
    // assert_eq!(response.status(), Status::Ok);
	dbg!(response);

}

async fn create_clients(port: i64) {
		let config = rocket::Config::figment().merge(("port", port));

		rocket::custom(config).mount("/hello", routes![world, test])
			.ignite()
			.await
			.unwrap()
			.launch()
			.await
			.expect("valid `Rocket`");
}
