mod parser;
mod chron_schema;
mod text_parsers;
mod fed_schema;

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::chron_schema::*;
use crate::parser::Parser;


async fn async_main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let response = reqwest::get("https://api2.sibr.dev/chronicler/v0/game-events?count=1000").await?;
    let json: Vec<GameResponse> = response.json().await?;

    let mut game_parsers: HashMap<_, Parser> = HashMap::new();
    for game in json {
        println!("Input: {:?}", game.data);

        let mut parser = game_parsers.entry(game.game_id).or_default();
        let parsed = parser.parse(game.data)?;

        println!("Output: {parsed:?}");
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            async_main().await
        })
}