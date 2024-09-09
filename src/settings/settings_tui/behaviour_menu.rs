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
use crate::settings::{SETTINGS, settings_tui::AppMenuTrait};

struct MenuItem {
    title: String,
    options: Vec<String>,
    selected: usize,
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
    fn new(title: &str, options: Vec<String>, selected: usize) -> Self {
        Self {
            title: title.to_string(),
            options,
            selected,
        }
    }
}

impl Default for Menu {
    fn default() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        
        let aux = SETTINGS.lock().unwrap();
        // find index of selected option
        let tobi_behaviour_ops = vec!["ctf", "context", "list", "solve", "unsolve"];
        let tobi_behaviour_selected = tobi_behaviour_ops.iter().position(|x| x == &aux.tobi_command).unwrap_or(0);
        
        Self {
            items: vec![
            MenuItem::new("`tobi` command should be an alias for ", 
            tobi_behaviour_ops.iter().map(|x| x.to_string()).collect(),
            tobi_behaviour_selected), 
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
                Ok(Some(0))
            },
            KeyCode::Right | KeyCode::Left => {
                let selected = self.state.selected().unwrap();
                let mut aux = SETTINGS.lock().unwrap();
                match selected {
                    0 => {
                        let item = &mut self.items[selected];
                        let selected = item.selected;
                        let new_selected = match event.code {
                            KeyCode::Right => {
                                if selected == item.options.len() - 1 {
                                    0
                                } else {
                                    selected + 1
                                }
                            },
                            KeyCode::Left => {
                                if selected == 0 {
                                    item.options.len() - 1
                                } else {
                                    selected - 1
                                }
                            },
                            _ => selected,
                        };
                        item.selected = new_selected;
                        aux.tobi_command = item.options[new_selected].clone();
                    },
                    _ => {}
                }
                Ok(None)
            },
            _ => Ok(None),
        }
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        self._render(area, buf);
    }
}

impl Menu {
    fn _render(&mut self, area: Rect, buf: &mut Buffer) {
        let header = Title::from(Line::from(vec![
            " Tobi ".bold().blue(),
            "Settings ".bold(),
        ]));

        let instructions = Title::from(Line::from(vec![
            " Use ".into(),
            "↑/↓".bold().blue(),
            " to navigate, ".into(),
            "←/→".bold().blue(),
            " to change settings, ".into(),
            "q".bold().blue(),
            " to exit ".into()
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
                    item.title.clone().bold(),
                    " - ".into(),
                    // pad with spaces
                    " ".to_string().repeat(50 - item.title.len()).into(),
                    "< ".into(),
                    item.options[item.selected].clone().italic().blue(),
                    " >".into(),
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
}
