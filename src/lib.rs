pub mod types;

use lz4::{Decoder, EncoderBuilder};
use std::fs::{self, File};
use std::io;
use std::path::Path;
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

/// Check soundmap directory
pub fn check_smap(smap_path: &str) -> Result<(), String> {
    // Set directory path
    let manifest_path = format!("{smap_path}/manifest.json");
    let soundmap_path = format!("{smap_path}/soundmap.json");
    let charts_dir_path = format!("{smap_path}/charts");

    // Check manifest if valid
    match fs::read_to_string(&manifest_path) {
        Ok(m) => match serde_json::from_str::<Manifest>(&m) {
            Ok(_manifest) => {}
            Err(e) => return Err(format!("Failed to parse manifest: {}", e)),
        },
        Err(e) => return Err(format!("Failed to read manifest: {}", e)),
    }

    // Check soundmap if valid
    match fs::read_to_string(&soundmap_path) {
        Ok(s) => match serde_json::from_str::<SoundMap>(&s) {
            Ok(_soundmap) => {}
            Err(e) => return Err(format!("Failed to parse soundmap: {}", e)),
        },
        Err(e) => return Err(format!("Failed to read soundmap: {}", e)),
    }

    // Check charts if valid
    if Path::new(&charts_dir_path).exists() {
        for entry in fs::read_dir(&charts_dir_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                match fs::read_to_string(&path.to_str().unwrap()) {
                    Ok(c) => match serde_json::from_str::<Chart>(&c) {
                        Ok(_chart) => {}
                        Err(e) => return Err(format!("Failed to parse chart: {}", e)),
                    },
                    Err(e) => return Err(format!("Failed to read chart: {}", e)),
                }
            }
        }
    } else {
        return Err("Cannot find charts directory".to_string());
    }

    Ok(())
}

/// Pack to `*.smap`(or starts with something) file. It uses tar with lz4 compression.
pub fn pack(target_path: &str, smap_dir_name: &str, filename: &str) -> io::Result<()> {
    let smap_filename = format!("{target_path}/{filename}");
    let smap_dir_path = format!("{target_path}/{smap_dir_name}");
    let temp_tar_name = format!("{target_path}/_temp.tar");

    // Make temp tar.
    let mut tar_file = File::create(&temp_tar_name)?;
    let mut temp_tar = tar::Builder::new(&mut tar_file);

    let target_charts_path = format!("{smap_dir_path}/charts");
    let target_sounds_path = format!("{smap_dir_path}/sounds");

    temp_tar.append_file(
        "manifest.json",
        &mut File::open(format!("{smap_dir_path}/manifest.json")).unwrap(),
    )?;
    temp_tar.append_file(
        "soundmap.json",
        &mut File::open(format!("{smap_dir_path}/soundmap.json")).unwrap(),
    )?;

    temp_tar.append_dir(format!("charts"), ".")?;
    temp_tar.append_dir(format!("sounds"), ".")?;

    let charts_ls = fs::read_dir(target_charts_path).unwrap();

    for dir_entry in charts_ls {
        let path = dir_entry.unwrap().path();
        let chart_path_str = path.to_str().unwrap();
        let chart_name = &chart_path_str.split('/').last().unwrap();
        let chart_tar_path = format!("charts/{chart_name}");
        temp_tar.append_file(chart_tar_path, &mut File::open(chart_path_str).unwrap())?;
    }

    let sounds_ls = fs::read_dir(target_sounds_path).unwrap();

    for dir_entry in sounds_ls {
        let path = dir_entry.unwrap().path();
        let sound_path_str = path.to_str().unwrap();
        let sound_name = &sound_path_str.split('/').last().unwrap();
        let sound_tar_path = format!("sounds/{sound_name}");
        temp_tar.append_file(sound_tar_path, &mut File::open(sound_path_str).unwrap())?;
    }

    temp_tar.finish()?;

    // Comression with LZ4
    let mut input_file = File::open(&temp_tar_name)?;
    let output_file = File::create(smap_filename)?;
    let mut encoder = EncoderBuilder::new().level(4).build(output_file)?;
    std::io::copy(&mut input_file, &mut encoder)?;
    let (_output, result) = encoder.finish();

    fs::remove_file(&temp_tar_name)?;
    fs::remove_dir_all(&smap_dir_path)?;

    result
}

/// Pack to `*.smap`(or starts with something) file. It uses tar with lz4 compression.
pub fn unpack(smap_file_path: &str, save_path: &str) -> io::Result<()> {
    let temp_tar_name = format!("{save_path}/_temp.tar");

    let input_file = File::open(smap_file_path)?;
    let mut decoder = Decoder::new(input_file)?;
    let mut temp_tar = File::create(&temp_tar_name)?;
    io::copy(&mut decoder, &mut temp_tar)?;

    let temp_tar_file = File::open(&temp_tar_name)?;
    let mut temp_tar = tar::Archive::new(temp_tar_file);
    temp_tar.unpack(save_path)?;

    fs::remove_file(&temp_tar_name)?;

    Ok(())
}

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

    #[test]
    fn check_smap_valid() {
        match check_smap("test_files/example") {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
    }

    // Pack and unpack soundmap test
    #[test]
    fn pack_smap() {
        let test_dir = "test_files";
        let smap_name = "pack_test";
        let smap_path = format!("{test_dir}/{smap_name}");
        let filename = format!("{smap_name}.smap");
        let smap_file_path = format!("{test_dir}/{filename}");
        let smap_result_path = format!("{test_dir}/unpack_result");

        // If the file and directory exists, delete it
        if Path::new(&smap_file_path).exists() {
            fs::remove_file(&smap_file_path).unwrap();
        }
        if Path::new(&smap_result_path).exists() {
            fs::remove_dir_all(&smap_result_path).unwrap();
        }

        let new_manifest = Manifest::new("Test", "Various Artists");
        let new_soundmap = SoundMap::new();
        let new_charts: Vec<Chart> = Vec::new();

        save_smap_dir(
            smap_name,
            test_dir,
            &new_manifest,
            &new_soundmap,
            &new_charts,
        )
        .unwrap();

        pack(test_dir, smap_name, &filename).unwrap();

        // Unpack `*.smap` file
        fs::create_dir(&smap_result_path).unwrap();
        unpack(&smap_file_path, &smap_result_path).unwrap();

        // Recheck which saved
        let (saved_manifest, saved_soundmap, saved_charts) =
            load_smap_dir(&smap_result_path).unwrap();

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
