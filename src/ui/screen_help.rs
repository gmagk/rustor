use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Stylize, Text, Widget};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;
use crate::app::KeyEventHandler;

#[derive(Default, Clone, Copy)]
pub struct HelpScreen {}

impl Widget for HelpScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Test App ".bold());
        let title_bottom = Line::from(vec![
            " Quit ".into(),
            "[Q] ".blue().bold()
        ]);
        let body = Text::from(vec![
            Line::from(vec![
                "Help Screen".into()
            ])
        ]);
        let main_block = Block::bordered()
            .title(title.centered())
            .title_bottom(title_bottom.centered())
            .border_set(border::THICK);
        Paragraph::new(body)
            .centered()
            .block(main_block)
            .render(area, buf);
    }
}

impl KeyEventHandler for HelpScreen {
    fn handle_key_event(&mut self, key_event: KeyEvent, event:Event) -> bool {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                // KeyCode::Char('e') if key_event.modifiers.contains(KeyModifiers::CONTROL) => self.input.active = !self.input.active,
                // do not leave (maybe it will change in the future)
                _ => { false }
            }
        } else if key_event.kind == KeyEventKind::Repeat {
            // TODO handle for command input
            true
        } else {
            false
        }
    }
}