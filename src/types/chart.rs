use serde::{Deserialize, Serialize};

/// A sound definition for the chart.
///
/// `sn_note_id` is the ID of the note in the content of soundmap.
/// `time` is the time of the sound in the chart.
///
/// If `smap_note_id` is `Some(u16)`, it means that the sound is associated with a specific note. and `time` is unused. but it recommends to be same as the note of soundmap defined.
/// If `smap_note_id` is `None`, it means that the sound is not associated with any specific note. instead `time` is used for specific note timing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteSound {
    pub smap_note_id: Option<u16>,
    pub time: u32,
}

impl Default for NoteSound {
    fn default() -> Self {
        Self {
            smap_note_id: None,
            time: 0,
        }
    }
}

/// A note definition for the chart.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayNote {
    /// A note definition of sound.
    pub sound: NoteSound,

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

    /// A note's lane on the chart
    /// It depends on the chart type
    pub lane: u8,
}

impl Default for PlayNote {
    fn default() -> Self {
        Self {
            sound: NoteSound::default(),
            note_type: 0,
            group: 0,
            lane: 0,
        }
    }
}

impl PlayNote {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_sound(mut self, smap_note_id: u16) -> Self {
        self.sound = NoteSound {
            smap_note_id: Some(smap_note_id),
            time: 0,
        };
        self
    }

    pub fn with_time(mut self, time: u32) -> Self {
        self.sound = NoteSound {
            smap_note_id: None,
            time,
        };
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

    pub fn with_lane(mut self, note_lane: u8) -> Self {
        self.lane = note_lane;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

    pub fn insert_note(&mut self, lane: u8, smap_note_id: u16) {
        let note = PlayNote::new().with_lane(lane).with_sound(smap_note_id);
        self.content.push(note);
    }

    pub fn insert_silent_note(&mut self, lane: u8, time: u32) {
        let note = PlayNote::new().with_lane(lane).with_time(time);
        self.content.push(note);
    }
}
