use serde::{Deserialize};
use crate::util::Util;

#[derive(Default, Clone,  Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct TransmissionTorrent {
    #[serde(rename = "activityDate")]
    pub activity_date: i64,
    #[serde(rename = "addedDate")]
    pub added_date: i64,
    #[serde(rename = "bandwidthPriority")]
    pub bandwidth_priority: i64,
    pub comment: String,
    #[serde(rename = "corruptEver")]
    pub corrupt_ever: i64,
    pub creator: String,
    #[serde(rename = "dateCreated")]
    pub date_created: i64,
    #[serde(rename = "desiredAvailable")]
    pub desired_available: i64,
    #[serde(rename = "doneDate")]
    pub done_date: i64,
    #[serde(rename = "downloadDir")]
    pub download_dir: String,
    #[serde(rename = "downloadLimit")]
    pub download_limit: i64,
    #[serde(rename = "downloadLimited")]
    pub download_limited: bool,
    #[serde(rename = "downloadedEver")]
    pub downloaded_ever: i64,
    pub error: i64,
    #[serde(rename = "errorString")]
    pub error_string: String,
    pub eta: i64,
    pub files: Vec<TransmissionTorrentFile>,
    pub group: String,
    #[serde(rename = "hashString")]
    pub hash_string: String,
    #[serde(rename = "haveUnchecked")]
    pub have_unchecked: i64,
    #[serde(rename = "haveValid")]
    pub have_valid: i64,
    #[serde(rename = "honorsSessionLimits")]
    pub honors_session_limits: bool,
    pub id: i64,
    #[serde(rename = "isFinished")]
    pub is_finished: bool,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    pub labels: Vec<String>,
    #[serde(rename = "leftUntilDone")]
    pub left_until_done: i64,
    #[serde(rename = "magnetLink")]
    pub magnet_link: String,
    // TODO add filePath
    pub name: String,
    pub peers: Vec<TransmissionTorrentPeer>,
    #[serde(rename = "peer-limit")]
    pub peer_limit: i64,
    #[serde(rename = "peersConnected")]
    pub peers_connected: i64,
    #[serde(rename = "peersGettingFromUs")]
    pub peers_getting_from_us: i64,
    #[serde(rename = "peersSendingToUs")]
    pub peers_sending_to_us: i64,
    #[serde(rename = "pieceCount")]
    pub piece_count: i64,
    #[serde(rename = "pieceSize")]
    pub piece_size: i64,
    #[serde(rename = "rateDownload")]
    pub rate_download: i64,
    #[serde(rename = "rateUpload")]
    pub rate_upload: i64,
    #[serde(rename = "recheckProgress")]
    pub recheck_progress: f64,
    #[serde(rename = "secondsDownloading")]
    pub seconds_downloading: i64,
    #[serde(rename = "secondsSeeding")]
    pub seconds_seeding: i64,
    #[serde(rename = "seedIdleLimit")]
    pub seed_idle_limit: i64,
    #[serde(rename = "seedIdleMode")]
    pub seed_idle_mode: i64,
    #[serde(rename = "seedRatioLimit")]
    pub seed_ratio_limit: f64,
    #[serde(rename = "seedRatioMode")]
    pub seed_ratio_mode: i64,
    pub sequential_download: bool,
    #[serde(rename = "sizeWhenDone")]
    pub size_when_done: i64,
    pub source: String,
    #[serde(rename = "startDate")]
    pub start_date: i64,
    pub status: i64,
    #[serde(rename = "totalSize")]
    pub total_size: i64,
    #[serde(rename = "trackerStats")]
    pub tracker_stats: Vec<TransmissionTorrentTrackerStat>,
    #[serde(rename = "uploadLimit")]
    pub upload_limit: i64,
    #[serde(rename = "uploadLimited")]
    pub upload_limited: bool,
    #[serde(rename = "uploadRatio")]
    pub upload_ratio: f64,
    #[serde(rename = "uploadedEver")]
    pub uploaded_ever: i64,
    pub webseeds: Vec<String>,
    #[serde(rename = "webseedsSendingToUs")]
    pub webseeds_sending_to_us: i64,
}

