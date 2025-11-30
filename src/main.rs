#![allow(unused_imports)]

mod app;
mod dto;
mod service;
mod mapper;
mod ui;
mod util;
mod config;

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