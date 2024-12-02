use aoc_runner_derive::{aoc, aoc_generator};
use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::cmp::Reverse;

static COLOR_EYRE: Lazy<()> = Lazy::new(|| color_eyre::install().unwrap());

#[derive(Clone, Default, Debug)]
pub(crate) struct Report(Vec<u64>);

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Result<Vec<Report>> {
    Lazy::get(&COLOR_EYRE);
    parsers::parse_input(input).map_err(|e| eyre!(e.to_string()))
}

#[aoc(day2, part1)]
pub fn part1(reports: &[Report]) -> u64 {
    reports.iter().filter(|rep| rep.is_safe()).count() as u64
}

impl Report {
    fn is_safe(&self) -> bool {
        if !(self.is_asc() || self.is_desc()) {
            return false;
        }

        self.0
            .iter()
            .tuple_windows()
            .all(|(a, b)| (1..=3).contains(&a.abs_diff(*b)))
    }

    fn is_dampened_safe(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        let k = self.0.len() - 1;
        let options = self.0.iter().copied().combinations_with_replacement(k);

        options.map(Report).any(|opt| opt.is_safe())
    }

    fn is_asc(&self) -> bool {
        self.0.is_sorted()
    }

    fn is_desc(&self) -> bool {
        self.0.is_sorted_by_key(|x| Reverse(*x))
    }
}

#[aoc(day2, part2)]
pub fn part2(reports: &[Report]) -> u64 {
    reports.iter().filter(|rep| rep.is_dampened_safe()).count() as u64
}

mod parsers {
    use crate::day2::Report;
    use nom::{
        character::complete::{newline, space1},
        multi::separated_list1,
        IResult, Parser,
    };
    use nom_locate::LocatedSpan;
    use nom_supreme::{error::ErrorTree, final_parser::final_parser};

    type ParseError<'a> = ErrorTree<Span<'a>>;

    type Span<'a> = LocatedSpan<&'a str>;

    pub(crate) fn parse_input(input: &str) -> color_eyre::Result<Vec<Report>, ParseError> {
        final_parser(reports)(Span::new(input))
    }

    fn reports(input: Span) -> IResult<Span, Vec<Report>, ParseError> {
        separated_list1(newline, report).parse(input)
    }

    fn report(input: Span) -> IResult<Span, Report, ParseError> {
        use nom::character::complete::u64;
        separated_list1(space1, u64).map(Report).parse(input)
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
        "7 6 4 2 1
         1 2 7 8 9
         9 7 6 2 1
         1 3 2 4 5
         8 6 4 4 1
         1 3 6 7 9"
    };

    #[test]
    fn test_parsing() {
        let reports = input_generator(SAMPLE).unwrap();
        insta::assert_debug_snapshot!(reports);
    }

    #[rstest]
    #[case::part1(part1, 2)]
    #[case::part2(part2, 4)]
    fn sample_tests(#[case] f: fn(&[Report]) -> u64, #[case] expected: u64) {
        let parsed = input_generator(SAMPLE).unwrap();
        let result = f(&parsed);

        assert_eq!(result, expected);
    }
    //
    #[rstest]
    #[case::part1(part1, 202)]
    #[case::part2(part2, 271)]
    fn prod_tests(#[case] f: fn(&[Report]) -> u64, #[case] expected: u64) {
        let input = fs::read_to_string("input/2024/day2.txt").unwrap();
        let parsed = input_generator(input.trim_end()).unwrap();
        let result = f(&parsed);

        assert_eq!(result, expected);
    }
}
