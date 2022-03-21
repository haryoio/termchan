pub fn sjis_to_utf8(sjis: &[u8]) -> String {
    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(sjis);
    res.to_owned().to_string()
}
