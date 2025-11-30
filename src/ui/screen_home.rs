use crate::dto::Torrent;
use chrono::{DateTime, Local, Utc};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::Constraint;
use ratatui::prelude::{Modifier, Style, Stylize, Text};
use ratatui::style::Color;
use ratatui::widgets::{Block, Cell, Padding, Paragraph, Row, Table, TableState};
use ratatui::Frame;
use std::fmt::Debug;
use std::time::{Duration, UNIX_EPOCH};
use ratatui::symbols::border;
use ratatui::text::{Line, StyledGrapheme};
use crate::app::{KeyEventHandler, Renderable, Screen};
use crate::mapper::Mapper;
use crate::service::Service;
use crate::ui::view::view_key_bindings::{KeyBindingView, KeyBindingItemView};
use crate::ui::view::view_torrent::TorrentView;
use crate::util::Util;

#[derive(Default, Clone)]
struct State {
    torrent_ids: Vec<i64>
}

#[derive(Clone)]
pub struct HomeScreen {
    table_state: TableState,
    state: State,
    service: Service,
    mapper: Mapper
}

impl HomeScreen {
    pub fn new(service: Service, mapper: Mapper) -> Self {
        Self { table_state: TableState::default().with_selected(0), state: State::default(), service, mapper }
    }

    pub fn next_row(&mut self) {
        let max_index = self.state.torrent_ids.len() as i32 - 1;
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

    fn table(self, torrents: &Vec<Torrent>) -> Table<'static> {
        let rows = torrents
            .iter()
            .enumerate()
            .map(|(i, data)| {
                let item = [
                    &data.id.to_string(),
                    &data.name,
                    &TorrentView::eta(data),
                    &TorrentView::percentage_done(data).to_string(),
                    &TorrentView::download_rate(data).to_string(),
                    &TorrentView::upload_rate(data).to_string(),
                    &TorrentView::total_size(data),
                    &TorrentView::downloaded(data),
                    &TorrentView::added_on(data),
                    &TorrentView::peers_client_name(data)
                ];
                // TODO show tor error
                // TODO show done status
                item.into_iter()
                    .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                    .collect::<Row>()
                    .height(3)
            });
        let header = ["Id", "Name", "ETA", "Done", "Download", "Upload", "Size", "Downloaded", "Added On"]
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
                Constraint::Length(20)
            ],
        ).header(header)
        .row_highlight_style(selected_row_style)
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
            .map(|t| { TorrentView::peers_client_name(t).chars().count() })
            .max()
            .unwrap_or(0) as u16
    }
}

impl Renderable for HomeScreen {
    fn render(&mut self, frame: &mut Frame, args: Vec<usize>) {
        let torrents: Vec<Torrent> = self.mapper.json_to_response(self.service.torrent_list()).arguments.torrents;
        self.state.torrent_ids.extend(torrents.iter().map(|t| t.id));

        let title = Line::from(" All torrents ".bold());
        let mut key_bindings = KeyBindingView::default();
        key_bindings
            .add(KeyBindingItemView::new_ctrl_and_char("Add", 'a'))
            .add(KeyBindingItemView::new_ctrl_and_char("Remove", 'd'))
            .add(KeyBindingItemView::new_ctrl_and_char("Torrent", 't'))
            .add(KeyBindingItemView::new_ctrl_and_char("Reannounce", 'r'))
            .add(KeyBindingView::quit());
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(key_bindings.items_as_line().centered())
            .padding(Padding::proportional(1))
            .border_set(border::THICK);
        let table = self.clone().table(&torrents)
            .block(block);


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
                },
                KeyCode::Char('k') | KeyCode::Up => {
                    self.previous_row();
                    false
                },
                // TODO fix
                KeyCode::Char('l') | KeyCode::Right => {
                    self.next_column();
                    false
                },
                // TODO fix
                KeyCode::Char('h') | KeyCode::Left => {
                    self.previous_column();
                    false
                },
                KeyCode::Char('s') => {
                    if shft {
                        let cur_sel_indx = self.table_state.selected().unwrap();
                        self.service.torrent_stop(self.state.torrent_ids[cur_sel_indx].to_string());
                        false
                    } else {
                        let cur_sel_indx = self.table_state.selected().unwrap();
                        self.service.torrent_start(self.state.torrent_ids[cur_sel_indx].to_string());
                        false
                    }
                },
                KeyCode::Char('o') => {
                    let cur_sel_index = self.table_state.selected().unwrap();
                    let torrent_info = self.service.torrent_info(self.state.torrent_ids[cur_sel_index].to_string());
                    let torrent: &Torrent = &self.mapper.json_to_response(torrent_info).arguments.torrents[0];
                    self.service.torrent_location(&torrent);
                    false
                },
                _ => { true }
            }
        } else {
            false
        }
    }
}