use itertools::Itertools;
use num::{
    traits::{SaturatingAdd, SaturatingSub},
    Float,
};
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::Add,
};

#[derive(Copy, Clone, Debug)]
pub struct Point<T>
where
    T: Clone + Copy + Debug + num::Num,
{
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Clone + Copy + Debug + num::Num,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub fn constrain(&self, min: &Point<T>, max: &Point<T>) -> Self
    where
        T: Ord,
    {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }

    #[must_use]
    pub fn constrain_floating(&self, min: &Point<T>, max: &Point<T>) -> Self
    where
        T: Float,
    {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }

    pub fn is_valid(&self, min: &Point<T>, max: &Point<T>) -> bool
    where
        T: PartialOrd,
    {
        self.x >= min.x && self.y >= min.y && self.x <= max.x && self.y <= max.y
    }

    pub fn manhattan_distance(&self, other: &Point<T>) -> T
    where
        T: num::traits::Signed,
    {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn manhattan_distance_unsigned(&self, other: &Point<T>) -> T
    where
        T: Ord + num::traits::Unsigned,
    {
        let max_x = self.x.max(other.x);
        let min_x = self.x.min(other.x);

        let max_y = self.y.max(other.y);
        let min_y = self.y.min(other.y);
        (max_x - min_x) + (max_y - min_y)
    }

    #[allow(clippy::too_many_lines)]
    pub fn neighbors(&self) -> Vec<Self>
    where
        T: SaturatingAdd + SaturatingSub + Hash + Eq,
    {
        enum Op {
            Sub,
            Add,
        }

        struct Shift<T> {
            op: Op,
            scale: T,
        }

        let modifiers = [
            (
                Shift {
                    op: Op::Sub,
                    scale: T::one(),
                },
                Shift {
                    op: Op::Add,
                    scale: T::zero(),
                },
            ),
            (
                Shift {
                    op: Op::Add,
                    scale: T::one(),
                },
                Shift {
                    op: Op::Add,
                    scale: T::zero(),
                },
            ),
            (
                Shift {
                    op: Op::Add,
                    scale: T::zero(),
                },
                Shift {
                    op: Op::Sub,
                    scale: T::one(),
                },
            ),
            (
                Shift {
                    op: Op::Add,
                    scale: T::zero(),
                },
                Shift {
                    op: Op::Add,
                    scale: T::one(),
                },
            ),
            (
                Shift {
                    op: Op::Sub,
                    scale: T::one(),
                },
                Shift {
                    op: Op::Sub,
                    scale: T::one(),
                },
            ),
            (
                Shift {
                    op: Op::Sub,
                    scale: T::one(),
                },
                Shift {
                    op: Op::Add,
                    scale: T::one(),
                },
            ),
            (
                Shift {
                    op: Op::Add,
                    scale: T::one(),
                },
                Shift {
                    op: Op::Sub,
                    scale: T::one(),
                },
            ),
            (
                Shift {
                    op: Op::Add,
                    scale: T::one(),
                },
                Shift {
                    op: Op::Add,
                    scale: T::one(),
                },
            ),
        ];

        modifiers
            .iter()
            .map(|shift| {
                let x = match shift.0.op {
                    Op::Sub => self.x.saturating_sub(&shift.0.scale),
                    Op::Add => self.x.saturating_add(&shift.0.scale),
                };

                let y = match shift.1.op {
                    Op::Sub => self.y.saturating_sub(&shift.1.scale),
                    Op::Add => self.y.saturating_add(&shift.1.scale),
                };

                Self::new(x, y)
            })
            .filter(|point| point != self)
            .unique()
            .collect()
    }
}

impl<T> Hash for Point<T>
where
    T: Clone + Copy + Debug + num::Num + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl<T> PartialEq for Point<T>
where
    T: Clone + Copy + Debug + num::Num + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> Eq for Point<T> where T: Clone + Copy + Debug + Eq + num::Num + PartialEq {}

impl<T> Add<Point<T>> for Point<T>
where
    T: Clone + Copy + Debug + num::Num,
{
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> From<(T, T)> for Point<T>
where
    T: Clone + Copy + Debug + num::Num,
{
    fn from(value: (T, T)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T> Default for Point<T>
where
    T: Clone + Copy + Debug + num::Num + Default,
{
    fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;
    use insta::assert_debug_snapshot;
    use pretty_assertions::assert_eq;

    #[test]
    fn constraints() {
        let min = Point::new(0, 0);
        let max = Point::new(10, 10);

        let test = Point::new(15, -16);
        let res = test.constrain(&min, &max);

        assert_eq!(res, Point::new(10, 0));

        let test = Point::new(-15, -16);
        let res = test.constrain(&min, &max);
        assert_eq!(res, Point::new(0, 0));

        let min = Point::new(0.0, 0.0);
        let max = Point::new(1.0, 1.0);

        let test = Point::new(-2.5, 1.01);
        let res = test.constrain_floating(&min, &max);
        assert_eq!(res, Point::new(0.0, 1.0));

        let test = Point::new(0.25, 0.3333);
        let res = test.constrain_floating(&min, &max);
        assert_eq!(res, Point::new(0.25, 0.3333));
    }

    #[test]
    fn validity() {
        let min = Point::new(0, 0);
        let max = Point::new(10, 10);

        let test = Point::new(15, -16);
        assert!(!test.is_valid(&min, &max));

        let test = Point::new(-15, -16);
        assert!(!test.is_valid(&min, &max));

        let test = Point::new(3, 6);
        assert!(test.is_valid(&min, &max));

        let min = Point::new(0.0, 0.0);
        let max = Point::new(1.0, 1.0);

        let test = Point::new(-2.5, 1.01);
        assert!(!test.is_valid(&min, &max));

        let test = Point::new(0.25, 0.3333);
        assert!(test.is_valid(&min, &max));
    }

    #[test]
    fn manhattan_distances() {
        let origin = Point::new(0, 0u64);
        let pt = Point::new(20, 15u64);

        assert_eq!(origin.manhattan_distance_unsigned(&pt), 35);

        let origin = Point::new(0, 0);
        let pt = Point::new(20, -15);
        assert_eq!(origin.manhattan_distance(&pt), 35);

        let origin = Point::new(0.0, 0.0);
        let pt = Point::new(1.25, 2.75);
        assert_approx_eq!(f64, origin.manhattan_distance(&pt), 4.);
    }

    #[test]
    fn neighborhoods() {
        let point = Point::new(0, 0u64);
        let neighbors = point.neighbors();
        assert_eq!(neighbors.len(), 3);
        assert_debug_snapshot!(neighbors);

        let point = Point::new(0, 0);
        let neighbors = point.neighbors();
        assert_eq!(neighbors.len(), 8);
        assert_debug_snapshot!(neighbors);
    }
}
