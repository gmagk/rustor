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
use std::ascii::AsciiExt;
use std::cmp::PartialEq;
use std::sync::mpsc::{TryRecvError, channel};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{io, thread};
use crate::app::Screen::SearchInfo;
use crate::client::http_client::HttpClient;
use crate::config::Config;
use crate::config::ConfigKeyBindingKey::{KbAdd, KbDel, KbDownload, KbHelp, KbHome, KbInfo, KbQuit, KbReAnn, KbSearch};
use crate::service::torrent_service::TorrentService;
use crate::service::transmission_service::TransmissionService;
use crate::screen::add_screen::AddScreen;
use crate::screen::help_screen::HelpScreen;
use crate::screen::home_screen::HomeScreen;
use crate::screen::info_screen::{InfoScreen, InfoScreenArgs};
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
    fn handle_key_event(&mut self, key_event: KeyEvent, event: Event) -> bool;
}

pub trait Renderable<T>
where T: RenderableArgs {
    fn render(&mut self, frame: &mut Frame, args: T);
}

pub trait RenderableArgs {}

#[derive(Default)]
pub struct EmptyRenderableArgs {}

impl RenderableArgs for EmptyRenderableArgs {}

#[derive(PartialEq)]
pub enum Screen { Home, Help, Add, ReAnn, Del, Info, Search, SearchRes, SearchInfo, Popup }

struct AppState {
    screen: Screen,
    popup_msg: String
}

impl AppState {
    pub fn new(screen: Screen) -> Self {
        Self { screen, popup_msg: String::new() }
    }
}

pub struct App {
    config: Config,
    terminal: Arc<Mutex<DefaultTerminal>>,
    state: AppState,
}

impl App {

