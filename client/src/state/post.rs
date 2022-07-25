pub struct ThreadPostStateItem {
    pub id:      i32,
    pub index:   i32,
    pub hash:    String,
    pub message: String,
    pub date:    Option<String>,
    pub email:   Option<String>,
}

impl Default for ThreadPostStateItem {
    fn default() -> Self {
        ThreadPostStateItem {
            id:      0,
            index:   0,
            hash:    String::new(),
            message: String::new(),
            date:    None,
            email:   None,
        }
    }
}
