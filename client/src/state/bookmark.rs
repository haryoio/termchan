pub struct BookmarkStateItem {
    pub id:   i32,
    pub name: String,
    pub url:  String,
}

impl Default for BookmarkStateItem {
    fn default() -> Self {
        BookmarkStateItem {
            id:   0,
            name: String::new(),
            url:  String::new(),
        }
    }
}
