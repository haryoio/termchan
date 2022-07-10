pub fn sjis_to_utf8(before: &[u8]) -> String {
    let (message, ..) = encoding_rs::SHIFT_JIS.decode(before);
    message.to_string()
}
pub fn utf8_to_sjis_byte(before: &str) -> Vec<u8> {
    let (message, ..) = encoding_rs::SHIFT_JIS.encode(before);
    message.to_vec()
}

pub fn utf8_to_sjis_string(s: &str) -> String {
    let (message, ..) = encoding_rs::SHIFT_JIS.encode(s);
    let message = message.to_vec();
    let message = unsafe { &*std::str::from_utf8_unchecked(&message) };

    message.to_string()
}
