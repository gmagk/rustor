use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub enum TorrentSource {
    PirateBay,
    TorrentsCsv,
    #[default]
    None
}

impl fmt::Display for TorrentSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Default, Clone, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct SearchTorrent {
    pub id: String,
    pub name: String,
    pub info_hash: String,
    pub seeders: i64,
    pub leechers: i64,
    pub size: i64,
    pub created_on: i64,
    pub description: String, // Used for PirateBay only
    pub source: TorrentSource,
    pub descr: String, // Used for PirateBay only
    pub files: Vec<SearchTorrentFile> // Used for PirateBay only
}

impl SearchTorrent {

    pub fn name_only(name: String) -> SearchTorrent {
        let mut torrent = SearchTorrent::default();
        torrent.name = name;

        torrent

    }
}

// For now works only for PirateBay
#[derive(Default, Clone, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct SearchTorrentFile {
    pub name: String,
    pub size: i64
}

impl SearchTorrentFile {

    pub fn new(name: String, size: i64) -> Self {
        Self { name, size }
    }
}

#[derive(Default, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct PirateBayListTorrent {
    pub id: String,
    pub name: String,
    pub info_hash: String,
    pub seeders: String,
    pub leechers: String,
    pub size: String,
    pub username: String,
    pub added: String,
    pub status: String,
    pub category: String,
    pub imdb: String
}

#[derive(Default, Clone, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct PirateBayInfoTorrent {
    pub id: i64,
    pub category: i64,
    pub status: String,
    pub name: String,
    pub num_files: i64,
    pub size: i64,
    pub seeders: i64,
    pub leechers: i64,
    pub username: String,
    pub added: i64,
    pub imdb: String,
    pub language: i64,
    pub info_hash: String,
    pub descr: String
}

impl PirateBayInfoTorrent {

    pub fn name_only(name: String) -> PirateBayInfoTorrent {
        let mut info = PirateBayInfoTorrent::default();
        info.name = name;

        info
    }
}

#[derive(Default, Serialize, Clone, Deserialize)]
pub struct PirateBayTorrentFile {
    pub name: Vec<String>,
    pub size: Vec<i64>
}

impl PirateBayTorrentFile {

    pub fn empty(name: String) -> PirateBayTorrentFile {
        let mut file = PirateBayTorrentFile::default();
        file.name = vec![name];

        file
    }
}

#[derive(Default, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct TorrentsCsvTorrent {
    #[serde(rename = "infohash")]
    pub info_hash: String,
    pub name: String,
    pub size_bytes: i64,
    pub created_unix: i64,
    pub seeders: i64,
    pub leechers: i64,
    pub completed: i64,
    pub scraped_date: i64,
    pub id: i64,
}

#[derive(Default, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct TorrentsCsvResponse {
    pub torrents: Vec<TorrentsCsvTorrent>
}