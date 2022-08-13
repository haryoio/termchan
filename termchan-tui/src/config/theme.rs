use serde::{Deserialize, Serialize};
use tui::{style::Color, widgets::BorderType as TuiBorderType};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BorderType {
    Plain,
    Rounded,
    Double,
    Thick,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ThumbnailSize {
    Small,
    Medium,
    Large,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Theme {
    pub status_bar:   Color,
    pub error_border: Color,
    pub error_text:   Color,
    pub hint:         Color,
    pub hovered:      Color,
    pub active:       Color,
    pub inactive:     Color,
    pub selected:     Color,
    pub text:         Color,

    // text
    pub active_unselected_text:   Color,
    pub active_selected_text:     Color,
    pub inactive_unselected_text: Color,
    pub inactive_selected_text:   Color,

    pub reset: Color,

    pub active_item_symbol:   String,
    pub inactive_item_symbol: String,
    pub unread_symbol:        String,
    pub read_symbol:          String,
    pub posted_symbol:        String,

    pub ikioi_low:         Color,
    pub ikioi_middle:      Color,
    pub ikioi_middle_high: Color,
    pub ikioi_high:        Color,

    /// border_type: Plain | Rounded | Double | Thick
    /// default: Plain
    /// 参照: https://docs.rs/tui-style/0.1.0/tui_style/enum.BorderStyle.html
    border_type: BorderType,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            status_bar:   Color::LightCyan,
            error_border: Color::Red,
            error_text:   Color::White,
            hint:         Color::LightCyan,
            hovered:      Color::LightCyan,
            active:       Color::LightCyan,
            inactive:     Color::Gray,
            selected:     Color::LightCyan,
            text:         Color::White,

            active_unselected_text:   Color::White,
            active_selected_text:     Color::LightCyan,
            inactive_unselected_text: Color::Gray,
            inactive_selected_text:   Color::Cyan,

            reset: Color::Reset,

            ikioi_low:         Color::LightCyan,
            ikioi_middle:      Color::LightCyan,
            ikioi_middle_high: Color::LightCyan,
            ikioi_high:        Color::LightCyan,

            active_item_symbol:   ">".to_string(),
            inactive_item_symbol: " ".to_string(),
            unread_symbol:        "●".to_string(),
            read_symbol:          "○".to_string(),
            posted_symbol:        "✎".to_string(),

            border_type: BorderType::Plain,
        }
    }
}

impl Theme {
    pub fn border_type(&self) -> TuiBorderType {
        match self.border_type {
            BorderType::Plain => TuiBorderType::Plain,
            BorderType::Rounded => TuiBorderType::Rounded,
            BorderType::Double => TuiBorderType::Double,
            BorderType::Thick => TuiBorderType::Thick,
        }
    }
}
