use std::process::Command;

#[derive(Default, Clone, Copy)]
pub struct Service {}

impl Service {

    pub fn transmission_daemon_is_active(&self) -> bool {
        let status_command = Command::new("sh")
            .arg("-c")
            .arg("systemctl is-active transmission-daemon.service")
            .output()
            .expect("failed to execute `systemctl is-active transmission-daemon.service`")
            .stdout;

        let value = String::from_utf8(status_command).unwrap_or_default();

        if value.is_empty() {
            println!("Could not retrieve the transmission-daemon's status, maybe is not running or is not even installed");
            false
        } else {
            !value.contains("inactive") && value.contains("active")
        }
    }

    /*
        @tor: Torrent local filename or magnet-link
     */
    pub fn torrent_add(&self, tor: String) -> Vec<String> {
        Self::run_command(format!("transmission-remote -j -a {tor}"))
    }

    pub fn torrent_remove(&self, tor_id: String) -> Vec<String> {
        Self::run_command(format!("transmission-remote -j -t {tor_id} -r"))
    }

    pub fn torrent_list(&self) -> Vec<String> {
        Self::run_command("transmission-remote -j -l".parse().unwrap())
    }

    pub fn torrent_start(&self, tor_id: String) -> Vec<String> {
        Self::run_command(format!("transmission-remote -j -t {tor_id} -s"))
    }

    pub fn torrent_stop(&self, tor_id: String) -> Vec<String> {
        Self::run_command(format!("transmission-remote -j -t {tor_id} -S"))
    }

    pub fn torrent_reannounce(&self, tor_id: String) -> Vec<String> {
        Self::run_command(format!("transmission-remote -j -t {tor_id} --reannounce"))
    }

    pub fn torrent_info(&self, tor_id: String) -> Vec<String> {
        Self::run_command(format!("transmission-remote -j -t {tor_id} -i"))
    }

    pub fn torrent_files(&self, tor_id: String) -> Vec<String> {
        Self::run_command(format!("transmission-remote -j -t {tor_id} -f"))
    }

    pub fn torrent_peers(&self, tor_id: String) -> Vec<String> {
        Self::run_command(format!("transmission-remote -j -t {tor_id} -ip"))
    }

    pub fn torrent_trackers(&self, tor_id: String) -> Vec<String> {
        Self::run_command(format!("transmission-remote -j -t {tor_id} -it"))
    }

    fn run_command(command: String) -> Vec<String> {
        let status_command = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute `transmission-remote -j -t <torrent-id> -f`")
            .stdout;

        vec![String::from_utf8(status_command).unwrap_or_default()]
    }
}