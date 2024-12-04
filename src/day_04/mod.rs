use std::{iter::successors, num::NonZeroU32, rc::Rc};

/// ...
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    /// Represents the non-zero number of rows in the grid.
    pub rows: usize,
    /// Represents the non-zero number of columns in the grid.
    pub cols: usize,
    /// Represents a shared flattened slice of all items in the grid.
    pub items: Rc<[T]>,
    /// Represents the current position of the grid view.
    pub cursor: (usize, usize),
}

// Construction method implementations.
impl<T> Grid<T> {
    /// ...
    fn unchecked_from<I>(items: I, rows: usize, cols: usize, cursor: (usize, usize)) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        // Flatten the nested iterators into a single stream and collect as `Rc<[T]>`.
        let items = items.into_iter().collect();

        // Create `Grid` instance under the assumption that the input is valid.
        Grid {
            rows,
            cols,
            items,
            cursor,
        }
    }

    /// ...
    pub fn try_from<I>(items: I) -> Option<Self>
    where
        I: IntoIterator,
        I::Item: IntoIterator<Item = T>,
    {
        // Collect iterator into nested `Vec`s to ensure proper row and column sizes.
        let items: Vec<Vec<T>> = items
            .into_iter()
            .map(|line| line.into_iter().collect())
            .collect();

        // Assert that the number of rows is non-zero.
        let rows = items.get(0).map(|_| items.len())?;
        // Assert that the number of cols is non-zero given that the number of rows is non-zero.
        let cols = (items[0].len() > 0).then(|| items[0].len())?;

        // Assert that all rows have equal lengths (number of columns).
        items.iter().all(|line| line.len() == cols).then_some(())?;

        // Construct `Grid` from unsafe interface given all conditions have been validated.
        Some(Grid::unchecked_from(
            items.into_iter().flatten(),
            rows,
            cols,
            (0, 0),
        ))
    }
}

// Iteration method implementations.
impl<T> Grid<T> {
    /// ...
    pub fn iter_items(&self) -> impl Iterator<Item = &T> {
        // Iterate over references to avoid preemptive cloning.
        self.items.into_iter()
    }

    /// ...
    pub fn iter_cursors(&self) -> impl Iterator<Item = (usize, usize)> {
        let rows = 0..self.rows;
        let cols = 0..self.cols;

        // Calculate the cartesian product of all cursors and iterate in order.
        rows.flat_map(move |row| cols.clone().map(move |col| (row, col)))
    }
}

// Grid-specific method implementations.
impl<T> Grid<T> {
    /// ...
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        // Convert cartesian coordinates to absolute index and borrow with bounds checking.
        self.items.get((row * self.cols) + col)
    }

    /// ...
    pub fn shift(&self, row_off: isize, col_off: isize) -> Option<Grid<T>> {
        // ...
        let row = (self.cursor.0 as isize + row_off).try_into().ok()?;
        let col = (self.cursor.1 as isize + col_off).try_into().ok()?;

        // ...
        self.focus(row, col)
    }

    /// ...
    pub fn focus(&self, row: usize, col: usize) -> Option<Grid<T>> {
        // ...
        (row < self.rows).then_some(())?;
        (col < self.cols).then_some(())?;

        // ...
        Some(Grid {
            rows: self.rows,
            cols: self.cols,
            items: self.items.clone(),
            cursor: (row, col),
        })
    }
}

// Comonad instance implementation.
impl<T> Grid<T> {
    /// ...
    pub fn extract(&self) -> &T {
        // Unsafe index because `self.cursor` is guaranteed to be in-bounds.
        &self.items[(self.cursor.0 * self.cols) + self.cursor.1]
    }

    /// ...
    pub fn extend<F, U>(&self, f: F) -> Grid<U>
    where
        F: Fn(Grid<T>) -> U,
    {
        let cursors = self.iter_cursors();

        // Apply the given function to every cursor view of the grid.
        let items = cursors.map(|(row, col)| f(self.focus(row, col).unwrap()));

        // Collect from unchecked iterator because we know that dimensions are safe.
        Grid::unchecked_from(items, self.rows, self.cols, self.cursor)
    }
}

/// ...
pub fn check_linear(grid: Grid<char>) -> Option<NonZeroU32> {
    // ...
    const OFFSETS: [(isize, isize); 8] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    // ...
    (*grid.extract() == 'X').then_some(())?;

    // ...
    let match_offset = |y_off, x_off| -> bool {
        // ...
        successors(grid.shift(y_off, x_off), |grid| grid.shift(y_off, x_off))
            // ...
            .map(|grid| *grid.extract())
            // ...
            .take("MAS".len())
            // ...
            .eq("MAS".chars())
    };

    // ...
    let matches = OFFSETS
        .into_iter()
        // ...
        .filter_map(|(y_off, x_off)| match_offset(y_off, x_off).then_some(1))
        // ...
        .sum::<u32>();

    // ...
    matches.try_into().ok()
}

/// ...
pub fn check_cross(grid: Grid<char>) -> bool {
    // ...
    let result = (*grid.extract() == 'A').then_some(());

    // ...
    let result = result.and_then(|_| {
        // ...
        let fst = *grid.shift(1, 1)?.extract();
        let snd = *grid.shift(-1, -1)?.extract();

        // ...
        [('M', 'S'), ('S', 'M')].contains(&(fst, snd)).then_some(())
    });

    // ...
    let result = result.and_then(|_| {
        // ...
        let fst = *grid.shift(1, -1)?.extract();
        let snd = *grid.shift(-1, 1)?.extract();

        // ...
        [('M', 'S'), ('S', 'M')].contains(&(fst, snd)).then_some(())
    });

    // ...
    result.map_or(false, |_| true)
}

#[aoc(day4, part1)]
pub fn solve_part_1(input: &str) -> u32 {
    // ...
    let grid = Grid::try_from(input.lines().map(|line| line.chars())).unwrap();

    grid.extend(check_linear)
        .iter_items()
        .filter_map(|x| *x)
        .map(|x| x.get())
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_part_2(input: &str) -> u32 {
    // ...
    let grid = Grid::try_from(input.lines().map(|line| line.chars())).unwrap();

    grid.extend(check_cross)
        .iter_items()
        .filter(|b| **b)
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input = include_str!("./samples/sample_1.txt");
        assert_eq!(solve_part_1(input), 18);
    }

    #[test]
    fn case_2() {
        let input = include_str!("./samples/sample_1.txt");
        assert_eq!(solve_part_2(input), 9);
    }
}
