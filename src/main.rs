#![allow(unused_imports)]

mod app;
mod dto;
mod service;
mod mapper;
mod ui;
mod util;

use crate::app::App;
use std::io;
use std::process::Command;
use std::sync::{Arc, Mutex};

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let result = App::new(Arc::new(Mutex::new(terminal))).run();
    ratatui::restore();
    result
}