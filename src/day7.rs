use aoc_runner_derive::{aoc, aoc_generator};
use color_eyre::eyre::{eyre, Result};
use once_cell::sync::Lazy;

static COLOR_EYRE: Lazy<()> = Lazy::new(|| color_eyre::install().unwrap());

#[aoc_generator(day7)]
fn parser(input: &str) -> Result<Vec<Equation>> {
    Lazy::get(&COLOR_EYRE);
    parsers::parse_input(input).map_err(|e| eyre!(e.to_string()))
}

#[derive(Debug)]
struct Equation {
    target: u64,
    operands: Vec<u64>,
}

#[aoc(day7, part1)]
fn part1(equations: &[Equation]) -> u64 {
    equations
        .iter()
        .filter(|eq| hits_target(eq))
        .map(|eq| eq.target)
        .sum()
}

fn hits_target(equation: &Equation) -> bool {
    equation
        .operands
        .iter()
        .copied()
        .fold(Vec::new(), |acc, op| {
            if acc.is_empty() {
                return vec![op];
            };

            let added = acc.iter().map(|v| v + op);
            let muled = acc.iter().map(|v| v * op);

            added.chain(muled).collect()
        })
        .iter()
        .any(|v| *v == equation.target)
}

fn hits_target_2(equation: &Equation) -> bool {
    equation
        .operands
        .iter()
        .copied()
        .fold(Vec::new(), |acc, op| {
            if acc.is_empty() {
                return vec![op];
            };

            let added = acc.iter().map(|v| v + op);
            let muled = acc.iter().map(|v| v * op);
            let concated = acc.iter().map(|v| {
                let v_str = v.to_string();
                let op_str = op.to_string();
                (v_str + &op_str).parse::<u64>().unwrap()
            });

            added.chain(muled).chain(concated).collect()
        })
        .iter()
        .any(|v| *v == equation.target)
}

#[aoc(day7, part2)]
fn part2(equations: &[Equation]) -> u64 {
    equations
        .iter()
        .filter(|eq| hits_target_2(eq))
        .map(|eq| eq.target)
        .sum()
}

mod parsers {
    use crate::day7::Equation;
    use nom::{
        bytes::complete::tag,
        character::{
            complete,
            complete::{newline, space0, space1},
        },
        multi::separated_list1,
        IResult, Parser,
    };
    use nom_locate::LocatedSpan;
    use nom_supreme::{error::ErrorTree, final_parser::final_parser, ParserExt};

    type ParseError<'a> = ErrorTree<Span<'a>>;

    type Span<'a> = LocatedSpan<&'a str>;

    pub(super) fn parse_input(input: &str) -> color_eyre::Result<Vec<Equation>, ParseError> {
        final_parser(equations)(Span::new(input))
    }

    fn equations(input: Span) -> IResult<Span, Vec<Equation>, ParseError> {
        separated_list1(newline, equation).parse(input)
    }

    fn equation(input: Span) -> IResult<Span, Equation, ParseError> {
        complete::u64
            .context("target error")
            .terminated(tag(":").precedes(space0).context("tag issues"))
            .and(separated_list1(space1, complete::u64).context("operand issue"))
            .map(|(target, operands)| Equation { target, operands })
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
        "190: 10 19
         3267: 81 40 27
         83: 17 5
         156: 15 6
         7290: 6 8 6 15
         161011: 16 10 13
         192: 17 8 14
         21037: 9 7 18 13
         292: 11 6 16 20"
    };

    #[test]
    fn test_parsing() {
        let reports = parser(SAMPLE).unwrap();
        insta::assert_debug_snapshot!(reports);
    }

    #[rstest]
    #[case::part1(part1, 3749)]
    #[case::part2(part2, 11387)]
    fn sample_tests(#[case] f: fn(&[Equation]) -> u64, #[case] expected: u64) {
        let parsed = parser(SAMPLE).unwrap();
        let result = f(&parsed);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::part1(part1, 3_245_122_495_150)]
    #[case::part2(part2, 105_517_128_211_543)]
    fn prod_tests(#[case] f: fn(&[Equation]) -> u64, #[case] expected: u64) {
        let input = fs::read_to_string("input/2024/day7.txt").unwrap();
        let parsed = parser(input.trim_end()).unwrap();
        let result = f(&parsed);

        assert_eq!(result, expected);
    }
}
