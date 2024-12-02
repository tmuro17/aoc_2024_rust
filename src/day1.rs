use aoc_runner_derive::{aoc, aoc_generator};
use color_eyre::eyre::{eyre, Result};
use once_cell::sync::Lazy;

static COLOR_EYRE: Lazy<()> = Lazy::new(|| color_eyre::install().unwrap());

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Result<(Vec<i64>, Vec<i64>)> {
    Lazy::get(&COLOR_EYRE);
    parsers::parse_input(input).map_err(|e| eyre!(e.to_string()))
}

#[aoc(day1, part1)]
pub fn part1(lists: &(Vec<i64>, Vec<i64>)) -> u64 {
    let mut left = lists.0.clone();
    let mut right = lists.1.clone();

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(left, right)| (left - right).unsigned_abs())
        .sum()
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
#[aoc(day1, part2)]
pub fn part2(lists: &(Vec<i64>, Vec<i64>)) -> u64 {
    let left = lists.0.clone();
    let right = lists.1.clone();

    left.iter().fold(0, |acc, item| {
        let freq = right.iter().filter(|y| &item == y).count() as i64;

        acc + (freq * item)
    }) as u64
}

mod parsers {
    use nom::{
        character::complete::{i64 as i64_parser, newline, space1},
        multi::separated_list1,
        sequence::separated_pair,
        IResult, Parser,
    };
    use nom_locate::LocatedSpan;
    use nom_supreme::{error::ErrorTree, final_parser::final_parser};

    type ParseError<'a> = ErrorTree<Span<'a>>;

    type Span<'a> = LocatedSpan<&'a str>;

    pub(crate) fn parse_input(input: &str) -> color_eyre::Result<(Vec<i64>, Vec<i64>), ParseError> {
        final_parser(lists)(Span::new(input))
    }

    fn lists(input: Span) -> IResult<Span, (Vec<i64>, Vec<i64>), ParseError> {
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

    fn pair(input: Span) -> IResult<Span, (i64, i64), ParseError> {
        separated_pair(i64_parser, space1, i64_parser).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;
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
}
