use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the command line argument is provided
    if args.len() != 2 {
        println!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    // Extract the filename from the command line argument
    let file_name = &args[1];

    // Attempt to open the file
    let f = File::open(&file_name)?;
    // Create a buffered reader for reading lines of text
    let rdr = io::BufReader::new(f);

    // Initialize variables to keep track of the current block and its sum
    let mut current_block = Vec::new();
    let mut current_sum = 0;
    let mut max_sum = std::i32::MIN;

    // Iterate over each line in the file
    for line in rdr.lines() {
        let line = line?;
        // Check if the line is empty, indicating a new block
        if line.is_empty() {
            // Process the current block and its sum
            process_block(&current_block, current_sum);
            // Clear the current block and reset the sum for the next set of lines
            current_block.clear();
            current_sum = 0;
        } else {
            // Parse the number on this line into an integer
            let line_as_number = line.parse::<i32>().unwrap();

            // assert!(line_as_number >= -5 && line_as_number <= 9);
            current_sum += line_as_number;
            // Add the line to the current block
            current_block.push(line_as_number);
        }
        max_sum = max_sum.max(current_sum);
    }
    if !current_block.is_empty() {
        process_block(&current_block, current_sum);
    }
    println!("Max sum is {}", max_sum);
    Ok(())
}

// Function that processes a single block of numbers
fn process_block(block: &Vec<i32>, sum: i32) {
    // Print the sum for the current block
    println!("Sum: {}, qty: {}", sum, block.len());
    // Add any additional processing for the block if needed
}
