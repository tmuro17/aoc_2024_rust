use crate::utils::point::Point;
use aoc_runner_derive::{aoc, aoc_generator};
use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::{collections::HashSet, hash::Hash};

static COLOR_EYRE: Lazy<()> = Lazy::new(|| color_eyre::install().unwrap());

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Tile {
    Space,
    Obstacle,
    Guard(Direction),
}

type GridRow = Vec<Tile>;
type Grid = Vec<GridRow>;

#[aoc_generator(day6)]
fn parser(input: &str) -> Result<Grid> {
    Lazy::get(&COLOR_EYRE);
    parsers::parse_input(input).map_err(|e| eyre!(e.to_string()))
}

#[aoc(day6, part1)]
fn part1(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    let grid_size = grid.len() as i64;

    let mut seen: HashSet<_> = HashSet::new();

    let (y, row) = grid
        .iter()
        .find_position(|row| row.contains(&Tile::Guard(Direction::Up)))
        .unwrap();

    let (x, _) = row
        .iter()
        .find_position(|tile| *tile == &Tile::Guard(Direction::Up))
        .unwrap();

    let mut guard_pos: Point<i64> = Point {
        x: x as i64,
        y: y as i64,
    };

    while guard_pos.x < grid_size && guard_pos.y < grid_size && guard_pos.x >= 0 && guard_pos.y >= 0
    {
        seen.insert(guard_pos);
        let guard = grid[guard_pos.y as usize][guard_pos.x as usize];

        let Tile::Guard(mut dir) = guard else {
            unreachable!()
        };

        let step_forward = match dir {
            Direction::Up => Point {
                y: guard_pos.y - 1,
                ..guard_pos
            },
            Direction::Down => Point {
                y: guard_pos.y + 1,
                ..guard_pos
            },
            Direction::Left => Point {
                x: guard_pos.x - 1,
                ..guard_pos
            },
            Direction::Right => Point {
                x: guard_pos.x + 1,
                ..guard_pos
            },
        };

        let step_forward = if step_forward.x > grid_size
            || step_forward.y > grid_size
            || step_forward.x < 0
            || step_forward.y < 0
        {
            continue;
        } else {
            Point::new(step_forward.x as usize, step_forward.y as usize)
        };

        let next_pos = if let Some(Tile::Obstacle) = grid
            .get(step_forward.y)
            .and_then(|row| row.get(step_forward.x))
        {
            dir = match dir {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };

            guard_pos
        } else {
            Point::new(step_forward.x as i64, step_forward.y as i64)
        };

        grid[guard_pos.y as usize][guard_pos.x as usize] = Tile::Space;

        if let (Ok(x), Ok(y)) = (usize::try_from(next_pos.x), usize::try_from(next_pos.y)) {
            if let Some(row) = grid.get_mut(y) {
                if let Some(tile) = row.get_mut(x) {
                    *tile = Tile::Guard(dir);
                }
            }
        }

        guard_pos = next_pos;
    }

    seen.len()
}

#[aoc(day6, part2)]
fn part2(grid: &Grid) -> usize {
    let (y, row) = grid
        .iter()
        .find_position(|row| row.contains(&Tile::Guard(Direction::Up)))
        .unwrap();

    let (x, _) = row
        .iter()
        .find_position(|tile| *tile == &Tile::Guard(Direction::Up))
        .unwrap();

    let guard_pos: Point<i64> = Point {
        x: x as i64,
        y: y as i64,
    };

    let possible_blocks = grid_run(grid);

    possible_blocks
        .iter()
        .filter(|block| **block != guard_pos)
        .map(|block| {
            let mut grid = grid.clone();
            grid[block.y as usize][block.x as usize] = Tile::Obstacle;
            grid
        })
        .filter(has_cycle)
        .count()
}

fn grid_run(grid: &Grid) -> HashSet<Point<i64>> {
    let mut grid = grid.clone();
    let grid_size = grid.len() as i64;

    let mut seen: HashSet<_> = HashSet::new();

    let (y, row) = grid
        .iter()
        .find_position(|row| row.contains(&Tile::Guard(Direction::Up)))
        .unwrap();

    let (x, _) = row
        .iter()
        .find_position(|tile| *tile == &Tile::Guard(Direction::Up))
        .unwrap();

    let mut guard_pos: Point<i64> = Point {
        x: x as i64,
        y: y as i64,
    };

    while guard_pos.x < grid_size && guard_pos.y < grid_size && guard_pos.x >= 0 && guard_pos.y >= 0
    {
        seen.insert(guard_pos);
        let guard = grid[guard_pos.y as usize][guard_pos.x as usize];

        let Tile::Guard(mut dir) = guard else {
            unreachable!()
        };

        let step_forward = match dir {
            Direction::Up => Point {
                y: guard_pos.y - 1,
                ..guard_pos
            },
            Direction::Down => Point {
                y: guard_pos.y + 1,
                ..guard_pos
            },
            Direction::Left => Point {
                x: guard_pos.x - 1,
                ..guard_pos
            },
            Direction::Right => Point {
                x: guard_pos.x + 1,
                ..guard_pos
            },
        };

        let step_forward = if step_forward.x > grid_size
            || step_forward.y > grid_size
            || step_forward.x < 0
            || step_forward.y < 0
        {
            continue;
        } else {
            Point::new(step_forward.x as usize, step_forward.y as usize)
        };

        let next_pos = if let Some(Tile::Obstacle) = grid
            .get(step_forward.y)
            .and_then(|row| row.get(step_forward.x))
        {
            dir = match dir {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };

            guard_pos
        } else {
            Point::new(step_forward.x as i64, step_forward.y as i64)
        };

        grid[guard_pos.y as usize][guard_pos.x as usize] = Tile::Space;

        if let (Ok(x), Ok(y)) = (usize::try_from(next_pos.x), usize::try_from(next_pos.y)) {
            if let Some(row) = grid.get_mut(y) {
                if let Some(tile) = row.get_mut(x) {
                    *tile = Tile::Guard(dir);
                }
            }
        }

        guard_pos = next_pos;
    }

    seen
}

