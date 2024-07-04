extern crate didi_file_format;

use didi_file_format::{sniffer, write_didi};
use std::fs;
use std::io;
use std::time::Instant;

fn generate_large_file(repetitions: usize) -> io::Result<()> {
    let letters = vec!["a", "b", "c"];
    let mut generated_string = String::new();

    for i in 0..repetitions {
        let letter = letters[i % letters.len()];
        for _ in 0..10 {
            generated_string.push_str(letter);
        }
    }

    // Append custom string at the end
    generated_string.push_str("custom_formats_are_interesting");

    fs::write("large_example.txt", &generated_string)?;

    Ok(())
}
fn manual_search(file_path: &str, search_string: &str) -> bool {
    if let Ok(content) = fs::read_to_string(file_path) {
        return content.contains(search_string);
    }
    false
}
fn file_exists(file_path: &str) -> io::Result<bool> {
    match fs::metadata(file_path) {
        Ok(_) => Ok(true),
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
    }
}
fn main() -> io::Result<()> {
    let search_string = "custom_formats_are_interesting";
    let file_path = "large_example.txt";

    // Check if the file exists
    if !file_exists(file_path)? {
        // File does not exist, run the desired function and generate a large file
        generate_large_file(10000000)?;
    } else {
        println!("File already exists: {}", file_path);
    }

    let data = fs::read_to_string(file_path)?;  // Read the generated file as a string

    // Write data to DIDI file
    write_didi("large_example.didi", &data, search_string)?;

    // Measure performance of sniffer
    let start_sniffer = Instant::now();

    let (stored_search_string, found_sniffer): (String, bool) = sniffer("large_example.didi")?;
    let duration_sniffer = start_sniffer.elapsed();

    // Measure performance of manual search
    let start_manual = Instant::now();
    let found_manual = manual_search("large_example.didi", search_string);
    let duration_manual = start_manual.elapsed();

    // Print results
    println!("Sniffer found the search: {}", stored_search_string);
    println!("Sniffer duration: {:?}", duration_sniffer);
    println!("Manual search found the string: {} {}", search_string,found_manual);
    println!("Manual search duration: {:?}", duration_manual);

    Ok(())
}