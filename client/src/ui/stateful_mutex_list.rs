use std::sync::Arc;

use tokio::sync::Mutex;
use tui::widgets::ListState;

#[derive(Debug, Clone)]
pub struct StatefulMutexList<T> {
    pub state: ListState,
    pub items: Arc<Vec<Mutex<T>>>,
}

#[allow(dead_code)]
impl<T> StatefulMutexList<T>
where
    T: Clone,
{
    pub fn with_items(items: Vec<T>) -> StatefulMutexList<T> {
        let mut new_items: Vec<Mutex<T>> = Vec::new();
        for item in items {
            new_items.push(Mutex::new(item));
        }
        StatefulMutexList {
            state: ListState::default(),
            items: Arc::new(new_items),
        }
    }
    pub async fn update(&mut self, f: impl Fn(&mut T)) {
        for item in self.items.iter() {
            let mut item = item.lock().await;
            f(&mut item);
        }
    }
    pub async fn update_with_items(&mut self, items: Vec<T>) {
        let mut new_items: Vec<Mutex<T>> = Vec::new();
        for item in items {
            new_items.push(Mutex::new(item));
        }

        self.items = Arc::new(new_items);
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn prev(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub async fn get(&self) -> Option<T> {
        Some(
            self.items
                .get(self.state.selected()?)
                .unwrap()
                .lock()
                .await
                .clone(),
        )
    }
    pub async fn get_index(&self, index: usize) -> Option<T> {
        Some(self.items.get(index).unwrap().lock().await.clone())
    }
}
