use std::cmp::Ordering;
use std::iter::once;

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
    fn try_from_empty<I>(margin: usize, item: u32, items: I) -> Option<Record>
    where
        I: Iterator<Item = u32> + Clone,
    {
        // ...
        Self::Single(item).try_from_inner(margin, items)
    }

    /// ...
    fn try_from_single<I>(margin: usize, prev: u32, item: u32, items: I) -> Option<Record>
    where
        I: Iterator<Item = u32> + Clone,
    {
        match (item.cmp(&prev), (item.max(prev) - item.min(prev)) <= 3) {
            // If the next element is larger than the previous,
            // recurse with a strictly increasing record.
            (Ordering::Greater, true) => {
                Self::Increasing(vec![prev, item]).try_from_inner(margin, items)
            }

            // If the next element is smaller than the previous,
            // recurse with a strictly decreasing record.
            (Ordering::Less, true) => {
                Self::Decreasing(vec![prev, item]).try_from_inner(margin, items)
            }

            // If the two elements are equal, do not evaluate the eager path.
            _ => None,
        }
    }

    /// ...
    fn try_from_increasing<I>(margin: usize, prevs: Vec<u32>, item: u32, items: I) -> Option<Record>
    where
        I: Iterator<Item = u32> + Clone,
    {
        // A strictly increasing record is guaranteed to have at least two items.
        let valid = prevs
            .last()
            .copied()
            .map(|prev| (item > prev) && (item.max(prev) - item.min(prev)) <= 3)?;

        valid.then_some(()).and_then(|_| {
            // Recurse eagerly with the record created from the original vector
            // followed by the new item.
            let values = prevs.clone().into_iter().chain(once(item)).collect();
            Self::Increasing(values).try_from_inner(margin, items.clone())
        })
    }

    /// ...
    fn try_from_decreasing<I>(margin: usize, prevs: Vec<u32>, item: u32, items: I) -> Option<Record>
    where
        I: Iterator<Item = u32> + Clone,
    {
        // A strictly decreasing record is guaranteed to have at least two items.
        let valid = prevs
            .last()
            .copied()
            .map(|prev| (item < prev) && (item.max(prev) - item.min(prev)) <= 3)?;

        valid.then_some(()).and_then(|_| {
            // Recurse eagerly with the record created from the original vector
            // followed by the new item.
            let values = prevs.clone().into_iter().chain(once(item)).collect();
            Self::Decreasing(values).try_from_inner(margin, items)
        })
    }

    /// ...
    fn try_from_eager<I>(self, margin: usize, item: u32, items: I) -> Option<Record>
    where
        I: Iterator<Item = u32> + Clone,
    {
        match self {
            // If the record is empty, recurse with a single element record.
            Self::Empty => Self::try_from_empty(margin, item, items),

            // If the record has a single element, recurse with a multi-element record.
            Self::Single(prev) => Self::try_from_single(margin, prev, item, items),

            // If the record is strictly increasing, recurse eagerly if the next item
            // is larger than the previous.
            Self::Increasing(prevs) => Self::try_from_increasing(margin, prevs, item, items),

            // If the record is strictly increasing, recurse eagerly if the next item
            // is larger than the previous.
            Self::Decreasing(prevs) => Self::try_from_decreasing(margin, prevs, item, items),
        }
    }

    /// ...
    fn try_from_inner<I>(self, margin: usize, mut items: I) -> Option<Record>
    where
        I: Iterator<Item = u32> + Clone,
    {
        // Base case: no more items to read, so return the current record.
        let Some(item) = items.next() else {
            return Some(self);
        };

        // Eager recursive case: attempt to evaluate the path in which
        // the next item is added to the record if valid.
        let solution = self.clone().try_from_eager(margin, item, items.clone());

        // Lazy recursive case: if the eager path found no solutions,
        // evaluate the path where the next item is ignored.
        solution.or_else(|| {
            // If there are no more allotted errors, return no solutions.
            (margin != 0)
                .then_some(())
                .and_then(|_| self.try_from_inner(margin - 1, items))
        })
    }

    /// ...
    pub fn try_from<I>(margin: usize, iter: I) -> Option<Record>
    where
        I: IntoIterator<Item = u32>,
        I::IntoIter: Clone,
    {
        // ...
        Self::Empty.try_from_inner(margin, iter.into_iter())
    }

    /// ...
    pub fn unwrap(self) -> Vec<u32> {
        // Flatten record into a vector of levels.
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
