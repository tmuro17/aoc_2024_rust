use aoc_runner_derive::{aoc, aoc_generator};
use color_eyre::eyre::{eyre, Result};
use once_cell::sync::Lazy;

static COLOR_EYRE: Lazy<()> = Lazy::new(|| color_eyre::install().unwrap());

#[aoc_generator(day3, part1)]
fn parse(input: &str) -> Result<Vec<Mul>> {
    Lazy::get(&COLOR_EYRE);
    parsers::parse_input(input).map_err(|e| eyre!(e.to_string()))
}

#[aoc_generator(day3, part2)]
fn parse_2(input: &str) -> Result<Vec<Instruction>> {
    Lazy::get(&COLOR_EYRE);
    parsers::parse_input_2(input).map_err(|e| eyre!(e.to_string()))
}

#[derive(Debug, Copy, Clone)]
struct Mul {
    x: u64,
    y: u64,
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Do,
    Dont,
    Multiply(Mul),
}

#[aoc(day3, part1)]
fn part1(input: &[Mul]) -> u64 {
    input.iter().map(|m| m.x * m.y).sum()
}

#[aoc(day3, part2)]
fn part2(input: &[Instruction]) -> u64 {
    let res = input.iter().fold(
        (0, true),
        |acc @ (sum, do_mul), instruction| match instruction {
            Instruction::Do => (sum, true),
            Instruction::Dont => (sum, false),
            Instruction::Multiply(Mul { x, y }) if do_mul => (sum + x * y, do_mul),
            Instruction::Multiply(_) => acc,
        },
    );

    res.0
}

mod parsers {
    use crate::day3::{Instruction, Mul};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::{complete, complete::anychar},
        combinator::value,
        multi::many1,
        sequence::{delimited, separated_pair},
        IResult, Parser,
    };
    use nom_locate::LocatedSpan;
    use nom_supreme::{error::ErrorTree, final_parser::final_parser, ParserExt};

    type ParseError<'a> = ErrorTree<Span<'a>>;

    type Span<'a> = LocatedSpan<&'a str>;

    pub(crate) fn parse_input(input: &str) -> color_eyre::Result<Vec<Mul>, ParseError> {
        final_parser(program)(Span::new(input))
    }

    pub(crate) fn parse_input_2(input: &str) -> color_eyre::Result<Vec<Instruction>, ParseError> {
        final_parser(program_2)(Span::new(input))
    }

    fn program(input: Span) -> IResult<Span, Vec<Mul>, ParseError> {
        many1(alt((mul.map(Some), value(None, anychar))))
            .map(|x| x.into_iter().flatten().collect())
            .parse(input)
    }

    fn program_2(input: Span) -> IResult<Span, Vec<Instruction>, ParseError> {
        many1(alt((
            alt((mul_inst, do_inst, dont_inst)).map(Some),
            value(None, anychar),
        )))
        .map(|x| x.into_iter().flatten().collect())
        .parse(input)
    }

    fn do_inst(input: Span) -> IResult<Span, Instruction, ParseError> {
        tag("do()").map(|_| Instruction::Do).parse(input)
    }

    fn dont_inst(input: Span) -> IResult<Span, Instruction, ParseError> {
        tag("don't()").map(|_| Instruction::Dont).parse(input)
    }

    fn mul(input: Span) -> IResult<Span, Mul, ParseError> {
        tag("mul")
            .precedes(delimited(
                tag("("),
                separated_pair(complete::u64, tag(","), complete::u64),
                tag(")"),
            ))
            .map(|(x, y)| Mul { x, y })
            .parse(input)
    }

    fn mul_inst(input: Span) -> IResult<Span, Instruction, ParseError> {
        mul.map(Instruction::Multiply).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day3::parsers::{parse_input, parse_input_2};
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use std::fs;

    const SAMPLE: &str = indoc! {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    };

    const SAMPLE2: &str = indoc! {
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    };

    #[test]
    fn test_parsing() {
        let muls = parse(SAMPLE).unwrap();
        insta::assert_debug_snapshot!(muls);
    }

    #[test]
    fn test_parsing2() {
        let inst = parse_2(SAMPLE2).unwrap();
        insta::assert_debug_snapshot!(inst);
    }

    #[test]
    fn sample_1() {
        let parsed = parse_input(SAMPLE).unwrap();
        let result = part1(&parsed);

        assert_eq!(result, 161);
    }

    #[test]
    fn sample_2() {
        let parsed = parse_input_2(SAMPLE2).unwrap();
        let result = part2(&parsed);

        assert_eq!(result, 48);
    }

    #[test]
    fn prod_1() {
        let input = fs::read_to_string("input/2024/day3.txt").unwrap();
        let parsed = parse_input(&input).unwrap();
        let result = part1(&parsed);

        assert_eq!(result, 153_469_856);
    }

    #[test]
    fn prod_2() {
        let input = fs::read_to_string("input/2024/day3.txt").unwrap();
        let parsed = parse_input_2(&input).unwrap();
        let result = part2(&parsed);

        assert_eq!(result, 77_055_967);
    }
}
