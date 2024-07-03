use std::io::{self, BufReader, BufWriter, Read, Write};
use std::fs::File;

/// Encodes data using Run-Length Encoding
///
/// This function takes a string as input and returns an encoded string where
/// consecutive repeated characters are replaced by a single character followed
/// by the number of repetitions.
pub fn rle_encode(data: &str) -> String {
    let mut encoded = String::new();
    let mut chars = data.chars();
    if let Some(mut prev) = chars.next() {
        let mut count = 1;
        for c in chars {
            if c == prev {
                count += 1;
            } else {
                encoded.push_str(&format!("{}{}", prev, count));
                prev = c;
                count = 1;
            }
        }
        encoded.push_str(&format!("{}{}", prev, count));
    }
    encoded
}

/// Decodes data from Run-Length Encoding
///
/// This function takes an encoded string and returns the original string by
/// expanding each character followed by a count to the repeated characters.
pub fn rle_decode(data: &str) -> String {
    let mut decoded = String::new();
    let mut chars = data.chars();
    while let Some(c) = chars.next() {
        if let Some(count) = chars.next() {
            let count = count.to_digit(10).unwrap();
            decoded.push_str(&c.to_string().repeat(count as usize));
        }
    }
    decoded
}

/// Writes data to a file with metadata about the presence of a search string
///
/// This function encodes the data using Run-Length Encoding, then writes it to
/// a file along with the search string used as metadata.
pub fn write_didi(file_path: &str, data: &str, search_string: &str) -> io::Result<()> {
    let encoded_data = rle_encode(data);

    let mut file = BufWriter::new(File::create(file_path)?);
    file.write_all(&[search_string.len() as u8])?;  // Write length of the search string
    file.write_all(search_string.as_bytes())?;  // Write the search string
    file.write_all(encoded_data.as_bytes())?;  // Write encoded data

    Ok(())
}

/// Reads data from a file, checking metadata for the presence of a search string
///
/// This function reads the metadata and encoded data from a file, decodes the
/// data, and returns it along with the search string stored in the metadata.
pub fn read_didi(file_path: &str) -> io::Result<(String, String)> {
    let mut file = BufReader::new(File::open(file_path)?);

    let mut length = [0; 1];
    file.read_exact(&mut length)?;  // Read length of the search string
    let length = length[0] as usize;

    let mut search_string = vec![0; length];
    file.read_exact(&mut search_string)?;  // Read the search string
    let search_string = String::from_utf8(search_string).unwrap();

    let mut encoded_data = String::new();
    file.read_to_string(&mut encoded_data)?;  // Read encoded data

    let decoded_data = rle_decode(&encoded_data);

    Ok((decoded_data, search_string))
}


/// Sniffs the file to determine what string is stored in the metadata
///
/// This function reads the metadata from the file to determine what string was
/// stored as the search string.
pub fn sniffer(file_path: &str) -> io::Result<String> {
    let mut file = BufReader::new(File::open(file_path)?);
    let mut length_buf = [0; 1];
    file.read_exact(&mut length_buf)?;  // Read exactly 1 byte to determine the length of the search string
    let search_string_length = length_buf[0] as usize;

    let mut search_string_buf = vec![0; search_string_length];
    file.read_exact(&mut search_string_buf)?;  // Read the search string bytes

    let search_string = String::from_utf8(search_string_buf)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(search_string)  
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle_encode() {
        let data = "aaabbcccc";
        assert_eq!(rle_encode(data), "a3b2c4");
    }

    #[test]
    fn test_rle_decode() {
        let data = "a3b2c4";
        assert_eq!(rle_decode(data), "aaabbcccc");
    }

    #[test]
    fn test_write_and_read_didi() {
        let data = "aaabbcccc";
        let search_string = "a3";
        write_didi("test.didi", data, search_string).unwrap();
        let (decoded_data, stored_search_string) = read_didi("test.didi").unwrap();
        assert_eq!(decoded_data, "aaabbcccc");
        assert_eq!(stored_search_string, search_string);
    }

    #[test]
    fn test_sniffer() {
        let data = "aaabbcccc";
        let search_string = "abccc";
        write_didi("test.didi", data, search_string).unwrap();
        let stored_search_string = sniffer("test.didi").unwrap();
        assert_eq!(stored_search_string, search_string);
    }
}
