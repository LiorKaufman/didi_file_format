
# LK File Format

This is a learning project that demonstrates how to create a custom file format using Rust. The LK file format includes capabilities for encoding data to save on storage and metadata to indicate if a specific string is present in the file.

## Features

- **Encoding Data**: Efficiently stores data by converting it into a specific format.
- **Metadata**: Indicates whether a specific string is present anywhere in the file.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
lk_file_format = "0.1.0"
```

## Usage

Here’s a basic example of how to use the LK file format in your Rust project.

### Writing to an LK File

```rust
use std::collections::HashMap;
use lk_file_format::write_lk;

fn main() -> std::io::Result<()> {
    let mut data = HashMap::new();
    data.insert("name".to_string(), "John Doe".to_string());
    data.insert("age".to_string(), "30".to_string());
    data.insert("city".to_string(), "New York".to_string());

    write_lk("example.lk", &data, "John")?;
    Ok(())
}
```

### Reading from an LK File

```rust
use lk_file_format::read_lk;

fn main() -> std::io::Result<()> {
    let (data, contains_string) = read_lk("example.lk")?;
    println!("Data: {:?}", data);
    println!("Contains 'John': {}", contains_string);
    Ok(())
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any questions or feedback, feel free to reach out.

---

Enjoy using the LK file format!
