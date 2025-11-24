use crate::dto::Torrent;
use crate::mapper::Mapper;
use crate::service::Service;
use crate::ui::screen_add::AddScreen;
use crate::ui::screen_del::DelScreen;
use crate::ui::screen_help::HelpScreen;
use crate::ui::screen_home::HomeScreen;
use crate::ui::screen_reann::ReannScreen;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
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

#[derive(PartialEq)]
pub enum Screen {
    Home,
    Help,
    Add,
    ReAnn,
    Del
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
    exit: bool,
    state: AppState
}

impl App {
    pub fn new() -> Self {
        Self { exit: false, state: AppState::new(Screen::Home) }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let service = Service::default();
        if !service.transmission_daemon_is_active() {
            println!("transmission-daemon does not look    active");
            return Ok(());
        }

        let mapper = Mapper::default();
        let home_screen = Arc::new(Mutex::new(HomeScreen::new(service)));
        let mut help_screen = HelpScreen::default();
        let mut add_screen = AddScreen::new(service);
        let mut reann_screen = ReannScreen::default();
        let mut del_screen = DelScreen::default();
        let terminal = Arc::new(Mutex::new(ratatui::init()));

        while !self.exit {
            let (tx, rx) = channel();
            let home_screen_clone_1 = home_screen.clone();
            let home_screen_clone_2 = home_screen.clone();
            let terminal_clone = terminal.clone();

            // home page (torrent list) needs refreshing
            if self.state.screen == Screen::Home {
                let _ = thread::spawn(move || loop {
                    let torrents: Vec<Torrent> = mapper.map_to_response(service.torrent_list()).arguments.torrents;
                    let _ = terminal_clone.lock().unwrap().draw(|frame| {
                            home_screen_clone_2.lock().unwrap().render(frame, &torrents)
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
            } else if self.state.screen == Screen::Help {
                let _ = terminal_clone.lock().unwrap().draw(|frame| {
                    frame.render_widget(help_screen, frame.area());
                    });
            } else if self.state.screen == Screen::Add {
                let _ = terminal_clone.lock().unwrap().draw(|frame| {
                        add_screen.render(frame);
                    });
            } else if self.state.screen == Screen::Del {
                let _ = terminal_clone.lock().unwrap().draw(|frame| {
                        del_screen.render(frame, home_screen_clone_1.lock().unwrap().active_row())
                    });
            } else if self.state.screen == Screen::ReAnn {
                let _ = terminal_clone.lock().unwrap().draw(|frame| {
                        reann_screen.render(frame, home_screen_clone_1.lock().unwrap().active_row())
                    });
            }

            let event = event::read()?;
            if let Event::Key(key_event) = event {

                // terminate spawned thread (in case of home page)
                if (self.state.screen != Screen::Home) {
                    let _ = tx.send(());
                }

                // switch screens or exit
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char(c) if c.is_numeric() && c.to_digit(10).unwrap() == 1 => {
                            self.state.screen = Screen::Home;
                        },
                        KeyCode::Char(c) if c.is_numeric() && c.to_digit(10).unwrap() == 2 => {
                            self.state.screen = Screen::Help;
                        },
                        KeyCode::Char('q') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.exit = true;
                        },
                        _ => {}
                    }
                }

                match self.state.screen {
                    Screen::Home => { home_screen_clone_1.lock().unwrap().handle_key_event(key_event, event); },
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
                }

                //  subscreens
                if self.state.screen == Screen::Home {
                    match key_event.code {
                        KeyCode::Char('a') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.state.screen = Screen::Add;
                        },
                        _ => {}
                    }
                    match key_event.code {
                        KeyCode::Char('d') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.state.screen = Screen::Del;
                        },
                        _ => {}
                    }
                    match key_event.code {
                        KeyCode::Char('r') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.state.screen = Screen::ReAnn;
                        },
                        _ => {}
                    }
                }
            }
        }
        ratatui::restore();
        Ok(())
    }
}
