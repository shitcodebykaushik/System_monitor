use crossterm::event::{self, Event, KeyCode};
// This funtion was designed to capture the input using the crossterm from the keywords 
pub fn handle_user_input() -> bool {
    if let Event::Key(key) = event::read().unwrap() {
        if key.code == KeyCode::Char('q') {
            return false;
        }
    }
    true
}
