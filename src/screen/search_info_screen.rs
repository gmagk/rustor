use std::ops::Add;
use std::sync::Arc;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{symbols, Frame};
use ratatui::layout::{Layout, Rect, Size};
use ratatui::layout::Constraint::{Length, Min};
use ratatui::prelude::{Color, Span, Style};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Block, HighlightSpacing, LineGauge, List, ListItem, Padding, Paragraph, ScrollbarState, Widget};
use tui_scrollview::{ScrollView, ScrollViewState};
use crate::app::{KeyEventHandler, Renderable, RenderableArgs};
use crate::config::{Config, ConfigKeyBinding};
use crate::dto::torrent_dto::{PirateBayTorrentFile, SearchTorrent, TorrentSource};
use crate::dto::transmission_dto::TransmissionTorrent;
use crate::key_bindings::KeyBinding;
use crate::mapper::Mapper;
use crate::screen::info_screen::{InfoScreen, InfoScreenArgs};
use crate::service::torrent_service::TorrentService;
use crate::service::transmission_service::TransmissionService;
use crate::util::Util;

pub struct SearchInfoScreen {
    config: Config,
    selected_row_torrent: SearchTorrent,
    vertical_scroll_state: ScrollbarState,
    scroll_view_state: ScrollViewState,
    vertical_scroll: usize
}

impl SearchInfoScreen {

    pub fn new(config: Config) -> Self {
        Self {
            config,
            selected_row_torrent: SearchTorrent::default(),
            vertical_scroll_state: ScrollbarState::default(),
            scroll_view_state: ScrollViewState::default(),
            vertical_scroll: 0
        }
    }
}

pub struct SearchInfoScreenArgs {
    selected_torrent: SearchTorrent
}

impl SearchInfoScreenArgs {

    pub fn new(selected_torrent: SearchTorrent) -> Self {
        Self { selected_torrent }
    }

    pub fn get_selected_torrent(&self) -> &SearchTorrent {
        &self.selected_torrent
    }
}

impl RenderableArgs for SearchInfoScreenArgs {}

impl Renderable<SearchInfoScreenArgs> for SearchInfoScreen {
    fn render(&mut self, frame: &mut Frame, args: SearchInfoScreenArgs) {
        self.selected_row_torrent = args.get_selected_torrent().clone();
        let scroll_view_height = 30;
        let buf = frame.buffer_mut();

        let width = if buf.area.height < scroll_view_height {
            buf.area.width - 1
        } else {
            buf.area.width
        };
        let mut scroll_view = ScrollView::new(Size::new(width, scroll_view_height));
        let scroll_view_buf = scroll_view.buf_mut();

        let mut bottom_area: Rect = Rect::default();
        let info_block = Block::bordered()
            .title(Line::from(
                Span::from(" ")
                    .add(Span::from(self.selected_row_torrent.name.clone()).bold().underlined())
                    .add(Span::from(" ")),
            ))
            .padding(Padding::uniform(1));

        if self.selected_row_torrent.source == TorrentSource::PirateBay {
            let [info_area, files_area, bottom_area_] =
                Layout::vertical([Min(1), Min(1), Length(1)])
                    .spacing(1)
                    .vertical_margin(1)
                    .horizontal_margin(1)
                    .areas(scroll_view_buf.area);
            bottom_area = bottom_area_;

            // info
            let torrent = self.selected_row_torrent.clone();
            let info = vec![
                Line::from(
                    "Size: "
                        .to_string()
                        .add(Util::print_bytes(torrent.size as f64).to_string().as_str()),
                ),
                Line::from(
                    "Added on: "
                        .to_string()
                        .add(Util::print_epoch(torrent.created_on as u64).as_str()),
                ),
                Line::from(
                    "Seeders: "
                        .to_string()
                        .add(torrent.seeders.to_string().as_str()),
                ),
                Line::from(
                    "Leechers: "
                        .to_string()
                        .add(torrent.leechers.to_string().as_str()),
                ),
                Line::from(
                    "Source: "
                        .to_string()
                        .add(torrent.source.to_string().as_str()),
                ),
                Line::from(
                    "Info Hash: "
                        .to_string()
                        .add(torrent.info_hash.to_string().as_str()),
                )
            ];
            Paragraph::new(info)
                .block(info_block)
                .render(info_area, scroll_view_buf);

            // files
            let files_block = Block::bordered()
                .title(" Files ")
                .padding(Padding::uniform(1));
            let mut list_items: Vec<ListItem> = vec![];
            torrent.files.iter().enumerate().for_each(|(i, file)| {
                list_items.push(ListItem::from(""));
                if !file.name.is_empty() {
                    list_items.push(ListItem::from(file.name.to_string()).bold());
                }
                if !file.size > 0 {
                    list_items.push(ListItem::from(Util::print_bytes(file.size as f64)));
                }
            });
            List::new(list_items)
                .block(files_block)
                .render(files_area, scroll_view_buf);
        } else {
            let [info_area, bottom_area_] =
                Layout::vertical([Min(1), Length(1)])
                    .spacing(1)
                    .vertical_margin(1)
                    .horizontal_margin(1)
                    .areas(scroll_view_buf.area);
            bottom_area = bottom_area_;

            Paragraph::new("Sorry, torrent info is only provided from the PirateBay source!")
                .centered()
                .block(info_block)
                .render(info_area, scroll_view_buf);
        }

        // bottom
        let mut key_bindings = KeyBinding::new(self.config.clone());
        key_bindings.init(vec![
                ConfigKeyBinding::KbHome,
                ConfigKeyBinding::KbAdd,
                ConfigKeyBinding::KbSearch,
                ConfigKeyBinding::KbHelp,
                ConfigKeyBinding::KbQuit,
            ]).add(KeyBinding::cancel_action());
        Line::from(key_bindings.items_as_line())
            .centered()
            .render(bottom_area, scroll_view_buf);

        frame.render_stateful_widget(scroll_view, frame.area(), &mut self.scroll_view_state);
    }
}

impl KeyEventHandler for SearchInfoScreen {
    fn handle_key_event(&mut self, key_event: KeyEvent, event: Event) -> bool {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    self.scroll_view_state.scroll_down();
                    true
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.scroll_view_state.scroll_up();
                    true
                }
                // leave
                KeyCode::Esc => false,
                // do not leave (maybe it will change in the future)
                _ => true,
            }
        } else {
            false
        }
    }
}