use std::fmt::{Display, Formatter};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Default, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TeamAtBat {
    #[default]
    Away,
    Home,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDesc {
    pub id: Uuid,
    pub name: String,
}

impl Display for PlayerDesc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.replace('\'', "&#x27;"))
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateDelta {
    #[serde(default, skip_serializing_if = "Option::is_none", with = "::serde_with::rust::double_option")]
    pub batter: Option<Option<PlayerDesc>>,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "::serde_with::rust::double_option")]
    pub defenders: Option<Option<Vec<PlayerDesc>>>,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "::serde_with::rust::double_option")]
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
    pub defenders: Option<Vec<PlayerDesc>>,
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
