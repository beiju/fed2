use anyhow::anyhow;
use uuid::Uuid;
use crate::chron_schema::{GameUpdate, PlayerDesc, State, TeamAtBat};

#[derive(Debug)]
pub enum BallFlavor {
    None,
    WayOutside,
}

#[derive(Debug)]
pub enum StrikeFlavor {
    None,
    NamedPitcher,
}

#[derive(Debug)]
pub enum Event {
    PlayBall,
    BatterUp,
    Ball(BallFlavor),
    Strike(StrikeFlavor),
}

impl Event {
    pub fn lines(&self, state: &State) -> anyhow::Result<Vec<String>> {
        Ok(match self {
            Event::PlayBall => {
                vec!["Play Ball!".to_string()]
            }
            Event::BatterUp => {
                let batter = state.batter.as_ref()
                    .ok_or_else(|| anyhow!("Expected non-null batter in BatterUp event"))?;
                vec![format!("{} steps up to bat.", batter.name)]
            }
            Event::Ball(flavor) => {
                let flavor_text = match flavor {
                    BallFlavor::None => { "Ball." }
                    BallFlavor::WayOutside => { "Ball, way outside." }
                };
                vec![
                    format!("{flavor_text} {}-{}", state.balls, state.strikes),
                    String::new(),
                ]
            }
            Event::Strike(flavor) => {
                let pitcher = state.pitcher.as_ref()
                    .ok_or_else(|| anyhow!("Expected non-null pitcher in a BatterUp event"))?;

                let flavor_text = match flavor {
                    StrikeFlavor::None => { "Strike.".to_string() }
                    StrikeFlavor::NamedPitcher => { format!("{} throws a strike.", pitcher.name) }
                };
                vec![
                    format!("{flavor_text} {}-{}.", state.strikes, state.strikes),
                    String::new(),
                ]
            }
        })
    }
}