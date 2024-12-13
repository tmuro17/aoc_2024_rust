use crate::utils::point::Point;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use once_cell::sync::Lazy;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

static COLOR_EYRE: Lazy<()> = Lazy::new(|| color_eyre::install().unwrap());

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Vec<char>> {
    Lazy::get(&COLOR_EYRE);
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

#[derive(Clone, Copy, EnumIter, Debug)]
enum Direction {
    Down,
    Up,
    Left,
    Right,
    DiagonalRightDown,
    DiagonalLeftDown,
    DiagonalRightUp,
    DiagonalLeftUp,
}

#[aoc(day4, part1)]
fn part1(input: &Vec<Vec<char>>) -> usize {
    let search_space = input.clone();
    let size = search_space.len();

    (0..size)
        .cartesian_product(0..size)
        .map(|(x, y)| Point { x, y })
        .cartesian_product(Direction::iter())
        .filter(|(pos, direction)| search_direction(&search_space, pos, *direction, "XMAS"))
        .count()
}

fn search_direction(
    grid: &Vec<Vec<char>>,
    pos: &Point<usize>,
    direction: Direction,
    word: &str,
) -> bool {
    if word.is_empty() {
        return true;
    }

    let first_letter = word.chars().next().unwrap();

    if grid.get(pos.y).and_then(|row| row.get(pos.x)) != Some(&first_letter) {
        return false;
    }

    let rest = &word[1..];

    let max_index = grid.len() - 1;
    let next_pos = match direction {
        Direction::Down if pos.y < max_index => Point::new(pos.x, pos.y + 1),
        Direction::Up if pos.y > 0 => Point::new(pos.x, pos.y - 1),
        Direction::Left if pos.x > 0 => Point::new(pos.x - 1, pos.y),
        Direction::Right if pos.x < max_index => Point::new(pos.x + 1, pos.y),
        Direction::DiagonalRightDown if pos.x < max_index && pos.y < max_index => {
            Point::new(pos.x + 1, pos.y + 1)
        }
        Direction::DiagonalLeftDown if pos.x > 0 && pos.y < max_index => {
            Point::new(pos.x - 1, pos.y + 1)
        }
        Direction::DiagonalRightUp if pos.x < max_index && pos.y > 0 => {
            Point::new(pos.x + 1, pos.y - 1)
        }
        Direction::DiagonalLeftUp if pos.x > 0 && pos.y > 0 => Point::new(pos.x - 1, pos.y - 1),
        _ if rest.is_empty() => return true,
        _ => return false,
    };

    search_direction(grid, &next_pos, direction, &word[1..])
}

#[aoc(day4, part2)]
fn part2(search_space: &Vec<Vec<char>>) -> usize {
    let size = search_space.len();

    (0..size)
        .cartesian_product(0..size)
        .filter_map(|pt| pairs(pt.into(), size))
        .filter(|(a, b)| {
            [
                ("MAS", "MAS"),
                ("MAS", "SAM"),
                ("SAM", "MAS"),
                ("SAM", "SAM"),
            ]
            .iter()
            .any(|(x, y)| {
                search_direction(search_space, a, Direction::DiagonalRightDown, x)
                    && search_direction(search_space, b, Direction::DiagonalLeftDown, y)
            })
        })
        .count()
}

fn pairs(point: Point<usize>, grid_size: usize) -> Option<(Point<usize>, Point<usize>)> {
    if point.x + 2 >= grid_size {
        return None;
    }

    Some((point, Point::new(point.x + 2, point.y)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use std::fs;

    const SAMPLE: &str = indoc! {
      "MMMSXXMASM
       MSAMXMSMSA
       AMXSXMAAMM
       MSAMASMSMX
       XMASAMXAMM
       XXAMMXXAMA
       SMSMSASXSS
       SAXAMASAAA
       MAMMMXMMMM
       MXMXAXMASX"
    };

    #[test]
    fn test_parsing() {
        let reports = parse(SAMPLE);
        insta::assert_debug_snapshot!(reports);
    }

    #[rstest]
    #[case::part1(part1, 18)]
    #[case::part2(part2, 9)]
    fn sample_tests(#[case] f: fn(&Vec<Vec<char>>) -> usize, #[case] expected: usize) {
        let parsed = parse(SAMPLE);
        let result = f(&parsed);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case((5, 0), Direction::Right)]
    #[case((4, 0), Direction::DiagonalRightDown)]
    #[case((4, 1), Direction::Left)]
    #[case((9, 3), Direction::Down)]
    #[case((0, 4), Direction::Right)]
    #[case((6, 4), Direction::Left)]
    #[case((6, 4), Direction::Up)]
    #[case((0, 5), Direction::DiagonalRightUp)]
    #[case((6, 5), Direction::DiagonalLeftUp)]
    #[case((1, 9), Direction::DiagonalRightUp)]
    #[case((3, 9), Direction::DiagonalLeftUp)]
    #[case((5, 9), Direction::DiagonalLeftUp)]
    #[case((5, 9), Direction::Right)]
    #[case((9, 9), Direction::Up)]
    fn search(#[case] (x, y): (usize, usize), #[case] direction: Direction) {
        let parsed = parse(SAMPLE);

        assert!(search_direction(
            &parsed,
            &Point { x, y },
            direction,
            "XMAS"
        ));
    }

    #[rstest]
    #[case::part1(part1, 2571)]
    #[case::part2(part2, 1992)]
    fn prod_tests(#[case] f: fn(&Vec<Vec<char>>) -> usize, #[case] expected: usize) {
        let input = fs::read_to_string("input/2024/day4.txt").unwrap();
        let parsed = parse(input.trim_end());
        let result = f(&parsed);

        assert_eq!(result, expected);
    }
}
