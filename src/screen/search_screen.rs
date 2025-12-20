use std::error::Error;
use std::sync::Arc;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout};
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;
use crate::app::{EmptyRenderableArgs, KeyEventHandler, Renderable, RenderableArgs};
use crate::config::{Config, ConfigKeyBinding};
use crate::dto::torrent_dto::{SearchTorrent, TorrentSource};
use crate::service::torrent_service::TorrentService;
use crate::screen::search_res_screen::SearchResScreen;
use crate::key_bindings::KeyBinding;
use crate::mapper::Mapper;

#[derive(Default)]
pub struct State {
    results: Vec<SearchTorrent>
}

impl State {
    
    pub fn get_results(&self) -> Vec<SearchTorrent> {
        self.results.clone()
    }
}

pub struct SearchScreen {
    config: Config,
    torrent_service: Arc<TorrentService>,
    input: Input,
    state: State,
    error_msg: String
}

impl SearchScreen {
    pub fn new(config: Config, torrent_service: Arc<TorrentService>) -> Self {
        Self { 
            config,
            torrent_service,
            input: Input::default(),
            state: State::default(),
            error_msg: String::new()
        }
    }
    
    pub fn get_state(&self) -> &State {
        &self.state
    }
}

impl Renderable<EmptyRenderableArgs> for SearchScreen {
    fn render(&mut self, frame: &mut Frame, args: EmptyRenderableArgs) {
        // frame
        let title = Line::from(" Search for torrents ".bold());
        let mut key_bindings = KeyBinding::new(self.config.clone());
        key_bindings
            .init(vec![
                ConfigKeyBinding::KbHome,
                ConfigKeyBinding::KbAdd,
                ConfigKeyBinding::KbHelp,
                ConfigKeyBinding::KbQuit,
            ]).add(KeyBinding::cancel_action());
        let main_block = Block::bordered()
            .title(title.centered())
            .title_bottom(key_bindings.items_as_line().centered())
            .border_set(border::THICK);
        let main_frame = Paragraph::new("").centered().block(main_block);
        frame.render_widget(main_frame, frame.area());

        // input
        let [input_area] = Layout::horizontal([Constraint::Percentage(50)])
            .flex(Flex::Center)
            .areas(frame.area());
        let [input_area] = Layout::vertical([Constraint::Length(3)]) // keep 2 for borders and 1 for cursor
            .flex(Flex::Center)
            .areas(input_area);
        let width = input_area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let title = Line::from(vec![
            " At least 3 letters".bold()
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::PLAIN);
        let input_ui = Paragraph::new(self.input.value())
            .scroll((0, scroll as u16))
            .block(block);
        frame.render_widget(input_ui, input_area);
        // Ratatui hides the cursor unless it's explicitly set. Position the  cursor past the
        // end of the input text and one line down from the border to the input line
        let x = self.input.visual_cursor().max(scroll) - scroll + 1;
        frame.set_cursor_position((input_area.x + x as u16, input_area.y + 1));
    }
}

impl KeyEventHandler for SearchScreen {

    fn handle_key_event(&mut self, key_event: KeyEvent, event: Event) -> bool {
        let ctrl = key_event.modifiers.contains(KeyModifiers::CONTROL);
        let shft = key_event.modifiers.contains(KeyModifiers::SHIFT);
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                // submit and leave
                KeyCode::Enter => {
                    let pirate_bay_result = match self.torrent_service.search_pirate_bay(self.input.value()) {
                        Ok(pirate_bay_result) => {
                            let result = pirate_bay_result[..20]
                                .iter()
                                .map(|torrent| Mapper::pirate_bay_list_torrent_to_search_torrent(torrent))
                                .collect();
                            Ok(result)
                        },
                        Err(e) => Err(e)
                    };
                    let torrents_csv_result = match self.torrent_service.search_torrents_csv(self.input.value()) {
                        Ok(torrents_csv_result) => {
                            let result = torrents_csv_result
                                .iter()
                                .map(|torrent| Mapper::torrents_csv_torrent_to_search_torrent(torrent))
                                .collect();
                            Ok(result)
                        },
                        Err(e) => Err(e)
                    };

                    self.input.reset();

                    self.state.results = match (&pirate_bay_result, &torrents_csv_result) {
                        (Err(_), Err(_)) => {
                            // TODO show error message (piratebay/torrentscsv failed)
                            let piratebay_error = pirate_bay_result.err().unwrap().to_string();
                            let torrentscsv_error = torrents_csv_result.err().unwrap().to_string();

                            vec![]
                        },
                        (Err(_), Ok(_)) => {
                            // TODO show error message (piratebay failed)
                            let piratebay_error = pirate_bay_result.err().unwrap();

                            torrents_csv_result.unwrap()
                        },
                        (Ok(_), Err(_)) => {
                            // TODO show error message (torrentscsv failed)
                            let torrentscsv_error = torrents_csv_result.err().unwrap().to_string();

                            pirate_bay_result.unwrap()
                        },
                        (Ok(_), Ok(_)) => {
                            let mut result = vec![pirate_bay_result.unwrap(), torrents_csv_result.unwrap()].concat();
                            result.sort_by(|a, b| (a.seeders*-1).partial_cmp(&(b.seeders*-1)).unwrap());
                            result
                        }
                    };
                    false
                }
                // leave
                KeyCode::Esc => {
                    self.input.reset();
                    false
                }
                // let input handle it
                _ => {
                    self.input.handle_event(&event);
                    true
                }
            }
        } else {
            false
        }
    }
}