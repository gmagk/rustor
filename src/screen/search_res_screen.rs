use std::collections::HashMap;
use std::sync::Arc;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout};
use ratatui::prelude::{Color, Modifier, Style, Text};
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::text::Line;
use ratatui::widgets::{Block, Cell, Padding, Paragraph, Row, Table, TableState};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;
use crate::app::{KeyEventHandler, Renderable, RenderableArgs, Screen};
use crate::config::{Config, ConfigKeyBindingKey};
use crate::config::ConfigKeyBindingKey::{KbDownload, KbHome};
use crate::dto::torrent_dto::{PirateBayInfoTorrent, PirateBayTorrentFile, SearchTorrent, TorrentSource};
use crate::dto::transmission_dto::TransmissionTorrent;
use crate::service::torrent_service::TorrentService;
use crate::service::transmission_service::TransmissionService;
use crate::screen::rm_screen::RmScreen;
use crate::screen::key_bindings_block::{KeyBindingItem, KeyBindingsBlock};
use crate::mapper::Mapper;
use crate::util::Util;

#[derive(Default, Clone)]
struct State {
    torrents: Vec<SearchTorrent>,
    row_index_last_used_for_fetching_torrent: usize,
    selected_row_torrent: SearchTorrent
}

#[derive(Clone)]
pub struct SearchResScreen {
    config_key_bindings: HashMap<ConfigKeyBindingKey, char>,
    torrent_service_arc: Arc<TorrentService>,
    table_state: TableState,
    state: State
}

impl SearchResScreen {

    pub fn new(config_key_bindings: HashMap<ConfigKeyBindingKey, char>, torrent_service_arc: Arc<TorrentService>) -> Self {
        Self {
            config_key_bindings,
            torrent_service_arc,
            table_state: TableState::default().with_selected(0),
            state: State::default()
        }
    }

    pub fn next_row(&mut self) {
        let max_index = self.state.torrents.len() as i32 - 1;
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

    pub fn active_row_torrent(&mut self) -> SearchTorrent {
        let cur_sel_id = self.table_state.selected().unwrap_or(0);
        if  !self.state.selected_row_torrent.name.is_empty() && // We have to check the `name` because in case of TorrentsCsv we do not get any info because they do not have an API for this.
            self.state.row_index_last_used_for_fetching_torrent == cur_sel_id {
            return self.state.selected_row_torrent.clone()
        }

        let cur_sel_index = self.table_state.selected().unwrap_or(0);
        self.state.row_index_last_used_for_fetching_torrent = cur_sel_index.clone();
        let active_torrent = self.state.torrents[cur_sel_index].clone();
        let active_torrent_id = active_torrent.id.parse().unwrap();

        self.state.selected_row_torrent = match active_torrent.source {

            // Get extra info from PirateBay
            TorrentSource::PirateBay => {
                Mapper::pirate_bay_torrent_info_and_files_result_to_search_torrent(
                    &self.torrent_service_arc.torrent_info_pirate_bay(active_torrent_id),
                    &self.torrent_service_arc.torrent_files_pirate_bay(active_torrent_id)
                )
            }
            _ => active_torrent
        };

        self.state.selected_row_torrent.clone()
    }

    fn table(self, torrents: &Vec<SearchTorrent>) -> Table<'static> {
        let rows = torrents.iter().enumerate().map(|(i, torrent)| {
            let item = [
                &torrent.id,
                &torrent.name,
                &torrent.seeders.to_string(),
                &torrent.leechers.to_string(),
                &Util::print_epoch(torrent.created_on as u64),
                &Util::print_bytes(torrent.size as f64),
                &torrent.source.to_string(),
                &torrent.info_hash
            ];
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Row>()
                .height(3)
        });
        let header = [
                "Id",
                "Name",
                "Seeders",
                "Leechers",
                "Created On",
                "Size",
                "Source",
                "Info Hash"
            ].into_iter()
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
                Constraint::Length(8),
                Constraint::Length(self.name_len(&torrents) + 1),
                Constraint::Length(8),
                Constraint::Length(8),
                Constraint::Length(20),
                Constraint::Length(10),
                Constraint::Length(16),
                Constraint::Length(41)
            ],
        ).header(header)
        .row_highlight_style(selected_row_style)
    }

    // Find for name column which row has the largest (this is done only for string values which might be too long)
    fn name_len(&self, items: &Vec<SearchTorrent>) -> u16 {
        items
            .iter()
            .map(|t| t.name.chars().count())
            .max()
            .unwrap_or(0) as u16
    }

    fn download(&mut self) {
        TransmissionService::torrent_add(format!("magnet:?xt=urn:btih:{}", self.active_row_torrent().info_hash.as_str()));
    }
}

pub struct SearchResArgs {
    torrents: Vec<SearchTorrent>
}

impl SearchResArgs {
    pub fn new(torrents: Vec<SearchTorrent>) -> Self {
        Self { torrents }
    }

    pub fn get_torrents(&self) -> Vec<SearchTorrent> {
        self.torrents.clone()
    }
}

impl RenderableArgs for SearchResArgs {}

impl Renderable<SearchResArgs> for SearchResScreen {
    fn render(&mut self, frame: &mut Frame, args: SearchResArgs) {
        self.state.torrents = args.get_torrents().clone();

        let title = Line::from(" Search results ".bold());
        let mut key_bindings_block = KeyBindingsBlock::new(self.config_key_bindings.clone());
        let key_bindings = vec![
            key_bindings_block.cnf_kb_home(),
            key_bindings_block.cnf_kb_add(),
            key_bindings_block.cnf_kb_search(),
            key_bindings_block.cnf_kb_info(),
            key_bindings_block.cnf_kb_download(),
            key_bindings_block.cnf_kb_help(),
            key_bindings_block.cnf_kb_quit()
        ];
        let bottom_line = KeyBindingsBlock::key_bindings_as_line(&key_bindings);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(bottom_line.centered())
            .padding(Padding::proportional(1))
            .border_set(border::THICK);
        let table = self.clone().table(&self.state.torrents).block(block);

        frame.render_stateful_widget(table, frame.area(), &mut self.table_state);
    }
}

impl KeyEventHandler for SearchResScreen {

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
                KeyCode::Char(c) if ctrl => {
                    if c == *self.config_key_bindings.get(&KbDownload).unwrap() {
                        self.download();
                        false
                    } else {
                        true
                    }
                },
                _ => true,
            }
        } else {
            false
        }
    }
}