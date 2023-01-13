use std::fmt::{Display, Formatter, write};
use anyhow::anyhow;
use uuid::Uuid;
use crate::chron_schema::{GameUpdate, PlayerDesc, State, TeamAtBat};

#[derive(Debug, Copy, Clone)]
pub enum PitchAdjective {
    Auspicious,
    Average,
    Disgusting,
    Dominant,
    Favorable,
    Horrible,
    Marvelous,
    Overpowering,
    Potent,
    Powerful,
    Revolting,
    WellLocated,
    WellPlaced,
}

impl Display for PitchAdjective {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PitchAdjective::Auspicious => { write!(f, "Auspicious") }
            PitchAdjective::Average => { write!(f, "Average") }
            PitchAdjective::Disgusting => { write!(f, "Disgusting") }
            PitchAdjective::Dominant => { write!(f, "Dominant") }
            PitchAdjective::Favorable => { write!(f, "Favorable") }
            PitchAdjective::Horrible => { write!(f, "Horrible") }
            PitchAdjective::Marvelous => { write!(f, "Marvelous") }
            PitchAdjective::Overpowering => { write!(f, "Overpowering") }
            PitchAdjective::Potent => { write!(f, "Potent") }
            PitchAdjective::Powerful => { write!(f, "Powerful") }
            PitchAdjective::Revolting => { write!(f, "Revolting") }
            PitchAdjective::WellLocated => { write!(f, "Well-located") }
            PitchAdjective::WellPlaced => { write!(f, "Well-placed") }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BallFlavor {
    // whyyyyyy
    BallPeriod,
    BallComma,
    WayOutside,
    JustOutside,
    MissesTheZone,
    Adjective(PitchAdjective),
    DoesNotChase,
}

#[derive(Debug, Copy, Clone)]
pub enum StrikeFlavor {
    None,
    Looking,
    ThrowsAStrike,
    CaughtLooking,
    Chases,
}

#[derive(Debug, Copy, Clone)]
pub enum SoundEffect {
    Bam,
    Boom,
    Crack,
    Smack,
    Smash,
    Thwack,
    Wham,
}

impl Display for SoundEffect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SoundEffect::Bam => { write!(f, "BAM") }
            SoundEffect::Boom => { write!(f, "BOOM") }
            SoundEffect::Crack => { write!(f, "CRACK") }
            SoundEffect::Smack => { write!(f, "SMACK") }
            SoundEffect::Smash => { write!(f, "SMASH") }
            SoundEffect::Thwack => { write!(f, "THWACK") }
            SoundEffect::Wham => { write!(f, "WHAM") }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ContactVerb {
    Bats,
    Chops,
    Clips,
    Drags,
    Dribbles,
    Hits,
    Knocks,
    Nudges,
    Pokes,
    Punches,
    Pushes,
    Rolls,
    Slaps,
    Smacks,
    Sputters,
    Swats,
    Taps,
    Thumps,
    Trickles,
    Whacks,
}

impl Display for ContactVerb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContactVerb::Bats => { write!(f, "bats") }
            ContactVerb::Chops => { write!(f, "chops") }
            ContactVerb::Clips => { write!(f, "clips") }
            ContactVerb::Drags => { write!(f, "drags") }
            ContactVerb::Dribbles => { write!(f, "dribbles") }
            ContactVerb::Hits => { write!(f, "hits") }
            ContactVerb::Knocks => { write!(f, "knocks") }
            ContactVerb::Nudges => { write!(f, "nudges") }
            ContactVerb::Pokes => { write!(f, "pokes") }
            ContactVerb::Punches => { write!(f, "punches") }
            ContactVerb::Pushes => { write!(f, "pushes") }
            ContactVerb::Rolls => { write!(f, "rolls") }
            ContactVerb::Slaps => { write!(f, "slaps") }
            ContactVerb::Smacks => { write!(f, "smacks") }
            ContactVerb::Sputters => { write!(f, "sputters") }
            ContactVerb::Swats => { write!(f, "swats") }
            ContactVerb::Taps => { write!(f, "taps") }
            ContactVerb::Thumps => { write!(f, "thumps") }
            ContactVerb::Trickles => { write!(f, "trickles") }
            ContactVerb::Whacks => { write!(f, "whacks") }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PitchDescriptor {
    It,
    One,
    TheBall,
    ThePitch,
}

impl Display for PitchDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PitchDescriptor::It => { write!(f, "it toward") }
            PitchDescriptor::One => { write!(f, "one to") }
            PitchDescriptor::TheBall => { write!(f, "the ball to") }
            PitchDescriptor::ThePitch => { write!(f, "the pitch to") }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FieldLocation {
    Infield,
    LeftField,
    DeepLeftField,
    CenterField,
    DeepCenterField,
    RightField,
    DeepRightField,
    Wall,
}

impl Display for FieldLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldLocation::Infield => { write!(f, "the Infield") }
            FieldLocation::LeftField => { write!(f, "Left Field") }
            FieldLocation::DeepLeftField => { write!(f, "Deep Left Field") }
            FieldLocation::CenterField => { write!(f, "Center Field") }
            FieldLocation::DeepCenterField => { write!(f, "Deep Center Field") }
            FieldLocation::RightField => { write!(f, "Right Field") }
            FieldLocation::DeepRightField => { write!(f, "Deep Right Field") }
            FieldLocation::Wall => { write!(f, "the Wall") }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ContactAdjective {
    Decent,
    Depressing,
    Hard,
    Sad,
    Solid,
    Strong,
    Weak,
}

impl Display for ContactAdjective {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContactAdjective::Decent => { write!(f, "decent") }
            ContactAdjective::Depressing => { write!(f, "depressing") }
            ContactAdjective::Hard => { write!(f, "hard") }
            ContactAdjective::Sad => { write!(f, "sad") }
            ContactAdjective::Solid => { write!(f, "solid") }
            ContactAdjective::Strong => { write!(f, "strong") }
            ContactAdjective::Weak => { write!(f, "weak") }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ContactFlavor {
    NamedWithSound {
        sound_effect: SoundEffect,
        verb: ContactVerb,
    },
    Named {
        verb: ContactVerb,
        pitch_descriptor: PitchDescriptor,
    },
    Adjective {
        adjective: ContactAdjective,
    }
}

#[derive(Debug, Clone)]
pub struct Contact {
    pub batter: PlayerDesc,
    pub location: Option<FieldLocation>,
    pub flavor: ContactFlavor
}

impl Display for Contact {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.flavor {
            ContactFlavor::NamedWithSound { sound_effect, verb } => {
                write!(f, "{sound_effect}! {} {verb} it ", self.batter.name)?;
                if let Some(location) = self.location {
                    write!(f, "to {location}...")
                } else {
                    write!(f, "into play...")
                }
            }
            ContactFlavor::Named { verb, pitch_descriptor } => {
                write!(f, "{} {verb} ", self.batter.name)?;
                if let Some(location) = self.location {
                    write!(f, "{pitch_descriptor} {location}...")
                } else {
                    write!(f, "it into play...")
                }
            }
            ContactFlavor::Adjective { adjective } => {
                write!(f, "A {adjective} hit ")?;
                if let Some(location) = self.location {
                    write!(f, "to {location}...")
                } else {
                    write!(f, "into play...")
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FoulFlavor {
    FoulBall,
    FoulTip,
    FoulsItBack,
    FoulsItOff,
}

#[derive(Debug)]
pub enum Event {
    PlayBall,
    BatterUp,
    Ball(BallFlavor),
    Strike(StrikeFlavor),
    FieldingOut {
        contact: Contact,
        defender: PlayerDesc,
    },
    Strikeout(PlayerDesc),
    Foul(FoulFlavor),
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
                    .ok_or_else(|| anyhow!("Expected non-null pitcher in a Ball event"))?;
                let batter = state.batter.as_ref()
                    .ok_or_else(|| anyhow!("Expected non-null batter in a Ball event"))?;
                let text = match flavor {
                    BallFlavor::BallPeriod => { format!("Ball. {count}")  }
                    BallFlavor::BallComma => { format!("Ball, {count}")  }
                    BallFlavor::WayOutside => { format!("Ball, way outside. {count}") }
                    BallFlavor::JustOutside => { format!("Ball, just outside. {count}.") }
                    BallFlavor::MissesTheZone => { format!("{} just misses the zone. Ball, {count}.", pitcher.name) }
                    BallFlavor::DoesNotChase => { format!("{} does not chase. Ball, {count}.", batter.name) }
                    BallFlavor::Adjective(adj) => { format!("{adj} pitch. Ball, {count}.") }
                };
                vec![text, String::new()]
            }
            Event::Strike(flavor) => {
                let count = Count(state.balls, state.strikes);
                let pitcher = state.pitcher.as_ref()
                    .ok_or_else(|| anyhow!("Expected non-null pitcher in a Strike event"))?;
                let batter = state.batter.as_ref()
                    .ok_or_else(|| anyhow!("Expected non-null batter in a Strike event"))?;

                let text = match flavor {
                    StrikeFlavor::None => { format!("Strike, {count}.") }
                    StrikeFlavor::Looking => { format!("Strike, looking. {count}.") }
                    StrikeFlavor::ThrowsAStrike => { format!("{} throws a strike. {count}.", pitcher.name) }
                    StrikeFlavor::CaughtLooking => { format!("{} is caught looking. Strike, {count}.", batter.name) }
                    StrikeFlavor::Chases => { format!("{} chases. Strike, {count}.", batter.name) }
                };
                vec![text, String::new()]
            }
            Event::FieldingOut { contact, defender } => {
                vec![
                    contact.to_string(),
                    format!("Fly out to {}.", defender.name)
                ]
            }
            Event::Strikeout(batter) => {
                let pitcher = state.pitcher.as_ref()
                    .ok_or_else(|| anyhow!("Expected non-null pitcher in a Strikeout event"))?;
                vec![
                    format!("{} strikes {} out.", pitcher.name, batter.name)
                ]
            }
            Event::Foul(flavor) => {
                let count = Count(state.balls, state.strikes);
                let batter = state.batter.as_ref()
                    .ok_or_else(|| anyhow!("Expected non-null batter in a Foul event"))?;

                let text = match flavor {
                    FoulFlavor::FoulBall => { format!("Foul ball. {count}.") }
                    FoulFlavor::FoulTip => { format!("Foul tip. {count}.") }
                    FoulFlavor::FoulsItBack => { format!("{} fouls it back. {count}.", batter.name) }
                    FoulFlavor::FoulsItOff => { format!("{} fouls it off. {count}.", batter.name) }
                };
                vec![text, String::new()]
            }
        })
    }
}