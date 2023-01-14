use anyhow::anyhow;
use nom::bytes::complete::tag;
use nom_supreme::error::ErrorTree;
use nom_supreme::final_parser::{final_parser, Location};
use nom::Parser as NomParser;
use nom::sequence::pair;
use crate::chron_schema::{GameUpdate, GameUpdateDelta, PlayerDesc, State, TeamAtBat};
use crate::fed_schema::{Contact, Event, FailedFielding, Fielding};
use crate::text_parsers::*;

#[derive(Debug, Default)]
enum ParserExpectedEvent {
    Invalid,
    #[default]
    GameStart,
    BatterUp,
    Pitch,
    PostPitchEmpty(Event),
    PostAppearanceEmpty(Event),
    Contact(Contact),
    Fielding(Contact, Fielding),
    FailedFielding(Contact, FailedFielding),
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
                return Err(anyhow!("Parser is in the Invalid state"));
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
                    let batter = self.state.batter.as_ref()
                        .ok_or_else(|| anyhow!("Expected non-null batter in a BatterUp event"))?;
                    let ball_flavor = run_parser(parse_ball(
                        self.state.balls, self.state.strikes, &pitcher.name, &batter.name,
                    ))(&delta.display_text)?;
                    self.next_event_genre = ParserExpectedEvent::PostPitchEmpty(Event::Ball(ball_flavor));
                    None
                } else if self.state.strikes == prev_state.strikes + 1 {
                    // Strike or Foul event
                    let pitcher = self.state.pitcher.as_ref()
                        .ok_or_else(|| anyhow!("Expected non-null pitcher in a Strike/Foul event"))?;
                    let batter = self.state.batter.as_ref()
                        .ok_or_else(|| anyhow!("Expected non-null batter in a Strike/Foul event"))?;

                    let parsed = run_parser(parse_strike_or_foul(self.state.balls, self.state.strikes, &pitcher.name, &batter.name))(&delta.display_text)?;
                    let event = match parsed {
                        ParsedStrikeOrFoul::Strike(flavor) => { Event::Strike(flavor) }
                        ParsedStrikeOrFoul::Foul(flavor) => { Event::Foul(flavor) }
                    };
                    self.next_event_genre = ParserExpectedEvent::PostPitchEmpty(event);
                    None
                } else if self.state.outs == prev_state.outs + 1 {
                    // The only way to get an out without an intermediate event is a strikeout
                    let pitcher = self.state.pitcher.as_ref()
                        .ok_or_else(|| anyhow!("Expected non-null pitcher in a Strike event"))?;
                    // Batter gets cleared from current state
                    let batter = prev_state.batter.as_ref()
                        .ok_or_else(|| anyhow!("Expected non-null batter before a Strike event"))?;

                    let flavor = run_parser(parse_strikeout(&pitcher.name, &batter.name))(&delta.display_text)?;
                    self.next_event_genre = ParserExpectedEvent::BatterUp;
                    Some(Event::Strikeout {
                        batter: batter.clone(),
                        flavor,
                    })
                } else {
                    // Batter gets cleared from current state
                    let batter = prev_state.batter.as_ref()
                        .ok_or_else(|| anyhow!("Expected non-null batter before a Foul/Contact sub-event"))?;

                    let parsed = run_parser(parse_foul_or_contact(
                        self.state.balls, self.state.strikes, &batter.name,
                    ))(&delta.display_text)?;
                    match parsed {
                        ParsedFoulOrContact::Foul(flavor) => {
                            self.next_event_genre = ParserExpectedEvent::PostPitchEmpty(Event::Foul(flavor));
                            None
                        }
                        ParsedFoulOrContact::Contact((flavor, location)) => {
                            self.next_event_genre = ParserExpectedEvent::Contact(Contact {
                                batter: batter.clone(),
                                flavor,
                                location,
                            });
                            None
                        }
                    }
                }
            }
            ParserExpectedEvent::PostPitchEmpty(event) => {
                run_parser(tag(""))(&delta.display_text)?;

                self.next_event_genre = ParserExpectedEvent::Pitch;
                Some(event)
            }
            ParserExpectedEvent::PostAppearanceEmpty(event) => {
                run_parser(tag(""))(&delta.display_text)?;

                self.next_event_genre = ParserExpectedEvent::BatterUp;
                Some(event)
            }
            ParserExpectedEvent::Contact(contact) => {
                if self.state.outs == prev_state.outs + 1 {
                    let defenders = prev_state.defenders.as_ref()
                        .ok_or_else(|| anyhow!("Expected non-null defenders after Contact"))?;

                    let (defender, flavor) = run_parser(parse_flyout(&defenders))(&delta.display_text)?;
                    self.next_event_genre = ParserExpectedEvent::BatterUp;
                    Some(Event::Flyout {
                        contact,
                        defender: defender.clone(),
                        flavor,
                    })
                } else {
                    let defenders = prev_state.defenders.as_ref()
                        .ok_or_else(|| anyhow!("Expected non-null defenders after Contact"))?;

                    let parsed = run_parser(parse_post_contact(&contact.batter, &defenders))(&delta.display_text)?;
                    match parsed {
                        ParsedPostContact::HomeRun => {
                            // TODO If there are runners, expect scores
                            self.next_event_genre = ParserExpectedEvent::PostAppearanceEmpty(Event::HomeRun {
                                contact,
                            });
                            None
                        }
                        ParsedPostContact::Fielding(defender, flavor) => {
                            self.next_event_genre = ParserExpectedEvent::Fielding(contact, Fielding {
                                defender,
                                flavor,
                            });
                            None
                        }
                        ParsedPostContact::FailedFielding(defender, flavor) => {
                            self.next_event_genre = ParserExpectedEvent::FailedFielding(contact, FailedFielding {
                                defender,
                                flavor,
                            });
                            None
                        }
                    }
                }
            }
            ParserExpectedEvent::Fielding(contact, fielding) => {
                if self.state.outs == prev_state.outs + 1 {
                    // Fielding out
                    run_parser(parse_groundout(&fielding.defender))(&delta.display_text)?;
                    self.next_event_genre = ParserExpectedEvent::BatterUp;
                    Some(Event::GroundOut {
                        contact,
                        defender: fielding.defender,
                        flavor: fielding.flavor,
                    })
                } else {
                    todo!()
                }
            }
            ParserExpectedEvent::FailedFielding(contact, fielding) => {
                let (hit_type, flavor) = run_parser(parse_base_hit(&contact.batter.name))(&delta.display_text)?;
                self.next_event_genre = ParserExpectedEvent::BatterUp;
                Some(Event::Hit {
                    contact,
                    fielding,
                    hit_type,
                    flavor,
                })
            }
        };

        Ok((event, &self.state))
    }
}