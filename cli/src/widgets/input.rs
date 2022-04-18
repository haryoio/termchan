use std::cell::Cell;

pub struct Input {
    pub texts: Vec<String>,
    pub cursor: Cursor,
    multiline: Cell<bool>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            texts: vec!["".to_string()],
            cursor: Cursor::new(),
            multiline: Cell::new(false),
        }
    }

    pub fn text(&self) -> String {
        self.texts.clone().join("\n").to_string()
    }

    pub fn backspace(&mut self) {
        if self.multiline.get() {
            // textsの長さが1以上で、textの長さが1以上であれば、texts[y]をpopする
            // textsの長さが0以下で、textの長さが1以上であれば、texts[y]をpopする
            // textsの長さが1以上で、textの長さが0以下であれば、textsをpopし、yを-1する。
            // textsの長さが0以下で、textの長さも0以下であれば、何もしない。
            if self.texts[self.cursor.y].chars().count() == 0 && self.texts.len() == 1 {
                return;
            } else if self.texts.len() > 1 && self.texts[self.cursor.y].chars().count() <= 0 {
                self.texts.pop();
                self.cursor.up();
                self.cursor.right();
            } else if self.texts.len() > 1 && self.texts[self.cursor.y].chars().count() > 0 {
                self.texts[self.cursor.y].pop();
                self.cursor.back();
            }
            self.set_guard();
        } else {
            if self.cursor.x > 0 {
                self.texts[self.cursor.y].pop();
                self.cursor.back();
            }
        }
    }

    pub fn multiline(self, multiline: bool) -> Self {
        self.multiline.set(multiline);
        self
    }

    pub fn set_guard(&mut self) {
        let x = self.texts[self.cursor.y].chars().count();
        let y = self.texts.len();
        self.cursor.set_guard(x, y);
    }

    pub fn current_line(&self) -> &str {
        &self.texts[self.cursor.y]
    }

    pub fn clear(&mut self) {
        self.texts.clear();
        self.cursor.set_cursor(0, 0);
    }

    pub fn char(&mut self, c: &str) {
        self.texts[self.cursor.y].insert_str(self.cursor.x, c);
        self.cursor
            .set_cursor(self.cursor.x + c.chars().count(), self.cursor.y);
        self.cursor
            .set_guard(self.texts[self.cursor.y].chars().count(), self.texts.len());
    }

    pub fn enter(&mut self) {
        if self.multiline.get() {
            self.texts.insert(self.cursor.y + 1, "".to_string());
            self.cursor.set_cursor(0, self.cursor.y + 1);
        }
    }

    pub fn down(&mut self) {
        if self.multiline.get() {
            if self.cursor.y < self.texts.len() - 1 {
                self.cursor.set_guard(
                    self.texts[self.cursor.y + 1].chars().count(),
                    self.cursor.y + 1,
                );
                self.cursor.down();
            }
        }
    }
    pub fn up(&mut self) {
        if self.multiline.get() {
            if self.cursor.y > 0 {
                self.cursor.set_guard(
                    self.texts[self.cursor.y - 1].chars().count(),
                    self.cursor.y - 1,
                );
                self.cursor.up();
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
    pub width: Option<usize>,
    pub height: Option<usize>,
}

impl Cursor {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            width: None,
            height: None,
        }
    }

    pub fn set_guard(&mut self, width: usize, height: usize) {
        self.width = Some(width);
        self.height = Some(height);
    }

    pub fn set_cursor(&mut self, x: usize, y: usize) {
        match (self.width, self.height) {
            (Some(width), Some(height)) => {
                if x > width {
                    self.x = width;
                } else {
                    self.x = x;
                }
                if y > height {
                    self.y = height;
                } else {
                    self.y = y;
                }
            }
            _ => {
                self.x = x;
                self.y = y;
            }
        }
    }

    pub fn next(&mut self) {
        match self.width {
            Some(width) => {
                if self.x < width {
                    self.x += 1;
                } else {
                    self.x = width;
                }
            }
            _ => {}
        }
    }

    pub fn back(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn up(&mut self) {
        match self.width {
            Some(width) => {
                if self.y > 0 {
                    self.y -= 1;
                } else {
                    self.y = 0;
                }
                if self.x > width {
                    self.x = width;
                }
            }
            _ => {}
        }
    }

    pub fn down(&mut self) {
        match (self.width, self.height) {
            (Some(width), Some(height)) => {
                if self.y < height {
                    self.y += 1;
                } else {
                    self.y = height;
                }
                if self.x > width {
                    self.x = width;
                }
            }
            _ => {}
        }
    }

    pub fn right(&mut self) {
        match self.width {
            Some(width) => self.set_cursor(width, self.y),
            None => {}
        }
    }

    pub fn left(&mut self) {
        self.set_cursor(0, self.y);
    }
}
