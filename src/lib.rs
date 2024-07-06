use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

const MAGIC_NUMBER: &[u8] = b"DIDI";

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
/// This function checks if the search string is present in the original data,
/// encodes the data using Run-Length Encoding, then writes it to a file along
/// with the search string and a true/false statement as metadata.
pub fn write_didi(file_path: &str, data: &str, search_string: &str) -> io::Result<()> {
    let contains_search_string = data.contains(search_string);
    let encoded_data = rle_encode(data);

    let mut file = BufWriter::new(File::create(file_path)?);
    file.write_all(MAGIC_NUMBER)?; // Write magic number
    file.write_all(&[search_string.len() as u8])?; // Write length of the search string
    file.write_all(search_string.as_bytes())?; // Write the search string
    file.write_all(&[contains_search_string as u8])?; // Write true/false statement
    file.write_all(encoded_data.as_bytes())?; // Write encoded data

    Ok(())
}
/// Reads data from a file, checking metadata for the presence of a search string
///
/// This function reads the metadata and encoded data from a file, decodes the
/// data, and returns it along with the search string and a boolean indicating
/// if the search string was found in the original data.
pub fn read_didi(file_path: &str) -> io::Result<(String, String, bool)> {
    let mut file = BufReader::new(File::open(file_path)?);

    let mut magic_buf = [0; 4];
    file.read_exact(&mut magic_buf)?; // Read magic number
    if magic_buf != MAGIC_NUMBER {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid magic number",
        ));
    }

    let mut length_buf = [0; 1];
    file.read_exact(&mut length_buf)?; // Read length of the search string
    let search_string_length = length_buf[0] as usize;

    let mut search_string_buf = vec![0; search_string_length];
    file.read_exact(&mut search_string_buf)?; // Read the search string
    let search_string = String::from_utf8(search_string_buf)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let mut contains_buf = [0; 1];
    file.read_exact(&mut contains_buf)?; // Read true/false statement
    let contains_search_string = contains_buf[0] != 0;

    let mut encoded_data = String::new();
    file.read_to_string(&mut encoded_data)?; // Read encoded data

    let decoded_data = rle_decode(&encoded_data);

    Ok((decoded_data, search_string, contains_search_string))
}

/// Sniffs the file to determine what string is stored in the metadata
///
/// This function reads the metadata from the file to determine what string was
/// stored as the search string and whether the search string was present in the
/// original data.
pub fn sniffer(file_path: &str) -> io::Result<(String, bool)> {
    let mut file = BufReader::new(File::open(file_path)?);

    let mut magic_buf = [0; 4];
    file.read_exact(&mut magic_buf)?; // Read magic number
    if magic_buf != MAGIC_NUMBER {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid magic number",
        ));
    }

    let mut length_buf = [0; 1];
    file.read_exact(&mut length_buf)?; // Read length of the search string
    let search_string_length = length_buf[0] as usize;

    let mut search_string_buf = vec![0; search_string_length];
    file.read_exact(&mut search_string_buf)?; // Read the search string bytes

    let search_string = String::from_utf8(search_string_buf)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let mut contains_buf = [0; 1];
    file.read_exact(&mut contains_buf)?; // Read true/false statement
    let contains_search_string = contains_buf[0] != 0;

    Ok((search_string, contains_search_string))

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
        let data = "aaabbccccxyz";
        let search_string = "xyz";
        write_didi("test.didi", data, search_string).unwrap();
        let (decoded_data, stored_search_string, found_search_string) =
            read_didi("test.didi").unwrap();
        assert_eq!(decoded_data, "aaabbccccxyz");
        assert_eq!(stored_search_string, search_string);
        assert_eq!(found_search_string, true);
    }

    #[test]
    fn test_sniffer() {
        let data = "aaabbcccc";
        let search_string = "abccc";
        write_didi("test_sniffer.didi", data, search_string).unwrap();
        let (stored_search_string, contains_search_string) = sniffer("test_sniffer.didi").unwrap();
        assert_eq!(stored_search_string, search_string);
        assert_eq!(contains_search_string, data.contains(search_string));
    }
}
