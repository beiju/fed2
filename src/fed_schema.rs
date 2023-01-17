use std::fmt::{Debug, Display, Formatter, write};
use anyhow::anyhow;
use uuid::Uuid;
use crate::chron_schema::{GameUpdate, PlayerDesc, RunnerDesc, State, TeamAtBat};

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
    DoesntBlink,
    JustMisses,
    LaysOffOutside,
    LooksAtBallOutside,
    MissesBigTime,
    Stumbles,
    ThrowsOutside,
    Adjective(PitchAdjective),
    BallComma,
    ExtremelyOutside,
    JustOutside,
    WayOutside,
    // whyyyyyy
    BallPeriod,
    MissesTheZone,
    DoesNotChase,
}

#[derive(Debug, Copy, Clone)]
pub enum SwingAdjective {
    Pathetic,
    Poor,
    Sad,
    Weak,
}

impl Display for SwingAdjective {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SwingAdjective::Pathetic => { write!(f, "pathetic") }
            SwingAdjective::Poor => { write!(f, "poor") }
            SwingAdjective::Sad => { write!(f, "sad") }
            SwingAdjective::Weak => { write!(f, "weak") }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum StrikeFlavor {
    None,
    Looking,
    Swinging,
    ThrowsAStrike,
    CaughtLooking,
    Chases,
    GuessesWrong,
    AdjectiveSwing(SwingAdjective),
    DropsItIn,
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
    },
}

#[derive(Debug, Clone)]
pub struct Contact {
    pub batter: PlayerDesc,
    pub location: Option<FieldLocation>,
    pub flavor: ContactFlavor,
}

