pub fn discord_token() -> String {
    let bot_name = std::env::var("BOT").expect("BOT environment variable not set");
    match bot_name.to_lowercase().as_str() {
        "rustical" => std::env::var("RUSTICAL").expect("RUSTICAL environment variable not set"),
        "moneymouth" => std::env::var("MONEYMOUTH").expect("MONEYMOUTH environment variable not set"),
        _ => {panic!("Unknown bot specified in .env")},
    }
}

pub fn data_path() -> String {
    std::env::var("DATA_PATH").unwrap_or_else(|_| "data/".into())
}

pub fn static_path() -> String {
    std::env::var("STATIC_PATH").unwrap_or_else(|_| "static/".into())
}

pub fn laptop() -> String {
    std::env::var("LAPTOP").unwrap_or_else(|_| "0".into())
}