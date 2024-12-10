#![allow(dead_code)]
#![allow(unused)]

pub trait Transformer {
    fn transform(&self, audio_data: Vec<u8>) -> Vec<u8>;
}

pub struct RobotTransformer {
    intensity: f32,
}

impl RobotTransformer {
    pub fn new(intensity: f32) -> Self {
        RobotTransformer { intensity }
    }
}

impl Transformer for RobotTransformer {
    fn transform(&self, audio_data: Vec<u8>) -> Vec<u8> {
        audio_data.iter().map(|&sample| (sample as f32 * self.intensity) as u8).collect()
    }
}

pub struct PitchShiftTransformer {
    pitch: f32,
}

impl PitchShiftTransformer {
    pub fn new(pitch: f32) -> Self {
        PitchShiftTransformer { pitch }
    }
}

impl Transformer for PitchShiftTransformer {
    fn transform(&self, audio_data: Vec<u8>) -> Vec<u8> {
        audio_data.iter().map(|&sample| (sample as f32 * self.pitch) as u8).collect()
    }
}
