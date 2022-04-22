use std::{cell::Cell, vec};

use crossterm::cursor;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

pub enum InputAction {
    CursorRight,
    CursorLeft,
    CursorUp,
    CursorDown,
    CursorUpMulti(usize),
    CursorDownMulti(usize),
    Return,
    Char(char),
    Clear,
}

#[derive(Debug)]
pub struct Input {
    pub texts: Vec<Vec<char>>,
    pub cursor: Cursor,
    multiline: Cell<bool>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            texts: vec![vec![]],
            cursor: Cursor::new(),
            multiline: Cell::new(false),
        }
    }

    pub fn text(&self) -> String {
        let mut text = String::new();
        for line in &self.texts {
            for c in line {
                text.push(*c);
            }
            text.push('\n');
        }
        text
    }

    pub fn count_current_line(&self) -> usize {
        let mut count = 0;
        for c in &self.texts[self.cursor.y] {
            count += UnicodeWidthChar::width(*c).unwrap();
        }
        count
    }
    pub fn count_prev_line(&self) -> usize {
        let mut count = 0;
        for c in &self.texts[self.cursor.y - 1] {
            count += UnicodeWidthChar::width(*c).unwrap();
        }
        count
    }
    pub fn prevent_char_size(&self) -> usize {
        if self.cursor.x == 0 {
            0
        } else {
            UnicodeWidthChar::width(self.texts[self.cursor.y][self.cursor.x - 1]).unwrap()
        }
    }
    pub fn current_char_size(&self) -> usize {
        if self.texts[self.cursor.y].len() - 1 <= self.cursor.x {
            0
        } else {
            UnicodeWidthChar::width(self.texts[self.cursor.y][self.cursor.x]).unwrap_or(0)
        }
    }

    pub fn multiline(self, multiline: bool) -> Self {
        self.multiline.set(multiline);
        self
    }

    pub fn set_guard(&mut self) {
        let x = self.texts[self.cursor.y].len() * 2;
        let y = self.texts.len();
        self.cursor.set_guard(x, y);
    }

    pub fn current_line(&self) -> String {
        let line = &self.texts[self.cursor.y];
        let mut line = line.iter().map(|c| *c).collect::<String>();
        line.truncate(self.cursor.x);
        line
    }

    pub fn backspace(&mut self) {
        if self.multiline.get() {
            // textsの長さが1以上で、textの長さが1以上であれば、texts[y]をpopする
            // textsの長さが0以下で、textの長さが1以上であれば、texts[y]をpopする
            // textsの長さが1以上で、textの長さが0以下であれば、textsをpopし、yを-1する。
            // textsの長さが0以下で、textの長さも0以下であれば、何もしない。
            if self.cursor.y <= 0 {
                if self.cursor.x <= 0 {
                    return;
                } else if self.cursor.x > 0 {
                    self.texts[self.cursor.y].remove(self.cursor.x - 1);
                    self.cursor.x -= 1;
                }
            } else {
                if self.cursor.x <= 0 {
                    self.texts.remove(self.cursor.y);
                    self.cursor.y -= 1;
                    self.set_guard();
                    self.cursor.x = self.texts[self.cursor.y].len();
                } else if self.cursor.x > 0 {
                    self.texts[self.cursor.y].remove(self.cursor.x - 1);
                    self.cursor.x -= 1;
                }
            }
            self.set_guard();
        } else {
            if self.cursor.x > 0 {
                self.texts[self.cursor.y].remove(self.cursor.x);

                self.cursor.back();
            }
        }

        // println!("\n{:?}", self.cursor);
    }

    pub fn clear(&mut self) {
        self.texts.clear();
        self.cursor.set_cursor(0, 0);
    }

    pub fn char(&mut self, c: char) {
        self.texts[self.cursor.y].insert(self.cursor.x, c);
        self.set_guard();
        self.cursor.set_cursor(self.cursor.x + 1, self.cursor.y);
        self.set_guard();
        // println!("\n{:?}", self.cursor);
    }

    pub fn enter(&mut self) {
        if self.multiline.get() {
            self.texts.insert(self.cursor.y + 1, vec![]);
            self.cursor.set_cursor(0, self.cursor.y + 1);
        }
    }

    pub fn down(&mut self) {
        if self.multiline.get() {
            if self.cursor.y < self.texts.len() - 1 {
                self.cursor
                    .set_guard(self.texts[self.cursor.y + 1].len(), self.cursor.y + 1);
                self.cursor.down();
            }
        }
    }

    pub fn up(&mut self) {
        if self.multiline.get() {
            if self.cursor.y > 0 {
                self.cursor
                    .set_guard(self.texts[self.cursor.y - 1].len(), self.cursor.y - 1);
                self.cursor.up();
            }
        }
    }

    pub fn next(&mut self) {
        let size = self.current_char_size();
        self.cursor.next();
        self.set_guard();
    }

    pub fn back(&mut self) {
        let size = self.prevent_char_size();
        self.cursor.back();
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
