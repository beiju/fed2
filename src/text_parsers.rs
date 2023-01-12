use anyhow::anyhow;
use nom::{Finish, IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until1};
use nom::combinator::{fail, recognize, verify};
use nom::error::ParseError;
use nom::sequence::terminated;
use nom_supreme::error::{BaseErrorKind, ErrorTree};
use nom_supreme::final_parser::{final_parser, Location};

pub(crate) type ParserError<'a> = nom::error::VerboseError<&'a str>;
pub(crate) type ParserResult<'a, Out> = IResult<&'a str, Out, ParserError<'a>>;

pub fn parse_literal<'a, E: ParseError<&'a str>>(literal: &str) -> impl Fn(&'a str) -> IResult<&'a str, (), E> + '_ {
    move |input| tag(literal).map(|_| ()).parse(input)
}


pub(crate) fn parse_terminated(tag_content: &str) -> impl Fn(&str) -> ParserResult<&str> + '_ {
    move |input| {
        let (input, parsed_value) = if tag_content == "." {
            alt((
                // The Kaj Statter Jr. rule
                verify(recognize(terminated(take_until1(".."), tag("."))), |s: &str| !s.contains('\n')),
                verify(take_until1(tag_content), |s: &str| !s.contains('\n')),
            )).parse(input)
        } else {
            verify(take_until1(tag_content), |s: &str| !s.contains('\n')).parse(input)
        }?;
        let (input, _) = tag(tag_content).parse(input)?;

        Ok((input, parsed_value))
    }
}

// This is for use in place of parse_terminated when the only remaining text in the string is ".",
// and so you can't use parse_terminated because that would improperly cut off names with periods
// like "Kaj Statter Jr."
pub(crate) fn parse_until_period_eof(input: &str) -> ParserResult<&str> {
    let (input, replacement_name_with_dot) = is_not("\n").parse(input)?;
    let replacement_name = replacement_name_with_dot.strip_suffix(".")
        .ok_or_else(|| {
            // I can't figure out how to make an error myself so I'm just gonna unwrap a fail
            fail::<_, (), _>(replacement_name_with_dot).unwrap_err()
        })?;

    Ok((input, replacement_name))
}
