pub struct BbsMenuStateItem {
    pub id:  i32,
    pub url: String,
}

impl Default for BbsMenuStateItem {
    fn default() -> Self {
        BbsMenuStateItem {
            id:  0,
            url: String::new(),
        }
    }
}
