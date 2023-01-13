mod parser;
mod chron_schema;
mod text_parsers;
mod fed_schema;

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use chrono::format::parse;
use itertools::Itertools;
use serde::Deserialize;
use uuid::Uuid;

use crate::chron_schema::*;
use crate::parser::Parser;


async fn async_main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let response = reqwest::get("https://api2.sibr.dev/chronicler/v0/game-events?count=1000").await?;
    let json: Vec<GameResponse> = response.json().await?;

    let groups = json.into_iter()
        .group_by(|item| item.timestamp);

    let mut game_parsers: HashMap<_, Parser> = HashMap::new();
    let mut pending_lines_for_game: HashMap<_, Vec<_>> = HashMap::new();
    for (_, group) in &groups {
        let mut group = group.collect_vec();
        group.sort_by_key(|item| item.data.display_order);
        for game in group {
            println!("For game {} at {}", game.game_id, game.timestamp);
            println!("    Input: {:?}", game.data);

            let mut parser = game_parsers.entry(game.game_id).or_default();
            let (parsed, state) = parser.parse(game.data.clone())?;

            let mut pending_lines = pending_lines_for_game.entry(game.game_id).or_default();
            pending_lines.push(game.data.display_text);
            if let Some(parsed) = parsed {
                println!("    Output: {parsed:?}");
                let reconstructed_description = parsed.lines(state)?;
                assert_eq!(pending_lines, &reconstructed_description);
                pending_lines.clear();
            }

        }
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