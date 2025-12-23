use std::collections::HashMap;
use crate::app::{KeyEventHandler, Renderable, RenderableArgs};
use crate::config::{Config, ConfigKeyBindingKey};
use crate::screen::key_bindings_block::{KeyBindingItem, KeyBindingsBlock};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Stylize, Text, Widget};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};
use crate::dto::transmission_dto::TransmissionTorrent;
use crate::service::transmission_service::TransmissionService;
use crate::screen::info_screen::InfoScreen;

#[derive(Clone)]
pub struct ReannScreen {
    config_key_bindings: HashMap<ConfigKeyBindingKey, char>,
    selected_row_index: usize,
}

impl ReannScreen {
    pub fn new(config_key_bindings: HashMap<ConfigKeyBindingKey, char>) -> Self {
        Self { config_key_bindings, selected_row_index: 0 }
    }
}

pub struct ReannScreenArgs {
    selected_row_index: usize
}

impl ReannScreenArgs {
    pub fn new(selected_row_index: usize) -> Self {
        Self { selected_row_index }
    }

    pub fn get_selected_row_index(&self) -> usize {
        self.selected_row_index
    }
}

impl RenderableArgs for ReannScreenArgs {}

impl Renderable<ReannScreenArgs> for ReannScreen {
    fn render(&mut self, frame: &mut Frame, args: ReannScreenArgs) {
        self.selected_row_index = args.get_selected_row_index();
        frame.render_widget(self.clone(), frame.area());
    }
}

impl Widget for ReannScreen {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let torrents: Vec<TransmissionTorrent> = TransmissionService::torrent_list()
            .arguments
            .torrents;
        let info = torrents.get(self.selected_row_index).unwrap();

        let title = Line::from(" Reannounce torrent ".bold());
        let mut key_bindings_block = KeyBindingsBlock::new(self.config_key_bindings.clone());
        let key_bindings = vec![
            key_bindings_block.cnf_kb_home(),
            key_bindings_block.cnf_kb_add(),
            key_bindings_block.cnf_kb_search(),
            KeyBindingsBlock::kb_cancel(),
            key_bindings_block.cnf_kb_help(),
            key_bindings_block.cnf_kb_quit()
        ];
        let bottom_line = KeyBindingsBlock::key_bindings_as_line(&key_bindings);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(bottom_line.centered())
            .border_set(border::THICK);
        let body = Text::from(vec![
            Line::from(vec!["".into()]),
            Line::from(vec![info.name.clone().into()]),
            Line::from(vec!["".into()]),
        ]);
        Paragraph::new(body)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl KeyEventHandler for ReannScreen {
    fn handle_key_event(&mut self, key_event: KeyEvent, event: Event) -> bool {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                // submit and leave
                KeyCode::Enter => {
                    let torrents: Vec<TransmissionTorrent> = TransmissionService::torrent_list()
                        .arguments
                        .torrents;
                    TransmissionService::torrent_reannounce(torrents[self.selected_row_index].id.to_string());
                    false
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
