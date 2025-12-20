use std::time::{Duration, UNIX_EPOCH};
use chrono::{DateTime, Local, Utc};
use crossterm::event::{KeyEvent, KeyModifiers};
use crate::dto::transmission_dto::TransmissionTorrent;

pub struct Util {}

impl Util {
    pub fn print_bytes(bytes: f64) -> String {
        let mut res = 0f64;
        let mut postfix = "bytes";
        res = bytes;
        if bytes > 1000f64 {
            let k_bytes = bytes / 1000f64;
            res = k_bytes;
            postfix = "kB";
            if k_bytes > 1000f64 {
                let m_bytes = k_bytes / 1000f64;
                res = m_bytes;
                postfix = "MB";
                if m_bytes > 1000f64 {
                    let g_bytes = m_bytes / 1000f64;
                    res = g_bytes;
                    postfix = "GB";
                    if g_bytes > 1000f64 {
                        let t_bytes = g_bytes / 1000f64;
                        res = t_bytes;
                        postfix = "TB";
                    }
                }
            }
        }

        format!("{:.2} {postfix}", res)
    }

    pub fn print_epoch(epoch: u64) -> String {
        let d = UNIX_EPOCH + Duration::from_secs(epoch);
        let datetime = DateTime::<Utc>::from(d).with_timezone(&Local);
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
