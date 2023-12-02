pub fn part_a(input: &str) -> i32 {
    let mut total_score = 0;

    for line in input.lines() {
        let trimmed_line = line.trim();
        let parts: Vec<&str> = trimmed_line.split_whitespace().collect();

        if parts.len() < 2 {
            continue; // Skip invalid lines
        }

        let (opponent, your_choice) = (parts[0].chars().next().unwrap(), parts[1].chars().next().unwrap());

        // Calculate the score for the shape you selected
        let shape_score = match your_choice {
            'X' => 1,      // Rock
            'Y' => 2,      // Paper
            'Z' => 3,      // Scissors
            _ => continue, // Invalid choice, skip line
        };

        // Determine the outcome of the round
        let outcome_score = match (opponent, your_choice) {
            ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6, // Win
            ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3, // Draw
            _ => 0,                                    // Loss
        };

        total_score += shape_score + outcome_score;
    }

    total_score
}

pub fn part_b(input: &str) -> i32 {
    let mut total_score = 0;

    for line in input.lines() {
        let trimmed_line = line.trim();
        let parts: Vec<&str> = trimmed_line.split_whitespace().collect();

        if parts.len() < 2 {
            continue; // Skip invalid lines
        }

        let (opponent, desired_outcome) = (parts[0].chars().next().unwrap(), parts[1].chars().next().unwrap());

        let your_choice = match (opponent, desired_outcome) {
            // Lose against Rock (A), choose Scissors (Z)
            ('A', 'X') => 'Z', 
            // Draw with Rock, choose Rock (X)
            ('A', 'Y') => 'X', 
            // Win against Rock, choose Paper (Y)
            ('A', 'Z') => 'Y', 
            // Lose against Paper (B), choose Rock (X)
            ('B', 'X') => 'X', 
            // Draw with Paper, choose Paper (Y)
            ('B', 'Y') => 'Y', 
            // Win against Paper, choose Scissors (Z)
            ('B', 'Z') => 'Z', 
            // Lose against Scissors (C), choose Paper (Y)
            ('C', 'X') => 'Y', 
            // Draw with Scissors, choose Scissors (Z)
            ('C', 'Y') => 'Z', 
            // Win against Scissors, choose Rock (X)
            ('C', 'Z') => 'X', 
            _ => continue, // Invalid or unhandled case
        };
        
        let shape_score = match your_choice {
            'X' => 1, // Rock
            'Y' => 2, // Paper
            'Z' => 3, // Scissors
            _ => continue, // Invalid choice, skip line
        };

        let outcome_score = match (opponent, your_choice) {
            ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6, // Win
            ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3, // Draw
            ('A', 'A') | ('B', 'B') | ('C', 'C') => 3, // Draw (same choices)
            _ => 0, // Loss
        };
        
        total_score += shape_score + outcome_score;
    }

    total_score
}


#[cfg(test)]
mod tests {
    use super::*;
    use common::{run_test_cases, TestCase};

    #[test]
    fn test_day_02_part_a() {
        let test_cases = [
            TestCase::<i32>::new("file:inputs/part_a_1.txt", 15),
            TestCase::<i32>::new("file:inputs/part_a_2.txt", 13446),
            // Add more test cases and the file-based test case here
        ];

        run_test_cases(part_a, &test_cases);
    }

    #[test]
    fn test_day_02_part_b() {
        let test_cases = [
            TestCase::<i32>::new("file:inputs/part_a_1.txt", 12),
            TestCase::<i32>::new("file:inputs/part_a_2.txt", 13509),
            // Add more test cases and the file-based test case here
        ];

        run_test_cases(part_b, &test_cases);
    }

}
