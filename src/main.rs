#![allow(unused_imports)]

mod app;
mod config;
mod dto;
mod service;
mod screen;
mod util;
mod client;
mod mapper;

use crate::app::App;
use crate::config::{Config, Params};
use std::sync::{Arc, Mutex};
use std::io;
use clap::Parser;

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let config = Config::new(Params::parse());
    let result = App::new(config, Arc::new(Mutex::new(terminal))).run();
    ratatui::restore();
    result
}
