# Discord Bot Template (Rust)
Uses Poise package for slash command functionality & Serenity package for general bot functionality.

PickleDB package included for json-style local storage (see `src/utils/db.rs`)

## Instructions:
Add .env file to root directory

Sample .env format:
```
BOT=testing_bot

TESTING_BOT=tokentokentokentokentokentoken
LIVE_BOT=tokentokentokentokentokentoken

DATA_PATH=data/
STATIC_PATH=static/
```
Use `BOT` to set the name of the desired active bot.

Register each bot token in `src/utils/env.rs::discord_token()`

## To run bot:
Root Directory: `cargo run`
