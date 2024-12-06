use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// ...
pub fn match_ordered<'a, T>(map: &'a HashMap<T, HashSet<T>>, items: &'a Vec<T>) -> Option<&'a T>
where
    T: Eq + Hash,
{
    items
        .iter()
        // Zip each element with the element to its right to form adjacent pairs.
        .zip(&items[1..])
        // Assert that all adjacent pairs obey the dependencies using the transitive property.
        .all(|(lhs, rhs)| map.get(lhs).map_or_else(|| false, |key| key.contains(rhs)))
        // Lazily evaluate the middle element upon successful matching.
        .then(|| &items[items.len() / 2])
}

/// ...
pub fn match_unordered<T>(map: &HashMap<T, HashSet<T>>, items: &Vec<T>) -> Option<T>
where
    T: Eq + Hash + Clone,
{
    let mut new_items = items.clone();

    new_items.sort_by(|lhs, rhs| {
        map.get(lhs).map_or_else(
            || Ordering::Greater,
            |key| {
                key.contains(rhs)
                    .then_some(())
                    .map_or_else(|| Ordering::Greater, |_| Ordering::Less)
            },
        )
    });

    (items != &new_items).then(move || new_items[new_items.len() / 2].clone())
}

/// ...
pub fn parse_rules(input: &str) -> Option<HashMap<u32, HashSet<u32>>> {
    // Collect all rules into a map of dependencies.
    let mut map = HashMap::new();

    input
        .lines()
        // Break each line into a pair of numbers to represent dependency pairs.
        .map(|line| line.split_once("|"))
        // Parse each side of the dependency pair.
        .map(|pair| pair.and_then(|(lhs, rhs)| Some((lhs.parse().ok()?, rhs.parse().ok()?))))
        // Insert each dependency pair as a value in the dependency map.
        .try_for_each(|pair| {
            let (lhs, rhs) = pair?;

            // If the left-hand value does not exist in the map, add it with an empty set.
            map.entry(lhs).or_insert_with(|| HashSet::new()).insert(rhs);

            Some(())
        })?;

    Some(map)
}

/// ...
pub fn parse_updates(input: &str) -> Option<Vec<Vec<u32>>> {
    input
        .lines()
        // Parse each line as an ordered collection of comma-delimited numbers.
        .map(|line| line.split(",").map(|x| x.parse().ok()).collect())
        // Lift the inner option of the parsing computation into the outer scope.
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part_1(input: &str) -> u32 {
    // ...
    let (rules, updates) = input.split_once("\n\n").unwrap();

    // ...
    let rules = parse_rules(rules).unwrap();
    let updates = parse_updates(updates).unwrap();

    updates
        .iter()
        .map(|update| match_ordered(&rules, update).map_or_else(|| 0, |x| *x))
        .sum()
}

#[aoc(day5, part2)]
pub fn solve_part_2(input: &str) -> u32 {
    // ...
    let (rules, updates) = input.split_once("\n\n").unwrap();

    // ...
    let rules = parse_rules(rules).unwrap();
    let updates = parse_updates(updates).unwrap();

    updates
        .iter()
        .map(|update| match_unordered(&rules, update).unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input = include_str!("./samples/sample_1");
        assert_eq!(solve_part_1(input), 143);
    }

    #[test]
    fn case_2() {
        let input = include_str!("./samples/sample_1");
        assert_eq!(solve_part_2(input), 123);
    }
}
