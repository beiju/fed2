use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TeamAtBat {
    #[default]
    Home,
    Away,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDesc {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateDelta {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option",
    )]
    pub batter: Option<Option<PlayerDesc>>,
    pub defenders: Option<Vec<PlayerDesc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option",
    )]
    pub pitcher: Option<Option<PlayerDesc>>,
    pub started: Option<bool>,
    pub team_at_bat: Option<TeamAtBat>,
    pub balls: Option<i64>,
    pub strikes: Option<i64>,
    pub outs: Option<i64>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameUpdateDelta {
    pub changed_state: StateDelta,
    pub display_delay: i64,
    pub display_order: i64,
    pub display_text: String,
    pub display_time: DateTime<Utc>,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub batter: Option<PlayerDesc>,
    pub defenders: Vec<PlayerDesc>,
    pub pitcher: Option<PlayerDesc>,
    pub started: bool,
    pub team_at_bat: TeamAtBat,
    pub balls: i64,
    pub strikes: i64,
    pub outs: i64,
}

impl State {
    pub fn update(&mut self, delta: StateDelta) {
        if let Some(val) = delta.batter { self.batter = val; }
        if let Some(val) = delta.defenders { self.defenders = val; }
        if let Some(val) = delta.pitcher { self.pitcher = val; }
        if let Some(val) = delta.started { self.started = val; }
        if let Some(val) = delta.team_at_bat { self.team_at_bat = val; }
        if let Some(val) = delta.balls { self.balls = val; }
        if let Some(val) = delta.strikes { self.strikes = val; }
        if let Some(val) = delta.outs { self.outs = val; }
    }
}


#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameUpdate {
    pub state: State,
    pub display_delay: i64,
    pub display_order: i64,
    pub display_text: String,
    pub display_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameResponse {
    pub game_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub data: GameUpdateDelta,
}

impl GameUpdate {
    pub fn new(delta: GameUpdateDelta) -> Self {
        let mut update = Self {
            state: Default::default(),
            display_delay: delta.display_delay,
            display_order: delta.display_order,
            display_text: delta.display_text,
            display_time: delta.display_time,
        };

        update.state.update(delta.changed_state);

        update
    }
}

async fn async_main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let response = reqwest::get("https://api2.sibr.dev/chronicler/v0/game-events?count=1000").await?;
    let json: Vec<GameResponse> = response.json().await?;

    let mut latest_update_for_game: HashMap<Uuid, GameUpdate> = HashMap::new();
    for game in json {
        let full_update = if let Some(update) = latest_update_for_game.get_mut(&game.game_id) {
            update.state.update(game.data.changed_state);
            update.display_delay = game.data.display_delay;
            update.display_order = game.data.display_order;
            update.display_text = game.data.display_text;
            update.display_time = game.data.display_time;
            update
        } else {
            // There doesn't seem to be a more ergonomic way to do this
            latest_update_for_game.entry(game.game_id)
                .or_insert(GameUpdate::new(game.data))
        };

        println!("{full_update:?}");
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