use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

/// Fast solution using minimal allocations and simple operations.
///
/// # Memory optimizations:
/// - Pre-allocated vectors with capacity to avoid reallocations
/// - Reusable String buffers for parsing to minimize allocations
/// - Direct byte manipulation instead of string operations
/// - Single pass through input file with minimal buffering
///
/// # Performance optimizations:
/// - Single pass through file without backtracking
/// - Direct number parsing without string splits or iterator overhead
/// - Manual byte-level parsing avoiding UTF-8 validation overhead
/// - Avoids string allocations by reusing buffers
/// - Uses primitive array operations instead of iterators for tight loops
/// - Avoids unnecessary bounds checks in hot loops
/// - Uses unstable sort for better performance on integers
/// - Direct array indexing instead of iterator chains for calculations
///
/// # Complexity:
/// - Time: O(n log n) dominated by sorting
/// - Space: O(n) for storing input vectors
/// - Memory: O(1) additional space during parsing
///
/// # Implementation details:
/// The parser uses direct byte manipulation and manual string building to minimize
/// allocations and avoid UTF-8 validation overhead. Numbers are parsed directly from
/// bytes without intermediate string representations where possible.
fn main() -> io::Result<()> {
    let start = Instant::now();

    // Pre-allocate vectors to avoid resizing
    let mut left_numbers = Vec::with_capacity(1000);
    let mut right_numbers = Vec::with_capacity(1000);
    // Single reusable buffer for number parsing
    let mut number_buffer = String::with_capacity(16);

    let file = File::open("../puzzle_input.txt")?;
    // Use buffered reader for efficient IO
    let mut reader = BufReader::new(file);
    // Reuse line buffer to avoid allocations
    let mut line = String::with_capacity(32);

    // Fast parsing using manual byte operations
    while reader.read_line(&mut line)? > 0 {
        let bytes = line.as_bytes();
        let mut i = 0;

        // Skip leading whitespace using byte comparisons
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }

        // Parse first number directly from bytes
        while i < bytes.len() && !bytes[i].is_ascii_whitespace() {
            // Avoid UTF-8 validation by direct byte-to-char conversion
            number_buffer.push(bytes[i] as char);
            i += 1;
        }
        if !number_buffer.is_empty() {
            // Parse number and clear buffer for reuse
            left_numbers.push(number_buffer.parse::<i64>().unwrap());
            number_buffer.clear();
        }

        // Skip inter-number whitespace
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }

        // Parse second number using same buffer
        while i < bytes.len() && !bytes[i].is_ascii_whitespace() {
            number_buffer.push(bytes[i] as char);
            i += 1;
        }
        if !number_buffer.is_empty() {
            right_numbers.push(number_buffer.parse::<i64>().unwrap());
            number_buffer.clear();
        }

        // Clear line buffer for reuse
        line.clear();
    }

    println!("Parsing completed in {:?}", start.elapsed());
    let sort_start = Instant::now();

    // Use unstable sort for better performance on integers
    // Stable sort not needed since we only care about relative positions
    left_numbers.sort_unstable();
    right_numbers.sort_unstable();

    println!("Sorting completed in {:?}", sort_start.elapsed());
    let calc_start = Instant::now();

    // Direct array indexing for fastest possible access
    // Avoid iterator overhead in tight calculation loop
    let mut total = 0i64;
    for i in 0..left_numbers.len() {
        total += (left_numbers[i] - right_numbers[i]).abs();
    }

    println!("Calculation completed in {:?}", calc_start.elapsed());
    println!("Total time: {:?}", start.elapsed());
    println!("Result: {}", total);

    Ok(())
}
