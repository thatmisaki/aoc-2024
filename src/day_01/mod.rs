use std::collections::HashMap;

/// Collect the input statement into two columns.
pub fn collect_columns<S, V>(input: S) -> (V, V)
where
    S: AsRef<str>,
    V: Default + Extend<u32>,
{
    input
        .as_ref()
        // Iterate over each line of input.
        .lines()
        // Map each line to a pair of numbers from each column.
        .map(|line| {
            let (left, right) = line.split_once("   ").unwrap();
            (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
        })
        // Unzip all pairs to get the two columns.
        .unzip()
}

#[aoc(day1, part1)]
pub fn solve_part_1(input: &str) -> u32 {
    // Collect two input columns as `Vec<u32>`.
    let (mut left, mut right): (Vec<_>, Vec<_>) = collect_columns(input);

    // Sort the columns in order to iterate pairwise.
    left.sort();
    right.sort();

    left.into_iter()
        // Iterate over sorted pairs from the left and right columns.
        .zip(right.into_iter())
        // Calculate the absolute distance between each pair.
        .map(|(left, right)| (left as i64 - right as i64).abs() as u32)
        // Total the absolute distance of all pairs.
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part_2(input: &str) -> u32 {
    // Collect two input columns as `Vec<u32>`.
    let (left, right): (Vec<_>, Vec<_>) = collect_columns(input);

    // Collect occurrences of all values in the right column.
    let mut count_map: HashMap<u32, u32> = HashMap::new();

    right
        .into_iter()
        // Set all occurrences for each number in the right column.
        .for_each(|num| {
            *count_map.entry(num).or_insert(0) += 1;
        });

    left.into_iter()
        // For all numbers from the left column in the map,
        // multiply the number by the count.
        .filter_map(|num| count_map.get(&num).map(|count| num * count))
        // Total the product of occurrences for all numbers in the left column.
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input = include_str!("./samples/sample_1.txt");
        assert_eq!(solve_part_1(input), 11);
    }

    #[test]
    fn case_2() {
        let input = include_str!("./samples/sample_1.txt");
        assert_eq!(solve_part_2(input), 31);
    }
}
