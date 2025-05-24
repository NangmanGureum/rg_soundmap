use serde::{Deserialize, Serialize};

/// A sound definition for the chart.
///
/// In `Silent(u32)`, this is the note sounds silent.
/// and it includes time information.
/// The time information appears in increments of 264ths of a beat.
///
/// In `Sound(u16)`, this is the note sounds playing sound.
/// and it includes note information on `SoundMap`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sound {
    #[serde(alias = "silent_note_time")]
    Silent(u32),
    #[serde(alias = "smap_id")]
    Sound(u16),
}

/// A note definition for the chart.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayNote {
    /// A note definition of sound.
    pub sound: Sound,

    /// A note type
    ///
    /// It depends on the chart type. but `0` is most normal note.
    ///
    /// ## Example of definition
    ///
    /// | # | Type name | Description |
    /// | - | --------- | ----------- |
    /// | 0 | Normal              | Normal note |
    /// | 1 | Flick               | Flick note |
    /// | 2 | Hold Start          | Start of hold note |
    /// | 3 | Hold End            | End of hold note |
    /// | 4 | Hold End with flick | End of hold note with flick note |
    /// | 5 | Slide Start         | Start of slide note |
    /// | 6 | Slide End           | End of slide note |
    /// | 7 | Slide End with flick | End of slide note with flick note |
    pub note_type: u8,

    /// A note group
    /// It depends on the note type. but `0` is using for normal note.
    pub group: u8,

    /// A note position(line) on the chart
    /// It depends on the chart type
    pub position: u8,
}

impl Default for PlayNote {
    fn default() -> Self {
        Self {
            sound: Sound::Silent(0),
            note_type: 0,
            group: 0,
            position: 0,
        }
    }
}

impl PlayNote {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_sound(mut self, smap_note_id: u16) -> Self {
        self.sound = Sound::Sound(smap_note_id);
        self
    }

    pub fn with_time(mut self, time: u32) -> Self {
        self.sound = Sound::Silent(time);
        self
    }

    pub fn with_type(mut self, note_type: u8) -> Self {
        self.note_type = note_type;
        self
    }

    pub fn with_group(mut self, note_group: u8) -> Self {
        self.group = note_group;
        self
    }

    pub fn with_position(mut self, note_position: u8) -> Self {
        self.position = note_position;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
    /// A name of chart
    pub name: String,

    /// A type of chart
    pub chart_type: String,

    /// A author of chart
    pub author: String,

    /// A difficulty type of chart
    /// It depends on the chart type.
    pub difficulty_type: u8,

    /// A difficulty level of chart
    /// It depends on the chart type.
    pub difficulty_level: u8,

    /// Notes on the chart
    pub content: Vec<PlayNote>,

    /// Variation (In BMS, called 'sabun(差分)') or not
    pub variation: bool,
}

impl Default for Chart {
    fn default() -> Self {
        Self {
            name: "Chart".to_string(),
            author: "Unknown".to_string(),
            chart_type: "Plain".to_string(),
            difficulty_type: 0,
            difficulty_level: 1,
            content: vec![],
            variation: false,
        }
    }
}

impl Chart {
    pub fn new(name: &str, author: &str) -> Self {
        let mut chart = Self::default();
        chart.name = name.to_string();
        chart.author = author.to_string();
        chart
    }

    pub fn with_chart_type(mut self, chart_type: &str) -> Self {
        self.chart_type = chart_type.to_string();
        self
    }

    pub fn with_difficulty_type(mut self, diff_type: u8) -> Self {
        self.difficulty_type = diff_type;
        self
    }

    pub fn with_level(mut self, diff_level: u8) -> Self {
        self.difficulty_level = diff_level;
        self
    }

    pub fn variation(mut self) -> Self {
        self.variation = true;
        self
    }

    pub fn insert_note(&mut self, pos: u8, smap_note_id: u16) {
        let note = PlayNote::new().with_position(pos).with_sound(smap_note_id);
        self.content.push(note);
    }

    pub fn insert_silent_note(&mut self, pos: u8, time: u32) {
        let note = PlayNote::new().with_position(pos).with_time(time);
        self.content.push(note);
    }
}
