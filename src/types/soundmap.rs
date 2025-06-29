//! This module contains the definition of related to sound stuff.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Defines a note in a soundmap.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    /// The ID of the note.
    pub id: u16,

    /// The ID of the sound associated with the note.
    pub sound_id: u16,

    /// The time at which the note should be played.
    /// It appears in increments of 264ths of a beat. (A beat in 0~263)
    pub time: u32,

    /// The pitch of the note.
    /// for example, C4(= Middle C) note goes 60 in decimal. It same as MIDI standard.
    /// If it is drum sound, it follows MIDI GM Drummap.
    pub pitch: u8,

    /// The line number of the note.
    pub line: u16,
}

/// Defines a BPM set or change in a soundmap.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bpm {
    /// A BPM value
    pub value: f64,

    /// The time at which the BPM change should take effect.
    /// Same as `Note.time`.
    pub time: u32,
}

impl Default for Bpm {
    fn default() -> Self {
        Self {
            value: 120.0,
            time: 0,
        }
    }
}

impl Bpm {
    pub fn new(value: f64, time: u32) -> Self {
        Self { value, time }
    }
}

/// Defines a beat-per-bar setting in a soundmap.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeatPerBar {
    /// The value of the beat-per-bar setting.
    /// If the value is `4`, it means 4 beats per a bar. (similar as 4/4 time)
    pub value: u8,

    /// The time at which the beat-per-bar change should take effect.
    /// Same as `Note.time`.
    pub time: u32,
}

impl Default for BeatPerBar {
    fn default() -> Self {
        Self { value: 4, time: 0 }
    }
}

impl BeatPerBar {
    pub fn new(value: u8, time: u32) -> Self {
        Self { value, time }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoundMap {
    /// The audio format which used in the soundmap.
    ///
    /// ## A list of audio formats
    /// |Format|Description|
    /// |---|---|
    /// |wav|Waveform Audio File Format|
    /// |wav-bwf|Broadcast Wave Format|
    /// |mp3|MPEG-1 Audio Layer III|
    /// |ogg|Ogg Vorbis|
    /// |ogg-flac|Free Lossless Audio Codec|
    /// |opus|Opus Audio Codec|
    /// |aac|Advanced Audio Coding|
    /// |aiff|Audio Interchange File Format|
    pub audio_format: String,

    /// The bit depth of the audio.
    /// If the value is `24`, it means 24-bit audio.
    pub audio_bits: u8,

    /// The sample rate of the audio.
    /// If the value is `48000`, it means 48kHz audio.
    pub audio_sample_rate: u32,

    /// A list of notes.
    pub notes: Vec<Note>,

    /// A name of the line.
    /// It depends `Note.line`
    pub note_line_name: HashMap<u16, String>,

    /// A list of beats per minute.
    pub bpm: Vec<Bpm>,

    /// A list of beat per bar.
    pub beat_per_bar: Vec<BeatPerBar>,
}

impl Default for SoundMap {
    fn default() -> Self {
        Self {
            audio_format: "wav".to_string(),
            audio_bits: 24,
            audio_sample_rate: 48000,
            notes: Vec::new(),
            note_line_name: HashMap::new(),
            // Default to 120 BPM
            bpm: vec![Bpm::default()],
            // Default to 4 beats per bar (similar to 4/4 time signature)
            beat_per_bar: vec![BeatPerBar::default()],
        }
    }
}

impl SoundMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_audio_format(mut self, audio_format: &str) -> Self {
        self.audio_format = audio_format.to_string();
        self
    }

    pub fn with_audio_bits(mut self, audio_bits: u8) -> Self {
        self.audio_bits = audio_bits;
        self
    }

    pub fn with_audio_sample_rate(mut self, audio_sample_rate: u32) -> Self {
        self.audio_sample_rate = audio_sample_rate;
        self
    }

    pub fn with_bpm(mut self, bpm: f64) -> Self {
        self.bpm = vec![Bpm {
            value: bpm,
            time: 0,
        }];
        self
    }

    pub fn with_beat_per_bar(mut self, beat_per_bar: u8) -> Self {
        self.beat_per_bar = vec![BeatPerBar {
            value: beat_per_bar,
            time: 0,
        }];
        self
    }

    pub fn set_note_line(&mut self, id: u16, name: &str) {
        self.note_line_name.insert(id, name.to_string());
    }

    pub fn insert_note(&mut self, sound_id: u16, time: u32, pitch: u8, line: u16) {
        let mut ids: Vec<u16> = Vec::new();

        for n in &self.notes {
            ids.push(n.id);
        }

        if self.notes.len() == 0 {
            self.notes.push(Note {
                id: 0,
                sound_id,
                time,
                pitch,
                line,
            });
        } else {
            for (index, note_id) in ids.iter().enumerate() {
                // If missing number (0:0, 1:1, '2:3', 3:4 ...)
                if index != *note_id as usize {
                    self.notes.push(Note {
                        id: index as u16,
                        sound_id,
                        time,
                        pitch,
                        line,
                    });
                    break;
                }
                // If last index
                else if index == (ids.len() - 1) {
                    self.notes.push(Note {
                        id: (index as u16) + 1,
                        sound_id,
                        time,
                        pitch,
                        line,
                    });
                }
            }
        }
    }
}
