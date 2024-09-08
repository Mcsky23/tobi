use std::io;
use ratatui::widgets::StatefulWidget;
use std::path::Path;
use crate::settings::SETTINGS;

use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Alignment, Rect, Margin},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Widget,
        Borders,List, ListItem, ListState, 
    },
};

use crate::settings::settings_tui::{list_selector_trait::{HasListState, ListStateSelector}, AppMenuTrait};

pub struct FileList {
    selecting_for: Box<String>,
    cur_path: Box<Path>,
    items: Vec<String>,
    pub state: ListState,
}

impl HasListState for FileList {
    fn get_state(&mut self) -> &mut ListState {
        &mut self.state
    }
}

impl ListStateSelector for FileList {}

fn get_files_in_dir(path: Box<Path>) -> io::Result<Vec<String>> {
    // returns a vector that contains only the directories in the path
    // also don't show hidden directories

    let mut items = vec![];
    for entry in path.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && !path.file_name().unwrap().to_str().unwrap().starts_with(".") {
            items.push(entry.file_name().into_string().unwrap());
        }
    }

    // sort the items
    items.sort();
    Ok(items)
}


impl FileList {
    pub fn new(selecting_for: Box<String>, path: Box<Path>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            selecting_for,
            cur_path: path.clone(),
            items: get_files_in_dir(path.clone()).unwrap(),
            state,
        }
    }

    pub fn default(selecting_for: Box<String>) -> Self {
        let path = Path::new("/");
        Self::new(selecting_for, path.into())
    }

    pub fn set_path(&mut self, path: Box<Path>) {
        self.cur_path = path.clone();
        self.items = get_files_in_dir(path.clone()).unwrap();
    }
}

impl AppMenuTrait for FileList {
    fn handle_events(&mut self, event: KeyEvent) -> Result<Option<i32>, io::Error> {
        match self.handle_list_key_event(event) {
            Ok(_) => return Ok(None),
            Err(_) => {}
        }

        match event.code {
            KeyCode::Char('q') => {
                return Ok(Some(1));
            },
            KeyCode::Right => {
                // change file explorer path
                let selected = self.state.selected().unwrap_or_else(|| 0);
                if self.items.len() == 0 {
                    return Ok(None);
                }

                let new_path = self.cur_path.join(&self.items[selected]);
                self.set_path(new_path.into());
                self.state.select(Some(0));
                return Ok(None);
            },
            KeyCode::Left => {
                // go back
                let new_path = self.cur_path.parent().unwrap_or_else(|| Path::new("/"));
                self.set_path(new_path.into());
                
                // TODO: recover selection state by searching for the index of the current path in the items
                
                self.state.select(Some(0));
                return Ok(None);
            },
            KeyCode::Enter => {
                // select the current path and update the setting
                let selected = self.state.selected().unwrap_or_else(|| 0);
                let selected_path = self.cur_path.join(&self.items[selected]).to_str().unwrap().to_string();
                match self.selecting_for.as_str() {
                    "CTF path" => {
                        SETTINGS.lock().unwrap().workdir = selected_path;
                        return Ok(Some(1));
                    },
                    "DB path" => {
                        SETTINGS.lock().unwrap().db_file = selected_path + "/tobi.db";
                        return Ok(Some(1));
                    },
                    "Context path" => {
                        SETTINGS.lock().unwrap().context_file = selected_path + "/.tobicntxt";
                        return Ok(Some(1));
                    },
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn poll_exit(&self) -> bool {
        false
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        self._render(area, buf);
    }
}

impl FileList {
    fn _render(&mut self, area: Rect, buf: &mut Buffer) {
        // let header = Title::from(Line::from(vec![
        //     " File ".bold().blue(),
        //     "Explorer ".bold(),
        // ]));
        let header = Title::from(Line::from(vec![
            " Editing ".into(),
            self.selecting_for.clone().bold(),
            " : ".into(),
            self.cur_path.to_str().unwrap().to_string().bold(),
            " ".into(),

        ]));

        let instructions = Title::from(Line::from(vec![
            " Use ".into(),
            "↑/↓".bold().blue(),
            " to navigate, ".into(),
            "←/→".bold().blue(),
            " to change directory, ".into(),
            "Enter".bold().blue(),
            " to select, ".into(),
            "q".bold().blue(),
            " to go back ".into(),
        ]));


        let block = Block::bordered()
        .title(instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .title(header
                .alignment(Alignment::Center)
                .position(Position::Top),
        )
        .border_set(border::THICK);
        
        self.render_list(area, buf, &block);
        block.render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer, block: &Block) {
        // self.items = get_files_in_dir(self.cur_path.clone()).unwrap();
        // dbg!("caca" , &self.items);
        let list_block = Block::new()
            .borders(Borders::NONE);
        let list_block_area = block.inner(area.inner(Margin {
            horizontal: 3,
            vertical: 1,
        }));
        
        // fix weird bug that causes nothing to be blue
        if self.items.len() > 0 && self.state.selected().unwrap_or_else(|| 0) >= self.items.len() {
            self.state.select(Some(self.items.len() - 1));
        }
        let items = self.items.iter().enumerate().map(|(index, item)| {
            if Some(index) == self.state.selected() {
                return ListItem::new(Line::from(vec![
                    item.clone().bold().blue(),
                ]));
            }
            ListItem::new(Text::from(item.clone()))
        })
        .collect::<Vec<ListItem>>();
        
        let list = List::new(items)
            .highlight_symbol(">")
            .block(list_block);

        StatefulWidget::render(list, list_block_area, buf, &mut self.state);
    }
}



