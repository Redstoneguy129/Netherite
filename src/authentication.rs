use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Agent {
    name: String,
    version: i32
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthenticatePayload {
    agent: Agent,
    username: String,
    password: String,
    client_token: Option<String>,
    request_user: Option<bool>
}

pub fn authenticate(username: String, password: String) {
    let agent = Agent {
        name: "Minecraft".to_string(),
        version: 1
    };
    let payload = AuthenticatePayload {
        agent,
        username: username.to_owned(),
        password: password.to_owned(),
        client_token: Option::from(Uuid::new_v4().to_string()).to_owned(),
        request_user: Option::from(true).to_owned(),
    };
    println!("{:?}", serde_json::to_string_pretty(&payload).unwrap());
}