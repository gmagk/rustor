use crate::config::{Config, ConfigKeyBinding};
use clap::Parser;
use crossterm::event::KeyCode::Null;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};

pub struct KeyBinding {
    config: Config,
    items: Vec<KeyBindingItem>,
}

impl KeyBinding {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            items: vec![],
        }
    }

    pub fn init(&mut self, initial_items: Vec<ConfigKeyBinding>) -> &mut Self {
        initial_items.iter().for_each(|item| {
            if ConfigKeyBinding::KbHome.eq(item) {
                let item = self.home_action();
                self.add(item);
            } else if ConfigKeyBinding::KbAdd.eq(item) {
                let item = self.add_action();
                self.add(item);
            } else if ConfigKeyBinding::KbSearch.eq(item) {
                let item = self.search_action();
                self.add(item);
            } else if ConfigKeyBinding::KbHelp.eq(item) {
                let item = self.help_action();
                self.add(item);
            } else if ConfigKeyBinding::KbQuit.eq(item) {
                let item = self.quit_action();
                self.add(item);
            }
        });
        self
    }

    pub fn add(&mut self, item: KeyBindingItem) -> &mut Self {
        self.items.push(item);
        self
    }

    pub fn items_as_line(&self) -> Line {
        let mut result: Vec<Span> = vec![Span::from(" ")];
        self.items.iter().enumerate().for_each(|item| {
            result.append(&mut item.1.as_span());
            if item.0 < self.items.len() - 1 {
                result.push(Span::from(" | "))
            }
        });
        result.push(Span::from(" "));
        Line::from(result)
    }

    pub fn cancel_action() -> KeyBindingItem {
        KeyBindingItem::new_key_code("Cancel", KeyCode::Esc)
    }

    pub fn quit_action(&mut self) -> KeyBindingItem {
        KeyBindingItem::new_ctrl_and_char("Quit", self.config.kb_quit())
    }

    pub fn home_action(&mut self) -> KeyBindingItem {
        KeyBindingItem::new_ctrl_and_char("Home", self.config.kb_home())
    }

    pub fn add_action(&mut self) -> KeyBindingItem {
        KeyBindingItem::new_ctrl_and_char("Add", self.config.kb_add())
    }

    pub fn search_action(&mut self) -> KeyBindingItem {
        KeyBindingItem::new_ctrl_and_char("Search", self.config.kb_search())
    }

    pub fn help_action(&mut self) -> KeyBindingItem {
        KeyBindingItem::new_ctrl_and_char("Help", self.config.kb_help())
    }

    pub fn action(act: &str) -> KeyBindingItem {
        KeyBindingItem::new_key_code(act, KeyCode::Enter)
    }
}

pub struct KeyBindingItem {
    action: String,
    ctrl_and_char: char, // ' ' => null
    key_code: KeyCode,
    key_modifier: KeyModifiers,
}

impl KeyBindingItem {
    pub fn new_ctrl_and_char(action: &str, ctrl_and_char: char) -> Self {
        Self {
            action: action.parse().unwrap(),
            ctrl_and_char,
            key_code: Null,
            key_modifier: KeyModifiers::NONE,
        }
    }

    pub fn new_key_code(action: &str, key_code: KeyCode) -> Self {
        Self {
            action: action.parse().unwrap(),
            ctrl_and_char: ' ',
            key_code,
            key_modifier: KeyModifiers::NONE,
        }
    }

    pub fn new_key_modifier(action: &str, key_modifier: KeyModifiers) -> Self {
        Self {
            action: action.parse().unwrap(),
            ctrl_and_char: ' ',
            key_code: Null,
            key_modifier,
        }
    }

    // return Line: "Action <Hotkey>" (e.g. "Quit <Ctrl+q>")
    pub fn as_span(&self) -> Vec<Span> {
        if self.ctrl_and_char != ' ' {
            vec![
                Span::from(&self.action),
                Span::from(format!(" <Ctrl+{}>", self.ctrl_and_char)).bold(),
            ]
        } else if self.key_code != Null {
            vec![
                Span::from(&self.action),
                Span::from(format!(" <{}>", self.key_code)).bold(),
            ]
        } else {
            vec![
                Span::from(&self.action),
                Span::from(format!(" <{}>", self.key_modifier)).bold(),
            ]
        }
    }
}