    pub fn new(config: Config, terminal: Arc<Mutex<DefaultTerminal>>) -> Self {
        Self {
            config,
            terminal,
            state: AppState::new(Screen::Home),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        if !TransmissionService::transmission_daemon_is_active() {
            println!("transmission-daemon does not look active");
            return Ok(());
        }

        let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()?;
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
            let home_screen_arc_clone = home_screen_arc.clone();
            let info_screen_arc_clone = info_screen_arc.clone();
            let terminal_clone = self.terminal.clone();

            // home page (torrent list) needs refreshing, info page (torrent info) needs refreshing
            match self.state.screen {
                Screen::Home | Screen::Info => {
                    let is_home = self.state.screen == Screen::Home;
                    thread::spawn(move || {
                        loop {
                            let _ = terminal_clone.lock().unwrap().draw(|frame| {
                                match is_home {

                                    // Home screen
                                    true => home_screen_arc_clone.lock().unwrap().render(frame, EmptyRenderableArgs::default()),

                                    // Info screen
                                    _ => {
                                        let selected_torrent = home_screen_arc_clone.lock().unwrap().active_row_torrent();
                                        info_screen_arc_clone
                                            .lock()
                                            .unwrap()
                                            .render(frame, InfoScreenArgs::new(selected_torrent))
                                    }
                                }
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
                } _ => {
                    let selected_index = home_screen_arc_clone.lock().unwrap().active_row();
                    let _ = terminal_clone
                        .lock()
                        .unwrap()
                        .draw(|frame|
                            match self.state.screen {
                                Screen::SearchRes => {
                                    search_res_screen.render(frame, SearchResArgs::new(search_screen.get_state().get_results()))
                                } Screen::SearchInfo => {
                                   search_info_screen.render(frame, SearchInfoScreenArgs::new(search_res_screen.active_row_torrent()))
                                } Screen::Popup => {
                                   // TODO
                                } Screen::Del | Screen::ReAnn => {
                                    match self.state.screen {
                                        Screen::Del => del_screen.render(frame, RmScreenArgs::new(selected_index)),
                                        Screen::ReAnn => reann_screen.render(frame, ReannScreenArgs::new(selected_index)),
                                        _ => {}
                                    }
                                } _ => {
                                    match self.state.screen {
                                        Screen::Help => help_screen.render(frame, EmptyRenderableArgs::default()),
                                        Screen::Add => add_screen.render(frame, EmptyRenderableArgs::default()),
                                        Screen::Search => search_screen.render(frame, EmptyRenderableArgs::default()),
                                        Screen::SearchRes => search_res_screen.render(frame, SearchResArgs::new(search_screen.get_state().get_results())),
                                        _ => {}
                                    }
                                }
                            }
                        );
                }
            }

            let event = event::read()?;
            if let Event::Key(key_event) = event {
                let ctrl = key_event.modifiers.contains(KeyModifiers::CONTROL);

                // terminate spawned thread (in case of home page)
                if self.state.screen != Screen::Home {
                    let _ = tx.send(());
                }

                let home_screen_arc_clone_2 = home_screen_arc.clone();
                let info_screen_arc_clone_2 = info_screen_arc.clone();

                // switch main screens or exit
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char(c) if ctrl => {
                            if c == *key_bindings.get(&KbHome).unwrap() { self.state.screen = Screen::Home }
                            else if c == *key_bindings.get(&KbAdd).unwrap() { self.state.screen = Screen::Add }
                            else if c == *key_bindings.get(&KbSearch).unwrap() { self.state.screen = Screen::Search }
                            else if c == *key_bindings.get(&KbHelp).unwrap() { self.state.screen = Screen::Help }
                            else if c == *key_bindings.get(&KbQuit).unwrap() { break }
                        },
                        _ => {}
                    }
                }

                match self.state.screen {
                    Screen::Home => {
                        match key_event.code {
                            // switch to subscreen
                            KeyCode::Char(c) if ctrl => {
                                if c == *key_bindings.get(&KbDel).unwrap() { self.state.screen = Screen::Del }
                                else if c == *key_bindings.get(&KbReAnn).unwrap() { self.state.screen = Screen::ReAnn }
                                else if c == *key_bindings.get(&KbInfo).unwrap() { self.state.screen = Screen::Info }
                                else {
                                    home_screen_arc_clone_2
                                        .lock()
                                        .unwrap()
                                        .handle_key_event(key_event, event);
                                }
                            }
                            _ => {
                                home_screen_arc_clone_2
                                    .lock()
                                    .unwrap()
                                    .handle_key_event(key_event, event);
                            }
                        }
                    } Screen::SearchRes => {
                        match key_event.code {
                            // switch to subscreen
                            KeyCode::Char(c) if ctrl => {
                                if c == *key_bindings.get(&KbInfo).unwrap() { self.state.screen = Screen::SearchInfo }
                                else if c == *key_bindings.get(&KbDownload).unwrap() {
                                    search_res_screen.handle_key_event(key_event, event);
                                    self.state.screen = Screen::Home
                                }
                            }
                            _ => {
                                search_res_screen.handle_key_event(key_event, event);
                            }
                        }
                    } Screen::Search => {
                        if !search_screen.handle_key_event(key_event, event) {
                            self.state.screen = Screen::SearchRes; // return to search results if we are done from this screen
                        }
                    } Screen::SearchInfo => {
                        if !search_info_screen.handle_key_event(key_event, event) {
                            self.state.screen = Screen::SearchRes; // return to search results if we are done from this screen
                        }
                    } Screen::Help => {
                        help_screen.handle_key_event(key_event, event);
                    } Screen::Add => {
                        if !add_screen.handle_key_event(key_event, event) {
                            self.state.screen = Screen::Home; // return to home if we are done from this screen
                        }
                    } Screen::ReAnn => {
                        if !reann_screen.handle_key_event(key_event, event) {
                            self.state.screen = Screen::Home; // return to home if we are done from this screen
                        }
                    } Screen::Del => {
                        if !del_screen.handle_key_event(key_event, event) {
                            self.state.screen = Screen::Home; // return to home if we are done from this screen
                        }
                    } Screen::Info => {
                        if !info_screen_arc_clone_2
                            .lock()
                            .unwrap()
                            .handle_key_event(key_event, event)
                        {
                            self.state.screen = Screen::Home; // return to home if we are done from this screen
                        }
                    } Screen::Popup => {

                    }
                }
            }
        }
        Ok(())
    }
}
