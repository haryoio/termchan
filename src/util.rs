pub fn sjis_to_utf8(sjis: &[u8]) -> String {
    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(sjis);
    res.to_owned().to_string()
}
pub fn is_gzip(buf: &[u8]) -> bool {
    buf.len() >= 2 && buf[0] == 0x1f && buf[1] == 0x8b
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_sjis_to_utf8() {
        let s = fs::read("src/shiftjis.txt").unwrap();
        let utf8 = sjis_to_utf8(&s);
        assert_eq!(utf8, "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわをん\n");
    }
}
