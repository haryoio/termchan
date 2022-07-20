
pub trait URL {
    fn new(url: &str) -> Self;
    fn origin(&self) -> String;
    fn host(&self) -> String;
    fn referer(&self) -> String;
}
