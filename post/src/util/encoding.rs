trait Encoding_sjis {
    fn sjis_to_utf8(&self) -> anyhow::Result<String>;
    fn utf8_to_sjis(&self) -> anyhow::Result<String>;
}
