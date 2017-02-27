extern crate screeps_api;
extern crate hyper;
extern crate hyper_rustls;
extern crate dotenv;

use hyper::client::Client;
use hyper::net::HttpsConnector;

fn env(var: &str) -> String {
    dotenv::dotenv().ok();
    match ::std::env::var(var) {
        Ok(value) => value,
        Err(_) => panic!("must have `{}` defined", var),
    }
}

fn opt_env(var: &str) -> bool {
    dotenv::dotenv().ok();
    match ::std::env::var(var) {
        Ok(value) => {
            match value.chars().next() {
                Some('t') => true,
                Some('T') => true,
                Some('1') => true,
                Some(_) => false,
                None => false,
            }
        }
        Err(_) => false,
    }
}

fn create_secure_client() -> hyper::Client {
    Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()))
}

fn logged_in<'a>(client: &'a hyper::Client) -> screeps_api::API<'a> {
    let username = env("SCREEPS_API_USERNAME");
    let password = env("SCREEPS_API_PASSWORD");
    let mut api = screeps_api::API::new(client);

    if let Err(err) = api.login(&screeps_api::LoginDetails::new(username, password)) {
        panic!("Error logging in: {:?}", err);
    }

    api
}

#[test]
fn test_logging_in() {
    if opt_env("NO_AUTH_TESTS") {
        return;
    }
    let client = create_secure_client();
    let _ = logged_in(&client);
}

#[test]
fn test_my_info() {
    if opt_env("NO_AUTH_TESTS") {
        return;
    }
    let client = create_secure_client();
    let mut api = logged_in(&client);

    let _ = api.my_info().unwrap();
}

#[test]
fn test_token_reretrieval() {
    if opt_env("NO_AUTH_TESTS") {
        return;
    }
    let client = create_secure_client();
    let mut api = logged_in(&client);

    let _ = api.my_info().unwrap();

    let _ = api.my_info().unwrap();

    let _ = api.my_info().unwrap();
}

#[test]
fn test_room_overview() {
    if opt_env("NO_AUTH_TESTS") {
        return;
    }
    let client = create_secure_client();
    let mut api = logged_in(&client);

    for &interval in &[8u32, 180u32, 1440u32] {
        // At the time of writing, a room owned by a user who does not have a custom badge.
        api.room_overview("W1N1", interval).unwrap();

        // At time of writing, one of dissi's rooms, a user who has a custom badge.
        api.room_overview("W3N9", interval).unwrap();

        // A room that can't be owned on the official server.
        api.room_overview("W0N0", interval).unwrap();
    }
}
