use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

pub fn sjis_to_utf8(bytes: &[u8]) -> anyhow::Result<String> {
    let (res, ..) = encoding_rs::SHIFT_JIS.decode(bytes);
    Ok(res.to_string())
}

pub fn sjis_to_form_value(s: &str) -> anyhow::Result<String> {
    let (res, encode, error) = encoding_rs::SHIFT_JIS.encode(&s);
    if error {
        return Err(anyhow::anyhow!(format!(
            "failed to encode: \ntype: {}\ntarget: {}\n",
            encode.name(),
            s,
        )));
    };

    let encoded = percent_encode(&res.to_vec(), NON_ALPHANUMERIC).to_string();
    Ok(encoded)
}

pub fn is_gzip(buf: &[u8]) -> bool { buf.len() >= 2 && buf[0] == 0x1f && buf[1] == 0x8b }
