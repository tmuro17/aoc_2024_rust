use aoc_runner_derive::{aoc, aoc_generator};
use color_eyre::eyre::{eyre, Result};
use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

static COLOR_EYRE: Lazy<()> = Lazy::new(|| color_eyre::install().unwrap());

#[derive(Debug, Clone)]
struct Rule {
    first: u64,
    second: u64,
}

type Rules = Vec<Rule>;
type Update = Vec<u64>;
type Updates = Vec<Update>;

#[derive(Debug, Clone)]
struct Printing {
    rules: Rules,
    updates: Updates,
}

#[aoc_generator(day5)]
fn parser(input: &str) -> Result<Printing> {
    Lazy::get(&COLOR_EYRE);
    parsers::parse_input(input).map_err(|e| eyre!(e.to_string()))
}

#[aoc(day5, part1)]
fn part1(printing: &Printing) -> u64 {
    let rule_map: HashMap<u64, Vec<u64>> = printing
        .rules
        .iter()
        .map(|r| (r.second, r.first))
        .into_group_map();
    printing
        .updates
        .iter()
        .filter(|update| is_valid_update(&rule_map, update))
        .map(|update| {
            let mid = update.len() / 2;
            update[mid]
        })
        .sum()
}

fn is_valid_update(rule_map: &HashMap<u64, Vec<u64>>, update: &Update) -> bool {
    let contents: HashSet<u64> = HashSet::from_iter(update.clone());
    let (_, valid) = update
        .iter()
        .fold_while((HashSet::new(), true), |(mut seen, valid), page| {
            let preceded_by = rule_map.get(page);
            if let Some(preceded_by) = preceded_by {
                if !preceded_by
                    .iter()
                    .all(|page| !contents.contains(page) || seen.contains(page))
                {
                    return Done((seen, false));
                }
            }
            seen.insert(*page);
            Continue((seen, valid))
        })
        .into_inner();
    valid
}

#[aoc(day5, part2)]
fn part2(printing: &Printing) -> u64 {
    let validation_rule_map: HashMap<u64, Vec<u64>> = printing
        .rules
        .iter()
        .map(|r| (r.second, r.first))
        .into_group_map();

    let bad_updates: Vec<_> = printing
        .updates
        .iter()
        .filter(|update| !is_valid_update(&validation_rule_map, update))
        .collect();

    let sort_rule_map: HashMap<u64, Vec<u64>> = printing
        .rules
        .iter()
        .map(|r| (r.first, r.second))
        .into_group_map();

    bad_updates
        .iter()
        .map(|update| {
            update
                .iter()
                .sorted_by(|a, b| {
                    if let Some(after) = sort_rule_map.get(a) {
                        if after.contains(b) {
                            return std::cmp::Ordering::Less;
                        };
                    };

                    std::cmp::Ordering::Equal
                })
                .collect::<Vec<_>>()
        })
        .map(|update| {
            let mid = update.len() / 2;
            update[mid]
        })
        .sum()
}

mod parsers {
    use crate::day5::{Printing, Rule, Rules, Update, Updates};
    use nom::{
        character::{complete, complete::newline},
        multi::{many1, separated_list1},
        sequence::separated_pair,
        IResult, Parser,
    };
    use nom_locate::LocatedSpan;
    use nom_supreme::{
        error::ErrorTree, final_parser::final_parser, tag::complete::tag, ParserExt,
    };

    type ParseError<'a> = ErrorTree<Span<'a>>;

    type Span<'a> = LocatedSpan<&'a str>;

    pub(super) fn parse_input(input: &str) -> color_eyre::Result<Printing, ParseError> {
        final_parser(printing)(Span::new(input))
    }

    fn printing(input: Span) -> IResult<Span, Printing, ParseError> {
        rules
            .and(updates.preceded_by(many1(newline)))
            .map(|(rules, updates)| Printing { rules, updates })
            .parse(input)
    }

    fn rules(input: Span) -> IResult<Span, Rules, ParseError> {
        separated_list1(newline, rule).parse(input)
    }

    fn rule(input: Span) -> IResult<Span, Rule, ParseError> {
        separated_pair(complete::u64, tag("|"), complete::u64)
            .map(|(first, second)| Rule { first, second })
            .parse(input)
    }

    fn updates(input: Span) -> IResult<Span, Updates, ParseError> {
        separated_list1(newline, update).parse(input)
    }

    fn update(input: Span) -> IResult<Span, Update, ParseError> {
        separated_list1(tag(","), complete::u64).parse(input)
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
        "47|53
         97|13
         97|61
         97|47
         75|29
         61|13
         75|53
         29|13
         97|29
         53|29
         61|53
         97|53
         61|29
         47|13
         75|47
         97|75
         47|61
         75|61
         47|29
         75|13
         53|13
         
         75,47,61,53,29
         97,61,53,29,13
         75,29,13
         75,97,47,61,53
         61,13,29
         97,13,75,29,47"
    };

    #[test]
    fn test_parsing() {
        let reports = parser(SAMPLE).unwrap();
        insta::assert_debug_snapshot!(reports);
    }

    #[rstest]
    #[case::part1(part1, 143)]
    #[case::part2(part2, 123)]
    fn sample_tests(#[case] f: fn(&Printing) -> u64, #[case] expected: u64) {
        let parsed = parser(SAMPLE).unwrap();
        let result = f(&parsed);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::part1(part1, 4905)]
    #[case::part2(part2, 6204)]
    fn prod_tests(#[case] f: fn(&Printing) -> u64, #[case] expected: u64) {
        let input = fs::read_to_string("input/2024/day5.txt").unwrap();
        let parsed = parser(input.trim_end()).unwrap();
        let result = f(&parsed);

        assert_eq!(result, expected);
    }
}
