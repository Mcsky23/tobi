pub static WORKDIR: &str = "/Users/mcsky/Desktop/Codeprojects/tobi_test";
pub static DB_FILE: &str = "/Users/mcsky/Desktop/Codeprojects/tobi_test/tobi.db";
pub static CONTEXT_FILE: &str = "/Users/mcsky/Desktop/Codeprojects/tobi_test/.context";

pub mod main_menu;
pub mod tui;
pub mod center;
pub mod list_selector_trait;

use center::center;
use std::io;


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
    fn handle_events(&mut self, event: KeyEvent) -> Result<(), io::Error>;
    fn render(&mut self, area: Rect, buf: &mut Buffer);
    fn poll_exit(&self) -> bool {
        false
    }
}

enum State {
    MainMenu(RefCell<Box<dyn AppMenuTrait>>),
    PathMenu(RefCell<Box<dyn AppMenuTrait>>),
}

impl State {
    fn new_main_menu() -> Self {
        Self::MainMenu(RefCell::new(Box::new(main_menu::Menu::default())))
    }
    fn new_path_menu() -> Self {
        Self::PathMenu(RefCell::new(Box::new(main_menu::Menu::default())))
    }

    fn handle_events(&self) -> Result<(), io::Error> {

        match self {
            Self::MainMenu(menu) | Self::PathMenu(menu) => {
                let event = match event::read()? {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        key_event
                    },
                    _ => return Ok(()),
                };
                menu.borrow_mut().handle_events(event)
            }
        }
    }

    fn poll_exit(&self) -> bool {
        match self {
            Self::MainMenu(menu) | Self::PathMenu(menu) => {
                menu.borrow().poll_exit()
            }
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        match self {
            Self::MainMenu(menu) | Self::PathMenu(menu) => {
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
        self.state.handle_events()
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
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