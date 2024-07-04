extern crate didi_file_format;

use didi_file_format::{write_didi, read_didi};

fn main() -> std::io::Result<()> {
    let data = "aaabbcccc";
    let search_string = "a3";

    // Write data to file
    write_didi("example.didi", data, search_string)?;

    // Read data from file
    let (decoded_data, stored_search_string, found_string) = read_didi("example.didi")?;
    println!("Decoded Data: {}", decoded_data);
    println!("Stored Search String: {}", stored_search_string);
    println!("Do Search String: {} exists in the file: {}", stored_search_string, found_string);


    Ok(())
}
