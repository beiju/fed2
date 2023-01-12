use std::fmt::{Display, Formatter};
use anyhow::anyhow;
use uuid::Uuid;
use crate::chron_schema::{GameUpdate, PlayerDesc, State, TeamAtBat};

#[derive(Debug)]
pub enum BallFlavor {
    None,
    WayOutside,
    JustOutside,
    MissesTheZone,
}

#[derive(Debug)]
pub enum StrikeFlavor {
    None,
    ThrowsAStrike,
    CaughtLooking,
}

#[derive(Debug)]
pub enum Event {
    PlayBall,
    BatterUp,
    Ball(BallFlavor),
    Strike(StrikeFlavor),
    FieldingOut,
}

struct Count(i64, i64);

impl Display for Count {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
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
                let count = Count(state.balls, state.strikes);
                let pitcher = state.pitcher.as_ref()
                    .ok_or_else(|| anyhow!("Expected non-null pitcher in a BatterUp event"))?;
                // let batter = state.batter.as_ref()
                //     .ok_or_else(|| anyhow!("Expected non-null batter in a BatterUp event"))?;
                let text = match flavor {
                    BallFlavor::None => { format!("Ball. {count}")  }
                    BallFlavor::WayOutside => { format!("Ball, way outside. {count}") }
                    BallFlavor::JustOutside => { format!("Ball, just outside. {count}.") }
                    BallFlavor::MissesTheZone => { format!("{} just misses the zone. Ball, {count}.", pitcher.name) }
                };
                vec![text, String::new()]
            }
            Event::Strike(flavor) => {
                let count = Count(state.balls, state.strikes);
                let pitcher = state.pitcher.as_ref()
                    .ok_or_else(|| anyhow!("Expected non-null pitcher in a BatterUp event"))?;
                let batter = state.batter.as_ref()
                    .ok_or_else(|| anyhow!("Expected non-null batter in a BatterUp event"))?;

                let text = match flavor {
                    StrikeFlavor::None => { format!("Strike, {count}.") }
                    StrikeFlavor::ThrowsAStrike => { format!("{} throws a strike. {count}.", pitcher.name) }
                    StrikeFlavor::CaughtLooking => { format!("{} is caught looking. Strike, {count}.", batter.name) }
                };
                vec![text, String::new()]
            }
            Event::FieldingOut => {
                vec![
                    "BAM! Ji-Eun Jasper slaps it to Left Field...".to_string(),
                    "Fly out to Jay Camacho.".to_string(),
                ]
            }
        })
    }
}