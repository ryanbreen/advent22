use std::collections::HashMap;

pub fn part_a(input: &str) -> String {
    let mut lines = input.lines();
    let mut buffer = Vec::new();
    let mut stack_positions = Vec::new();
    let mut stacks = HashMap::new();

    // Buffer lines and identify stack positions
    for line in lines.by_ref() {
        if line.chars().nth(1) == Some('1') {
            for (i, c) in line.chars().enumerate() {
                if c.is_numeric() {
                    stack_positions.push(i);
                }
            }
            break;
        }
        buffer.push(line.to_string());
    }

    // Populate stacks
    for line in buffer.iter().rev() {
        for (stack_num, &pos) in stack_positions.iter().enumerate() {
            if let Some(crate_char) = line.chars().nth(pos) {
                if crate_char.is_alphabetic() {
                    stacks.entry(stack_num as i32 + 1)
                          .or_insert_with(Vec::new)
                          .push(crate_char);
                }
            }
        }
    }

    // Process the moves and print the state of stacks after each move
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
    
        // Check if parts vector has expected number of elements
        if parts.len() < 6 {
            println!("Warning: Unexpected format or insufficient data in line '{}'", line);
            continue;
        }
    
        let count: usize = match parts[1].parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error parsing count in line '{}': {}", line, e);
                continue;
            }
        };
        let from_stack: i32 = parts[3].parse().unwrap();
        let to_stack: i32 = parts[5].parse().unwrap();

        if let Some(crates_to_move) = stacks.get_mut(&from_stack) {
            if count <= crates_to_move.len() {
                let crates_moved: Vec<_> = crates_to_move
                    .drain(crates_to_move.len() - count..)
                    .collect();
                if let Some(target_stack) = stacks.get_mut(&to_stack) {
                    for crate_char in crates_moved.into_iter().rev() {
                        target_stack.push(crate_char);
                    }
                }
            }
        }
    }

    // Extract the top crate from each stack and combine them
    let mut result = String::new();
    for i in 1..=stacks.len() as i32 {
        if let Some(stack) = stacks.get(&i) {
            if let Some(&crate_char) = stack.last() {
                result.push(crate_char);
            }
        }
    }

    result
}

fn print_stacks(stacks: &HashMap<i32, Vec<char>>) {
    let max_height = stacks.values().map(|v| v.len()).max().unwrap_or(0);
    for level in (0..max_height).rev() {
        for i in 1..=stacks.len() as i32 {
            if let Some(stack) = stacks.get(&i) {
                if level < stack.len() {
                    print!("[{}]", stack[level]);
                } else {
                    print!("   ");
                }
            }
            print!(" ");
        }
        println!();
    }
    for i in 1..=stacks.len() as i32 {
        print!(" {:^3}", i);
    }
    println!("\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{run_test_cases, TestCase};

    #[test]
    fn test_day_05_part_a() {
        let test_cases = [
            TestCase::<String>::new("file:inputs/part_a_1.txt", String::from("CMZ")),
            TestCase::<String>::new("file:inputs/part_a_2.txt", String::from("GFTNRBZPF")),
            // Add more test cases and the file-based test case here
        ];

        run_test_cases(part_a, &test_cases);
    }

    /*

    #[test]
    fn test_day_03_part_b() {
        let test_cases = [
            TestCase::<i32>::new("file:inputs/part_a_1.txt", 4),
            TestCase::<i32>::new("file:inputs/part_a_2.txt", 874),
            // Add more test cases and the file-based test case here
        ];

        run_test_cases(part_b, &test_cases);
    } */
}
