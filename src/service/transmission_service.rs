use std::process::Command;
use crate::client::cli_client::CliClient;
use crate::dto::transmission_dto::{TransmissionResponse, TransmissionTorrent};

pub struct TransmissionService {}

impl TransmissionService {

    /*
       @tor: Torrent local filename or magnet-link
    */
    pub fn torrent_add(tor: String) -> TransmissionResponse {
        Self::json_to_response(CliClient::run_command(format!("transmission-remote -j -a {}", tor)))
    }

    pub fn torrent_remove(tor_id: String) -> TransmissionResponse {
        Self::json_to_response(CliClient::run_command(format!("transmission-remote -j -t {} -r", tor_id)))
    }

    pub fn torrent_list() -> TransmissionResponse {
        Self::json_to_response(CliClient::run_command("transmission-remote -j -l".parse().unwrap()))
    }

    pub fn torrent_start(tor_id: String) -> TransmissionResponse {
        Self::json_to_response(CliClient::run_command(format!("transmission-remote -j -t {} -s", tor_id)))
    }

    pub fn torrent_stop(tor_id: String) -> TransmissionResponse {
        Self::json_to_response(CliClient::run_command(format!("transmission-remote -j -t {} -S", tor_id)))
    }

    pub fn torrent_reannounce(tor_id: String) -> TransmissionResponse {
        Self::json_to_response(CliClient::run_command(format!("transmission-remote -j -t {} --reannounce", tor_id)))
    }

    pub fn torrent_info(tor_id: String) -> TransmissionResponse {
        Self::json_to_response(CliClient::run_command(format!("transmission-remote -j -t {} -i", tor_id)))
    }

    pub fn torrent_files(tor_id: String) -> TransmissionResponse {
        Self::json_to_response(CliClient::run_command(format!("transmission-remote -j -t {} -f", tor_id)))
    }

    pub fn torrent_peers(tor_id: String) -> TransmissionResponse {
        Self::json_to_response(CliClient::run_command(format!("transmission-remote -j -t {} -ip", tor_id)))
    }

    pub fn torrent_trackers(tor_id: String) -> TransmissionResponse {
        Self::json_to_response(CliClient::run_command(format!("transmission-remote -j -t {} -it", tor_id)))
    }

    pub fn torrent_location(tor: &TransmissionTorrent) {
        CliClient::run_command(format!("xdg-open {}", tor.download_dir));
    }

    pub fn transmission_daemon_is_active() -> bool {
        let result = &CliClient::run_command("systemctl is-active transmission-daemon.service".to_string())[0];

        if result.is_empty() {
            println!(
                "Could not retrieve the transmission-daemon's status, maybe is not running or is not even installed"
            );
            false
        } else {
            !result.contains("inactive") && result.contains("active")
        }
    }

    fn json_to_response(source: Vec<String>) -> TransmissionResponse {
        let str = source.iter().map(|x| x.to_string()).collect::<String>();
        serde_json::from_str(str.as_str()).unwrap()
    }
}
