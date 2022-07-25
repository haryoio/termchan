#[derive(Debug, Clone)]
pub struct LayoutState {
    pub visible_sidepane: bool,
    pub focus_pane:       Pane,
}
#[derive(PartialEq, Clone, Debug)]
pub enum Pane {
    Side,
    Main,
}

impl LayoutState {
    pub fn new() -> LayoutState {
        LayoutState {
            visible_sidepane: true,
            focus_pane:       Pane::Side,
        }
    }
    pub fn toggle_visible_sidepane(&mut self) {
        if self.visible_sidepane {
            self.focus_pane = Pane::Main;
        }
        self.visible_sidepane = !self.visible_sidepane;
    }
    pub fn toggle_focus_pane(&mut self) {
        self.focus_pane = match self.focus_pane {
            Pane::Side => Pane::Main,
            Pane::Main => {
                if self.visible_sidepane {
                    Pane::Side
                } else {
                    Pane::Main
                }
            }
        };
    }
}
