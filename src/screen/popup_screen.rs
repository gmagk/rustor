use std::collections::HashMap;
use std::io::Error;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::Frame;
use ratatui::layout::Constraint::{Length, Min};
use ratatui::layout::{Constraint, Flex, Layout};
use ratatui::prelude::{Line, Stylize, Text};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Clear, Padding, Paragraph, Widget};
use crate::app::{EmptyRenderableArgs, KeyEventHandler, Renderable, RenderableArgs};
use crate::config::ConfigKeyBindingKey;
use crate::config::ConfigKeyBindingKey::KbOpen;
use crate::dto::torrent_dto::SearchTorrent;
use crate::dto::transmission_dto::TransmissionTorrent;
use crate::screen::home_screen::HomeScreen;
use crate::screen::key_bindings_block::KeyBindingsBlock;
use crate::screen::search_info_screen::SearchInfoScreenArgs;
use crate::service::transmission_service::TransmissionService;

pub struct PopupScreen {
    config_key_bindings: HashMap<ConfigKeyBindingKey, char>
}

impl PopupScreen {

    pub fn new(config_key_bindings: HashMap<ConfigKeyBindingKey, char>) -> PopupScreen {
        Self {config_key_bindings}
    }
}

pub struct PopupScreenArgs {
    message: String
}

impl PopupScreenArgs {

    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl RenderableArgs for PopupScreenArgs {}

impl Renderable<PopupScreenArgs> for PopupScreen {
    fn render(&mut self, frame: &mut Frame, args: PopupScreenArgs) {
        let area = frame.area();

        let vertical = Layout::vertical([Constraint::Percentage(60)]).flex(Flex::Center).spacing(3);
        let horizontal = Layout::horizontal([Constraint::Percentage(60)]).flex(Flex::Center).spacing(3);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);

        let title = Line::from(" Error ".bold());
        let body = Text::from(vec![Line::from(vec![args.message.into()])]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(Line::from(" [ Hit any key to close] ").centered());
        let content = Paragraph::new(body)
            .centered()
            .block(block);

        frame.render_widget(Clear, area); //this clears out the background
        frame.render_widget(content, area);
    }
}