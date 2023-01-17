use std::fmt::{Display, Formatter};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;
use crate::fed_schema::Base;

#[derive(Debug, Copy, Clone, Default, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TeamAtBat {
    #[default]
    Away,
    Home,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct PlayerDesc {
    pub id: Uuid,
    pub name: String,
}

impl Display for PlayerDesc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.replace('\'', "&#x27;"))
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RunnerDesc {
    pub id: Uuid,
    pub name: String,
    pub base: i64,
}

impl Display for RunnerDesc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.replace('\'', "&#x27;"))
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct StateDelta {
    #[serde(default, skip_serializing_if = "Option::is_none", with = "::serde_with::rust::double_option")]
    pub batter: Option<Option<PlayerDesc>>,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "::serde_with::rust::double_option")]
    pub defenders: Option<Option<Vec<PlayerDesc>>>,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "::serde_with::rust::double_option")]
    pub pitcher: Option<Option<PlayerDesc>>,
    pub baserunners: Option<Vec<RunnerDesc>>,
    pub started: Option<bool>,
    pub team_at_bat: Option<TeamAtBat>,
    pub inning: Option<i64>,
    pub top_of_inning: Option<bool>,
    pub balls: Option<i64>,
    pub strikes: Option<i64>,
    pub outs: Option<i64>,
    pub home_score: Option<f64>,
    pub away_score: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct GameUpdateDelta {
    pub changed_state: StateDelta,
    pub display_delay: i64,
    pub display_order: i64,
    pub display_text: String,
    pub display_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub batter: Option<PlayerDesc>,
    pub defenders: Option<Vec<PlayerDesc>>,
    pub pitcher: Option<PlayerDesc>,
    pub baserunners: Vec<RunnerDesc>,
    pub started: bool,
    pub team_at_bat: TeamAtBat,
    pub inning: i64,
    pub top_of_inning: bool,
    pub balls: i64,
    pub strikes: i64,
    pub outs: i64,
    pub home_score: f64,
    pub away_score: f64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            batter: None,
            defenders: None,
            pitcher: None,
            baserunners: vec![],
            started: false,
            team_at_bat: Default::default(),
            inning: 0,
            top_of_inning: true,
            balls: 0,
            strikes: 0,
            outs: 0,
            home_score: 0.0,
            away_score: 0.0,
        }
    }
}

impl State {
    pub fn update(&mut self, delta: StateDelta) {
        if let Some(val) = delta.batter { self.batter = val; }
        if let Some(val) = delta.defenders { self.defenders = val; }
        if let Some(val) = delta.pitcher { self.pitcher = val; }
        if let Some(val) = delta.baserunners { self.baserunners = val; }
        if let Some(val) = delta.started { self.started = val; }
        if let Some(val) = delta.team_at_bat { self.team_at_bat = val; }
        if let Some(val) = delta.top_of_inning { self.top_of_inning = val; }
        if let Some(val) = delta.inning { self.inning = val; }
        if let Some(val) = delta.balls { self.balls = val; }
        if let Some(val) = delta.strikes { self.strikes = val; }
        if let Some(val) = delta.outs { self.outs = val; }
        if let Some(val) = delta.home_score { self.home_score = val; }
        if let Some(val) = delta.home_score { self.home_score = val; }
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
#[serde(deny_unknown_fields)]
pub struct GameResponse {
    pub game_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub data: GameUpdateDelta,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameEventsResponse {
    pub items: Vec<GameResponse>,
    pub next_page: String,
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
