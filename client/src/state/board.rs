pub struct BoardStateItem {
    pub id:   i32,
    pub url:  String,
    pub name: String,
}

impl Default for BoardStateItem {
    fn default() -> Self {
        BoardStateItem {
            id:   0,
            url:  String::new(),
            name: String::new(),
        }
    }
}
