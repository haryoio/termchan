use tui::layout::Rect;

use super::mylist::ListState;

#[derive(Debug, Clone)]
pub struct StatefulList<T> {
    pub state:      ListState,
    pub items:      Vec<T>,
    pub loop_items: bool,
}

#[allow(dead_code)]
impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> Self {
        Self {
            state: ListState::default(),
            items,
            loop_items: false,
        }
    }

    pub fn next(&mut self) {
        // aitem no length - 4 ni narumade offset wo ugokasu
        // aitem no length - 4 ni nattara select wo ugokasu

        let selected = self.selected();
        if selected >= self.items.len() - 1 {
            if self.loop_items {
                self.state.offset = 0;
                self.state.select(Some(0));
            } else {
                return;
            }
        } else if self.items.len().saturating_sub(3) <= selected {
            self.state.select(Some(selected + 1));
        } else if selected <= 4 {
            self.state.select(Some(selected + 1));
        } else {
            self.state.select(Some(selected + 1));
            self.state.next();
        }
    }

    pub fn loop_items(&mut self, loop_items: bool) -> &mut Self {
        self.loop_items = loop_items;
        self
    }

    pub fn prev(&mut self) {
        let selected = self.selected();

        if self.selected() <= 0 {
            if self.loop_items {
                self.state.offset = 8;
                self.state.select(Some(self.items.len() - 1));
            }
        } else if self.items.len().saturating_sub(3) <= selected {
            self.state.select(Some(selected - 1));
        } else if selected <= 4 {
            self.state.select(Some(selected - 1));
        } else {
            self.state.select(Some(selected - 1));
            self.state.prev();
        }
    }
    pub fn unselect(&mut self) {
        self.state.select(None);
    }
    pub fn selected(&self) -> usize {
        self.state.selected().unwrap_or(0)
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
        self.state.offset = 0;
    }
}
