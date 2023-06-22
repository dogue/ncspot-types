use chrono::offset::Utc;
use chrono::DateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Mode {
    Playing {
        secs_since_epoch: usize,
        nanos_since_epoch: usize,
    },

    Paused {
        secs: usize,
        nanos: usize,
    },

    Stopped,
}

// #[derive(Debug, Deserialize)]
// pub struct PlayingMode {
//     pub secs_since_epoch: usize,
//     pub nanos_since_epoch: usize,
// }

// #[derive(Debug, Deserialize)]
// pub struct PausedMode {
//     pub secs: usize,
//     pub nanos: usize,
// }

#[derive(Debug, Deserialize)]
pub enum PlayableType {
    Track,
}

#[derive(Debug, Deserialize)]
pub struct Playable {
    #[serde(rename = "type")]
    pub playable_type: PlayableType,
    pub id: String,
    pub uri: String,
    pub title: String,
    pub track_number: usize,
    pub disc_number: usize,
    pub duration: usize,
    pub artists: Vec<String>,
    pub artist_ids: Vec<String>,
    pub album: String,
    pub album_id: String,
    pub album_artists: Vec<String>,
    pub cover_url: String,
    pub url: String,
    pub added_at: Option<DateTime<Utc>>,
    pub list_index: usize,
}

#[derive(Debug, Deserialize)]
pub struct Track {
    pub mode: Mode,
    pub playable: Playable,
}
