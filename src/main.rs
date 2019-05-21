use std::env;
use camunda_client::api;

fn main() {
    match env::args().nth(1) {
        Some(d) => {
            let url = "http://127.0.0.1:8080/engine-rest/deployment/create";
            api::deployment::create(url, &d);
        }
        None => println!("Nothing passed!")
    }
}
