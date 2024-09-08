use std::io;
use ratatui::widgets::StatefulWidget;


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

use crate::settings::settings_tui::list_selector_trait::{HasListState, ListStateSelector};
use crate::settings::settings_tui::AppMenuTrait;

struct MenuItem {
    title: String,
    description: String,
}

pub struct Menu {
    items: Vec<MenuItem>,
    pub state: ListState,
    pub should_exit: bool,
}

impl HasListState for Menu {
    fn get_state(&mut self) -> &mut ListState {
        &mut self.state
    }
}

impl ListStateSelector for Menu {}

impl MenuItem {
    fn new(title: &str, description: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
        }
    }
}

impl Default for Menu {
    fn default() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            items: vec![
                MenuItem::new("Path Settings", "Change default path for CTFs, db, etc."),
                MenuItem::new("Behavior Settings", "Customize tobi behavior to your liking"),
                MenuItem::new("Info", "Info about tobi"),
            ],
            state: state,
            should_exit: false,
        }
    }
}

impl AppMenuTrait for Menu {
    fn handle_events(&mut self, event: KeyEvent) -> Result<Option<i32>, io::Error> {
        match self.handle_list_key_event(event) {
            Ok(_) => return Ok(None),
            Err(_) => {}
        }
        match event.code {
            KeyCode::Char('q') => {
                self.should_exit = true;
            },
            KeyCode::Enter => {
                match self.state.selected() {
                    Some(0) => {
                        // open path menu
                        return Ok(Some(1))
                    },
                    _ => {}
                }
            },

            _ => {}
        }
        Ok(None)
    }

    fn poll_exit(&self) -> bool {
        self.should_exit
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        ("rendering menu");
        self._render(area, buf);
    }
}

impl Menu {
    fn _render(&mut self, area: Rect, buf: &mut Buffer) {
        self.render_block(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer, block: &Block) {
        let list_block = Block::new()
            .borders(Borders::NONE);
        let list_block_area = block.inner(area.inner(Margin {
            horizontal: 3,
            vertical: 1,
        }));
        
        // fix weird bug that causes nothing to be blue
        if self.state.selected().unwrap_or_else(|| 0) >= self.items.len() {
            self.state.select(Some(self.items.len() - 1));
        }
        let items = self.items.iter().enumerate().map(|(index, item)| {
            if Some(index) == self.state.selected() {
                return ListItem::new(Line::from(vec![
                    item.title.clone().bold().blue(),
                    " - ".into(),
                    item.description.clone().italic().gray(),
                ]))
            }
            ListItem::new(Text::from(item.title.clone()))
        })
        .collect::<Vec<ListItem>>();
        
        let list = List::new(items)
            .highlight_symbol(">")
            .block(list_block);

        StatefulWidget::render(list, list_block_area, buf, &mut self.state);
    }

    fn render_block(&mut self, area: Rect, buf: &mut Buffer) {
        let header = Title::from(Line::from(vec![
            " Tobi ".bold().blue(),
            "Settings ".bold(),
        ]));

        let instructions = Title::from(Line::from(vec![
            " Use ".into(),
            "↑/↓".bold().blue(),
            " to navigate, ".into(),
            "Enter".bold().blue(),
            " to select, ".into(),
            "q".bold().blue(),
            " to exit ".into(),
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
}