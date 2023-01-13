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
    None,
    WayOutside,
    JustOutside,
    MissesTheZone,
    Adjective(PitchAdjective),
    DoesNotChase,
}

#[derive(Debug, Copy, Clone)]
pub enum StrikeFlavor {
    None,
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
    IntoPlay,
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
            FieldLocation::IntoPlay => { write!(f, "into play") }
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

#[derive(Debug, Clone)]
pub struct Contact {
    pub sound_effect: Option<SoundEffect>,
    pub batter: PlayerDesc,
    pub verb: ContactVerb,
    pub pitch_descriptor: Option<PitchDescriptor>,
    pub location: FieldLocation,
}

impl Display for Contact {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(sound_effect) = self.sound_effect {
            write!(f, "{sound_effect}! ")?;
        }

        if self.location == FieldLocation::IntoPlay {
            write!(f, "{} {} into play...", self.batter.name, self.verb)
        } else if let Some(descriptor) = self.pitch_descriptor {
            write!(f, "{} {} {} {}...", self.batter.name, self.verb, descriptor, self.location)
        } else {
            write!(f, "{} {} it to {}...", self.batter.name, self.verb, self.location)
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
                    BallFlavor::None => { format!("Ball. {count}")  }
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