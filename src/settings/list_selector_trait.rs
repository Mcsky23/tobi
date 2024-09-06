use ratatui::widgets::ListState;
use ratatui::crossterm::event::{KeyCode, KeyEvent};

pub trait ListStateSelector: HasListState {
    fn select_next(&mut self) {self.get_state().select_next()}
    fn select_previous(&mut self) {self.get_state().select_previous()}
    fn select_first(&mut self) {self.get_state().select_first()}
    fn select_last(&mut self) {self.get_state().select_last()}
    fn handle_list_key_event(&mut self, key: KeyEvent) -> Result<(), ()> {
        match key.code {
            KeyCode::Down => {self.select_next(); Ok(())},
            KeyCode::Up => {self.select_previous(); Ok(())},
            KeyCode::Home => {self.select_first(); Ok(())},
            KeyCode::End => {self.select_last(); Ok(())},
            _ => Err(()),
        }
    }
}

pub trait HasListState {
    fn get_state(&mut self) -> &mut ListState;
}






