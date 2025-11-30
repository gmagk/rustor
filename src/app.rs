// TODO:
// When a combination like Ctrl+Shift+<something> into tui_input in Add page the app fails,
// this seems to be fixed by removing any  "KeyCode::Char('<digit>')" matching from the code.

use crate::mapper::Mapper;
use crate::service::Service;
use crate::ui::screen_add::AddScreen;
use crate::ui::screen_help::HelpScreen;
use crate::ui::screen_home::HomeScreen;
use crate::ui::screen_info::InfoScreen;
use crate::ui::screen_reann::ReannScreen;
use crate::ui::screen_rm::RmScreen;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame, Terminal};
use std::cmp::PartialEq;
use std::sync::mpsc::{channel, TryRecvError};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{io, thread};

pub trait KeyEventHandler {

    /*
        Proposed usage of `bool` return value:
            [false] handling is finished from the specific call (can continue with maybe another handling)
            [true] do not continue handling logic (?what ever that might mean)
     */
    fn handle_key_event(&mut self, key_event: KeyEvent, event: Event) -> bool;
}

pub trait Renderable {

    fn render(&mut self, frame: &mut Frame, args: Vec<usize>);
}

#[derive(PartialEq)]
pub enum Screen {
    Home,
    Help,
    Add,
    ReAnn,
    Del,
    Info
}

struct AppState {
    screen: Screen
}

impl AppState {
    pub fn new(screen: Screen) -> Self {
        Self { screen }
    }
}

pub struct App {
    terminal: Arc<Mutex<DefaultTerminal>>,
    exit: bool,
    state: AppState
}

impl App {

    pub fn new(terminal: Arc<Mutex<DefaultTerminal>>) -> Self {
        Self { terminal, exit: false, state: AppState::new(Screen::Home) }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let service = Service::default();
        if !service.transmission_daemon_is_active() {
            println!("transmission-daemon does not look active");
            return Ok(());
        }

        let mapper = Mapper::default();
        let home_screen_arc = Arc::new(Mutex::new(HomeScreen::new(service, mapper)));
        let info_screen_arc = Arc::new(Mutex::new(InfoScreen::new(service, mapper)));
        let mut help_screen = HelpScreen::default();
        let mut add_screen = AddScreen::new(service);
        let mut reann_screen = ReannScreen::new(service, mapper);
        let mut del_screen = RmScreen::new(service, mapper);

        while !self.exit {
            let (tx, rx) = channel();
            let home_screen_arc_clone = home_screen_arc.clone();
            let info_screen_arc_clone = info_screen_arc.clone();
            let terminal_clone = self.terminal.clone();

            // home page (torrent list) needs refreshing
            if self.state.screen == Screen::Home {
                let _ = thread::spawn(move || loop {
                    let _ = terminal_clone.lock().unwrap().draw(|frame| {
                            home_screen_arc_clone.lock().unwrap().render(frame, vec![])
                        });
                    thread::sleep(Duration::from_millis(3000));

                    // thread control
                    match rx.try_recv() {
                        Ok(_) | Err(TryRecvError::Disconnected) => {
                            break;
                        }
                        Err(TryRecvError::Empty) => {}
                    }
                });
            }
            // info page (torrent info) needs refreshing
            else if self.state.screen == Screen::Info {
                let _ = thread::spawn(move || loop {
                    let selected_index = home_screen_arc_clone.lock().unwrap().active_row();
                    let _ = terminal_clone.lock().unwrap().draw(|frame| {
                        info_screen_arc_clone.lock().unwrap().render(frame, vec![selected_index])
                    });
                    thread::sleep(Duration::from_millis(3000));

                    // thread control
                    match rx.try_recv() {
                        Ok(_) | Err(TryRecvError::Disconnected) => {
                            break;
                        }
                        Err(TryRecvError::Empty) => {}
                    }
                });
            } else if self.state.screen == Screen::Del || self.state.screen == Screen::ReAnn {
                        let selected_index = home_screen_arc_clone.lock().unwrap().active_row();
                        let _ = terminal_clone.lock().unwrap().draw(|frame| {
                            match self.state.screen {
                                Screen::Del => del_screen.render(frame, vec![selected_index]),
                                Screen::ReAnn => reann_screen.render(frame, vec![selected_index]),
                                _ => {}
                            }
                        });
            } else {
                let _ = terminal_clone.lock().unwrap().draw(|frame| {
                    match self.state.screen {
                        Screen::Help => help_screen.render(frame, vec![]),
                        Screen::Add => add_screen.render(frame, vec![]),
                        _ => {}
                    }
                });
            }

            let event = event::read()?;
            if let Event::Key(key_event) = event {
                let ctrl = key_event.modifiers.contains(KeyModifiers::CONTROL);
                let shft = key_event.modifiers.contains(KeyModifiers::SHIFT);

                // terminate spawned thread (in case of home page)
                if self.state.screen != Screen::Home {
                    let _ = tx.send(());
                }

                let home_screen_arc_clone_2 = home_screen_arc.clone();
                let info_screen_arc_clone_2 = info_screen_arc.clone();

                // switch screens or exit
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Backspace => self.state.screen = Screen::Home,
                        KeyCode::Char('h') if ctrl => self.state.screen = Screen::Help,
                        KeyCode::Char('q') if ctrl => self.exit = true,
                        _ => {}
                    }
                }

                match self.state.screen {
                    Screen::Home => {
                        match key_event.code { // switch to subscreen
                            KeyCode::Char('a') if ctrl => self.state.screen = Screen::Add,
                            KeyCode::Char('d') if ctrl => self.state.screen = Screen::Del,
                            KeyCode::Char('r') if ctrl => self.state.screen = Screen::ReAnn,
                            KeyCode::Char('t') if ctrl => self.state.screen = Screen::Info,
                            _ => {
                                home_screen_arc_clone_2 .lock().unwrap().handle_key_event(key_event, event);
                            }
                        }
                    },
                    Screen::Help => { help_screen.handle_key_event(key_event, event); }
                    Screen::Add => {
                        if !add_screen.handle_key_event(key_event, event) {
                            self.state.screen = Screen::Home; // return to home if we are done from this screen
                        }
                    }
                    Screen::ReAnn => {
                        if !reann_screen.handle_key_event(key_event, event) {
                            self.state.screen = Screen::Home; // return to home if we are done from this screen
                        }
                    }
                    Screen::Del => {
                        if !del_screen.handle_key_event(key_event, event) {
                            self.state.screen = Screen::Home; // return to home if we are done from this screen
                        }
                    }
                    Screen::Info => {
                        if !info_screen_arc_clone_2.lock().unwrap().handle_key_event(key_event, event) {
                            self.state.screen = Screen::Home; // return to home if we are done from this screen
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
