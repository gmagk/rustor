use std::collections::HashMap;
use crate::app::{EmptyRenderableArgs, KeyEventHandler, Renderable, RenderableArgs};
use crate::config::{Config, ConfigKeyBindingKey};
use crate::screen::key_bindings_block::{KeyBindingItem, KeyBindingsBlock};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Stylize, Text, Widget};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};

#[derive(Clone)]
pub struct HelpScreen {
    config_key_bindings: HashMap<ConfigKeyBindingKey, char>
}

impl HelpScreen {
    pub fn new(config_key_bindings: HashMap<ConfigKeyBindingKey, char>) -> Self {
        Self { config_key_bindings }
    }
}

impl Renderable<EmptyRenderableArgs> for HelpScreen {
    fn render(&mut self, frame: &mut Frame, args: EmptyRenderableArgs) {
        frame.render_widget(self.clone(), frame.area());
    }
}

impl Widget for HelpScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Help ".bold());
        let mut key_bindings_block = KeyBindingsBlock::new(self.config_key_bindings.clone());
        let key_bindings = vec![
            key_bindings_block.cnf_kb_home(),
            key_bindings_block.cnf_kb_add(),
            key_bindings_block.cnf_kb_search(),
            KeyBindingsBlock::kb_cancel(),
            key_bindings_block.cnf_kb_quit()
        ];
        let bottom_line = KeyBindingsBlock::key_bindings_as_line(&key_bindings);
        let body = Text::from(vec![Line::from(vec![
            "All available keybindings are shown on the bottom of each screen.".into(),
        ])]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(bottom_line.centered())
            .border_set(border::THICK);
        Paragraph::new(body)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl KeyEventHandler for HelpScreen {
    fn handle_key_event(&mut self, key_event: KeyEvent, event: Event) -> bool {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                // KeyCode::Char('e') if key_event.modifiers.contains(KeyModifiers::CONTROL) => self.input.active = !self.input.active,
                // do not leave (maybe it will change in the future)
                _ => false,
            }
        } else {
            false
        }
    }
}
