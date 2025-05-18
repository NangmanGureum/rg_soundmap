//! Soundmap format handling and processing
//!
//! This module provides functionality for parsing, representing, and manipulating soundmap data.

pub mod chart;
pub mod manifest;
pub mod soundmap;

pub mod prelude {
    pub use crate::types::chart::Chart;
    pub use crate::types::manifest::Manifest;
    pub use crate::types::soundmap::SoundMap;
}

pub use crate::types::prelude::*;
