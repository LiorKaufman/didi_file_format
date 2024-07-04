extern crate didi_file_format;

use didi_file_format::{write_didi, sniffer};

fn main() -> std::io::Result<()> {
    let data = "aaabbccccxyz";
    let search_string = "xyz";

    // Write data to file
    write_didi("example.didi", data, search_string)?;

    let (stored_search_string, contains_search_string) = sniffer("example.didi")?;
    println!("Stored Search String: {}", stored_search_string);
    println!("Contains Search String: {}", contains_search_string);
    Ok(())
}
