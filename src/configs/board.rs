use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct BoardConfig {
    pub save_liked_board: bool,
    pub liked_path:       String,
}

impl Default for BoardConfig {
    fn default() -> Self {
        BoardConfig {
            save_liked_board: false,
            liked_path:       "".to_string(),
        }
    }
}
