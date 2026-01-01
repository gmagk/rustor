// TODO:
// When a combination like Ctrl+Shift+<something> into tui_input in Add page the app fails,
// this seems to be fixed by removing any  "KeyCode::Char('<digit>')" matching from the code.

use crate::service::transmission_service;
use crate::screen::add_screen;
use crate::screen::help_screen;
use crate::screen::home_screen;
use crate::screen::info_screen;
use crate::screen::reann_screen;
use crate::screen::rm_screen;
use crate::screen::search_screen;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::text::ToText;
use ratatui::{DefaultTerminal, Frame, Terminal};
use std::cmp::PartialEq;
use std::sync::mpsc::{TryRecvError, channel, Receiver};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{io, thread};
use std::io::Error;
use crate::app::Screen::SearchInfo;
use crate::client::http_client::HttpClient;
use crate::config::{Config, ConfigKeyBindingKey};
use crate::config::ConfigKeyBindingKey::{KbAdd, KbDel, KbDownload, KbHelp, KbHome, KbInfo, KbQuit, KbReAnn, KbSearch};
use crate::dto::transmission_dto::TransmissionTorrent;
use crate::service::torrent_service::TorrentService;
use crate::service::transmission_service::TransmissionService;
use crate::screen::add_screen::AddScreen;
use crate::screen::help_screen::HelpScreen;
use crate::screen::home_screen::HomeScreen;
use crate::screen::info_screen::{InfoScreen, InfoScreenArgs};
use crate::screen::popup_screen::{PopupScreen, PopupScreenArgs};
use crate::screen::reann_screen::{ReannScreen, ReannScreenArgs};
use crate::screen::rm_screen::{RmScreen, RmScreenArgs};
use crate::screen::search_info_screen::{SearchInfoScreen, SearchInfoScreenArgs};
use crate::screen::search_res_screen::{SearchResArgs, SearchResScreen};
use crate::screen::search_screen::SearchScreen;

pub trait KeyEventHandler {
    /*
       Proposed usage of `bool` return value:
           [false] handling is finished from the specific call (can continue with maybe another handling)
           [true] do not continue handling logic (?what ever that might mean)
    */
    fn handle_key_event(&mut self, key_event: KeyEvent, event: Event) -> Result<bool, Error>;
}

pub trait Renderable<T>
where T: RenderableArgs {
    fn render(&mut self, frame: &mut Frame, args: T);
}

pub trait RenderableArgs {}

#[derive(Default)]
pub struct EmptyRenderableArgs {}

impl RenderableArgs for EmptyRenderableArgs {}

#[derive(Default, Clone, PartialEq)]
pub enum Screen {
    #[default]
    Home,
    Help,
    Add,
    ReAnn,
    Del,
    Info,
    Search,
    SearchRes,
    SearchInfo
}

#[derive(Default)]
struct AppState {
    screen: Screen,
    popup_state: bool,
    popup_show: bool,
    popup_message: String
}

pub struct App {
    config: Config,
    terminal: Arc<Mutex<DefaultTerminal>>,
    state: Arc<Mutex<AppState>>,
}

impl App {

