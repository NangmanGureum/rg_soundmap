//! Format manifest
//!
//! It contains JSON data

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sound {
    pub id: u16,
    pub path: String,

    /// The pitch of the note.
    /// for example, C4(= Middle C) note goes 60 in decimal. It same as MIDI standard.
    /// If it is drum sound, it follows MIDI GM Drummap.
    pub pitch: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// A title of the song
    pub title: String,

    /// A list of artists
    pub artists: Vec<String>,

    /// A list of writers
    pub writers: Vec<String>,

    /// A map of path of sound file
    pub sounds: Vec<Sound>,

    pub genre: String,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            title: "Title".to_string(),
            artists: vec!["Various Artists".to_string()],
            writers: Vec::new(),
            sounds: Vec::new(),
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

    pub fn insert_sound(&mut self, id: u16, path: &str, pitch: u8) {
        let target_pos = self.sounds.iter().position(|x| x.id == id).unwrap();
        self.sounds.insert(
            target_pos,
            Sound {
                id,
                path: path.to_string(),
                pitch,
            },
        );
    }

    pub fn push_sound(&mut self, path: &str, pitch: u8) {
        let mut ids: Vec<u16> = Vec::new();

        for sound in &self.sounds {
            ids.push(sound.id);
        }

        if self.sounds.len() == 0 {
            self.sounds.push(Sound {
                id: 0,
                path: path.to_string(),
                pitch,
            });
        } else {
            for (index, sound_id) in ids.iter().enumerate() {
                // If missing number (0:0, 1:1, '2:3', 3:4 ...)
                if index != *sound_id as usize {
                    self.sounds.push(Sound {
                        id: index as u16,
                        path: path.to_string(),
                        pitch,
                    });
                    break;
                }
                // If last index
                else if index == (ids.len() - 1) {
                    self.sounds.push(Sound {
                        id: (index as u16) + 1,
                        path: path.to_string(),
                        pitch,
                    });
                }
            }
        }
    }

    pub fn get_sound_path(&self, id: u16) -> Option<&str> {
        for s in &self.sounds {
            if s.id == id {
                return Some(&s.path);
            }
        }
        None
    }
}
