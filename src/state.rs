use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct AppState {
    pub visits: usize,
}

pub type SharedState = Arc<Mutex<AppState>>;

pub fn create_state() -> SharedState {
    Arc::new(Mutex::new(AppState::default()))
}
