use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct Torrent {
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
    pub files: Vec<File>,
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
    pub peers: Vec<Peer>,
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
    pub tracker_stats: Vec<TrackerStat>,
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

impl Torrent {
    pub fn percentage_done(&self) -> f64 {
        if self.left_until_done == 0 {
            return 100f64;
        }

        let left_undone: f64 = self.left_until_done as f64;
        let total_size: f64 = self.size_when_done as f64;
        (100f64 - 100f64 * left_undone / total_size) % 100f64
    }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct Peer {
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

#[derive(Default, Serialize, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct TrackerStat {
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

#[derive(Default, Serialize, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct File {
    pub begin_piece: i64,
    #[serde(rename = "bytesCompleted")]
    pub bytes_completed: i64,
    pub end_piece: i64,
    pub length: i64,
    pub name: String,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct Files {
    pub files: Vec<File>,
    pub name: String,
    pub priorities: Vec<i64>,
    pub wanted: Vec<i64>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct Arguments {
    pub torrents: Vec<Torrent>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct Response {
    pub arguments: Arguments,
    pub result: String,
    pub tag: i64,
}