impl TransmissionTorrent {
    pub fn eta(&self) -> String {
        if self.left_until_done == 0 {
            return "Done".to_string();
        }

        if self.eta <= 0 {
            return "Unknown".to_string();
        }

        let seconds = self.eta % 60;
        let minutes = (self.eta / 60) % 60;
        let hours = (self.eta / 60 / 60) % 60;
        let days = (self.eta / 60 / 60 / 24) % 24;

        let time = format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds);
        if days > 0 {
            format!("{days} days {time}")
        } else {
            time
        }
    }

    pub fn percentage_done(&self) -> String {
        format!("{:.2} %", self.calc_percentage_done())
    }

    pub fn calc_ratio(&self) -> f64 {
        self.calc_percentage_done() / 100f64
    }

    pub fn download_rate(&self) -> String {
        format!("\u{2193} {} kB/s", (self.rate_download / 1000) % 1000)
    }

    pub fn upload_rate(&self) -> String {
        format!("\u{2191} {} kB/s", (self.rate_upload / 1000) % 1000)
    }

    pub fn total_size(&self) -> String {
        if self.size_when_done <= 0 {
            return "".to_string();
        }

        Util::print_bytes(self.size_when_done as f64)
    }

    pub fn downloaded(&self) -> String {
        Util::print_bytes((self.size_when_done - self.left_until_done) as f64)
    }

    pub fn peers_client_name(&self) -> String {
        if self.peers.len() == 0 {
            return String::default();
        }

        let mut res = self
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

    fn calc_percentage_done(&self) -> f64 {
        if self.left_until_done == 0 {
            return 100f64;
        }

        let left_undone: f64 = self.left_until_done as f64;
        let total_size: f64 = self.size_when_done as f64;
        (100f64 - 100f64 * left_undone / total_size) % 100f64
    }
}

#[derive(Default, Clone, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct TransmissionTorrentPeer {
    pub address: String,
    #[serde(rename = "clientIsChoked")]
    pub client_is_choked: bool,
    #[serde(rename = "clientIsInterested")]
    pub client_is_interested: bool,
    #[serde(rename = "clientName")]
    pub client_name: String,
    #[serde(rename = "flagStr")]
    pub flag_str: String,
    #[serde(rename = "isDownloadingFrom")]
    pub is_downloading_from: bool,
    #[serde(rename = "isEncrypted")]
    pub is_encrypted: bool,
    #[serde(rename = "isIncoming")]
    pub is_incoming: bool,
    #[serde(rename = "isUTP")]
    pub is_utp: bool,
    #[serde(rename = "isUploadingTo")]
    pub is_uploading_to: bool,
    #[serde(rename = "peerIsChoked")]
    pub peer_is_choked: bool,
    #[serde(rename = "peerIsInterested")]
    pub peer_is_interested: bool,
    pub port: i64,
    pub progress: f64,
    #[serde(rename = "rateToClient")]
    pub rate_to_client: i64,
    #[serde(rename = "rateToPeer")]
    pub rate_to_peer: i64,
}

#[derive(Default, Clone, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct TransmissionTorrentTrackerStat {
    pub announce: String,
    #[serde(rename = "announceState")]
    pub announce_state: i64,
    #[serde(rename = "downloadCount")]
    pub download_count: i64,
    #[serde(rename = "hasAnnounced")]
    pub has_announced: bool,
    #[serde(rename = "hasScraped")]
    pub has_scraped: bool,
    pub host: String,
    pub id: i64,
    #[serde(rename = "isBackup")]
    pub is_backup: bool,
    #[serde(rename = "lastAnnouncePeerCount")]
    pub last_announce_peer_count: i64,
    #[serde(rename = "lastAnnounceResult")]
    pub last_announce_result: String,
    #[serde(rename = "lastAnnounceStartTime")]
    pub last_announce_start_time: i64,
    #[serde(rename = "lastAnnounceSucceeded")]
    pub last_announce_succeeded: bool,
    #[serde(rename = "lastAnnounceTime")]
    pub last_announce_time: i64,
    #[serde(rename = "lastAnnounceTimedOut")]
    pub last_announce_timed_out: bool,
    #[serde(rename = "lastScrapeResult")]
    pub last_scrape_result: String,
    #[serde(rename = "lastScrapeStartTime")]
    pub last_scrape_start_time: i64,
    #[serde(rename = "lastScrapeSucceeded")]
    pub last_scrape_succeeded: bool,
    #[serde(rename = "lastScrapeTime")]
    pub last_scrape_time: i64,
    #[serde(rename = "lastScrapeTimedOut")]
    pub last_scrape_timed_out: bool,
    #[serde(rename = "leecherCount")]
    pub leecher_count: i64,
    #[serde(rename = "nextAnnounceTime")]
    pub next_announce_time: i64,
    #[serde(rename = "nextScrapeTime")]
    pub next_scrape_time: i64,
    pub scrape: String,
    #[serde(rename = "scrapeState")]
    pub scrape_state: i64,
    #[serde(rename = "seederCount")]
    pub seeder_count: i64,
    pub sitename: String,
    pub tier: i64,
}

#[derive(Default, Clone, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct TransmissionTorrentFile {
    pub begin_piece: i64,
    #[serde(rename = "bytesCompleted")]
    pub bytes_completed: i64,
    pub end_piece: i64,
    pub length: i64,
    pub name: String,
}

#[derive(Default, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct TransmissionTorrentFiles {
    pub files: Vec<TransmissionTorrentFile>,
    pub name: String,
    pub priorities: Vec<i64>,
    pub wanted: Vec<i64>,
}

#[derive(Default, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct TransmissionResponseArguments {
    pub torrents: Vec<TransmissionTorrent>,
}

#[derive(Default, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct TransmissionResponse {
    pub arguments: TransmissionResponseArguments,
    pub result: String,
    pub tag: i64,
}
