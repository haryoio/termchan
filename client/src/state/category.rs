pub struct CategoryStateItem {
    pub id:   i32,
    pub name: String,
    pub url: String,
}

impl Default for CategoryStateItem {
    fn default() -> Self {
        CategoryStateItem {
            id:   0,
            name: String::new(),
            url: String::new(),
        }
    }
}
