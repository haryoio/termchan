#[derive(Debug, Clone)]
pub struct LayoutState {
    pub visible_sidepane: bool,
    pub visible_sideopt:  bool,
    pub visible_mainopt:  bool,
    pub visible_popup:    bool,
    pub focus_pane:       Pane,
}
#[derive(PartialEq, Clone, Debug)]
pub enum Pane {
    Side,
    SideOpt,
    Main,
    MainOpt,
    Popup,
}

impl LayoutState {
    pub fn new() -> LayoutState {
        LayoutState {
            visible_sidepane: true,
            visible_sideopt:  true,
            visible_mainopt:  false,
            visible_popup:    false,
            focus_pane:       Pane::Side,
        }
    }
    pub fn toggle_visible_sidepane(&mut self) {
        if self.visible_sidepane {
            self.focus_pane = Pane::Main;
        }
        self.visible_sidepane = !self.visible_sidepane;
    }
}
