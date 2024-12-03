/// ...
pub enum Expr {
    /// ...
    Mul(u32, u32),
    /// ...
    Enable,
    /// ...
    Disable,
}

impl Expr {
    /// ...
    fn parse_mul(input: &str) -> Option<Self> {
        input
            // ...
            .strip_prefix("mul(")
            // ...
            .and_then(|rest| rest.split_once(","))
            // ...
            .and_then(|(num1, rest)| rest.split_once(")").map(|(num2, _)| (num1, num2)))
            // ...
            .and_then(|(num1, num2)| {
                // ...
                let eval_num1 = || num1.parse::<u32>().ok();
                let eval_num2 = || num2.parse::<u32>().ok();

                // ...
                eval_num1().and_then(|num1| eval_num2().map(|num2| Self::Mul(num1, num2)))
            })
    }

    /// ...
    fn parse_enable(input: &str) -> Option<Self> {
        input
            // ...
            .strip_prefix("do()")
            // ...
            .map(|_| Self::Enable)
    }

    /// ...
    fn parse_disable(input: &str) -> Option<Self> {
        input
            // ...
            .strip_prefix("don't()")
            // ...
            .map(|_| Self::Disable)
    }

    /// ...
    pub fn prefix<S>(input: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        let input = input.as_ref();

        // ...
        let parse_mul = || Self::parse_mul(input);
        // ...
        let parse_enable = || Self::parse_enable(input);
        // ...
        let parse_disable = || Self::parse_disable(input);

        // ...
        parse_mul().or_else(parse_enable).or_else(parse_disable)
    }

    /// ...
    pub fn collect<S>(input: S) -> Vec<Self>
    where
        S: AsRef<str>,
    {
        let input = input.as_ref();

        // ...
        (0..input.len())
            // ...
            .filter_map(|idx| Self::prefix(&input[idx..]))
            // ...
            .collect()
    }

    /// ...
    fn eval_inner<I>(mut exprs: I, enabled: bool, total: u32) -> u32
    where
        I: Iterator<Item = Self>,
    {
        // ...
        let Some(expr) = exprs.next() else {
            return total;
        };

        // ...
        let (enabled, total) = match expr {
            // ...
            Self::Mul(num1, num2) if enabled => (enabled, total + (num1 * num2)),
            // ...
            Self::Enable => (true, total),
            // ...
            Self::Disable => (false, total),
            // ...
            _ => (enabled, total),
        };

        // ...
        Self::eval_inner(exprs, enabled, total)
    }

    /// ...
    pub fn eval<I>(exprs: I) -> u32
    where
        I: IntoIterator<Item = Self>,
    {
        Self::eval_inner(exprs.into_iter(), true, 0)
    }
}

/// ...
pub fn prefix_expr(input: &str) -> Option<(u32, u32)> {
    input
        // ...
        .strip_prefix("mul(")
        // ...
        .and_then(|rest| rest.split_once(","))
        // ...
        .and_then(|(num1, rest)| rest.split_once(")").map(|(num2, _)| (num1, num2)))
        // ...
        .and_then(|(num1, num2)| {
            // ...
            let eval_num1 = || num1.parse::<u32>().ok();
            let eval_num2 = || num2.parse::<u32>().ok();

            // ...
            eval_num1().and_then(|num1| eval_num2().map(|num2| (num1, num2)))
        })
}

/// ...
pub fn extend<T, F>(input: &str, f: F) -> Vec<T>
where
    F: Fn(&str) -> Option<T>,
{
    (0..input.len())
        .filter_map(|idx| f(&input[idx..]))
        .collect()
}

/// ...
#[aoc(day3, part1)]
pub fn solve_part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            extend(line, prefix_expr)
                .into_iter()
                .map(|(num1, num2)| num1 * num2)
                .sum::<u32>()
        })
        .sum::<u32>()
}

/// ...
#[aoc(day3, part2)]
pub fn solve_part_2(input: &str) -> u32 {
    let exprs = input
        .lines()
        .flat_map(|line| Expr::collect(line))
        .collect::<Vec<_>>();

    Expr::eval(exprs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input = include_str!("./samples/sample_1.txt");
        assert_eq!(solve_part_1(input), 161);
    }

    #[test]
    fn case_2() {
        let input = include_str!("./samples/sample_2.txt");
        assert_eq!(solve_part_2(input), 48);
    }
}
