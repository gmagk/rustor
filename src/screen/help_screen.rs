use crate::app::{EmptyRenderableArgs, KeyEventHandler, Renderable, RenderableArgs};
use crate::config::{Config, ConfigKeyBinding};
use crate::key_bindings::{KeyBindingItem, KeyBinding};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Stylize, Text, Widget};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};

#[derive(Clone, Copy)]
pub struct HelpScreen {
    config: Config,
}

impl HelpScreen {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Renderable<EmptyRenderableArgs> for HelpScreen {
    fn render(&mut self, frame: &mut Frame, args: EmptyRenderableArgs) {
        frame.render_widget(*self, frame.area());
    }
}

impl Widget for HelpScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Help ".bold());
        let mut key_bindings = KeyBinding::new(self.config.clone());
        key_bindings.init(vec![
            ConfigKeyBinding::KbHome,
            ConfigKeyBinding::KbAdd,
            ConfigKeyBinding::KbSearch,
            ConfigKeyBinding::KbQuit
        ]);
        let body = Text::from(vec![Line::from(vec![
            "All available keybindings are shown on the bottom of each screen.".into(),
        ])]);
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
    fn handle_key_event(&mut self, key_event: KeyEvent, event: Event) -> bool {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                // KeyCode::Char('e') if key_event.modifiers.contains(KeyModifiers::CONTROL) => self.input.active = !self.input.active,
                // do not leave (maybe it will change in the future)
                _ => false,
            }
        } else if key_event.kind == KeyEventKind::Repeat {
            // TODO handle for command input
            true
        } else {
            false
        }
    }
}
