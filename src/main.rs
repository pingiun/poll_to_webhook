extern crate reqwest;
extern crate serde_json;

use std::env;

fn main() {
    run().unwrap();
}

fn run() -> Result<(), reqwest::Error> {
    let token = env::var("TG_TOKEN").unwrap();
    let timeout = env::var("TG_TIMEOUT").unwrap_or("20".to_string());
    let mut offset = 0;

    let webhook = env::var("TG_WEBHOOK").unwrap_or("http://127.0.0.1:8080/_telegram".to_string());
    let client = reqwest::Client::new();

    loop {
        let url = format!("https://api.telegram.org/bot{}/getUpdates?timeout={}&offset={}", token, timeout, offset);

        println!("{:?}", url);

        let mut resp = reqwest::get(&url)?;
        assert!(resp.status().is_success());

        let body: serde_json::Value = resp.json()?;

        println!("body = {:?}", body);

        if body["ok"].as_bool().unwrap() {
            let n_results = body["result"].as_array().unwrap().len();

            for ref update in body["result"].as_array().unwrap() {
                let res = client.post(&webhook).body(update.to_string()).send()?;
                assert!(res.status().is_success());
            }

            if n_results > 0 {
                offset = body["result"][n_results - 1]["update_id"].as_u64().unwrap() + 1;
            }
        }
    }
}