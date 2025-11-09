use colored::*;
use dotenvy::dotenv;
use std::env;

pub fn load_env() {
    dotenv().ok();
    println!("{}", format!("Environment: {}", app_env()).green());
}

pub fn app_env() -> String {
    env::var("APP_ENV").unwrap_or_else(|_| "development".into())
}

pub fn app_port() -> u16 {
    env::var("APP_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .map(|port| {
            if port == 0 {
                8080
            } else {
                port
            }
        })
        .unwrap_or(8080)
}
