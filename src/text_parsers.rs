use anyhow::anyhow;
use nom::{Finish, IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until1};
use nom::combinator::{fail, opt, recognize, verify};
use nom::error::ParseError;
use nom::sequence::{pair, preceded, terminated};
use nom_supreme::error::{BaseErrorKind, ErrorTree};
use nom_supreme::final_parser::{final_parser, Location};
use crate::chron_schema::PlayerDesc;
use crate::fed_schema::*;

pub(crate) type ParserError<'a> = nom::error::VerboseError<&'a str>;
pub(crate) type ParserResult<'a, Out, Er> = IResult<&'a str, Out, Er>;

pub fn parse_literal<'a, E: ParseError<&'a str>>(literal: &str) -> impl FnMut(&'a str) -> IResult<&'a str, (), E> + '_ {
    move |input| tag(literal).map(|_| ()).parse(input)
}

fn count<'a, O, E, F>(balls: i64, strikes: i64, mut child: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where E: ParseError<&'a str>,
          F: FnMut(&'a str) -> IResult<&'a str, O, E> {
    move |input| {
        let (input, out) = child.parse(input)?;
        let (input, _) = tag(format!(" {}-{}", balls, strikes).as_str()).parse(input)?;

        Ok((input, out))
    }
}

fn count_dot<'a, O, E, F>(balls: i64, strikes: i64, mut child: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where E: ParseError<&'a str>,
          F: FnMut(&'a str) -> IResult<&'a str, O, E> {
    move |input| {
        let (input, out) = child.parse(input)?;
        let (input, _) = tag(format!(" {}-{}.", balls, strikes).as_str()).parse(input)?;

        Ok((input, out))
    }
}

pub fn parse_ball<'a, 'b, E: ParseError<&'a str>>(balls: i64, strikes: i64, pitcher_name: &'b str, batter_name: &'b str) -> impl FnMut(&'a str) -> IResult<&'a str, BallFlavor, E> + 'b {
    move |input| {
        alt((
            count(balls, strikes, tag("Ball."))
                .map(|_| BallFlavor::None),
            count(balls, strikes, tag("Ball, way outside."))
                .map(|_| BallFlavor::WayOutside),
            count_dot(balls, strikes, tag("Ball, just outside."))
                .map(|_| BallFlavor::JustOutside),
            count_dot(balls, strikes,pair(tag(pitcher_name), tag(" just misses the zone. Ball,")))
                .map(|_| BallFlavor::MissesTheZone),
            count_dot(balls, strikes,pair(tag(batter_name), tag(" does not chase. Ball,")))
                .map(|_| BallFlavor::DoesNotChase),
            count_dot(balls, strikes,terminated(parse_pitch_adjective, tag(" pitch. Ball,")))
                .map(|adj| BallFlavor::Adjective(adj)),
        )).parse(input)
    }
}

pub fn parse_strike<'a, 'b, E: ParseError<&'a str>>(balls: i64, strikes: i64, pitcher_name: &'b str, batter_name: &'b str) -> impl FnMut(&'a str) -> IResult<&'a str, StrikeFlavor, E> + 'b {
    move |input| {
        alt((
            count_dot(balls, strikes, tag("Strike,"))
                .map(|_| StrikeFlavor::None),
            count_dot(balls, strikes, preceded(tag(pitcher_name), tag(" throws a strike.")))
                .map(|_| StrikeFlavor::ThrowsAStrike),
            count_dot(balls, strikes, preceded(tag(batter_name), tag(" is caught looking. Strike,")))
                .map(|_| StrikeFlavor::CaughtLooking),
            count_dot(balls, strikes, preceded(tag(batter_name), tag(" chases. Strike,")))
                .map(|_| StrikeFlavor::Chases),
        )).parse(input)
    }
}

