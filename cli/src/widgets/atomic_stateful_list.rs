use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

use tui::widgets::ListState;

#[derive(Debug, Clone)]
pub struct AtomicStatefulList<T> {
    pub state: ListState,
    pub items: Arc<Mutex<RefCell<Vec<T>>>>,
    is_loop: bool,
}

impl<T: Clone> AtomicStatefulList<T> {
    pub fn with_items(items: Vec<T>) -> Self {
        Self {
            state: ListState::default(),
            items: Arc::new(Mutex::new(RefCell::new(items))),
            is_loop: true,
        }
    }

    pub fn to_vec(&mut self) -> Vec<T> {
        self.items.lock().unwrap().borrow_mut().to_vec()
    }

    pub fn enable_loop(&mut self) {
        self.is_loop = true;
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        *self.items.lock().unwrap().borrow_mut() = items;
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.lock().unwrap().borrow().len() - 1 {
                    if self.is_loop {
                        0
                    } else {
                        i
                    }
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i <= 0 {
                    if self.is_loop {
                        self.items.lock().unwrap().borrow().len() - 1
                    } else {
                        0
                    }
                } else {
                    i - 1
                }
            }
            None => self.items.lock().unwrap().borrow().len() - 1,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
