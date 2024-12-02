//! Day 2: Red-Nosed Reports
//!
//! This solution validates sequences of numbers from a reactor safety report to determine if they
//! follow specific safety rules. The implementation focuses on performance through:
//!
//! - Using an inline validation function to reduce function call overhead
//! - Employing iterators and windows for efficient sequence traversal
//! - Early returns for invalid cases
//! - Minimal allocations by reusing vectors
//!
//! Time Complexity:
//! - O(n) for reading the file, where n is file size
//! - O(m) for parsing each line, where m is line length  
//! - O(k) for validating each sequence, where k is sequence length
//! - Overall: O(n + L*(m + k)) where L is number of lines
//!
//! Space Complexity:
//! - O(m) for storing each line's numbers vector
//! - O(1) additional space for validation logic

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

/// Represents the validation result of a sequence.
/// Using an enum rather than bool provides more clarity and extensibility.
#[derive(Debug)]
enum ValidationResult {
    /// Sequence follows all safety rules
    Valid,
    /// Sequence violates at least one safety rule
    Invalid,
}

/// Validates if a sequence of numbers follows the reactor safety rules.
///
/// # Rules
/// 1. Numbers must be strictly increasing or decreasing (no plateaus)
/// 2. Adjacent differences must be between 1 and 3 inclusive
///
/// # Implementation Details
/// - Uses windows(2) for efficient pairwise comparison
/// - Determines direction (increasing/decreasing) from first pair
/// - Early returns for invalid cases to avoid unnecessary computation
///
/// # Arguments
/// * `numbers` - Slice of integers representing the sequence to validate
///
/// # Returns
/// * `ValidationResult::Valid` if sequence follows all rules
/// * `ValidationResult::Invalid` if any rule is violated
///
/// # Time Complexity
/// O(n) where n is length of sequence
///
/// # Space Complexity
/// O(1) - only uses a few variables regardless of input size
#[inline]
fn validate_sequence(numbers: &[i32]) -> ValidationResult {
    if numbers.len() < 2 {
        return ValidationResult::Valid;
    }

    // Check first difference to determine if sequence should increase or decrease
    let first_diff = numbers[1] - numbers[0];
    if first_diff == 0 || first_diff.abs() > 3 {
        return ValidationResult::Invalid;
    }

    let should_increase = first_diff > 0;

    // Use windows to compare adjacent pairs
    numbers
        .windows(2)
        .skip(1)
        .all(|pair| {
            let diff = pair[1] - pair[0];
            if should_increase {
                diff > 0 && diff <= 3
            } else {
                diff < 0 && diff >= -3
            }
        })
        .then(|| ValidationResult::Valid)
        .unwrap_or(ValidationResult::Invalid)
}

/// Main function that reads and processes the reactor safety report.
///
/// # Implementation Details
/// - Uses BufReader for efficient file reading
/// - Tracks timing metrics for performance analysis
/// - Processes file line by line to minimize memory usage
///
/// # Error Handling
/// Returns IO errors if file operations fail
///
/// # Time Complexity
/// O(n) where n is file size
///
/// # Space Complexity
/// O(m) where m is maximum line length
fn main() -> io::Result<()> {
    let start = Instant::now();
    let file = File::open("./puzzle_input.txt")?;
    let file_read_time = start.elapsed();

    let reader = BufReader::new(file);
    let mut safe_count = 0;

    let parse_start = Instant::now();
    for line in reader.lines() {
        let line = line?;
        // Parse space-separated numbers into a vector
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        if matches!(validate_sequence(&numbers), ValidationResult::Valid) {
            safe_count += 1;
        }
    }
    let parse_time = parse_start.elapsed();

    println!("Safe reports: {}", safe_count);
    println!("File read time: {:?}", file_read_time);
    println!("Parse and process time: {:?}", parse_time);
    println!("Total time: {:?}", start.elapsed());

    Ok(())
}
