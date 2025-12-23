use std::collections::HashMap;
use crate::config::{Config, ConfigKeyBindingKey};
use clap::Parser;
use crossterm::event::KeyCode::Null;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::style::Stylize;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::Paragraph;

#[derive(Clone)]
pub struct KeyBindingItem {
    action: String,
    ctrl_and_char: char, // ' ' => null
    key_code: KeyCode
}

impl KeyBindingItem {

    pub fn new_ctrl_and_char(action: &str, ctrl_and_char: char) -> Self {
        Self {
            action: action.parse().unwrap(),
            ctrl_and_char,
            key_code: Null
        }
    }

    pub fn new_key_code(action: &str, key_code: KeyCode) -> Self {
        Self {
            action: action.parse().unwrap(),
            ctrl_and_char: ' ',
            key_code
        }
    }

    // return Line: "<action> <<key>>" (e.g. "Quit <Ctrl+q>")
    pub fn as_span(&self) -> Span {
        if self.ctrl_and_char != ' ' {
            Span::from(format!("{} <Ctrl+{}>", self.action.clone(), self.ctrl_and_char)).bold()
        } else if self.key_code != Null {
            Span::from(format!("{} <{}>", self.action.clone(), self.key_code)).bold()
        } else {
            Span::from("")
        }
    }
}

pub struct KeyBindingsBlock {
    config_key_bindings: HashMap<ConfigKeyBindingKey, char>
}

impl KeyBindingsBlock {

    pub fn new(config_key_bindings: HashMap<ConfigKeyBindingKey, char>) -> Self {
        Self { config_key_bindings }
    }

    pub fn key_bindings_as_line(items: &Vec<KeyBindingItem>) -> Line<> {
        let mut result: Vec<Span> = vec![Span::from(" ")];
        items.iter().enumerate().for_each(| (index, item) | {
            result.push(item.as_span());
            if index < items.len() - 1 {
                result.push(Span::from(" | "))
            }
        });
        result.push(Span::from(" "));
        Line::from(result)
    }

    pub fn cnf_kb_add(&mut self) -> KeyBindingItem {
        KeyBindingItem::new_ctrl_and_char("Add", *self.config_key_bindings.get(&ConfigKeyBindingKey::KbAdd).unwrap())
    }

    pub fn cnf_kb_del(&mut self) -> KeyBindingItem {
        KeyBindingItem::new_ctrl_and_char("Del", *self.config_key_bindings.get(&ConfigKeyBindingKey::KbDel).unwrap())
    }

    pub fn cnf_kb_download(&mut self) -> KeyBindingItem {
        KeyBindingItem::new_ctrl_and_char("Download", *self.config_key_bindings.get(&ConfigKeyBindingKey::KbDownload).unwrap())
    }

    pub fn cnf_kb_info(&mut self) -> KeyBindingItem {
        KeyBindingItem::new_ctrl_and_char("Info", *self.config_key_bindings.get(&ConfigKeyBindingKey::KbInfo).unwrap())
    }

    pub fn cnf_kb_help(&mut self) -> KeyBindingItem {
        KeyBindingItem::new_ctrl_and_char("Help", *self.config_key_bindings.get(&ConfigKeyBindingKey::KbHelp).unwrap())
    }

    pub fn cnf_kb_home(&mut self) -> KeyBindingItem {
        KeyBindingItem::new_ctrl_and_char("Home", *self.config_key_bindings.get(&ConfigKeyBindingKey::KbHome).unwrap())
    }

    pub fn cnf_kb_open(&mut self) -> KeyBindingItem {
        KeyBindingItem::new_ctrl_and_char("Open", *self.config_key_bindings.get(&ConfigKeyBindingKey::KbOpen).unwrap())
    }

    pub fn cnf_kb_quit(&mut self) -> KeyBindingItem {
        KeyBindingItem::new_ctrl_and_char("Quit", *self.config_key_bindings.get(&ConfigKeyBindingKey::KbQuit).unwrap())
    }

    pub fn cnf_kb_reann(&mut self) -> KeyBindingItem {
        KeyBindingItem::new_ctrl_and_char("ReAnnounce", *self.config_key_bindings.get(&ConfigKeyBindingKey::KbReAnn).unwrap())
    }

    pub fn cnf_kb_search(&mut self) -> KeyBindingItem {
        KeyBindingItem::new_ctrl_and_char("Search", *self.config_key_bindings.get(&ConfigKeyBindingKey::KbSearch).unwrap())
    }

    pub fn kb_cancel() -> KeyBindingItem {
        KeyBindingItem::new_key_code("Cancel", KeyCode::Esc)
    }
}
