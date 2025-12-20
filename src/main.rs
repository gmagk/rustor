#![allow(unused_imports)]

mod app;
mod config;
mod dto;
mod service;
mod screen;
mod util;
mod client;
mod key_bindings;
mod mapper;

use crate::app::App;
use crate::config::Config;
use std::sync::{Arc, Mutex};
use std::{env, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let terminal = ratatui::init();
    let result = App::new(Config::new(args), Arc::new(Mutex::new(terminal))).run();
    ratatui::restore();
    result
}