pub fn parse_strikeout<'a, 'b, E: ParseError<&'a str>>(pitcher_name: &'b str, batter_name: &'b str) -> impl FnMut(&'a str) -> IResult<&'a str, (), E> + 'b {
    move |input| {
        let (input, _) = tag(pitcher_name).parse(input)?;
        let (input, _) = tag(" strikes ").parse(input)?;
        let (input, _) = tag(batter_name).parse(input)?;
        let (input, _) = tag(" out.").parse(input)?;
        Ok((input, ()))
    }
}

pub fn parse_contact<'a, 'b, E: ParseError<&'a str>>(batter_name: &'b str)
    -> impl FnMut(&'a str) -> IResult<&'a str, (Option<SoundEffect>, ContactVerb, Option<PitchDescriptor>, FieldLocation), E> + 'b {
    move |input| {
        alt((
            parse_contact_with_sound(batter_name)
                .map(|(s, v, l)| (Some(s), v, None, l)),
            parse_contact_without_sound(batter_name)
                .map(|(v, d, l)| (None, v, Some(d), l)),
        )).parse(input)
    }
}

fn parse_location<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, FieldLocation, E> {
    alt((
        tag("the Infield").map(|_| FieldLocation::Infield),
        tag("Left Field").map(|_| FieldLocation::LeftField),
        tag("Deep Left Field").map(|_| FieldLocation::DeepLeftField),
        tag("Center Field").map(|_| FieldLocation::CenterField),
        tag("Deep Center Field").map(|_| FieldLocation::DeepCenterField),
        tag("Right Field").map(|_| FieldLocation::RightField),
        tag("Deep Right Field").map(|_| FieldLocation::DeepRightField),
        tag("the Wall").map(|_| FieldLocation::Wall),
    )).parse(input)
}

fn parse_pitch_adjective<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, PitchAdjective, E> {
    alt((
        tag("Auspicious").map(|_| PitchAdjective::Auspicious),
        tag("Average").map(|_| PitchAdjective::Average),
        tag("Disgusting").map(|_| PitchAdjective::Disgusting),
        tag("Dominant").map(|_| PitchAdjective::Dominant),
        tag("Favorable").map(|_| PitchAdjective::Favorable),
        tag("Horrible").map(|_| PitchAdjective::Horrible),
        tag("Marvelous").map(|_| PitchAdjective::Marvelous),
        tag("Overpowering").map(|_| PitchAdjective::Overpowering),
        tag("Potent").map(|_| PitchAdjective::Potent),
        tag("Powerful").map(|_| PitchAdjective::Powerful),
        tag("Revolting").map(|_| PitchAdjective::Revolting),
        tag("Well-located").map(|_| PitchAdjective::WellLocated),
        tag("Well-placed").map(|_| PitchAdjective::WellPlaced),
    )).parse(input)
}

pub fn parse_contact_with_sound<'a, 'b, E: ParseError<&'a str>>(batter_name: &'b str)
    -> impl FnMut(&'a str) -> IResult<&'a str, (SoundEffect, ContactVerb, FieldLocation), E> + 'b {
    move |input| {
        let (input, sound_effect) = alt((
            tag("BAM! ").map(|_| SoundEffect::Bam),
            tag("BOOM! ").map(|_| SoundEffect::Boom),
            tag("CRACK! ").map(|_| SoundEffect::Crack),
            tag("SMACK! ").map(|_| SoundEffect::Smack),
            tag("SMASH! ").map(|_| SoundEffect::Smash),
            tag("THWACK! ").map(|_| SoundEffect::Thwack),
            tag("WHAM! ").map(|_| SoundEffect::Wham),
        )).parse(input)?;
        let (input, _) = tag(batter_name).parse(input)?;
        let (input, _) = tag(" ").parse(input)?;
        let (input, verb) = parse_contact_verb.parse(input)?;
        let (input, _) = tag(" it to ").parse(input)?;
        let (input, location) = parse_location.parse(input)?;
        let (input, _) = tag("...").parse(input)?;
        Ok((input, (sound_effect, verb, location)))
    }
}

