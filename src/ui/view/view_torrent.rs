use crate::dto::Torrent;
use crate::util::Util;
use chrono::{DateTime, Local, Utc};
use std::time::{Duration, UNIX_EPOCH};

pub struct TorrentView {}

impl TorrentView {
    pub fn eta(torrent: &Torrent) -> String {
        if torrent.left_until_done == 0 {
            return "Done".to_string();
        }

        if torrent.eta <= 0 {
            return "Unknown".to_string();
        }

        let seconds = torrent.eta % 60;
        let minutes = (torrent.eta / 60) % 60;
        let hours = (torrent.eta / 60 / 60) % 60;
        let days = (torrent.eta / 60 / 60 / 24) % 24;

        let time = format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds);
        if days > 0 {
            format!("{days} days {time}")
        } else {
            time
        }
    }

    pub fn percentage_done(torrent: &Torrent) -> String {
        format!("{:.2} %", torrent.percentage_done())
    }

    pub fn download_rate(torrent: &Torrent) -> String {
        format!("\u{2193} {} kB/s", (torrent.rate_download / 1000) % 1000)
    }

    pub fn upload_rate(torrent: &Torrent) -> String {
        format!("\u{2191} {} kB/s", (torrent.rate_upload / 1000) % 1000)
    }

    pub fn total_size(torrent: &Torrent) -> String {
        if torrent.size_when_done <= 0 {
            return "".to_string();
        }

        Util::print_bytes(torrent.size_when_done as f64)
    }

    pub fn downloaded(torrent: &Torrent) -> String {
        Util::print_bytes((torrent.size_when_done - torrent.left_until_done) as f64)
    }

    pub fn added_on(torrent: &Torrent) -> String {
        let d = UNIX_EPOCH + Duration::from_secs(torrent.added_date as u64);
        let datetime = DateTime::<Utc>::from(d).with_timezone(&Local);
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn peers_client_name(torrent: &Torrent) -> String {
        if torrent.peers.len() == 0 {
            return String::default();
        }

        let mut res = torrent
            .peers
            .iter()
            .map(|peer| peer.client_name.clone())
            .reduce(|mut accumulator, s| {
                accumulator.push_str(&s);
                accumulator.push(',');
                accumulator.push(' ');
                accumulator
            })
            .unwrap();
        res.pop();
        res.pop();
        res
    }
}
