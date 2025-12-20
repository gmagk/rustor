![screeshot](./screenshot.png)
# About
A simple torrent TUI for Linux (tested only on Ubuntu 25.10).  
Uses [transmission-remote](https://transmissionbt.com) API and is written in Rust using Ratatui.

# Built With
- [Rust](https://rust-lang.org/)
- [Ratatui](https://ratatui.rs/)
- [tui-scrollview](https://crates.io/crates/tui-scrollview)
- [tui-input](https://crates.io/crates/tui-input/0.14.0)

# Getting Started

## Prerequisites
1. Install Rust from [here](https://github.com/othneildrew/Best-README-Template?tab=readme-ov-file).
2. Install transmission from [here](https://transmissionbt.com/) or only the transmission-daemon using  
   `sudo apt-get install transmission-daemon`.
>    Proposed transmission configuration:  
>    ```shell
>    sudo systemctl stop transmission-daemon
>    sudo {my-editor} /etc/transmission-daemon/settings.json
>    ```
>     
>    - changing these directories might produce permission errors  
>        "download-dir": "{leave the default}",  
>        "incomplete-dir": "{leave the default}",
>    - disable authentication  
>        "rpc-authentication-required": false,
>    - disable host whitelist  
>        "rpc-host-whitelist-enabled": false,
>    - disable whitelist  
>        "rpc-whitelist-enabled": false,
>    - change the file creation mask, so that the downloaded files would be writeable by $USER  
>      "umask": 022 => "umask": 002
> 
>    ```shell
>    sudo systemctl start transmission-daemon
>    ```

## Installation
Release (pre-production): https://github.com/gmagk/rustube/releases/tag/1.0.0-alpha

## Key Bindings Customization
#### _(no check is done for any conflicting keys)_
#### example: `../rustor --kb-home=g` _now can use *Ctrl+g* for *home* screen_
| action        | arg         |
|---------------|-------------|
| home screen   | --kb-home   |
| add screen    | --kb-add    |
| search screen | --kb-search |
| help screen   | --kb-help   |
| quit          | --kb-quit   |
> For convenience better add the whole command as an alias in the **.bashrc* file.

# Contributing
Any contributions made will be **really appreciated**.  
Please open a new issue [here](https://github.com/gmagk/rustor/issues/new).

# License
Copyright (c) gman <tbuhtexis@yahoo.com>  
This project is licensed under the MIT license ([LICENSE](./LICENSE) or http://opensource.org/licenses/MIT)
