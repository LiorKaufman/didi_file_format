
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

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any questions or feedback, feel free to reach out.

---

Enjoy using the DIDI file format!
