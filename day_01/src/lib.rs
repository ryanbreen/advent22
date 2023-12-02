// Part A implementation
pub fn part_a(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|elf_inventory| {
            elf_inventory
                .lines()
                .filter_map(|line| line.parse::<i32>().ok())
                .sum::<i32>()
        })
        .max()
        .unwrap_or(0)
}

pub fn part_b(input: &str) -> i32 {
    let mut calories: Vec<i32> = input
        .split("\n\n")
        .map(|elf_inventory| {
            elf_inventory
                .lines()
                .filter_map(|line| line.parse::<i32>().ok())
                .sum::<i32>()
        })
        .collect();

    // Sort in descending order and take the top three values
    calories.sort_by(|a, b| b.cmp(a));
    calories.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{run_test_cases, TestCase};

    #[test]
    fn test_day_01_part_a() {
        let test_cases = [
            TestCase::<i32>::new("file:inputs/part_a_1.txt", 24000),
            TestCase::<i32>::new("file:inputs/part_a_2.txt", 69501),
            // Add more test cases and the file-based test case here
        ];

        run_test_cases(part_a, &test_cases);
    }

    #[test]
    fn test_day_01_part_b() {
        let test_cases = [
            TestCase::<i32>::new("file:inputs/part_a_1.txt", 45000),
            TestCase::<i32>::new("file:inputs/part_a_2.txt", 202346),
            // Add more test cases and the file-based test case here
        ];

        run_test_cases(part_b, &test_cases);
    }

}


