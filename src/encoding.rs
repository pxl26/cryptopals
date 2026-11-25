pub fn hex_to_base64(s: &str) -> String {
    encode_base64(
        &s.as_bytes()
            .chunks(2)
            .map(|a| {
                let high = (a[0] as char).to_digit(16).unwrap() as u8;
                let low = (a[1] as char).to_digit(16).unwrap() as u8;
                (high << 4) | low
            })
            .collect::<Vec<u8>>(),
    )
}

const BASE64_CHARACTER: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode_base64(input: &[u8]) -> String {
    let mut rs = "".to_string();
    for i in (2..input.len()).step_by(3) {
        rs.push(BASE64_CHARACTER[(input[i - 2] >> 2) as usize] as char);
        rs.push(
            BASE64_CHARACTER[(((input[i - 2] & 0b0000_0011) << 4) | (input[i - 1] >> 4)) as usize]
                as char,
        );
        rs.push(
            BASE64_CHARACTER[((input[i - 1] & 0b0000_1111) << 2 | input[i] >> 6) as usize] as char,
        );
        rs.push(BASE64_CHARACTER[(input[i] & 0b0011_1111) as usize] as char);
    }
    match input.len() % 3 {
        1 => {
            rs.push(BASE64_CHARACTER[(input[input.len() - 1] >> 2) as usize] as char);
            rs.push(
                BASE64_CHARACTER[((input[input.len() - 1] & 0b0000_0011) << 4) as usize] as char,
            );
            rs.push_str("==");
        }
        2 => {
            rs.push(BASE64_CHARACTER[(input[input.len() - 2] >> 2) as usize] as char);
            rs.push(
                BASE64_CHARACTER[((input[input.len() - 2] & 0b0000_0011) << 4
                    | input[input.len() - 1] >> 4) as usize] as char,
            );
            rs.push(
                BASE64_CHARACTER[((input[input.len() - 1] & 0b0000_1111) << 2) as usize] as char,
            );
            rs.push('=');
        }
        _ => {}
    }
    rs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64() {
        assert_eq!(encode_base64(b""), "");
        assert_eq!(encode_base64(b"f"), "Zg==");
        assert_eq!(encode_base64(b"fo"), "Zm8=");
        assert_eq!(encode_base64(b"foo"), "Zm9v");
        assert_eq!(encode_base64(b"foob"), "Zm9vYg==");
        assert_eq!(encode_base64(b"fooba"), "Zm9vYmE=");
        assert_eq!(encode_base64(b"foobar"), "Zm9vYmFy");
    }

    #[test]
    fn test_result() {
        assert_eq!(
            hex_to_base64(
                "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
            ),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        )
    }
}
