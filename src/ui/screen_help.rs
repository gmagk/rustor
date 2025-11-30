use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Stylize, Text, Widget};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;
use crate::app::{KeyEventHandler, Renderable};
use crate::ui::view::view_key_bindings::KeyBindingView;

#[derive(Default, Clone, Copy)]
pub struct HelpScreen {}

impl Renderable for HelpScreen {
    fn render(&mut self, frame: &mut Frame, args: Vec<usize>) {
        frame.render_widget(*self, frame.area());
    }
}

impl Widget for HelpScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Help ".bold());
        let mut key_bindings = KeyBindingView::default();
        key_bindings
            .add(KeyBindingView::home())
            .add(KeyBindingView::quit());
        let body = Text::from(vec![
            Line::from(vec![
                "All available keybindings are shown on the bottom of each screen.".into()
            ])
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