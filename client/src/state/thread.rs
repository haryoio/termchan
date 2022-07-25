pub struct ThreadStateItem {
    pub id:         i32,
    pub url:        String,
    pub name:       String,
    pub count:      i32,
    pub ikioi:      f64,
    pub updated_at: String,
}

impl Default for ThreadStateItem {
    fn default() -> Self {
        ThreadStateItem {
            id:         0,
            url:        String::new(),
            name:       String::new(),
            count:      0,
            ikioi:      0.0,
            updated_at: String::new(),
        }
    }
}
