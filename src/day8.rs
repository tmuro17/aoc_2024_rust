use aoc_runner_derive::{aoc, aoc_generator};
use color_eyre::eyre::{eyre, Result};
use once_cell::sync::Lazy;

static COLOR_EYRE: Lazy<()> = Lazy::new(|| color_eyre::install().unwrap());

#[aoc_generator(day8)]
fn parser(input: &str) -> Result<Grid> {
    Lazy::get(&COLOR_EYRE);
    parsers::parse_input(input).map_err(|e| eyre!(e.to_string()))
}

#[derive(Clone, Debug)]
struct Antenna(char);

type GridRow = Vec<Option<Antenna>>;
type Grid = Vec<GridRow>;

#[aoc(day8, part1)]
fn part1(equations: &Grid) -> u64 {
    todo!()
}

#[aoc(day8, part2)]
fn part2(equations: &Grid) -> u64 {
    todo!()
}

mod parsers {
    use crate::day8::{Antenna, Grid, GridRow};
    use nom::{
        branch::alt,
        character::{
            complete,
            complete::{anychar, newline, satisfy},
        },
        combinator::{peek, value},
        multi::{many1, many_till, separated_list1},
        IResult, Parser,
    };
    use nom_locate::LocatedSpan;
    use nom_supreme::{error::ErrorTree, final_parser::final_parser, ParserExt};

    type ParseError<'a> = ErrorTree<Span<'a>>;

    type Span<'a> = LocatedSpan<&'a str>;

    pub(super) fn parse_input(input: &str) -> color_eyre::Result<Grid, ParseError> {
        final_parser(grid)(Span::new(input))
    }

    fn grid(input: Span) -> IResult<Span, Grid, ParseError> {
        separated_list1(newline, grid_row).parse(input)
    }

    fn grid_row(input: Span) -> IResult<Span, GridRow, ParseError> {
        many1(antenna).parse(input)
    }

    fn antenna(input: Span) -> IResult<Span, Option<Antenna>, ParseError> {
        satisfy(|c| c.is_ascii_alphanumeric())
            .map(|c| Some(Antenna(c)))
            .or(complete::char('.').value(None))
            .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use std::fs;

    const SAMPLE: &str = indoc! {
        "............
         ........0...
         .....0......
         .......0....
         ....0.......
         ......A.....
         ............
         ............
         ........A...
         .........A..
         ............
         ............"
    };

    #[test]
    fn test_parsing() -> Result<()> {
        let reports = parser(SAMPLE)?;
        insta::assert_debug_snapshot!(reports);
        Ok(())
    }

    #[rstest]
    #[case::part1(part1, 3749)]
    #[case::part2(part2, 11387)]
    fn sample_tests(#[case] f: fn(&[Equation]) -> u64, #[case] expected: u64) {
        let parsed = parser(SAMPLE).unwrap();
        let result = f(&parsed);
    
        assert_eq!(result, expected);
    }

    // #[rstest]
    // #[case::part1(part1, 3_245_122_495_150)]
    // #[case::part2(part2, 105_517_128_211_543)]
    // fn prod_tests(#[case] f: fn(&[Equation]) -> u64, #[case] expected: u64) {
    //     let input = fs::read_to_string("input/2024/day7.txt").unwrap();
    //     let parsed = parser(input.trim_end()).unwrap();
    //     let result = f(&parsed);
    //
    //     assert_eq!(result, expected);
    // }
}
