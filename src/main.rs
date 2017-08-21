#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

use rocket_contrib::{Json, Value};
use std::collections::HashMap;
use reqwest::StatusCode;

#[derive(Deserialize, Debug)]
struct PushToken {
    value: String,
}

#[derive(Deserialize, Debug)]
struct RequestData {
    token: PushToken,
}

#[post("/", format = "application/json", data = "<request_data>")]
fn index(request_data: Json<RequestData>) -> Json<Value> {
    let push_token = request_data.token.value.clone();
    let mut notification = HashMap::new();
    notification.insert("to", push_token);
    notification.insert("body", "Hello here from Rust land".to_owned());
    notification.insert("sound", "default".to_owned());

    let payload = vec![notification];
    let client = reqwest::Client::new().unwrap();
    let expo_api_response = client
        .post("https://exp.host/--/api/v2/push/send")
        .unwrap()
        .json(&payload)
        .unwrap()
        .send()
        .unwrap();

    println!("{:?}", expo_api_response);

    let response = match expo_api_response.status() {
        StatusCode::Ok => json!({"success": true}),
        _ => json!({"success": false}),
    };

    Json(response)
}

#[post("/ping")]
fn ping() -> Json<Value> {
    Json(json!({
        "response": "pong"
    }))
}

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, ping])
        .catch(errors![not_found])
        .launch();
}
