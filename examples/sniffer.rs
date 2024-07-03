extern crate didi_file_format;

use didi_file_format::{write_didi, sniffer};

fn main() -> std::io::Result<()> {
    let data = "aaabbcccc";
    let search_string = "jjjjj";

    // Write data to file
    write_didi("example.didi", data, search_string)?;

    // Sniffer to get the stored search string
    let stored_search_string = sniffer("example.didi")?;
    println!("Sniffer found the stored search string: {}", stored_search_string);

    Ok(())
}
