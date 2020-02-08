use std::env;
use tokio::runtime::Runtime;
use camunda_client::api::deployment;

fn main() {
    match env::args().nth(1) {
        Some(host) => {
            match env::args().nth(2) {
                Some(file_name) => {
                    let url = host + "/engine-rest/deployment/create";                            
                    let mut rt = Runtime::new().expect("failed to create runtime");
                    let res = rt.block_on(deployment::create(&url, &file_name));
                    println!("{:?}", res);
                }
                None => println!("file name not passed")
            }            
        }
        None => println!("nothing passed")
    }
}
