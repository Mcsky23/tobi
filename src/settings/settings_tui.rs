pub mod main_menu;
pub mod path_menu;
pub mod tui;
pub mod center;
pub mod list_selector_trait;

use center::center;
use std::io;
use crate::settings;


use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyEvent, KeyEventKind},
    layout::{Constraint, Rect},
    widgets::Widget,
    Frame,
};
use std::cell::RefCell;

pub struct App {
    state: State,
}

pub trait AppMenuTrait {
    fn handle_events(&mut self, event: KeyEvent) -> Result<Option<i32>, io::Error>;
    fn render(&mut self, area: Rect, buf: &mut Buffer);
    fn poll_exit(&self) -> bool {
        false
    }
    fn get_selected_item(&self) -> Option<Box<String>> {
        None
    }
}

enum State {
    MainMenu(RefCell<Box<dyn AppMenuTrait>>),
    PathMenu(RefCell<Box<dyn AppMenuTrait>>),
    FileExplorer(RefCell<Box<dyn AppMenuTrait>>),
}

impl State {
    fn new_main_menu() -> Self {
        Self::MainMenu(RefCell::new(Box::new(main_menu::Menu::default())))
    }
    fn new_path_menu() -> Self {
        Self::PathMenu(RefCell::new(Box::new(path_menu::Menu::default())))
    }

    fn new_file_explorer(&self) -> Self {
        // create a file explorer that will change the specified setting by passing a Box<String> to the FileList
        Self::FileExplorer(RefCell::new(Box::new(path_menu::explorer::FileList::default(match self {
            Self::PathMenu(menu) => menu.borrow().get_selected_item().unwrap_or_else(|| Box::new("".to_string())),
            _ => Box::new("".to_string()),
        }))))
    }

    fn to_idx(&self) -> i32 {
        match self {
            Self::MainMenu(_) => 0,
            Self::PathMenu(_) => 1,
            Self::FileExplorer(_) => 2,
        }
    }

    fn from_idx(&self, idx: i32) -> Self {
        match idx {
            0 => Self::new_main_menu(),
            1 => Self::new_path_menu(),
            2 => Self::new_file_explorer(&self),
            _ => Self::new_main_menu(),
        }
    }

    fn handle_events(&self) -> Result<Option<i32>, io::Error> {

        match self {
            Self::MainMenu(menu) | Self::PathMenu(menu) | Self::FileExplorer(menu) => {
                let event = match event::read()? {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        key_event
                    },
                    _ => return Ok(Some(self.to_idx())),
                };
                menu.borrow_mut().handle_events(event)
            }
        }
    }

    fn poll_exit(&self) -> bool {
        match self {
            Self::MainMenu(menu) | Self::PathMenu(menu) | Self::FileExplorer(menu) => {
                menu.borrow().poll_exit()
            }
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        match self {
            Self::MainMenu(menu) | Self::PathMenu(menu) | Self::FileExplorer(menu) => {
                menu.borrow_mut().render(area, buf)
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: State::new_main_menu(),
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.state.poll_exit() {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        // save settings
        settings::save_settings_to_file()?;
        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let area = center(
            frame.area(),
            Constraint::Length(120),
            Constraint::Length(30),
        );

        frame.render_widget(self, area);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        let rez = self.state.handle_events()?;
        if !rez.is_none() && rez.unwrap() != self.state.to_idx() {
            self.state = self.state.from_idx(rez.unwrap());
        }
        Ok(())
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        (self.state.to_idx());
        self.state.render(area, buf);
    }
}

pub fn run_setting_menu() -> io::Result<()> {
    let mut terminal = tui::init_terminal()?;
    terminal.clear()?;

    let mut app = App::default();
    let app_result = app.run(&mut terminal);
    tui::restore_terminal()?;
    app_result
}