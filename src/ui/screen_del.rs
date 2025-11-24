use crate::app::KeyEventHandler;
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

#[derive(Default, Clone, Copy)]
pub struct DelScreen {
    service: Service,
    mapper: Mapper,
    selected_row_index: usize
}

impl DelScreen {

    pub fn render(&mut self, frame: &mut Frame, index: usize) {
        self.selected_row_index = index;
        frame.render_widget(*self, frame.area());
    }
}

impl Widget for DelScreen {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        let torrents: Vec<Torrent> = self.mapper.map_to_response(self.service.torrent_list()).arguments.torrents;
        let info = torrents.get(self.selected_row_index).unwrap();

        let title = Line::from(" Remove torrent ".bold());
        let title_bottom = Line::from(vec![
            " Remove ".into(),
            "[Enter]".gray().bold(),
            " | ".gray(),
            "Cancel ".into(),
            "[Esc] ".gray().bold()
        ]);
        let body = Text::from(vec![
            Line::from(vec!["".into()]),
            Line::from(vec![info.name.clone().into()]),
            Line::from(vec!["".into()])
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(title_bottom.centered())
            .border_set(border::THICK);
        Paragraph::new(body)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl KeyEventHandler for DelScreen {
    fn handle_key_event(&mut self, key_event: KeyEvent, event: Event) -> bool {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                // submit and leave
                KeyCode::Enter => {
                    let torrents: Vec<Torrent> = self.mapper.map_to_response(self.service.torrent_list()).arguments.torrents;
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