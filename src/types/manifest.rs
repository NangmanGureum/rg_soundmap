//! Format manifest
//!
//! It contains JSON data

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// A title of the song
    pub title: String,

    /// A list of artists
    pub artists: Vec<String>,

    /// A list of writers
    pub writers: Vec<String>,

    /// A map of path of sound file
    pub sounds: HashMap<u16, String>,

    pub genre: String,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            title: "Title".to_string(),
            artists: vec!["Various Artists".to_string()],
            writers: Vec::new(),
            sounds: HashMap::new(),
            genre: String::new(),
        }
    }
}

impl Manifest {
    pub fn new(title: &str, artist: &str) -> Self {
        let mut manifest = Self::default();
        manifest.title = title.to_string();
        manifest.artists = vec![artist.to_string()];
        manifest
    }

    pub fn with_artists(mut self, artists: Vec<String>) -> Self {
        self.artists = artists;
        self
    }

    pub fn with_writers(mut self, writers: Vec<String>) -> Self {
        self.writers = writers;
        self
    }

    pub fn add_artist(&mut self, artist: &str) {
        self.artists.push(artist.to_string());
    }

    pub fn add_writer(&mut self, writer: &str) {
        self.writers.push(writer.to_string());
    }

    pub fn insert_sound(&mut self, id: u16, path: &str) {
        self.sounds.insert(id, path.to_string());
    }
}
