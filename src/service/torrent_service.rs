use crate::client::http_client::HttpClient;
use crate::dto::torrent_dto::{TorrentsCsvTorrent, TorrentsCsvResponse, PirateBayTorrentFile, PirateBayInfoTorrent, PirateBayListTorrent};
use std::io::{Error, ErrorKind};

pub struct TorrentService {
    http_client: HttpClient
}

impl TorrentService {

    pub fn new(http_client: HttpClient) -> Self {
        Self { http_client }
    }

    /*
        - request: https://apibay.org/q.php?q=<search-sterm>
        - response:
            [
              {
                "id": "58930804",
                "name": "The Batman (2022) [1080p] [WEBRip] [5.1]",
                "info_hash": "0C23E50E075C634CFD5CD8A09A82F6EEE18D72A8",
                "leechers": "58",
                "seeders": "356",
                "size": "3488057368",
                "num_files": "3",
                "username": "surferbroadband",
                "added": "1652002730",
                "status": "vip",
                "category": "207",
                "imdb": "tt1877830"
              },
              ...
          ]
     */
    pub fn search_pirate_bay(&self, search_term: &str) -> Result<Vec<PirateBayListTorrent>, Error> {
        self.http_client.request(format!("https://apibay.org/q.php?q={}", search_term))
    }

    /*
        - request: https://apibay.org/t.php?id=<torrent-id>>
        - response:
            {
              "id": 58930804,
              "category": 207,
              "status": "vip",
              "name": "The Batman (2022) [1080p] [WEBRip] [5.1]",
              "num_files": 3,
              "size": 3488057368,
              "seeders": 425,
              "leechers": 68,
              "username": "surferbroadband",
              "added": 1652002730,
              "descr": "The Batman (2022) [1080p] [WEBRip] [5.1]\r\n\r\nWhen a sadistic serial killer begins murdering key political figures in Gotham, Batman is forced to investigate the city's hidden corruption and question his family's involvement.\r\n\r\n    Director\r\n        Matt Reeves\r\n\r\n    Writers\r\n        Matt Reeves\r\n        Peter Craig\r\n        Bill Finger(Batman created by)\r\n\r\nStars\r\n    Robert Pattinson\r\n    Zoë Kravitz\r\n    Jeffrey Wright",
              "imdb": "tt1877830",
              "language": 1,
              "textlanguage": 1,
              "info_hash": "0C23E50E075C634CFD5CD8A09A82F6EEE18D72A8"
            }
     */
    pub fn torrent_info_pirate_bay(&self, torrent_id: i64) -> Result<PirateBayInfoTorrent, Error> {
        self.http_client.request(format!("https://apibay.org/t.php?id={}", torrent_id))
    }

    /*
        - request: https://apibay.org/f.php?id=<torrent-id>
        - response:
            [
              {
                "name": [
                  "The.Batman.2022.1080p.WEBRip.x264.AAC5.1-[YTS.MX].mp4"
                ],
                "size": [
                  3488003784
                ]
              },
              ...
            ]
     */
    pub fn torrent_files_pirate_bay(&self, torrent_id: i64) -> Result<Vec<PirateBayTorrentFile>, Error> {
        self.http_client.request(format!("https://apibay.org/f.php?id={}", torrent_id))
    }

    /*
        - request: https://torrents-csv.ml/service/search?q=<search-term>[&size=<max-results>][&page=<page>]
        - response:
            {
              "torrents": [
                {
                  "infohash": "0c23e50e075c634cfd5cd8a09a82f6eee18d72a8",
                  "name": "The Batman (2022) [1080p] [WEBRip] [5.1] [YTS.MX]",
                  "size_bytes": 3488057368,
                  "created_unix": 1650355800,
                  "seeders": 331,
                  "leechers": 40,
                  "completed": 147771,
                  "scraped_date": 1765419470,
                  "id": 2576
                },
                ...
            ],
            "next": <next-torrent-id>
     */
    pub fn search_torrents_csv(&self, search_term: &str) -> Result<Vec<TorrentsCsvTorrent>, Error> {
        if search_term.is_empty() {
            Err(Error::new(ErrorKind::InvalidData, "Empty search term!"))
        } else {
            let size_result = search_term.chars().size_hint();
            match size_result.1 {
                Some(size) =>
                    if size < 3 {
                        Err(Error::new(ErrorKind::InvalidData, "Search term too short!"))
                    } else {
                        let result: TorrentsCsvResponse = self.http_client.request(format!("https://torrents-csv.com/service/search?size=20&q={}", search_term))?;
                        Ok(result.torrents)
                    },
                None => Err(Error::new(ErrorKind::InvalidData, "Can't calculate search term length!"))
            }
        }
    }
}

