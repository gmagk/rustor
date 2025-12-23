use std::collections::HashMap;
use crate::app::{EmptyRenderableArgs, KeyEventHandler, Renderable, RenderableArgs};
use crate::screen::key_bindings_block::{KeyBindingItem, KeyBindingsBlock};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout};
use ratatui::prelude::{Line, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;
use crate::config::{Config, ConfigKeyBindingKey};
use crate::service::torrent_service::TorrentService;
use crate::service::transmission_service::TransmissionService;

pub struct AddScreen {
    config_key_bindings: HashMap<ConfigKeyBindingKey, char>,
    input: Input
}

impl AddScreen {
    pub fn new(config_key_bindings: HashMap<ConfigKeyBindingKey, char>) -> Self {
        Self { config_key_bindings, input: Input::new(String::default()) }
    }
}

impl Renderable<EmptyRenderableArgs> for AddScreen {
    fn render(&mut self, frame: &mut Frame, args: EmptyRenderableArgs) {
        // frame
        let title = Line::from(" Add torrent ".bold());
        let mut key_bindings_block = KeyBindingsBlock::new(self.config_key_bindings.clone());
        let key_bindings = vec![
            key_bindings_block.cnf_kb_home(),
            key_bindings_block.cnf_kb_search(),
            KeyBindingsBlock::kb_cancel(),
            key_bindings_block.cnf_kb_help(),
            key_bindings_block.cnf_kb_quit()
        ];
        let bottom_line = KeyBindingsBlock::key_bindings_as_line(&key_bindings);
        let main_block = Block::bordered()
            .title(title.centered())
            .title_bottom(bottom_line.centered())
            .border_set(border::THICK);
        let main_frame = Paragraph::new("").centered().block(main_block);
        frame.render_widget(main_frame, frame.area());

        // input
        let [input_area] = Layout::horizontal([Constraint::Percentage(50)])
            .flex(Flex::Center)
            .areas(frame.area());
        let [input_area] = Layout::vertical([Constraint::Length(3)]) // keep 2 for borders and 1 for cursor
            .flex(Flex::Center)
            .areas(input_area);
        let width = input_area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let title = Line::from(vec![
            " Local filepath".bold(),
            " or ".gray(),
            "magnet link ".bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::PLAIN);
        let input_ui = Paragraph::new(self.input.value())
            .scroll((0, scroll as u16))
            .block(block);
        frame.render_widget(input_ui, input_area);
        // Ratatui hides the cursor unless it's explicitly set. Position the  cursor past the
        // end of the input text and one line down from the border to the input line
        let x = self.input.visual_cursor().max(scroll) - scroll + 1;
        frame.set_cursor_position((input_area.x + x as u16, input_area.y + 1));
    }
}

impl KeyEventHandler for AddScreen {
    /*
       Returns false if we are done from this screen
    */
    fn handle_key_event(&mut self, key_event: KeyEvent, event: Event) -> bool {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                // submit and leave
                KeyCode::Enter => {
                    TransmissionService::torrent_add(self.input.value().to_string());
                    self.input.reset();
                    false
                }
                // leave
                KeyCode::Esc => {
                    self.input.reset();
                    false
                }
                // let input handle it
                _ => {
                    self.input.handle_event(&event);
                    true
                }
            }
        } else {
            false
        }
    }
}
