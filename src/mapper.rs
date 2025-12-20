use crate::dto::torrent_dto::{PirateBayInfoTorrent, PirateBayListTorrent, PirateBayTorrentFile, SearchTorrent, SearchTorrentFile, TorrentSource, TorrentsCsvTorrent};

pub struct Mapper {}

impl Mapper {

    pub fn pirate_bay_list_torrent_to_search_torrent(source: &PirateBayListTorrent) -> SearchTorrent {
        let mut torrent = SearchTorrent::default();
        torrent.id = source.id.clone();
        torrent.name = source.name.clone();
        torrent.seeders = source.seeders.parse().unwrap();
        torrent.leechers = source.leechers.parse().unwrap();
        torrent.size = source.size.parse().unwrap();
        torrent.created_on = source.added.parse().unwrap();
        torrent.info_hash = source.info_hash.clone();
        torrent.source = TorrentSource::PirateBay;

        torrent
    }

    pub fn pirate_bay_torrent_info_and_files_to_search_torrent(
        info: &PirateBayInfoTorrent,
        files: &Vec<PirateBayTorrentFile>
    ) -> SearchTorrent {
        let mut torrent = SearchTorrent::default();
        torrent.id = info.id.to_string();
        torrent.name = info.name.clone();
        torrent.seeders = info.seeders;
        torrent.leechers = info.leechers;
        torrent.size = info.size;
        torrent.created_on = info.added;
        torrent.info_hash = info.info_hash.clone();
        torrent.description = info.descr.clone();
        torrent.source = TorrentSource::PirateBay;
        torrent.files = files
            .iter()
            .map(| file | SearchTorrentFile::new(
                if file.name.len() > 0 { file.name[0].clone() } else { "".to_string() },
                if file.size.len() > 0 { file.size[0].clone() } else { 0 }
            ))
            .collect();

        torrent
    }

    pub fn torrents_csv_torrent_to_search_torrent(source: &TorrentsCsvTorrent) -> SearchTorrent {
        let mut torrent = SearchTorrent::default();
        torrent.id = source.id.to_string();
        torrent.name = source.name.clone();
        torrent.info_hash = source.info_hash.clone();
        torrent.seeders = source.seeders;
        torrent.leechers = source.leechers;
        torrent.size = source.size_bytes;
        torrent.created_on = source.created_unix;
        torrent.descr = "".to_string();
        torrent.source = TorrentSource::TorrentsCsv;

        torrent
    }
}