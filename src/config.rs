#![allow(dead_code)]
#![allow(unused)]

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub struct Config {
    pub input_file: String,
    pub output_file: String,
    pub transformer: String,
    pub intensity: f32,
    pub pitch: f32,
    pub sample_rate: u32,
    pub bit_depth: u16,
    pub channels: u8,
    pub reverb_intensity: f32,
    pub echo_delay: u32,
    pub echo_decay: f32,
    pub gain: f32,
    pub pan: f32,
    pub low_gain: f32,
    pub mid_gain: f32,
    pub high_gain: f32,
    pub threshold: u8,
    pub compression_ratio: f32,
    pub fade_in_duration: usize,
    pub fade_out_duration: usize,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        let config_data = Self::read_config_file(path).unwrap_or_else(|_| String::from("[default config data]"));
        Self::parse_config_data(&config_data)
    }

    fn read_config_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn parse_config_data(data: &str) -> Self {
        Config {
            input_file: Self::get_value(data, "input_file", "input.wav"),
            output_file: Self::get_value(data, "output_file", "output.wav"),
            transformer: Self::get_value(data, "transformer", "robot"),
            intensity: Self::get_float(data, "intensity", 0.8),
            pitch: Self::get_float(data, "pitch", 1.0),
            sample_rate: Self::get_u32(data, "sample_rate", 44100),
            bit_depth: Self::get_u16(data, "bit_depth", 16),
            channels: Self::get_u8(data, "channels", 2),
            reverb_intensity: Self::get_float(data, "reverb_intensity", 0.5),
            echo_delay: Self::get_u32(data, "echo_delay", 500),
            echo_decay: Self::get_float(data, "echo_decay", 0.3),
            gain: Self::get_float(data, "gain", 1.0),
            pan: Self::get_float(data, "pan", 0.5),
            low_gain: Self::get_float(data, "low_gain", 1.0),
            mid_gain: Self::get_float(data, "mid_gain", 1.0),
            high_gain: Self::get_float(data, "high_gain", 1.0),
            threshold: Self::get_u8(data, "threshold", 128),
            compression_ratio: Self::get_float(data, "compression_ratio", 2.0),
            fade_in_duration: Self::get_usize(data, "fade_in_duration", 5000),
            fade_out_duration: Self::get_usize(data, "fade_out_duration", 5000),
        }
    }

    fn get_value(data: &str, key: &str, default: &str) -> String {
        data.lines()
            .find(|line| line.starts_with(key))
            .and_then(|line| line.split('=').nth(1))
            .map(|val| val.trim().to_string())
            .unwrap_or_else(|| default.to_string())
    }

    fn get_float(data: &str, key: &str, default: f32) -> f32 {
        data.lines()
            .find(|line| line.starts_with(key))
            .and_then(|line| line.split('=').nth(1))
            .and_then(|val| val.trim().parse().ok())
            .unwrap_or(default)
    }

    fn get_u32(data: &str, key: &str, default: u32) -> u32 {
        data.lines()
            .find(|line| line.starts_with(key))
            .and_then(|line| line.split('=').nth(1))
            .and_then(|val| val.trim().parse().ok())
            .unwrap_or(default)
    }

    fn get_u16(data: &str, key: &str, default: u16) -> u16 {
        data.lines()
            .find(|line| line.starts_with(key))
            .and_then(|line| line.split('=').nth(1))
            .and_then(|val| val.trim().parse().ok())
            .unwrap_or(default)
    }

    fn get_u8(data: &str, key: &str, default: u8) -> u8 {
        data.lines()
            .find(|line| line.starts_with(key))
            .and_then(|line| line.split('=').nth(1))
            .and_then(|val| val.trim().parse().ok())
            .unwrap_or(default)
    }

    fn get_usize(data: &str, key: &str, default: usize) -> usize {
        data.lines()
            .find(|line| line.starts_with(key))
            .and_then(|line| line.split('=').nth(1))
            .and_then(|val| val.trim().parse().ok())
            .unwrap_or(default)
    }
}
