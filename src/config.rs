#[derive(Default, Clone, Copy)]
pub struct Config {
    kb_home: char,
    kb_help: char,
    kb_quit: char,
}

#[derive(PartialEq)]
pub enum ConfigKeyBinding {
    KbHome,
    KbHelp,
    KbQuit,
}

impl Config {
    pub fn new(args: Vec<String>) -> Self {
        let parsed_args = Self::parse_args(args);
        Self {
            kb_home: parsed_args.get(0).unwrap().1.chars().next().unwrap(),
            kb_help: parsed_args.get(1).unwrap().1.chars().next().unwrap(),
            kb_quit: parsed_args.get(2).unwrap().1.chars().next().unwrap(),
        }
    }

    pub fn kb_home(&self) -> char {
        self.kb_home
    }

    pub fn kb_help(&self) -> char {
        self.kb_help
    }

    pub fn kb_quit(&self) -> char {
        self.kb_quit
    }

    fn parse_args(args: Vec<String>) -> Vec<(ConfigKeyBinding, String)> {
        let mut result = vec![
            (ConfigKeyBinding::KbHome, "b".to_string()),
            (ConfigKeyBinding::KbHelp, "h".to_string()),
            (ConfigKeyBinding::KbQuit, "q".to_string()),
        ];
        args.iter().for_each(|arg| {
            if arg.starts_with("--kb-home=") {
                Self::replace_kb(&mut result, arg, 0, ConfigKeyBinding::KbHome);
            } else if arg.starts_with("--kb-help=") {
                Self::replace_kb(&mut result, arg, 1, ConfigKeyBinding::KbHelp);
            } else if arg.starts_with("--kb-quit=") {
                Self::replace_kb(&mut result, arg, 2, ConfigKeyBinding::KbQuit);
            }
        });
        result
    }

    fn replace_kb(
        result: &mut Vec<(ConfigKeyBinding, String)>,
        arg: &String,
        index: usize,
        key: ConfigKeyBinding,
    ) {
        let _ = std::mem::replace(&mut result[index], (key, Self::parse_arg(arg).1));
    }

    fn parse_arg(arg: &String) -> (String, String) {
        let split = arg.split_once('=').unwrap();
        (String::from(split.0), String::from(split.1))
    }
}
