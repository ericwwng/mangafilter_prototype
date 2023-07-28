use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Credentials<'a> {
    username: &'a str,
    password: &'a str,
}

#[derive(Deserialize, Debug)]
struct ApiError {
    /// The error id.
    id: i64,

    /// The error status.
    status: i32,

    /// The error title.
    title: Option<String>,

    /// Details about the error.
    detail: Option<String>,
}

fn main() {
    let creds = Credentials {
        username: "EmmKay",
        password: "eric1275678",
    };

    let base_url = "https://api.mangadex.org";
    let login_url = format!("{}{}", base_url, "/auth/login");

    let login_body = json!(creds);

    let mut login_body_map = HashMap::new();
    login_body_map.insert("username", "EmmKay");
    login_body_map.insert("password", "eric1275678");

    println!("{}, {:?}, {:?}", &login_url, &login_body, &login_body_map);

    let client = reqwest::blocking::Client::new();

    let res = client.post(login_url).json(&login_body_map).send().unwrap();

    println!("{:?}", res.text());

    //let res_json: ApiError = res.expect("api error").json().unwrap();

    //println!("{:?}", res_json);
}
