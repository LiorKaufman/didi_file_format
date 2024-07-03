
# DIDI File Format

This is a *learning* project that demonstrates how to create a custom file format using Rust. The DIDI file format includes capabilities for encoding data to save on storage and metadata to indicate if a specific string is present in the file.

## Features

- **Encoding Data**:  Stores data by converting it into a specific format using Run-Length Encoding (RLE).
- **Metadata**: Indicates whether a specific string is present anywhere in the file.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
didi_file_format = "0.1.0"
```

## Usage

Here’s a basic example of how to use the DIDI file format in your Rust project.

### Writing to a DIDI File

```rust
use didi_file_format::write_didi;

fn main() -> std::io::Result<()> {
    let data = "aaabbcccc";
    let search_string = "a3";

    write_didi("example.didi", data, search_string)?;
    Ok(())
}
```

### Reading from a DIDI File

```rust
use didi_file_format::read_didi;

fn main() -> std::io::Result<()> {
    let (decoded_data, stored_search_string) = read_didi("example.didi")?;
    println!("Decoded Data: {}", decoded_data);
    println!("Stored Search String: {}", stored_search_string);
    Ok(())
}
```

### Using the Sniffer Function

```rust
use didi_file_format::sniffer;

fn main() -> std::io::Result<()> {
    let stored_search_string = sniffer("example.didi")?;
    println!("Sniffer found the stored search string: {}", stored_search_string);
    Ok(())
}
```

### Generating a Large File and Comparing Performance

To generate a large file and compare the performance of finding a string using the DIDI file format vs. manually searching the file, use the `compare_performance` example.

```rust
use std::fs;
use std::io;
use std::time::Instant;
use didi_file_format::{write_didi, sniffer};

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

    // Save to a text file
    fs::write("large_example.txt", &generated_string)?;

    Ok(())
}

fn manual_search(file_path: &str, search_string: &str) -> bool {
    if let Ok(content) = fs::read_to_string(file_path) {
        return content.contains(search_string);
    }
    false
}

fn main() -> io::Result<()> {
    let search_string = "specific_string";
    let file_path = "large_example.txt";

    if !file_exists(file_path)? {
        generate_large_file(100000)?;
    } else {
        println!("File already exists: {}", file_path);
    }

    let data = fs::read_to_string(file_path)?;  // Read the generated file as a string

    write_didi("large_example.didi", &data, search_string)?;

    let start_sniffer = Instant::now();
    let found_sniffer = sniffer("large_example.didi")?;
    let duration_sniffer = start_sniffer.elapsed();

    let start_manual = Instant::now();
    let found_manual = manual_search("large_example.didi", search_string);
    let duration_manual = start_manual.elapsed();

    println!("Sniffer found: {}", found_sniffer);
    println!("Sniffer duration: {:?}", duration_sniffer);
    println!("Manual search found: {}", found_manual);
    println!("Manual search duration: {:?}", duration_manual);

    Ok(())
}

fn file_exists(file_path: &str) -> io::Result<bool> {
    match fs::metadata(file_path) {
        Ok(_) => Ok(true),
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
    }
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any questions or feedback, feel free to reach out.

---

Enjoy using the DIDI file format!
