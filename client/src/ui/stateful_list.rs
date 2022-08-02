use tui::layout::Rect;

use super::mylist::ListState;

#[derive(Debug, Clone)]
pub struct StatefulList<T> {
    pub state:  ListState,
    pub items:  Vec<T>,
    pub height: usize,
}

#[allow(dead_code)]
impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> Self {
        Self {
            state: ListState::default(),
            items,
            height: 0,
        }
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
    }

    pub fn next(&mut self) {
        // aitem no length - 4 ni narumade offset wo ugokasu
        // aitem no length - 4 ni nattara select wo ugokasu

        let selected = self.selected();
        if selected >= self.items.len() - 1 {
        } else if self.items.len().saturating_sub(3) <= selected {
            self.state.select(Some(selected + 1));
        } else if selected <= 4 {
            self.state.select(Some(selected + 1));
        } else {
            self.state.select(Some(selected + 1));
            self.state.next();
        }
    }
    pub fn prev(&mut self) {
        let selected = self.selected();
        if self.selected() <= 0 {
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
}
