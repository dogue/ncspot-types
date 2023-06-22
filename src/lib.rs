use chrono::offset::Utc;
use chrono::DateTime;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct Status {
    pub mode: Mode,
    pub playable: Playable,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Playable {
    Track(Track),
    Episode(Episode),
}

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

#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct Track {
    pub id: String,
    pub uri: String,
    pub title: String,
    pub track_number: usize,
    pub disc_number: usize,
    #[serde_as(as = "serde_with::DurationMilliSeconds<u64>")]
    pub duration: Duration,
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

#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct Episode {
    pub id: String,
    pub uri: String,
    pub duration: Duration,
    pub name: String,
    pub description: String,
    pub release_date: DateTime<Utc>,
    pub cover_url: Option<String>,
    pub added_at: Option<DateTime<Utc>>,
    pub list_index: usize,
}
