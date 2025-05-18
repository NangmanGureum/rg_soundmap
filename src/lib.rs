pub mod types;

use std::fs;
use std::io;
use std::path::Path;
use types::manifest;
use types::{Chart, Manifest, SoundMap};

/// Load soundmap format files.
pub fn load_smap_dir(smap_path: &str) -> io::Result<(Manifest, SoundMap, Vec<Chart>)> {
    // Load manifest
    let manifest_path = format!("{smap_path}/manifest.json");
    let manifest = fs::read_to_string(&manifest_path)?;
    let manifest: Manifest = serde_json::from_str(&manifest)?;

    // Load soundmap
    let soundmap_path = format!("{smap_path}/soundmap.json");
    let soundmap = fs::read_to_string(&soundmap_path)?;
    let soundmap: SoundMap = serde_json::from_str(&soundmap)?;

    // Load charts
    let charts_dir = format!("{smap_path}/charts");
    let mut charts = Vec::new();
    for entry in fs::read_dir(&charts_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let chart = fs::read_to_string(path)?;
            let chart: Chart = serde_json::from_str(&chart)?;
            charts.push(chart);
        }
    }

    Ok((manifest, soundmap, charts))
}

/// Generate soundmap format files.
pub fn save_smap_dir(
    smap_name: &str,
    save_path: &str,
    manifest: &Manifest,
    soundmap: &SoundMap,
    charts: &Vec<Chart>,
) -> io::Result<()> {
    // Make a soundmap format directory
    let format_path = format!("{save_path}/{smap_name}");
    let charts_dir = format!("{format_path}/charts");
    fs::create_dir(&format_path)?;
    fs::create_dir(&charts_dir)?;

    // Also, create sound directory
    let sound_path = format!("{format_path}/sounds");
    fs::create_dir(sound_path)?;

    // Save manifest
    let manifest_path = format!("{format_path}/manifest.json");
    fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?)?;

    // Save soundmap
    let soundmap_path = format!("{format_path}/soundmap.json");
    fs::write(&soundmap_path, serde_json::to_string_pretty(&soundmap)?)?;

    // Save charts
    for chart in charts {
        let chart_path = format!("{charts_dir}/{}.json", chart.name);
        fs::write(&chart_path, serde_json::to_string_pretty(&chart)?)?;
    }

    Ok(())
}

// Pack to `*.smap` file. It uses tar with lz4 compression.
// pub fn pack(smap_path: &str) {
//     to be implemented
// }

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn load_smap() {
        let (manifest, soundmap, charts) = load_smap_dir("test_files/example").unwrap();

        // Check manifest
        assert_eq!(manifest.title, "Example");
        assert_eq!(manifest.artists[0], "Example");
        assert_eq!(manifest.sounds[&0], "kick.wav");

        // Check soundmap
        assert_eq!(soundmap.audio_format, "wav");
        assert_eq!(soundmap.audio_bits, 24);
        assert_eq!(soundmap.audio_sample_rate, 48000);
        assert_eq!(soundmap.notes.len(), 4);
        assert_eq!(soundmap.notes[0].time, 1056);
        assert_eq!(soundmap.notes[1].time, 1320);

        // Check charts
        assert_eq!(charts.len(), 1);
    }

    #[test]
    fn save_smap() {
        let dir_name = "test_files/test";

        // If the dir exists, delete it
        if Path::new(dir_name).exists() {
            fs::remove_dir_all(dir_name).unwrap();
        }

        let new_manifest = Manifest::new("Test", "Various Artists");
        let new_soundmap = SoundMap::new();
        let new_charts: Vec<Chart> = Vec::new();

        save_smap_dir(
            "test",
            "test_files",
            &new_manifest,
            &new_soundmap,
            &new_charts,
        )
        .unwrap();

        // Recheck which saved
        let (saved_manifest, saved_soundmap, saved_charts) = load_smap_dir(dir_name).unwrap();

        // Check manifest
        assert_eq!(saved_manifest.title, "Test");
        assert_eq!(saved_manifest.artists[0], "Various Artists");

        // Check soundmap
        assert_eq!(saved_soundmap.audio_format, "wav");
        assert_eq!(saved_soundmap.audio_bits, 24);
        assert_eq!(saved_soundmap.audio_sample_rate, 48000);

        // Check charts
        assert_eq!(saved_charts.len(), 0);
    }
}
