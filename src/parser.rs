use nom_supreme::error::ErrorTree;
use nom_supreme::final_parser::{final_parser, Location};
use nom::Parser as NomParser;
use nom_supreme::tag::complete::tag;
use crate::chron_schema::GameUpdateDelta;
use crate::fed_schema::Event;

#[derive(Debug, Default)]
enum ParserExpectedEvent {
    #[default]
    GameStart,
}

#[derive(Debug, Default)]
pub struct Parser {
    next_event_genre: ParserExpectedEvent,
}

fn run_parser_generic<'a, T>(
    expression: impl NomParser<&'a str, T, ErrorTree<&'a str>>,
) -> impl FnMut(&'a str) -> Result<T, ErrorTree<Location>> {
    final_parser(expression)
}


impl Parser {
    pub fn parse(&mut self, delta: GameUpdateDelta) -> anyhow::Result<Event> {
        Ok(match self.next_event_genre {
            ParserExpectedEvent::GameStart => {
                run_parser_generic(tag("Play Ball!"))(&delta.display_text)?;
                Event::PlayBall
            }
        })
    }
}