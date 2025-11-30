use crate::app::{KeyEventHandler, Renderable};
use crate::dto::Torrent;
use crate::mapper::Mapper;
use crate::service::Service;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Stylize, Text, Widget};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;
use crate::config::{Config, ConfigKeyBinding};
use crate::ui::view::view_key_bindings::KeyBindingView;

#[derive(Clone, Copy)]
pub struct RmScreen {
    config: Config,
    service: Service,
    mapper: Mapper,
    selected_row_index: usize
}

impl RmScreen {

    pub fn new(config: Config, service: Service, mapper: Mapper) -> Self {
        Self { config, service, mapper, selected_row_index: 0 }
    }
}

impl Renderable for RmScreen {
    fn render(&mut self, frame: &mut Frame, args: Vec<usize>) {
        self.selected_row_index = args[0];
        frame.render_widget(*self, frame.area());
    }
}

impl Widget for RmScreen {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        let torrents: Vec<Torrent> = self.mapper.json_to_response(self.service.torrent_list()).arguments.torrents;
        let info = torrents.get(self.selected_row_index).unwrap();

        let title = Line::from(" Remove torrent ".bold());
        let mut key_bindings = KeyBindingView::new(self.config.clone());
        key_bindings
            .init(vec![ConfigKeyBinding::KbHome, ConfigKeyBinding::KbHelp, ConfigKeyBinding::KbQuit])
            .add(KeyBindingView::action("Remove"))
            .add(KeyBindingView::cancel());
        let body = Text::from(vec![
            Line::from(vec!["".into()]),
            Line::from(vec![info.name.clone().into()]),
            Line::from(vec!["".into()])
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(key_bindings.items_as_line().centered())
            .border_set(border::THICK);
        Paragraph::new(body)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl KeyEventHandler for RmScreen {
    fn handle_key_event(&mut self, key_event: KeyEvent, event: Event) -> bool {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                // submit and leave
                KeyCode::Enter => {
                    let torrents: Vec<Torrent> = self.mapper.json_to_response(self.service.torrent_list()).arguments.torrents;
                    self.service.torrent_remove(torrents[self.selected_row_index].id.to_string());
                    false
                },
                // leave
                KeyCode::Esc => {
                    false
                },
                // do not leave (maybe it will change in the future)
                _ => { true }
            }
        } else {
            false
        }
    }
}