    pub fn new(config: Config, terminal: Arc<Mutex<DefaultTerminal>>) -> Self {
        Self {
            config,
            terminal,
            state: Arc::new(Mutex::new(AppState::default())),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        if !TransmissionService::transmission_daemon_is_active() {
            println!("transmission-daemon does not look active");
            return Ok(());
        }

        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
        let http_client = HttpClient::new(runtime);
        let torrent_service_arc = Arc::new(TorrentService::new(http_client));
        let config_values = self.config.values();
        let key_bindings = config_values.key_bindings();

        let home_screen_arc = Arc::new(Mutex::new(HomeScreen::new(key_bindings.clone())));
        let info_screen_arc = Arc::new(Mutex::new(InfoScreen::new(key_bindings.clone())));
        let mut help_screen = HelpScreen::new(key_bindings.clone());
        let mut add_screen = AddScreen::new(key_bindings.clone());
        let mut reann_screen = ReannScreen::new(key_bindings.clone());
        let mut del_screen = RmScreen::new(key_bindings.clone());
        let mut search_screen = SearchScreen::new(key_bindings.clone(), torrent_service_arc.clone());
        let mut search_res_screen = SearchResScreen::new(key_bindings.clone(), torrent_service_arc.clone());
        let mut search_info_screen = SearchInfoScreen::new(key_bindings.clone());

        loop {
            let (tx, rx) = channel();
            let terminal_clone = self.terminal.clone();
            let app_state_clone = self.state.clone();

            let home_screen_arc_clone = home_screen_arc.clone();
            let info_screen_arc_clone = info_screen_arc.clone();
            let current_screen_clone = app_state_clone.lock().unwrap().screen.clone();

            // draw
            match current_screen_clone {
                Screen::Home => {
                    thread::spawn(move || {
                        loop {
                            let _ = terminal_clone.lock().unwrap().draw(|frame| {
                                home_screen_arc_clone.lock().unwrap().render(frame, EmptyRenderableArgs::default());
                            });
                            thread::sleep(Duration::from_millis(3000));
                            // thread control
                            match rx.try_recv() {
                                Ok(_) | Err(TryRecvError::Disconnected) => {
                                    break;
                                }
                                Err(TryRecvError::Empty) => {}
                            }
                        }
                    });
                } Screen::Info => {
                    thread::spawn(move || {
                        loop {
                            let mut do_break = false;
                            let _ = terminal_clone.lock().unwrap().draw(|frame| {
                                match home_screen_arc_clone.lock().unwrap().active_row_torrent() {
                                    Ok(torrent) => info_screen_arc_clone.lock().unwrap().render(frame, InfoScreenArgs::new(torrent)),
                                    Err(e) => {
                                        Self::show_popup(app_state_clone.clone(), e.to_string());
                                        do_break = true;
                                    }
                                }
                            });
                            if do_break {
                                break;
                            }
                            thread::sleep(Duration::from_millis(3000));
                            // thread control
                            match rx.try_recv() {
                                Ok(_) | Err(TryRecvError::Disconnected) => {
                                    break;
                                }
                                Err(TryRecvError::Empty) => {}
                            }
                        }
                    });
                } Screen::SearchRes => self.draw(|f|search_res_screen.render(f, SearchResArgs::new(search_screen.get_state().get_results()))),
                Screen::SearchInfo => self.draw(|f|search_info_screen.render(f, SearchInfoScreenArgs::new(search_res_screen.active_row_torrent()))),
                Screen::Del => self.draw(|f|del_screen.render(f, RmScreenArgs::new(home_screen_arc_clone.lock().unwrap().active_row()))),
                Screen::ReAnn => self.draw(|f|reann_screen.render(f, ReannScreenArgs::new(home_screen_arc_clone.lock().unwrap().active_row()))),
                Screen::Help => self.draw(|f|help_screen.render(f, EmptyRenderableArgs::default())),
                Screen::Add => self.draw(|f|add_screen.render(f, EmptyRenderableArgs::default())),
                Screen::Search => self.draw(|f|search_screen.render(f, EmptyRenderableArgs::default()))
            }

            // block and wait for user event
            let event = event::read()?;

            // terminate any spawned thread
            let _ = tx.send(());

            if let Event::Key(key_event) = event {

                // handle only keyboard keys
                if key_event.kind != KeyEventKind::Press {
                    continue
                }

                // check quit
                let quit_bind = *key_bindings.get(&KbQuit).unwrap();
                if key_event.code == KeyCode::Char(quit_bind) { break }

                // try change main screen
                if  self.try_change_screen(key_event, KbHome, Screen::Home) ||
                    self.try_change_screen(key_event, KbAdd, Screen::Add) ||
                    self.try_change_screen(key_event, KbSearch, Screen::Search) ||
                    self.try_change_screen(key_event, KbHelp, Screen::Help) {
                    continue
                }

                let current_screen = self.state.lock().unwrap().screen.clone(); // keep this dereferenced by assigning it to a variable (use this expression as it is in the match block)

                // try change sub-screen
                match current_screen {
                    Screen::Home => {
                        if  self.try_change_screen(key_event, KbDel, Screen::Del) ||
                            self.try_change_screen(key_event, KbReAnn, Screen::ReAnn) ||
                            self.try_change_screen(key_event, KbInfo, Screen::Info) {
                            continue
                        }
                    } Screen::SearchRes => {
                        if  self.try_change_screen(key_event, KbInfo, Screen::SearchInfo) {
                            continue
                        }
                    } _ => {}
                }

                // handle key-event by current screen
                match current_screen {
                    Screen::Home => self.handle_key_event_or_popup(home_screen_arc.lock().unwrap().handle_key_event(key_event, event)),
                    Screen::SearchRes => self.handle_key_event_or_popup(search_res_screen.handle_key_event(key_event, event)),
                    Screen::Search => self.handle_key_event_and_change_screen_or_popup(search_screen.handle_key_event(key_event, event), Screen::SearchRes),
                    Screen::SearchInfo => self.handle_key_event_and_change_screen_or_popup(search_info_screen.handle_key_event(key_event, event), Screen::SearchRes),
                    Screen::Help => self.handle_key_event_and_change_screen_or_popup(help_screen.handle_key_event(key_event, event), Screen::Home),
                    Screen::Add => self.handle_key_event_and_change_screen_or_popup(add_screen.handle_key_event(key_event, event), Screen::Home),
                    Screen::ReAnn => self.handle_key_event_and_change_screen_or_popup(reann_screen.handle_key_event(key_event, event), Screen::Home),
                    Screen::Del => self.handle_key_event_and_change_screen_or_popup(del_screen.handle_key_event(key_event, event), Screen::Home),
                    Screen::Info => self.handle_key_event_and_change_screen_or_popup(info_screen_arc.lock().unwrap().handle_key_event(key_event, event), Screen::Home)
                }
            }
        }
        Ok(())
    }

    fn handle_key_event_or_popup(&self, result: Result<bool, Error>) {
        match result {
            Ok(_) => {}
            Err(e) => { Self::show_popup(self.state.clone(), e.to_string()) }
        }
    }

    fn handle_key_event_and_change_screen_or_popup(&self, result: Result<bool, Error>, next_screen: Screen) {
        match result {
            Ok(keep_handling) => {
                if !keep_handling {
                    self.state.lock().unwrap().screen = next_screen; // if we are done return to another screen
                }
            } Err(e) => { Self::show_popup(self.state.clone(), e.to_string()) }
        }
    }

    fn draw<F>(&self, render_callback: F)
    where F: FnOnce(&mut Frame) {
        let _ = self.terminal.clone().lock().unwrap().draw(|frame | {
            render_callback(frame);
            Self::popup(self.config.clone(), self.state.clone(), frame);
        });
    }

    fn try_change_screen(&self, key_event: KeyEvent, key: ConfigKeyBindingKey, screen: Screen) -> bool {
        if !key_event.modifiers.contains(KeyModifiers::CONTROL) {
            return false;
        }
        match key_event.code {
            KeyCode::Char(c) => {
                if c == *self.config.values().key_bindings().get(&key).unwrap() {
                    self.state.lock().unwrap().screen = screen;
                    return true
                };
                false
            } _ => { false }
        }
    }

    fn show_popup(state: Arc<Mutex<AppState>>, message: String) {
        let mut st = state.lock().unwrap();
        if st.popup_state {
            return;
        }
        st.popup_message = message;
        st.popup_show = true;
        let state_clone = state.clone();
        thread::spawn(move || {
            thread::sleep(Duration::new(1, 0));
            state_clone.lock().unwrap().popup_show = false;
            state_clone.lock().unwrap().popup_state = false;
        });

    }

    fn popup(config: Config, state: Arc<Mutex<AppState>>, frame: &mut Frame) {
        let mut state = state.lock().unwrap();
        if state.popup_show {
            PopupScreen::new(config.values().key_bindings().clone()).render(frame, PopupScreenArgs::new(state.popup_message.clone()));
            state.popup_state = true
        }
    }
}
