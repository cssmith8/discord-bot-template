#![allow(dead_code)]
pub fn discord_token() -> String {
    let bot_name = std::env::var("BOT").expect("Must set BOT variable in .env");
    match bot_name.to_lowercase().as_str() {
        "name" => std::env::var("NAME").expect("Token for NAME not set"),
        _ => panic!("Unknown bot specified in .env: [{}] Edit bot names in src/utils/env.rs", bot_name),
    }
}

pub fn data_path() -> String {
    std::env::var("DATA_PATH").unwrap_or_else(|_| "data/".into())
}

pub fn static_path() -> String {
    std::env::var("STATIC_PATH").unwrap_or_else(|_| "static/".into())
}