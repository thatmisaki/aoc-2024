#[aoc(day1, part1)]
pub fn solve_part_1(input: &str) -> u32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("   ").unwrap();
            (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
        })
        .unzip();

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right.into_iter())
        .map(|(left, right)| (left as i64 - right as i64).abs() as u32)
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part_2(_input: &str) -> u32 {
    // ![TODO]: implement `solve_part_2`.
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        let input = include_str!("./samples/sample_1.txt");
        assert_eq!(solve_part_1(input), 11);
    }
}
