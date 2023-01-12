use anyhow::anyhow;
use nom::bytes::complete::tag;
use nom_supreme::error::ErrorTree;
use nom_supreme::final_parser::{final_parser, Location};
use nom::Parser as NomParser;
use nom::sequence::pair;
use crate::chron_schema::{GameUpdate, GameUpdateDelta, State, TeamAtBat};
use crate::fed_schema::Event;
use crate::text_parsers::{parse_ball, parse_strike, parse_strikeout};

#[derive(Debug, Default)]
enum ParserExpectedEvent {
    Invalid,
    #[default]
    GameStart,
    BatterUp,
    Pitch,
    PostPitchEmpty(Event),
    Contact,
}

#[derive(Debug, Default)]
pub struct Parser {
    next_event_genre: ParserExpectedEvent,
    state: State,
    last_update: String,
}

fn run_parser<'a, T>(
    expression: impl NomParser<&'a str, T, ErrorTree<&'a str>>,
) -> impl FnMut(&'a str) -> Result<T, ErrorTree<Location>> {
    final_parser(expression)
}


impl Parser {
    pub fn parse(&mut self, delta: GameUpdateDelta) -> anyhow::Result<(Option<Event>, &State)> {
        let prev_state = self.state.clone();
        self.state.update(delta.changed_state);
        let event = match std::mem::replace(&mut self.next_event_genre, ParserExpectedEvent::Invalid) {
            ParserExpectedEvent::Invalid => {
                return Err(anyhow!("Parser is in the Invalid state"))
            }
            ParserExpectedEvent::GameStart => {
                run_parser(tag("Play Ball!"))(&delta.display_text)?;
                self.next_event_genre = ParserExpectedEvent::BatterUp;
                Some(Event::PlayBall)
            }
            ParserExpectedEvent::BatterUp => {
                let batter = self.state.batter.as_ref()
                    .ok_or_else(|| anyhow!("Expected non-null batter in a BatterUp event"))?;

                run_parser(pair(tag(batter.name.as_str()), tag(" steps up to bat.")))(&delta.display_text)?;
                self.next_event_genre = ParserExpectedEvent::Pitch;
                Some(Event::BatterUp)
            }
            ParserExpectedEvent::Pitch => {
                if self.state.balls == prev_state.balls + 1 {
                    // Ball event
                    let pitcher = self.state.pitcher.as_ref()
                        .ok_or_else(|| anyhow!("Expected non-null pitcher in a Ball event"))?;
                    // let batter = self.state.batter.as_ref()
                    //     .ok_or_else(|| anyhow!("Expected non-null batter in a BatterUp event"))?;
                    let ball_flavor = run_parser(parse_ball(self.state.balls, self.state.strikes, &pitcher.name))(&delta.display_text)?;
                    self.next_event_genre = ParserExpectedEvent::PostPitchEmpty(Event::Ball(ball_flavor));
                    None
                } else if self.state.strikes == prev_state.strikes + 1 {
                    // Strike event
                    let pitcher = self.state.pitcher.as_ref()
                        .ok_or_else(|| anyhow!("Expected non-null pitcher in a Strike event"))?;
                    let batter = self.state.batter.as_ref()
                        .ok_or_else(|| anyhow!("Expected non-null batter in a Strike event"))?;

                    let strike_flavor = run_parser(parse_strike(self.state.balls, self.state.strikes, &pitcher.name, &batter.name))(&delta.display_text)?;
                    self.next_event_genre = ParserExpectedEvent::PostPitchEmpty(Event::Strike(strike_flavor));
                    None
                } else if self.state.outs == prev_state.outs + 1 {
                    // The only way to get an out without an intermediate event is a strikeout
                    let pitcher = self.state.pitcher.as_ref()
                        .ok_or_else(|| anyhow!("Expected non-null pitcher in a Strike event"))?;
                    // Batter gets cleared from current state
                    let batter = prev_state.batter.as_ref()
                        .ok_or_else(|| anyhow!("Expected non-null batter before a Strike event"))?;

                    run_parser(parse_strikeout(&pitcher.name, &batter.name))(&delta.display_text)?;
                    self.next_event_genre = ParserExpectedEvent::BatterUp;
                    Some(Event::Strikeout(batter.clone()))
                } else {
                    // TODO
                    run_parser(tag("BAM! Ji-Eun Jasper slaps it to Left Field..."))(&delta.display_text)?;
                    self.next_event_genre = ParserExpectedEvent::Contact;
                    None
                }
            }
            ParserExpectedEvent::PostPitchEmpty(event) => {
                run_parser(tag(""))(&delta.display_text)?;

                self.next_event_genre = ParserExpectedEvent::Pitch;
                Some(event)
            }
            ParserExpectedEvent::Contact => {
                run_parser(tag("Fly out to Jay Camacho."))(&delta.display_text)?;
                self.next_event_genre = ParserExpectedEvent::BatterUp;
                Some(Event::FieldingOut)
            }
        };

        Ok((event, &self.state))
    }
}