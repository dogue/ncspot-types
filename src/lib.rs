#![crate_name = "ncspot_types"]

//! This crate provides a set of types that derive `serde::Deserialize` that match the JSON
//! output from [ncspot](https://github.com/hrkfdn/ncspot).
//!
//! Interior types are matched as closely as possible to the way the data is modelled inside
//! of ncspot, with one exception: `duration` is deserialized as a `std::time::Duration` whereas
//! internally, ncspot represents this as a `u32`.
//!
//! I'm not entirely convinced this is necessary yet, but it may be convenient when using the library.

use chrono::offset::Utc;
use chrono::DateTime;
use serde::Deserialize;
use std::time::Duration;

/// A single event from ncspot
#[derive(Debug, Deserialize, Clone)]
pub struct Event {
    pub mode: Mode,
    pub playable: Playable,
}

/// Type of event, [Track] or [Episode]
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Playable {
    Track(Track),
    Episode(Episode),
}

/// Current playback mode
#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Mode {
    /// A [Track] or [Episode] is currently playing
    Playing {
        secs_since_epoch: usize,
        nanos_since_epoch: usize,
    },

    /// A [Track] or [Episode] is currently paused
    Paused { secs: usize, nanos: usize },

    /// Emitted briefly when playing a chosen song
    /// before then emitting a `Playing` event
    Stopped,
}

/// A music track
#[serde_with::serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct Track {
    pub id: Option<String>,
    pub uri: String,
    pub title: String,
    pub track_number: u32,
    pub disc_number: i32,
    #[serde_as(as = "serde_with::DurationMilliSeconds<u64>")]
    pub duration: Duration,
    pub artists: Vec<String>,
    pub artist_ids: Vec<String>,
    pub album: Option<String>,
    pub album_id: Option<String>,
    pub album_artists: Vec<String>,
    pub cover_url: Option<String>,
    pub url: String,
    pub added_at: Option<DateTime<Utc>>,
    pub list_index: usize,
}

/// A podcast episode
#[serde_with::serde_as]
#[derive(Debug, Deserialize, Clone)]
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