fn parse_contact_verb<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ContactVerb, E> {
    alt((
        tag("bats").map(|_| ContactVerb::Bats),
        tag("chops").map(|_| ContactVerb::Chops),
        tag("clips").map(|_| ContactVerb::Clips),
        tag("drags").map(|_| ContactVerb::Drags),
        tag("dribbles").map(|_| ContactVerb::Dribbles),
        tag("hits").map(|_| ContactVerb::Hits),
        tag("knocks").map(|_| ContactVerb::Knocks),
        tag("nudges").map(|_| ContactVerb::Nudges),
        tag("pokes").map(|_| ContactVerb::Pokes),
        tag("punches").map(|_| ContactVerb::Punches),
        tag("pushes").map(|_| ContactVerb::Pushes),
        tag("rolls").map(|_| ContactVerb::Rolls),
        tag("slaps").map(|_| ContactVerb::Slaps),
        tag("smacks").map(|_| ContactVerb::Smacks),
        tag("sputters").map(|_| ContactVerb::Sputters),
        tag("swats").map(|_| ContactVerb::Swats),
        tag("taps").map(|_| ContactVerb::Taps),
        tag("thumps").map(|_| ContactVerb::Thumps),
        tag("trickles").map(|_| ContactVerb::Trickles),
        tag("whacks").map(|_| ContactVerb::Whacks),
    )).parse(input)
}

pub fn parse_contact_without_sound<'a, 'b, E: ParseError<&'a str>>(batter_name: &'b str)
    -> impl FnMut(&'a str) -> IResult<&str, (ContactVerb, PitchDescriptor, FieldLocation), E> + 'b {
    move |input| {
        let (input, _) = tag(batter_name).parse(input)?;
        let (input, _) = tag(" ").parse(input)?;
        let (input, verb) = parse_contact_verb.parse(input)?;
        let (input, (descriptor, location)) = alt((
            pair(tag(" it toward ").map(|_| PitchDescriptor::It), parse_location),
            pair(tag(" one to ").map(|_| PitchDescriptor::One), parse_location),
            pair(tag(" the ball to ").map(|_| PitchDescriptor::TheBall), parse_location),
            pair(tag(" the pitch to ").map(|_| PitchDescriptor::ThePitch), parse_location),
            tag(" the pitch into play").map(|_| (PitchDescriptor::ThePitch, FieldLocation::IntoPlay)),
        )).parse(input)?;
        let (input, _) = tag("...").parse(input)?;
        Ok((input, (verb, descriptor, location)))
    }
}

pub fn parse_flyout<'a, 'b, E: ParseError<&'a str>>(defenders: &'b[PlayerDesc])
    -> impl FnMut(&'a str) -> IResult<&str, &'b PlayerDesc, E> + 'b {
    move |input| {
        let (input, _) = tag("Fly out to ").parse(input)?;
        let (input, defender) = parse_name_from_list(defenders).parse(input)?;
        let (input, _) = tag(".").parse(input)?;
        Ok((input, defender))
    }
}

pub fn parse_name_from_list<'a, 'b, E: ParseError<&'a str>>(players: &'b[PlayerDesc])
    -> impl FnMut(&'a str) -> IResult<&str, &'b PlayerDesc, E> + 'b {
    move |input| {
        for player in players {
            let (input, recognized) = opt(tag(player.name.as_str())).parse(input)?;
            if recognized.is_some() { return Ok((input, player)) }
        }
        fail(input)
    }
}


pub(crate) fn parse_terminated<'s, E: ParseError<&'s str>>(tag_content: &str) -> impl Fn(&'s str) -> IResult<&'s str, &'s str, E> + '_ {
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
pub(crate) fn parse_until_period_eof<'s, Er: ParseError<&'s str>>(input: &'s str) -> ParserResult<&'s str, Er> {
    let (input, replacement_name_with_dot) = is_not("\n").parse(input)?;
    let replacement_name = replacement_name_with_dot.strip_suffix(".")
        .ok_or_else(|| {
            // I can't figure out how to make an error myself so I'm just gonna unwrap a fail
            fail::<_, (), _>(replacement_name_with_dot).unwrap_err()
        })?;

    Ok((input, replacement_name))
}
