use std::collections::HashMap;
use crate::app::{EmptyRenderableArgs, KeyEventHandler, Renderable, RenderableArgs, Screen};
use crate::config::{Config, ConfigKeyBindingKey};
use crate::screen::key_bindings_block::{KeyBindingItem, KeyBindingsBlock};
use crate::util::Util;
use chrono::{DateTime, Local, Utc};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::Frame;
use ratatui::layout::Constraint;
use ratatui::prelude::{Modifier, Style, Stylize, Text};
use ratatui::style::Color;
use ratatui::symbols::border;
use ratatui::text::{Line, StyledGrapheme};
use ratatui::widgets::{Block, Cell, Padding, Paragraph, Row, Table, TableState};
use std::fmt::Debug;
use std::time::{Duration, UNIX_EPOCH};
use crate::config::ConfigKeyBindingKey::{KbDel, KbOpen};
use crate::dto::transmission_dto::TransmissionTorrent;
use crate::service::transmission_service::TransmissionService;

#[derive(Default, Clone)]
struct State {
    torrent_ids: Vec<i64>,
    row_index_last_used_for_fetching_torrent: usize,
    selected_row_torrent: TransmissionTorrent
}

#[derive(Clone)]
pub struct HomeScreen {
    config_key_bindings: HashMap<ConfigKeyBindingKey, char>,
    table_state: TableState,
    state: State
}

impl HomeScreen {

    pub fn new(config_key_bindings: HashMap<ConfigKeyBindingKey, char>) -> Self {
        Self {
            config_key_bindings,
            table_state: TableState::default().with_selected(0),
            state: State::default()
        }
    }