impl Display for Contact {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.flavor {
            ContactFlavor::NamedWithSound { sound_effect, verb } => {
                write!(f, "{sound_effect}! {} {verb} it ", self.batter)?;
                if let Some(location) = self.location {
                    write!(f, "to {location}...")
                } else {
                    write!(f, "into play...")
                }
            }
            ContactFlavor::Named { verb, pitch_descriptor } => {
                write!(f, "{} {verb} ", self.batter)?;
                if let Some(location) = self.location {
                    write!(f, "{pitch_descriptor} {location}...")
                } else {
                    write!(f, "the pitch into play...")
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CatchAdjective {
    Cool,
    Decent,
    Diving,
    Fine,
    Good,
    Lazy,
    Nice,
    Poor,
    Satisfactory,
    Simple,
    Solid,
}

impl Display for CatchAdjective {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CatchAdjective::Cool => { write!(f, "cool") }
            CatchAdjective::Decent => { write!(f, "decent") }
            CatchAdjective::Diving => { write!(f, "diving") }
            CatchAdjective::Fine => { write!(f, "fine") }
            CatchAdjective::Good => { write!(f, "good") }
            CatchAdjective::Lazy => { write!(f, "lazy") }
            CatchAdjective::Nice => { write!(f, "nice") }
            CatchAdjective::Poor => { write!(f, "poor") }
            CatchAdjective::Satisfactory => { write!(f, "satisfactory") }
            CatchAdjective::Simple => { write!(f, "simple") }
            CatchAdjective::Solid => { write!(f, "solid") }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FlyoutFlavor {
    FlyOutTo,
    IsRightThere,
    MakesCatch,
    MakesCatchWithAdjective(CatchAdjective),
}

#[derive(Debug, Copy, Clone)]
pub enum GroundoutFlavor {
    GroundOutTo,
    HitsAGroundout,
    ForcedOutAtFirst,
}

#[derive(Debug, Copy, Clone)]
pub enum FieldingFlavor {
    ChargesForIt,
    CollectsIt,
    CorralsIt,
    DashesForIt,
    DivesForIt,
    // can this precede a flyout?
    FieldsIt,
    GetsInFrontOfIt,
    GetsIt,
    GoesForIt,
    HasABeadOnIt,
    IsThereToCollectIt,
    IsThereToCorralIt,
    IsThereToFieldIt,
    IsThereToGetIt,
    IsThereToScoopIt,
    IsThereToSecureIt,
    LurchesForIt,
    RacesForIt,
    RacesIn,
    RacesTowardIt,
    ReachesForIt,
    RunsForIt,
    ScoopsIt,
    SecuresIt,
    TracksItDown,
    TriesForIt,
}

impl Display for FieldingFlavor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldingFlavor::ChargesForIt => { write!(f, "charges for it...") }
            FieldingFlavor::CollectsIt => { write!(f, "collects it...") }
            FieldingFlavor::CorralsIt => { write!(f, "corrals it...") }
            FieldingFlavor::DashesForIt => { write!(f, "dashes for it...") }
            FieldingFlavor::DivesForIt => { write!(f, "dives for it...") }
            FieldingFlavor::FieldsIt => { write!(f, "fields it...") }
            FieldingFlavor::GetsInFrontOfIt => { write!(f, "gets in front of it...") }
            FieldingFlavor::GetsIt => { write!(f, "gets it...") }
            FieldingFlavor::GoesForIt => { write!(f, "goes for it...") }
            FieldingFlavor::HasABeadOnIt => { write!(f, "has a bead on it...") }
            FieldingFlavor::IsThereToCollectIt => { write!(f, "is there to collect it...") }
            FieldingFlavor::IsThereToCorralIt => { write!(f, "is there to corral it...") }
            FieldingFlavor::IsThereToFieldIt => { write!(f, "is there to field it...") }
            FieldingFlavor::IsThereToGetIt => { write!(f, "is there to get it...") }
            FieldingFlavor::IsThereToScoopIt => { write!(f, "is there to scoop it...") }
            FieldingFlavor::IsThereToSecureIt => { write!(f, "is there to secure it...") }
            FieldingFlavor::LurchesForIt => { write!(f, "lurches for it...") }
            FieldingFlavor::RacesForIt => { write!(f, "races for it...") }
            FieldingFlavor::RacesIn => { write!(f, "races in...") }
            FieldingFlavor::RacesTowardIt => { write!(f, "races toward it...") }
            FieldingFlavor::ReachesForIt => { write!(f, "reaches for it...") }
            FieldingFlavor::RunsForIt => { write!(f, "runs for it...") }
            FieldingFlavor::ScoopsIt => { write!(f, "scoops it...") }
            FieldingFlavor::SecuresIt => { write!(f, "secures it...") }
            FieldingFlavor::TracksItDown => { write!(f, "tracks it down...") }
            FieldingFlavor::TriesForIt => { write!(f, "tries for it...") }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Fielding {
    pub defender: PlayerDesc,
    pub flavor: FieldingFlavor,
}

impl Display for Fielding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.defender, self.flavor)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FailedFieldingFlavor {
    BobblesIt,
    CantCollectIt,
    CantCorralIt,
    CantFieldIt,
    CantGetIt,
    CantMakeTheCatch,
    CantScoopIt,
    CantSecureIt,
    DropsIt,
    IsLateGettingThere,
    JustMissesTheCatch,
    LosesIt,
}

impl Display for FailedFieldingFlavor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FailedFieldingFlavor::BobblesIt => { write!(f, "bobbles it!") }
            FailedFieldingFlavor::CantCollectIt => { write!(f, "can't collect it...") }
            FailedFieldingFlavor::CantCorralIt => { write!(f, "can't corral it...") }
            FailedFieldingFlavor::CantFieldIt => { write!(f, "can't field it...") }
            FailedFieldingFlavor::CantGetIt => { write!(f, "can't get it...") }
            FailedFieldingFlavor::CantMakeTheCatch => { write!(f, "can't make the catch...") }
            FailedFieldingFlavor::CantScoopIt => { write!(f, "can't scoop it...") }
            FailedFieldingFlavor::CantSecureIt => { write!(f, "can't secure it...") }
            FailedFieldingFlavor::DropsIt => { write!(f, "drops it!") }
            FailedFieldingFlavor::IsLateGettingThere => { write!(f, "is late getting there...") }
            FailedFieldingFlavor::JustMissesTheCatch => { write!(f, "just misses the catch!") }
            FailedFieldingFlavor::LosesIt => { write!(f, "loses it!") }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FailedFielding {
    pub defender: PlayerDesc,
    pub flavor: FailedFieldingFlavor,
}

impl Display for FailedFielding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.defender, self.flavor)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum StrikeoutFlavor {
    NamedBoth,
    NamedBatter,
}

#[derive(Debug, Copy, Clone)]
pub enum HitType {
    Single,
    Double,
    Triple,
}

impl Display for HitType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HitType::Single => { write!(f, "Single") }
            HitType::Double => { write!(f, "Double") }
            HitType::Triple => { write!(f, "Triple") }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum HitFlavor {
    Hits,
    IsOnWith,
}

impl Display for HitFlavor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HitFlavor::Hits => { write!(f, "hits a") }
            HitFlavor::IsOnWith => { write!(f, "is on with a") }
        }
    }
}

#[derive(Debug, Clone)]
pub enum MaybeFailedFielding {
    Fielding(Fielding),
    FailedFielding(FailedFielding),
}

impl Display for MaybeFailedFielding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MaybeFailedFielding::Fielding(fielding) => { write!(f, "{fielding}") }
            MaybeFailedFielding::FailedFielding(fielding) => { write!(f, "{fielding}") }
        }
    }
}

impl Into<MaybeFailedFielding> for Fielding {
    fn into(self) -> MaybeFailedFielding {
        MaybeFailedFielding::Fielding(self)
    }
}

