pub fn part_a(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            let mut ranges = line.split(',').map(|range_str| {
                let (start, end) = range_str.split_once('-').unwrap();
                let start = start.parse::<i32>().unwrap();
                let end = end.parse::<i32>().unwrap();
                (start, end)
            });

            let range1 = ranges.next()?;
            let range2 = ranges.next()?;

            // Check if range1 fully contains range2 or vice versa
            if (range1.0 <= range2.0 && range1.1 >= range2.1) || (range2.0 <= range1.0 && range2.1 >= range1.1) {
                Some(())
            } else {
                None
            }
        })
        .count() as i32
}

pub fn part_b(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            let mut ranges = line.split(',').map(|range_str| {
                let (start, end) = range_str.split_once('-').unwrap();
                let start = start.parse::<i32>().unwrap();
                let end = end.parse::<i32>().unwrap();
                (start, end)
            });

            let range1 = ranges.next()?;
            let range2 = ranges.next()?;

            // Check if ranges overlap
            if range1.0 <= range2.1 && range2.0 <= range1.1 {
                Some(())
            } else {
                None
            }
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{run_test_cases, TestCase};

    #[test]
    fn test_day_03_part_a() {
        let test_cases = [
            TestCase::<i32>::new("file:inputs/part_a_1.txt", 2),
            TestCase::<i32>::new("file:inputs/part_a_2.txt", 483),
            // Add more test cases and the file-based test case here
        ];

        run_test_cases(part_a, &test_cases);
    }

    #[test]
    fn test_day_03_part_b() {
        let test_cases = [
            TestCase::<i32>::new("file:inputs/part_a_1.txt", 4),
            TestCase::<i32>::new("file:inputs/part_a_2.txt", 874),
            // Add more test cases and the file-based test case here
        ];

        run_test_cases(part_b, &test_cases);
    }

}
