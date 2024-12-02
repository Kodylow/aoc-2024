//! Day 2: Red-Nosed Reports - Parts 1 & 2
//!
//! This solution validates sequences with two different rule sets:
//! Part 1: Basic validation of strictly increasing/decreasing sequences
//! Part 2: Allows removal of one number to make an invalid sequence valid
//!
//! Performance optimizations:
//! - Inline validation functions
//! - Early returns for known cases
//! - Reuse of vectors to minimize allocations
//! - Efficient sequence checking with windows()
//! - For Part 2: Skip rechecking already valid sequences
//!
//! Time Complexity (per sequence):
//! - Part 1: O(k) where k is sequence length
//! - Part 2: O(k²) due to checking each possible number removal
//!
//! Space Complexity:
//! - O(k) for temporary vector in Part 2 validation

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

/// Represents the validation result of a sequence.
/// Using an enum rather than bool provides more clarity and extensibility.
#[derive(Debug, PartialEq)]
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

/// Validates a sequence allowing one number to be removed to make it valid.
///
/// # Implementation Details
/// - First checks if sequence is already valid
/// - If invalid, tries removing each number once and validates resulting sequence
/// - Uses a reusable vector to minimize allocations
/// - Early returns on first valid combination found
///
/// # Arguments
/// * `numbers` - Original sequence to validate
///
/// # Returns
/// * `ValidationResult::Valid` if sequence is valid or can be made valid by removing one number
/// * `ValidationResult::Invalid` otherwise
///
/// # Time Complexity
/// O(n²) where n is sequence length
#[inline]
fn validate_sequence_with_dampener(numbers: &[i32]) -> ValidationResult {
    // Only called if sequence is already invalid from Part 1, so we can skip that check

    // Try removing each number once
    let mut temp_sequence = Vec::with_capacity(numbers.len() - 1);
    for skip_idx in 0..numbers.len() {
        temp_sequence.clear();
        temp_sequence.extend(
            numbers
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != skip_idx)
                .map(|(_, &n)| n),
        );

        if matches!(validate_sequence(&temp_sequence), ValidationResult::Valid) {
            return ValidationResult::Valid;
        }
    }

    ValidationResult::Invalid
}

/// Performance metrics for each part of the solution
#[derive(Default, Debug)]
struct Metrics {
    file_read_time: std::time::Duration,
    parsing_time: std::time::Duration,
    part1_validation_time: std::time::Duration,
    part2_validation_time: std::time::Duration,
    total_sequences: usize,
    part1_valid_count: usize,
    part2_valid_count: usize,
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
    let mut metrics = Metrics::default();

    // File reading
    let file_start = Instant::now();
    let file = File::open("./puzzle_input.txt")?;
    metrics.file_read_time = file_start.elapsed();

    let reader = BufReader::new(file);
    let mut sequences = Vec::with_capacity(1000);

    // Optimized parsing
    let parse_start = Instant::now();
    for line in reader.lines() {
        let line = line?;
        // Pre-allocate fixed size array since we know sequences are 5 numbers
        let mut numbers = Vec::with_capacity(5);

        // Manual parsing is faster than split+parse
        let mut start = 0;
        let bytes = line.as_bytes();

        for i in 0..bytes.len() {
            if bytes[i] == b' ' || i == bytes.len() - 1 {
                let end = if i == bytes.len() - 1 { i + 1 } else { i };
                // SAFETY: We know input is valid ASCII numbers with spaces
                let num = unsafe {
                    std::str::from_utf8_unchecked(&bytes[start..end])
                        .parse::<i32>()
                        .unwrap_unchecked()
                };
                numbers.push(num);
                start = i + 1;
            }
        }

        sequences.push(numbers);
    }
    metrics.parsing_time = parse_start.elapsed();
    metrics.total_sequences = sequences.len();

    // Combined validation
    let part1_start = Instant::now();
    for sequence in &sequences {
        let validation_result = validate_sequence(sequence);
        if matches!(validation_result, ValidationResult::Valid) {
            metrics.part1_valid_count += 1;
            metrics.part2_valid_count += 1; // Valid sequences are also valid for part 2
        } else if matches!(
            validate_sequence_with_dampener(sequence),
            ValidationResult::Valid
        ) {
            metrics.part2_valid_count += 1;
        }
    }
    metrics.part1_validation_time = part1_start.elapsed();

    let part2_start = Instant::now();
    metrics.part2_validation_time = part2_start.elapsed();

    // Report results
    println!("\nResults:");
    println!("Part 1 - Safe reports: {}", metrics.part1_valid_count);
    println!(
        "Part 2 - Safe reports with dampener: {}",
        metrics.part2_valid_count
    );

    println!("\nPerformance Breakdown:");
    println!("Total sequences processed: {}", metrics.total_sequences);
    println!("File read time: {:?}", metrics.file_read_time);
    println!("Parsing time: {:?}", metrics.parsing_time);
    println!(
        "Part 1 validation time: {:?}",
        metrics.part1_validation_time
    );
    println!(
        "Part 2 validation time: {:?}",
        metrics.part2_validation_time
    );
    println!("Total time: {:?}", start.elapsed());

    println!("\nPer-sequence averages:");
    if metrics.total_sequences > 0 {
        println!(
            "Parsing: {:?}/sequence",
            metrics.parsing_time / metrics.total_sequences as u32
        );
        println!(
            "Part 1: {:?}/sequence",
            metrics.part1_validation_time / metrics.total_sequences as u32
        );
        println!(
            "Part 2: {:?}/sequence",
            metrics.part2_validation_time / metrics.total_sequences as u32
        );
    }

    Ok(())
}
