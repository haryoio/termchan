/// @Author: haryoiro  /// @Date: 2022-03-25 22:31:06  ///@Last Modified by:   haryoiro  /// @Last Modified time: 2022-03-25 22:31:06  ///#[cfg(test)]
mod test {
    use std::fs;

    use anyhow::{Context, Ok};
    use termch::utils::encoder::{sjis_to_form_value, sjis_to_utf8};

    #[test]
    fn test_sjis_to_utf8() -> anyhow::Result<()> {
        let s = fs::read("tests/sjis_test1.txt")?;
        let utf8 = sjis_to_utf8(&s)?;
        assert_eq!(utf8, "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわをん\n");
        Ok(())
    }
    #[test]
    fn test_url_encode_from_kanji_to_sjis() -> anyhow::Result<()> {
        let s = "書き込む";

        let percented = sjis_to_form_value(&s)?;

        let ok_string = "%8F%91%82%AB%8D%9E%82%DE";

        assert_eq!(percented, ok_string);
        Ok(())
    }
}
