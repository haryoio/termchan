/// flux architecture
///
/// move_up(&mut store);
/// move_dowm(&mut store);
/// store.update(|state| {
///    state.move_up();
/// });
use std::{cell::RefCell, collections::HashMap};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crate::config::Theme;

/// UI
/// tab name / bredcrumb /

pub struct State {
    message:   Option<String>,
    tab_index: Mutex<Arc<usize>>,
    tab_items: Arc<Vec<Mutex<String>>>,
    theme:     Mutex<Arc<Theme>>,
}
