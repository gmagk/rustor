use crate::dto::Torrent;
use chrono::{DateTime, Local, Utc};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::Constraint;
use ratatui::prelude::{Modifier, Style, Stylize, Text};
use ratatui::style::Color;
use ratatui::widgets::{Cell, Row, Table, TableState};
use ratatui::Frame;
use std::fmt::Debug;
use std::time::{Duration, UNIX_EPOCH};
use ratatui::text::{Line, StyledGrapheme};
use crate::app::{KeyEventHandler, Screen};
use crate::service::Service;
use crate::util::Util;

#[derive(Default, Clone)]
struct State {
    total_items: usize
}

#[derive(Clone)]
pub struct HomeScreen {
    table_state: TableState,
    state: State,
    service: Service
}

impl HomeScreen {
    pub fn new(service: Service) -> Self {
        Self { table_state: TableState::default().with_selected(0), state: State::default(), service }
    }

    pub fn next_row(&mut self) {
        let max_index = self.state.total_items as i32 - 1;
        let i = match self.table_state.selected() {
            Some(i) => { if i as i32 == max_index { i } else { i + 1 } }
            None => 0,
        };
        self.table_state.select(Some(i));
        // self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => { if i > 0 { i - 1 } else { i } }
            None => 0,
        };
        self.table_state.select(Some(i));
        // self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn render(&mut self, frame: &mut Frame, torrents: &Vec<Torrent>) {
        self.state.total_items = torrents.len();
        frame.render_stateful_widget(self.clone().table(torrents), frame.area(), &mut self.table_state);
    }

    pub fn active_row(&self) -> usize {
        self.table_state.selected().unwrap_or(0)
    }

    fn table(self, torrents: &Vec<Torrent>) -> Table<'static> {
        let rows = torrents
            .iter()
            .enumerate()
            .map(|(i, data)| {
                let item = [
                    &data.id.to_string(),
                    &data.name,
                    &self.eta(data),
                    &self.percentage_done(data).to_string(),
                    &self.download_rate(data).to_string(),
                    &self.upload_rate(data).to_string(),
                    &self.total_size(data),
                    &self.downloaded(data),
                    &self.added_on(data),
                    &self.peers_client_name(data)
                ];
                // TODO show tor error
                // TODO show done status
                item.into_iter()
                    .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                    .collect::<Row>()
                    .height(3)
            });
        let header = ["Id", "Name", "ETA", "Done", "Download", "Upload", "Size", "Downloaded", "Added On", "Peers"]
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
                Constraint::Min(self.peer_client_name_len(&torrents) + 1),
            ],
        ).header(header)
        .row_highlight_style(selected_row_style)
    }

    fn eta(&self, torrent: &Torrent) -> String {
        if torrent.is_finished {
            return "Done".to_string();
        }

        if torrent.eta <= 0 {
            return "Unknown".to_string();
        }

        let seconds = torrent.eta % 60;
        let minutes = (torrent.eta /60) % 60;
        let hours = (torrent.eta / 60 / 60) % 60;
        let days = (torrent.eta / 60 / 60 / 24) % 24;

        let time = format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds);
        if days > 0 {
            format!("{days} days {time}")
        } else {
            time
        }
    }

    fn percentage_done(&self, torrent: &Torrent) -> String {
        if torrent.left_until_done == 0 {
            return "100 %".to_string()
        }

        let left_undone: f64 = torrent.left_until_done as f64;
        let total_size: f64 = torrent.size_when_done as f64;
        let res = (100f64 - 100f64*left_undone/total_size) % 100f64;

        format!("{:.2} %", res)
    }

    fn download_rate(&self, torrent: &Torrent) -> String {
        format!("\u{2193} {} kB/s", (torrent.rate_download / 1000) % 1000)
    }

    fn upload_rate(&self, torrent: &Torrent) -> String {
        format!("\u{2191} {} kB/s", (torrent.rate_upload / 1000) % 1000)
    }

    fn total_size(&self, torrent: &Torrent) -> String {
        if torrent.size_when_done <= 0 {
            return "".to_string();
        }

        Util::print_bytes(torrent.size_when_done as f64)
    }

    fn downloaded(&self, torrent: &Torrent) -> String {
        Util::print_bytes((torrent.size_when_done - torrent.left_until_done) as f64)
    }

    fn added_on(&self, torrent: &Torrent) -> String {
        let d = UNIX_EPOCH + Duration::from_secs(torrent.added_date as u64);
        let datetime = DateTime::<Utc>::from(d).with_timezone(&Local);
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    // Find for name column which row has the largest (this is done only for string values which might be too long)
    fn name_len(&self, items: &Vec<Torrent>) -> u16 {
        items
            .iter()
            .map(|t| t.name.chars().count())
            .max()
            .unwrap_or(0) as u16
    }


    // Find for combined peers.client_name column which row has the largest (this is done only for string values which might be too long)
    fn peer_client_name_len(&self, items: &Vec<Torrent>) -> u16 {
        items
            .iter()
            .map(|t| { self.peers_client_name(t).chars().count() })
            .max()
            .unwrap_or(0) as u16
    }

    fn peers_client_name(&self, torrent: &Torrent) -> String {
        if torrent.peers.len() == 0 {
            return String::default();
        }

        let mut res = torrent.peers
            .iter()
            .map(|peer| peer.client_name.clone())
            .reduce(|mut accumulator, s| {
                accumulator.push_str(&s);
                accumulator.push(',');
                accumulator.push(' ');
                accumulator
            }).unwrap();
        res.pop();
        res.pop();
        res
    }
}

impl KeyEventHandler for HomeScreen {
    fn handle_key_event(&mut self, key_event: KeyEvent, event: Event) -> bool {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    self.next_row();
                    false
                },
                KeyCode::Char('k') | KeyCode::Up => {
                    self.previous_row();
                    false
                },
                KeyCode::Char('g') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.service.torrent_start(self.table_state.selected().unwrap().to_string());
                    false
                },
                KeyCode::Char('s') if key_event.modifiers.contains(KeyModifiers::CONTROL) && key_event.modifiers.contains(KeyModifiers::SHIFT) => {
                    self.service.torrent_stop(self.table_state.selected().unwrap().to_string());
                    false
                },
                _ => { true }
            }
        } else {
            false
        }
    }
}