impl Into<MaybeFailedFielding> for FailedFielding {
    fn into(self) -> MaybeFailedFielding {
        MaybeFailedFielding::FailedFielding(self)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Base {
    First,
    Second,
    Third,
}

impl Display for Base {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Base::First => { write!(f, "First") }
            Base::Second => { write!(f, "Second") }
            Base::Third => { write!(f, "Third") }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AdvancementFlavor {
    To,
    AdvancesTo,
}

impl Display for AdvancementFlavor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AdvancementFlavor::To => { write!(f, "to") }
            AdvancementFlavor::AdvancesTo => { write!(f, "advances to") }
        }
    }
}

#[derive(Debug)]
pub struct Advancement {
    pub runner: PlayerDesc,
    pub to_base: Base,
}

#[derive(Debug, Copy, Clone)]
pub enum RunnerAdvancement {
    None,
    Advanced(Base, AdvancementFlavor),
    Scored,
}

#[derive(Debug)]
pub struct RunnerAdvancementDesc {
    pub runner: RunnerDesc,
    pub advancement: RunnerAdvancement,
}

#[derive(Debug, Copy, Clone)]
pub enum WalkFlavor {
    Ball4,
    DrawsWalk,
    EarnsWalk,
}

#[derive(Debug)]
pub enum Event {
    PlayBall,
    BatterUp,
    Ball(BallFlavor),
    Strike(StrikeFlavor),
    Flyout {
        contact: Contact,
        defender: PlayerDesc,
        flavor: FlyoutFlavor,
    },
    GroundOut {
        contact: Contact,
        fielding: Fielding,
        flavor: GroundoutFlavor,
        advancements: Vec<RunnerAdvancementDesc>,
    },
    Strikeout {
        batter: PlayerDesc,
        flavor: StrikeoutFlavor,
    },
    Foul(FoulFlavor),
    HomeRun {
        contact: Contact,
    },
    Hit {
        contact: Contact,
        fielding: MaybeFailedFielding,
        hit_type: HitType,
        flavor: HitFlavor,
        advancements: Vec<Advancement>,
        scores: Vec<PlayerDesc>,
    },
    Walk {
        batter: PlayerDesc,
        flavor: WalkFlavor,
    },
    EndOfHalfInning {
        top_of_inning: bool,
        inning: i64,
    },
    FieldersChoice {
        contact: Contact,
        fielding: Fielding,
        runner_out: RunnerDesc,
    }
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
                vec![format!("{} steps up to bat.", batter)]
            }
            Event::Ball(flavor) => {
                let count = Count(state.balls, state.strikes);
                let pitcher = state.pitcher.as_ref()
                    .ok_or_else(|| anyhow!("Expected non-null pitcher in a Ball event"))?;
                let batter = state.batter.as_ref()
                    .ok_or_else(|| anyhow!("Expected non-null batter in a Ball event"))?;
                let text = match flavor {
                    BallFlavor::BallPeriod => { format!("Ball. {count}.") }
                    BallFlavor::BallComma => { format!("Ball, {count}.") }
                    BallFlavor::WayOutside => { format!("Ball, way outside. {count}") }
                    BallFlavor::JustOutside => { format!("Ball, just outside. {count}.") }
                    BallFlavor::ExtremelyOutside => { format!("Ball, extremely outside. {count}.") }
                    BallFlavor::MissesTheZone => { format!("{} misses the zone. {count}.", pitcher) }
                    BallFlavor::DoesNotChase => { format!("{} does not chase. Ball, {count}.", batter) }
                    BallFlavor::Adjective(adj) => { format!("{adj} pitch. Ball, {count}.") }
                    BallFlavor::DoesntBlink => { format!("{} doesn't blink. {count}.", batter) }
                    BallFlavor::JustMisses => { format!("{} just misses the zone. Ball, {count}.", pitcher) }
                    BallFlavor::LaysOffOutside => { format!("{} lays off outside. {count}.", pitcher) }
                    BallFlavor::LooksAtBallOutside => { format!("{} looks at a ball outside. {count}.", batter) }
                    BallFlavor::MissesBigTime => { format!("{} misses big time. Ball, {count}.", pitcher) }
                    BallFlavor::Stumbles => { format!("{} stumbles. {count}.", pitcher) }
                    BallFlavor::ThrowsOutside => { format!("{} throws it outside. Ball, {count}.", pitcher) }
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
                    StrikeFlavor::Swinging => { format!("Strike, swinging. {count}.") }
                    StrikeFlavor::ThrowsAStrike => { format!("{} throws a strike. {count}.", pitcher) }
                    StrikeFlavor::CaughtLooking => { format!("{} is caught looking. Strike, {count}.", batter) }
                    StrikeFlavor::Chases => { format!("{} chases. Strike, {count}.", batter) }
                    StrikeFlavor::GuessesWrong => { format!("{} guesses wrong. Strike, {count}.", batter) }
                    StrikeFlavor::AdjectiveSwing(adj) => { format!("{} takes a {adj} swing. Strike, {count}.", batter) }
                    StrikeFlavor::DropsItIn => { format!("{} drops it in. Strike, {count}.", pitcher) }
                };
                vec![text, String::new()]
            }
            Event::Flyout { contact, defender, flavor } => {
                let flyout_text = match flavor {
                    FlyoutFlavor::FlyOutTo => { format!("Fly out to {}.", defender) }
                    FlyoutFlavor::IsRightThere => { format!("{} is right there to make the catch.", defender) }
                    FlyoutFlavor::MakesCatch => { format!("{} makes the catch.", defender) }
                    FlyoutFlavor::MakesCatchWithAdjective(adj) => { format!("{} makes a {adj} catch.", defender) }
                };
                vec![contact.to_string(), flyout_text]
            }
            Event::Strikeout { batter, flavor } => {
                let pitcher = state.pitcher.as_ref()
                    .ok_or_else(|| anyhow!("Expected non-null pitcher in a Strikeout event"))?;
                let text = match flavor {
                    StrikeoutFlavor::NamedBoth => { format!("{} strikes {} out.", pitcher, batter) }
                    StrikeoutFlavor::NamedBatter => { format!("{} strikes out.", batter) }
                };
                vec![text]
            }
            Event::Foul(flavor) => {
                let count = Count(state.balls, state.strikes);
                let batter = state.batter.as_ref()
                    .ok_or_else(|| anyhow!("Expected non-null batter in a Foul event"))?;

                let text = match flavor {
                    FoulFlavor::FoulBall => { format!("Foul ball. {count}.") }
                    FoulFlavor::FoulTip => { format!("Foul tip. {count}.") }
                    FoulFlavor::FoulsItBack => { format!("{} fouls it back. {count}.", batter) }
                    FoulFlavor::FoulsItOff => { format!("{} fouls it off. {count}.", batter) }
                };
                vec![text, String::new()]
            }
            Event::GroundOut { contact, fielding, flavor, advancements } => {
                let text = match flavor {
                    GroundoutFlavor::GroundOutTo => { format!("Groundout to {}.", fielding.defender) }
                    GroundoutFlavor::HitsAGroundout => { format!("{} hits a groundout.", contact.batter) }
                    GroundoutFlavor::ForcedOutAtFirst => { format!("{} is forced out at first.", contact.batter) }
                };
                let mut result = vec![
                    contact.to_string(),
                    fielding.to_string(),
                    text,
                ];

                for advancement in advancements {
                    match advancement.advancement {
                        RunnerAdvancement::None => {}
                        RunnerAdvancement::Advanced(base, flavor) => {
                            result.push(format!("{} {flavor} {base}.", advancement.runner));
                        }
                        RunnerAdvancement::Scored => {
                            todo!()
                        }
                    }
                }

                result
            }
            Event::HomeRun { contact } => {
                vec![
                    contact.to_string(),
                    format!("{} hits a Home Run!", contact.batter),
                    String::new(),
                ]
            }
            Event::Hit { contact, fielding, hit_type, flavor, advancements, scores } => {
                let mut lines = vec![
                    contact.to_string(),
                    fielding.to_string(),
                    format!("{} {} {}!", contact.batter, flavor, hit_type),
                ];

                for advancement in advancements {
                    lines.push(format!("{} advances to {}!", advancement.runner, advancement.to_base));
                }

                for runner in scores {
                    lines.push(format!("{runner} scores!"));
                }

                lines
            }
            Event::Walk { batter, flavor } => {
                let walk_text = match flavor {
                    WalkFlavor::Ball4 => { format!("Ball 4. {batter} takes their base.") }
                    WalkFlavor::DrawsWalk => { format!("{batter} draws a walk.") }
                    WalkFlavor::EarnsWalk => { format!("{batter} earns a walk.") }
                };

                vec![walk_text]
            }
            Event::EndOfHalfInning { top_of_inning, inning } => {
                vec![format!("End of the {} of the {}.",
                             if *top_of_inning { "top" } else { "bottom" }, inning + 1)]
            }
            Event::FieldersChoice { contact, fielding, runner_out } => {
                vec![
                    contact.to_string(),
                    fielding.to_string(),
                    format!("{} is forced out at {}.", runner_out, match runner_out.base {
                        1 => "Second",
                        2 => "Third",
                        3 => "Fourth",
                        _ => return Err(anyhow!("Unexpected base in force out"))
                    }),
                    "Fielder's choice.".to_string(),
                ]
            }
        })
    }
}