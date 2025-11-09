use colored::*;
use dotenvy::dotenv;
use std::env;

pub fn load_env() -> () {
    dotenv().ok();
    let app_env =
        env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

    println!("{}", format!("Environment: {}", app_env).green());
}
