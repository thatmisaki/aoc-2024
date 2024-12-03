use std::{cmp::Ordering, iter::once};

/// ...
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Record {
    /// Represents a `Record` containing no levels.
    Empty,
    /// Represents a `Record` containing only one level.
    Single(u32),
    /// Represents a `Record` containing strictly increasing levels.
    Increasing(Vec<u32>),
    /// Represents a `Record` containing strictly decreasing levels.
    Decreasing(Vec<u32>),
}

impl Record {
    /// ...
    #[inline]
    fn build_empty(margin: usize, items: &[u32], item: u32) -> Option<Self> {
        // ...
        Self::Single(item).try_from_inner(margin, items)
    }

    /// ...
    #[inline]
    fn build_single(margin: usize, items: &[u32], item: u32, prev: u32) -> Option<Self> {
        // If the absolute difference between the current and previous items is
        // greater than 3, then abort evaluation.
        (item.abs_diff(prev) <= 3).then_some(())?;

        // ...
        match item.cmp(&prev) {
            // ...
            Ordering::Equal => None,
            // ...
            Ordering::Greater => Self::Increasing(vec![prev, item]).try_from_inner(margin, items),
            // ...
            Ordering::Less => Self::Decreasing(vec![prev, item]).try_from_inner(margin, items),
        }
    }

    /// ...
    #[inline]
    fn build_increasing(margin: usize, items: &[u32], item: u32, seen: &Vec<u32>) -> Option<Self> {
        // `Report::Decreasing` will always be non-empty, so this is safe.
        let prev = seen.last().copied().unwrap();

        // If the current item is not more than the previous, or their absolute difference
        // is greater than 3, then abort evaluation.
        (item > prev && item.abs_diff(prev) <= 3).then_some(())?;

        // ...
        let seen = seen.clone().into_iter().chain(once(item)).collect();

        // ...
        Self::Increasing(seen).try_from_inner(margin, items)
    }

    /// ...
    #[inline]
    fn build_decreasing(margin: usize, items: &[u32], item: u32, seen: &Vec<u32>) -> Option<Self> {
        // `Report::Decreasing` will always be non-empty, so this is safe.
        let prev = seen.last().copied().unwrap();

        // If the current item is not less than the previous, or their absolute difference
        // is greater than 3, then abort evaluation.
        (item < prev && item.abs_diff(prev) <= 3).then_some(())?;

        // ...
        let seen = seen.clone().into_iter().chain(once(item)).collect();

        // ...
        Self::Decreasing(seen).try_from_inner(margin, items)
    }

    /// ...
    #[inline]
    fn try_from_match(self, margin: usize, items: &[u32], item: u32) -> Option<Self> {
        match self {
            // ...
            Self::Empty => Self::build_empty(margin, items, item),
            // ...
            Self::Single(prev) => Self::build_single(margin, items, item, prev),
            // ...
            Self::Increasing(ref seen) => Self::build_increasing(margin, items, item, seen),
            // ...
            Self::Decreasing(ref seen) => Self::build_decreasing(margin, items, item, seen),
        }
    }

    /// ...
    fn try_from_inner(self, margin: usize, items: &[u32]) -> Option<Self> {
        // Base Case: ...
        let Some((&item, rest)) = items.split_first() else {
            return Some(self);
        };

        // Recursive Case: ...
        let next_branch = || self.clone().try_from_match(margin, rest, item);

        // Recursive Case: ...
        let skip_branch = || self.clone().try_from_inner(margin - 1, rest);

        // ...
        next_branch().or_else(|| (margin > 0).then(skip_branch).flatten())
    }

    /// ...
    #[inline]
    pub fn try_from<I>(margin: usize, items: I) -> Option<Self>
    where
        I: IntoIterator<Item = u32>,
    {
        // Collect items into a `Vec` to access as a slice for recursion.
        let items = items.into_iter().collect::<Vec<_>>();

        // ...
        Self::Empty.try_from_inner(margin, &items)
    }

    /// ...
    pub fn unwrap(self) -> Vec<u32> {
        match self {
            Self::Empty => vec![],
            Self::Single(item) => vec![item],
            Self::Increasing(items) => items,
            Self::Decreasing(items) => items,
        }
    }
}

#[aoc(day2, part1)]
pub fn solve_part_1(input: &str) -> u32 {
    input
        // ...
        .lines()
        // ...
        .filter_map(|line| {
            let line = line.split(" ").map(|item| item.parse::<u32>().unwrap());

            let record = Record::try_from(0, line).map(Record::unwrap)?;

            record
                .iter()
                .copied()
                .zip(record.iter().copied().skip(1))
                .map(|(x, y)| (x.max(y) - x.min(y)) <= 3)
                .all(|valid| valid)
                .then_some(())
        })
        // ...
        .count() as u32
}

#[aoc(day2, part2)]
pub fn solve_part_2(input: &str) -> u32 {
    input
        // ...
        .lines()
        // ...
        .filter_map(|line| {
            let line = line.split(" ").map(|item| item.parse::<u32>().unwrap());

            let record = Record::try_from(1, line).map(Record::unwrap)?;

            record
                .iter()
                .copied()
                .zip(record.iter().copied().skip(1))
                .map(|(x, y)| (x.max(y) - x.min(y)) <= 3)
                .all(|valid| valid)
                .then_some(())
        })
        // ...
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input = include_str!("./samples/sample_1.txt");
        assert_eq!(solve_part_1(input), 2);
    }

    #[test]
    fn case_2() {
        let input = include_str!("./samples/sample_1.txt");
        assert_eq!(solve_part_2(input), 4);
    }
}
