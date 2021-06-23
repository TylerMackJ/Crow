use std::env;

#[test]
fn api_key() {
    match env::var("API") {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn api_secret_key() {
    match env::var("API_SECRET") {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn bearer_key() {
    match env::var("BEARER") {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
}