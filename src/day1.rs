use aoc_runner_derive::{aoc, aoc_generator};
use color_eyre::eyre::{eyre, Result};
use once_cell::sync::Lazy;

static COLOR_EYRE: Lazy<()> = Lazy::new(|| color_eyre::install().unwrap());

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Result<(Vec<u64>, Vec<u64>)> {
    Lazy::get(&COLOR_EYRE);
    parsers::parse_input(input).map_err(|e| eyre!(e.to_string()))
}

#[aoc(day1, part1)]
pub fn part1(lists: &(Vec<u64>, Vec<u64>)) -> u64 {
    let mut left = lists.0.clone();
    let mut right = lists.1.clone();

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(lists: &(Vec<u64>, Vec<u64>)) -> u64 {
    let left = lists.0.clone();
    let right = lists.1.clone();

    left.iter().fold(0, |acc, item| {
        let freq = right.iter().filter(|y| &item == y).count() as u64;

        acc + (freq * item)
    })
}

mod parsers {
    use nom::{
        character::complete::{newline, space1, u64 as u64_parser},
        multi::separated_list1,
        sequence::separated_pair,
        IResult, Parser,
    };
    use nom_locate::LocatedSpan;
    use nom_supreme::{error::ErrorTree, final_parser::final_parser};

    type ParseError<'a> = ErrorTree<Span<'a>>;

    type Span<'a> = LocatedSpan<&'a str>;

    pub(crate) fn parse_input(input: &str) -> color_eyre::Result<(Vec<u64>, Vec<u64>), ParseError> {
        final_parser(lists)(Span::new(input))
    }

    fn lists(input: Span) -> IResult<Span, (Vec<u64>, Vec<u64>), ParseError> {
        separated_list1(newline, pair)
            .map(|pairs| {
                pairs
                    .into_iter()
                    .fold((vec![], vec![]), |mut result, item| {
                        result.0.push(item.0);
                        result.1.push(item.1);

                        result
                    })
            })
            .parse(input)
    }

    fn pair(input: Span) -> IResult<Span, (u64, u64), ParseError> {
        separated_pair(u64_parser, space1, u64_parser).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day1::parsers::parse_input;
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use std::fs;

    const SAMPLE: &str = indoc! {
        "3   4
         4   3
         2   5
         1   3
         3   9
         3   3"
    };

    #[test]
    fn test_parsing() {
        let (left, right) = input_generator(SAMPLE).unwrap();

        assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_part1() {
        let parsed = input_generator(SAMPLE).unwrap();
        let result = part1(&parsed);

        assert_eq!(result, 11);
    }

    #[test]
    fn test_part2() {
        let parsed = input_generator(SAMPLE).unwrap();
        let result = part2(&parsed);

        assert_eq!(result, 31);
    }

    #[test]
    fn prod_part_1() {
        let input = fs::read_to_string("input/2024/day1.txt").unwrap();
        let parsed = parse_input(input.trim()).unwrap();
        let result = part1(&parsed);

        assert_eq!(result, 936_063);
    }

    #[test]
    fn prod_part_2() {
        let input = fs::read_to_string("input/2024/day1.txt").unwrap();
        let parsed = parse_input(input.trim_end()).unwrap();
        let result = part2(&parsed);

        assert_eq!(result, 23_150_395);
    }
}
