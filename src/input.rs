use crossterm::event::{self, Event, KeyCode};

pub fn handle_user_input() -> bool {
    if let Event::Key(key) = event::read().unwrap() {
        if key.code == KeyCode::Char('q') {
            return false;
        }
    }
    true
}
