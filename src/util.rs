pub fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut rs = vec![];
    for i in 0..a.len() {
        rs.push(a[i] ^ b[i]);
    }
    rs
}

#[cfg(test)]
mod test {
    use crate::{encoding::*, util::xor_bytes};

    #[test]
    fn test_xor_hex() {
        let a = hex_str_to_bytes("1c0111001f010100061a024b53535009181c").unwrap_or_default();
        let b = hex_str_to_bytes("686974207468652062756c6c277320657965").unwrap_or_default();
        let xor_bytes = xor_bytes(&a, &b);
        let expected = hex_str_to_bytes("746865206b696420646f6e277420706c6179").unwrap_or_default();
        assert_eq!(xor_bytes, expected)
    }
}
