use std::cmp::Ordering;

/// ...
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Report {
    /// ...
    Increasing(Vec<u32>),
    /// ...
    Decreasing(Vec<u32>),
}

impl Report {
    /// ...
    pub fn from<S>(line: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        let nums = line
            .as_ref()
            .split(" ")
            .map(|item| item.parse::<u32>().ok())
            .collect::<Option<Vec<_>>>()?;

        let (fst, snd) = nums
            .get(0)
            .and_then(|fst| nums.get(1).map(|snd| (*fst, *snd)))?;

        match fst.cmp(&snd) {
            Ordering::Equal => None,
            // Increasing order
            Ordering::Less => {
                let test = nums.into_iter().skip(1).fold(Some(vec![fst]), |nums, num| {
                    nums.and_then(|mut nums| {
                        nums.last().copied().and_then(|prev| {
                            (num > prev && (num - prev) <= 3).then(|| {
                                nums.push(num);
                                nums
                            })
                        })
                    })
                });
                test.map(Report::Increasing)
            }
            // Decreasing order
            Ordering::Greater => {
                let test = nums.into_iter().skip(1).fold(Some(vec![fst]), |nums, num| {
                    nums.and_then(|mut nums| {
                        nums.last().copied().and_then(|prev| {
                            (num < prev && (prev - num) <= 3).then(|| {
                                nums.push(num);
                                nums
                            })
                        })
                    })
                });
                test.map(Report::Decreasing)
            }
        }
    }
}

#[aoc(day2, part1)]
pub fn solve_part_1(input: &str) -> u32 {
    let reports = input
        .lines()
        .map(|line| Report::from(line))
        .collect::<Vec<Option<_>>>();

    reports.into_iter().filter_map(|x| x).count() as u32
}

#[aoc(day2, part2)]
pub fn solve_part_2(input: &str) -> u32 {
    // ![TODO]: implement `solve_part_2()`.
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input = include_str!("./samples/sample_1.txt");
        assert_eq!(solve_part_1(input), 2);
    }
}
