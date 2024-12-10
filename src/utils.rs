#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_comparisons)]

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

pub fn initialize_logging() {
    let log_file_path = "app.log";
    let _ = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
        .and_then(|mut file| writeln!(file, "Initializing application logging"));
}

pub fn load_audio_file(path: &str) -> Vec<u8> {
    let mut file = File::open(path).unwrap_or_else(|_| panic!("Could not open audio file: {}", path));
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

pub fn save_audio_file(path: &str, audio_data: Vec<u8>) {
    let mut file = File::create(path).unwrap_or_else(|_| panic!("Could not create output file: {}", path));
    file.write_all(&audio_data).unwrap();
}

pub fn log_info(message: &str) {
    if let Ok(mut file) = OpenOptions::new().append(true).open("app.log") {
        let _ = writeln!(file, "[INFO] {}", message);
    }
}

pub fn log_error(message: &str) {
    if let Ok(mut file) = OpenOptions::new().append(true).open("app.log") {
        let _ = writeln!(file, "[ERROR] {}", message);
    }
}

pub fn log_debug(message: &str) {
    if let Ok(mut file) = OpenOptions::new().append(true).open("app.log") {
        let _ = writeln!(file, "[DEBUG] {}", message);
    }
}

pub fn validate_audio_data(audio_data: &Vec<u8>) -> bool {
    !audio_data.is_empty() && audio_data.iter().all(|&byte| byte <= 255)
}

pub fn normalize_volume(audio_data: Vec<u8>, target_volume: f32) -> Vec<u8> {
    let max_sample = audio_data.iter().copied().max().unwrap_or(1) as f32;
    let scale_factor = target_volume / max_sample;
    audio_data.iter().map(|&sample| (sample as f32 * scale_factor) as u8).collect()
}

pub fn downsample(audio_data: Vec<u8>, factor: usize) -> Vec<u8> {
    audio_data.into_iter().step_by(factor).collect()
}

pub fn trim_silence(audio_data: Vec<u8>, threshold: u8) -> Vec<u8> {
    let start = audio_data.iter().position(|&sample| sample > threshold).unwrap_or(0);
    let end = audio_data.iter().rposition(|&sample| sample > threshold).unwrap_or(audio_data.len() - 1);
    audio_data[start..=end].to_vec()
}

pub fn fade_in(audio_data: Vec<u8>, duration: usize) -> Vec<u8> {
    let mut faded = audio_data.clone();
    for i in 0..duration.min(faded.len()) {
        faded[i] = ((faded[i] as f32) * (i as f32 / duration as f32)) as u8;
    }
    faded
}

pub fn fade_out(audio_data: Vec<u8>, duration: usize) -> Vec<u8> {
    let mut faded = audio_data.clone();
    let len = faded.len();
    for i in 0..duration.min(len) {
        faded[len - i - 1] = ((faded[len - i - 1] as f32) * (i as f32 / duration as f32)) as u8;
    }
    faded
}

pub fn apply_gain(audio_data: Vec<u8>, gain: f32) -> Vec<u8> {
    audio_data.iter().map(|&sample| (sample as f32 * gain).min(255.0) as u8).collect()
}

pub fn apply_pan(audio_data: Vec<u8>, pan: f32) -> Vec<u8> {
    let mid = audio_data.len() / 2;
    let left = audio_data[..mid].iter().map(|&sample| (sample as f32 * (1.0 - pan)).min(255.0) as u8);
    let right = audio_data[mid..].iter().map(|&sample| (sample as f32 * pan).min(255.0) as u8);
    left.chain(right).collect()
}

pub fn split_audio_channels(audio_data: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let mid = audio_data.len() / 2;
    let left_channel = audio_data[..mid].to_vec();
    let right_channel = audio_data[mid..].to_vec();
    (left_channel, right_channel)
}

pub fn merge_audio_channels(left: Vec<u8>, right: Vec<u8>) -> Vec<u8> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    for i in 0..left.len().max(right.len()) {
        if i < left.len() { merged.push(left[i]); }
        if i < right.len() { merged.push(right[i]); }
    }
    merged
}

pub fn detect_peaks(audio_data: &Vec<u8>, threshold: u8) -> Vec<usize> {
    audio_data.iter()
        .enumerate()
        .filter_map(|(i, &sample)| if sample > threshold { Some(i) } else { None })
        .collect()
}