fn has_cycle(grid: &Grid) -> bool {
    let mut grid = grid.clone();
    let grid_size = grid.len() as i64;

    let mut seen: HashSet<_> = HashSet::new();

    let (y, row) = grid
        .iter()
        .find_position(|row| row.contains(&Tile::Guard(Direction::Up)))
        .unwrap();

    let (x, _) = row
        .iter()
        .find_position(|tile| *tile == &Tile::Guard(Direction::Up))
        .unwrap();

    let mut guard_pos: Point<i64> = Point {
        x: x as i64,
        y: y as i64,
    };

    while guard_pos.x < grid_size && guard_pos.y < grid_size && guard_pos.x >= 0 && guard_pos.y >= 0
    {
        let guard = grid[guard_pos.y as usize][guard_pos.x as usize];
        let Tile::Guard(mut dir) = guard else {
            unreachable!()
        };

        if seen.contains(&(guard_pos, dir)) {
            return true;
        }
        seen.insert((guard_pos, dir));

        let step_forward = match dir {
            Direction::Up => Point {
                y: guard_pos.y - 1,
                ..guard_pos
            },
            Direction::Down => Point {
                y: guard_pos.y + 1,
                ..guard_pos
            },
            Direction::Left => Point {
                x: guard_pos.x - 1,
                ..guard_pos
            },
            Direction::Right => Point {
                x: guard_pos.x + 1,
                ..guard_pos
            },
        };

        let step_forward = if step_forward.x > grid_size
            || step_forward.y > grid_size
            || step_forward.x < 0
            || step_forward.y < 0
        {
            guard_pos = step_forward;
            continue;
        } else {
            Point::new(step_forward.x as usize, step_forward.y as usize)
        };

        let next_pos = if let Some(Tile::Obstacle) = grid
            .get(step_forward.y)
            .and_then(|row| row.get(step_forward.x))
        {
            dir = match dir {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };

            guard_pos
        } else {
            Point::new(step_forward.x as i64, step_forward.y as i64)
        };

        grid[guard_pos.y as usize][guard_pos.x as usize] = Tile::Space;

        if let (Ok(x), Ok(y)) = (usize::try_from(next_pos.x), usize::try_from(next_pos.y)) {
            if let Some(row) = grid.get_mut(y) {
                if let Some(tile) = row.get_mut(x) {
                    *tile = Tile::Guard(dir);
                }
            }
        }

        guard_pos = next_pos;
    }

    false
}

mod parsers {
    use crate::day6::{Direction, Grid, GridRow, Tile};
    use nom::{
        branch::alt,
        character::complete::newline,
        multi::{many1, separated_list1},
        IResult, Parser,
    };
    use nom_locate::LocatedSpan;
    use nom_supreme::{
        error::ErrorTree, final_parser::final_parser, tag::complete::tag, ParserExt,
    };

    type ParseError<'a> = ErrorTree<Span<'a>>;

    type Span<'a> = LocatedSpan<&'a str>;

    pub(super) fn parse_input(input: &str) -> color_eyre::Result<Grid, ParseError> {
        final_parser(grid)(Span::new(input))
    }

    fn grid(input: Span) -> IResult<Span, Grid, ParseError> {
        separated_list1(newline, grid_row).parse(input)
    }

    fn grid_row(input: Span) -> IResult<Span, GridRow, ParseError> {
        many1(tile).context("grid row").parse(input)
    }

    fn tile(input: Span) -> IResult<Span, Tile, ParseError> {
        alt((space, obstacle, guard))
            .context("grid characters")
            .parse(input)
    }

    fn space(input: Span) -> IResult<Span, Tile, ParseError> {
        tag(".").map(|_| Tile::Space).parse(input)
    }

    fn obstacle(input: Span) -> IResult<Span, Tile, ParseError> {
        tag("#").map(|_| Tile::Obstacle).parse(input)
    }

    fn guard(input: Span) -> IResult<Span, Tile, ParseError> {
        tag("^").map(|_| Tile::Guard(Direction::Up)).parse(input)
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
        "....#.....
         .........#
         ..........
         ..#.......
         .......#..
         ..........
         .#..^.....
         ........#.
         #.........
         ......#..."
    };

    #[test]
    fn test_parsing() {
        let reports = parser(SAMPLE).unwrap();
        insta::assert_debug_snapshot!(reports);
    }

    #[rstest]
    #[case::part1(part1, 41)]
    #[case::part2(part2, 6)]
    fn sample_tests(#[case] f: fn(&Grid) -> usize, #[case] expected: usize) {
        let parsed = parser(SAMPLE).unwrap();
        let result = f(&parsed);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::part1(part1, 5329)]
    #[case::part2(part2, 2162)]
    fn prod_tests(#[case] f: fn(&Grid) -> usize, #[case] expected: usize) {
        let input = fs::read_to_string("input/2024/day6.txt").unwrap();
        let parsed = parser(input.trim_end()).unwrap();
        let result = f(&parsed);

        assert_eq!(result, expected);
    }
}