    pub fn next_row(&mut self) {
        let max_index = self.state.torrent_ids.len() as i32 - 1;
        let i = match self.table_state.selected() {
            Some(i) => {
                if i as i32 == max_index {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        // self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i > 0 {
                    i - 1
                } else {
                    i
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        // self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    // TODO fix
    pub fn next_column(&mut self) {
        self.table_state.select_next_column();
    }

    // TODO fix
    pub fn previous_column(&mut self) {
        self.table_state.select_previous_column();
    }

    pub fn active_row(&self) -> usize {
        self.table_state.selected().unwrap_or(0)
    }

    pub fn active_row_torrent(&mut self) -> TransmissionTorrent {
        let cur_sel_index = self.table_state.selected().unwrap_or(0);
        if  self.state.selected_row_torrent.id != 0 &&
            self.state.row_index_last_used_for_fetching_torrent == cur_sel_index {

            return self.state.selected_row_torrent.clone()
        }

        self.state.row_index_last_used_for_fetching_torrent = cur_sel_index.clone();
        let torrent_id = self.state.torrent_ids[cur_sel_index].to_string();
        let torrent = TransmissionService::torrent_info(torrent_id)
            .arguments
            .torrents[0].clone();
        self.state.selected_row_torrent = torrent.clone();

        torrent.clone()
    }

    fn table(self, torrents: &Vec<TransmissionTorrent>) -> Table<'static> {
        let rows = torrents.iter().enumerate().map(|(i, torrent)| {
            let item = [
                &torrent.id.to_string(),
                &torrent.name,
                &torrent.eta(),
                &torrent.percentage_done(),
                &torrent.download_rate(),
                &torrent.upload_rate(),
                &torrent.total_size(),
                &torrent.downloaded(),
                &Util::print_epoch(torrent.added_date as u64),
                &torrent.peers_client_name(),
            ];
            // TODO show tor error
            // TODO show done status
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Row>()
                .height(3)
        });
        let header = [
            "Id",
            "Name",
            "ETA",
            "Done",
            "Download",
            "Upload",
            "Size",
            "Downloaded",
            "Added On",
        ]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .height(1)
        .bg(Color::Indexed(236)) // https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
        .fg(Color::Indexed(255));
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(Color::Indexed(255)) // https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
            .bg(Color::Black);
        Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Length(4),
                Constraint::Length(self.name_len(&torrents) + 1),
                Constraint::Length(16),
                Constraint::Length(16),
                Constraint::Length(16),
                Constraint::Length(16),
                Constraint::Length(10),
                Constraint::Length(10),
                Constraint::Length(20),
            ],
        )
        .header(header)
        .row_highlight_style(selected_row_style)
    }

    // Find for name column which row has the largest (this is done only for string values which might be too long)
    fn name_len(&self, items: &Vec<TransmissionTorrent>) -> u16 {
        items
            .iter()
            .map(|t| t.name.chars().count())
            .max()
            .unwrap_or(0) as u16
    }

    // Find for combined peers.client_name column which row has the largest (this is done only for string values which might be too long)
    fn peer_client_name_len(&self, items: &Vec<TransmissionTorrent>) -> u16 {
        items
            .iter()
            .map(|torrent| torrent.peers_client_name().chars().count())
            .max()
            .unwrap_or(0) as u16
    }
}

impl Renderable<EmptyRenderableArgs> for HomeScreen {
    fn render(&mut self, frame: &mut Frame, args: EmptyRenderableArgs) {
        let torrents: Vec<TransmissionTorrent> = TransmissionService::torrent_list()
            .arguments
            .torrents;
        self.state.torrent_ids.extend(torrents.iter().map(|t| t.id));

        let title = Line::from(" All torrents ".bold());
        let mut key_bindings_block = KeyBindingsBlock::new(self.config_key_bindings.clone());
        let key_bindings = vec![
            key_bindings_block.cnf_kb_add(),
            key_bindings_block.cnf_kb_search(),
            key_bindings_block.cnf_kb_del(),
            key_bindings_block.cnf_kb_info(),
            key_bindings_block.cnf_kb_open(),
            key_bindings_block.cnf_kb_reann(),
            key_bindings_block.cnf_kb_help(),
            key_bindings_block.cnf_kb_quit()
        ];
        let bottom_line = KeyBindingsBlock::key_bindings_as_line(&key_bindings);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(bottom_line.centered())
            .padding(Padding::proportional(1))
            .border_set(border::THICK);
        let table = self.clone().table(&torrents).block(block);

        frame.render_stateful_widget(table, frame.area(), &mut self.table_state);
    }
}

impl KeyEventHandler for HomeScreen {
    fn handle_key_event(&mut self, key_event: KeyEvent, event: Event) -> bool {
        let ctrl = key_event.modifiers.contains(KeyModifiers::CONTROL);
        let shft = key_event.modifiers.contains(KeyModifiers::SHIFT);
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    self.next_row();
                    false
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.previous_row();
                    false
                }
                // TODO fix
                KeyCode::Char('l') | KeyCode::Right => {
                    self.next_column();
                    false
                }
                // TODO fix
                KeyCode::Char('h') | KeyCode::Left => {
                    self.previous_column();
                    false
                }
                KeyCode::Char('s') => {
                    if shft {
                        let cur_sel_indx = self.active_row();
                        TransmissionService::torrent_stop(self.state.torrent_ids[cur_sel_indx].to_string());
                        false
                    } else {
                        let cur_sel_indx = self.table_state.selected().unwrap();
                        TransmissionService::torrent_start(self.state.torrent_ids[cur_sel_indx].to_string());
                        false
                    }
                }
                KeyCode::Char(c) if ctrl => {
                    if c == *self.config_key_bindings.get(&KbOpen).unwrap() {
                        let cur_sel_index = self.table_state.selected().unwrap();
                        let torrent = &TransmissionService::torrent_info(self.state.torrent_ids[cur_sel_index].to_string())
                            .arguments
                            .torrents[0];
                        TransmissionService::torrent_location(&torrent);
                    }
                    false
                }
                _ => true,
            }
        } else {
            false
        }
    }
}
