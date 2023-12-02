pub fn part_a(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (first_compartment, second_compartment) = line.split_at(line.len() / 2);
            let mut common_items = first_compartment
                .chars()
                .filter(|&c| second_compartment.contains(c))
                .collect::<Vec<char>>();

            // Remove duplicates
            common_items.sort_unstable();
            common_items.dedup();

            // There should be only one common item per rucksack
            if let Some(&item) = common_items.first() {
                if item.is_ascii_lowercase() {
                    // Lowercase priorities: 1 - 26
                    item as i32 - 'a' as i32 + 1
                } else {
                    // Uppercase priorities: 27 - 52
                    item as i32 - 'A' as i32 + 27
                }
            } else {
                0
            }
        })
        .sum()
}

use std::collections::HashSet;
pub fn part_b(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut total_priority = 0;

    for chunk in lines.chunks(3) {
        let common_items = chunk
            .iter()
            .map(|&rucksack| rucksack.chars().collect::<HashSet<char>>())
            .fold(None, |acc: Option<HashSet<char>>, items| {
                match acc {
                    Some(common) => Some(common.intersection(&items).cloned().collect()),
                    None => Some(items),
                }
            })
            .unwrap_or_default();

        if let Some(&badge) = common_items.iter().next() {
            if badge.is_ascii_lowercase() {
                total_priority += badge as i32 - 'a' as i32 + 1;
            } else {
                total_priority += badge as i32 - 'A' as i32 + 27;
            }
        }
    }

    total_priority
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{run_test_cases, TestCase};

    #[test]
    fn test_day_03_part_a() {
        let test_cases = [
            TestCase::<i32>::new("file:inputs/part_a_1.txt", 157),
            TestCase::<i32>::new("file:inputs/part_a_2.txt", 8233),
            // Add more test cases and the file-based test case here
        ];

        run_test_cases(part_a, &test_cases);
    }

    #[test]
    fn test_day_03_part_b() {
        let test_cases = [
            TestCase::<i32>::new("file:inputs/part_a_1.txt", 70),
            TestCase::<i32>::new("file:inputs/part_a_2.txt", 2821),
            // Add more test cases and the file-based test case here
        ];

        run_test_cases(part_b, &test_cases);
    }

}
