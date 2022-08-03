#[derive(Debug, Clone)]
pub struct LayoutState {
    pub visible_sidepane: bool,
    pub visible_sideopt:  bool,
    pub visible_mainopt:  bool,
    pub focus_pane:       Pane,
}
#[derive(PartialEq, Clone, Debug)]
pub enum Pane {
    Side,
    SideOpt,
    Main,
    MainOpt,
}

impl LayoutState {
    pub fn new() -> LayoutState {
        LayoutState {
            visible_sidepane: true,
            visible_sideopt:  true,
            visible_mainopt:  false,
            focus_pane:       Pane::Side,
        }
    }
    pub fn toggle_visible_sidepane(&mut self) {
        if self.visible_sidepane {
            self.focus_pane = Pane::Main;
        }
        self.visible_sidepane = !self.visible_sidepane;
    }
    pub fn next(&mut self) {
        match self.focus_pane {
            Pane::Side => {
                if self.visible_sideopt {
                    self.focus_pane = Pane::SideOpt;
                } else {
                    self.focus_pane = Pane::Main;
                }
            }
            Pane::SideOpt => self.focus_pane = Pane::Main,
            Pane::Main => {
                if self.visible_mainopt {
                    self.focus_pane = Pane::MainOpt
                } else if self.visible_sidepane {
                    self.focus_pane = Pane::Side
                }
            }
            Pane::MainOpt => self.focus_pane = Pane::Side,
        }
    }
    pub fn prev(&mut self) {
        match self.focus_pane {
            Pane::Side => {
                if self.visible_mainopt {
                    self.focus_pane = Pane::MainOpt;
                } else {
                    self.focus_pane = Pane::Main;
                }
            }
            Pane::SideOpt => self.focus_pane = Pane::Side,
            Pane::Main => {
                if self.visible_mainopt {
                    self.focus_pane = Pane::MainOpt
                } else if self.visible_sideopt {
                    self.focus_pane = Pane::SideOpt
                } else if self.visible_sidepane {
                    self.focus_pane = Pane::Side
                }
            }
            Pane::MainOpt => self.focus_pane = Pane::Main,
        }
    }
    pub fn toggle_focus_pane(&mut self) {
        self.focus_pane = match self.focus_pane {
            Pane::Side => {
                if self.visible_sideopt {
                    Pane::SideOpt
                } else {
                    Pane::Main
                }
            }
            Pane::SideOpt => Pane::Side,
            Pane::Main => {
                if self.visible_sidepane {
                    Pane::Side
                } else {
                    Pane::Main
                }
            }
            Pane::MainOpt => Pane::Main,
        };
    }
}
