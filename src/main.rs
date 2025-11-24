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

fn main() -> io::Result<()> {
    App::new().run()
}