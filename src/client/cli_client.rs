use std::process::Command;

#[derive(Default, Clone, Copy)]
pub struct CliClient {}

impl CliClient {

    pub fn run_command(command: String) -> Vec<String> {
        let cmd = command.clone();
        let status_command = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect(format!("failed to execute `{}`", cmd).as_str())
            .stdout;

        vec![String::from_utf8(status_command).unwrap_or_default()]
    }
}