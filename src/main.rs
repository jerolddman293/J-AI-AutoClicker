#![allow(dead_code)]
#![allow(unused)]

mod audio_processor;
mod config;
mod utils;
mod voice_transformer;
mod gui;

use audio_processor::AudioProcessor;
use config::Config;
use utils::{initialize_logging, load_audio_file, save_audio_file};
use voice_transformer::{RobotTransformer, PitchShiftTransformer};
use gui::{Window, Slider, Button, TextBox, CheckBox};
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::DerefMut;

fn main() {
    initialize_logging();

    let config = Config::load("config.toml");

    let window = Rc::new(RefCell::new(Window::new("Voice Changer", 800, 600)));
    
    let input_file_textbox = Rc::new(RefCell::new(TextBox::new(20, 20, 200, 30, &config.input_file)));
    let output_file_textbox = Rc::new(RefCell::new(TextBox::new(20, 60, 200, 30, &config.output_file)));
    
    let intensity_slider = Rc::new(RefCell::new(Slider::new(20, 100, 300, 30, config.intensity)));
    let pitch_slider = Rc::new(RefCell::new(Slider::new(20, 140, 300, 30, config.pitch)));
    
    let transformer_selector = Rc::new(RefCell::new(TextBox::new(20, 180, 200, 30, &config.transformer)));

    let start_button = Button::new(20, 220, 100, 40, "Start", {
        let input_file_textbox = Rc::clone(&input_file_textbox);
        let output_file_textbox = Rc::clone(&output_file_textbox);
        let transformer_selector = Rc::clone(&transformer_selector);
        let intensity_slider = Rc::clone(&intensity_slider);
        let pitch_slider = Rc::clone(&pitch_slider);

        move || {
            let audio_data = load_audio_file(&input_file_textbox.borrow().get_text());

            let processor = AudioProcessor::new();
            let transformed_audio = match transformer_selector.borrow().get_text().as_str() {
                "robot" => processor.apply_transform(audio_data, RobotTransformer::new(intensity_slider.borrow().get_value())),
                "pitch_shift" => processor.apply_transform(audio_data, PitchShiftTransformer::new(pitch_slider.borrow().get_value())),
                _ => audio_data,
            };

            save_audio_file(&output_file_textbox.borrow().get_text(), transformed_audio);
        }
    });

    let realtime_checkbox = CheckBox::new(20, 260, "Real-time", false);

    let realtime_button = Button::new(20, 300, 100, 40, "Enable Realtime", {
        let input_file_textbox = Rc::clone(&input_file_textbox);
        let output_file_textbox = Rc::clone(&output_file_textbox);
        let transformer_selector = Rc::clone(&transformer_selector);
        let intensity_slider = Rc::clone(&intensity_slider);
        let pitch_slider = Rc::clone(&pitch_slider);

        move || {
            if realtime_checkbox.is_checked() {
                let audio_data = load_audio_file(&input_file_textbox.borrow().get_text());
                let processor = AudioProcessor::new();
                let transformed_audio = match transformer_selector.borrow().get_text().as_str() {
                    "robot" => processor.apply_transform(audio_data, RobotTransformer::new(intensity_slider.borrow().get_value())),
                    "pitch_shift" => processor.apply_transform(audio_data, PitchShiftTransformer::new(pitch_slider.borrow().get_value())),
                    _ => audio_data,
                };

                save_audio_file(&output_file_textbox.borrow().get_text(), transformed_audio);
            }
        }
    });

    let volume_slider = Rc::new(RefCell::new(Slider::new(20, 340, 300, 30, 1.0)));
    let pan_slider = Rc::new(RefCell::new(Slider::new(20, 380, 300, 30, 0.5)));

    let reset_button = Button::new(20, 420, 100, 40, "Reset", {
        let intensity_slider = Rc::clone(&intensity_slider);
        let pitch_slider = Rc::clone(&pitch_slider);
        let volume_slider = Rc::clone(&volume_slider);
        let pan_slider = Rc::clone(&pan_slider);
        let transformer_selector = Rc::clone(&transformer_selector);
        let input_file_textbox = Rc::clone(&input_file_textbox);
        let output_file_textbox = Rc::clone(&output_file_textbox);

        move || {
            intensity_slider.borrow_mut().set_value(0.8);
            pitch_slider.borrow_mut().set_value(1.0);
            volume_slider.borrow_mut().set_value(1.0);
            pan_slider.borrow_mut().set_value(0.5);
            transformer_selector.borrow_mut().set_text("robot");
            input_file_textbox.borrow_mut().set_text("input.wav");
            output_file_textbox.borrow_mut().set_text("output.wav");
        }
    });

    window.borrow_mut().add(None, None, Some(input_file_textbox.borrow_mut().deref_mut()), None);
    window.borrow_mut().add(None, None, Some(output_file_textbox.borrow_mut().deref_mut()), None);
    window.borrow_mut().add(None, Some(intensity_slider.borrow_mut().deref_mut()), None, None);
    window.borrow_mut().add(None, Some(pitch_slider.borrow_mut().deref_mut()), None, None);
    window.borrow_mut().add(None, None, Some(transformer_selector.borrow_mut().deref_mut()), None);
    window.borrow_mut().add(Some(start_button), None, None, None);
    window.borrow_mut().add(Some(realtime_button), None, None, None);
    window.borrow_mut().add(None, Some(volume_slider.borrow_mut().deref_mut()), None, None);
    window.borrow_mut().add(None, Some(pan_slider.borrow_mut().deref_mut()), None, None);
    window.borrow_mut().add(Some(reset_button), None, None, None);

    window.borrow_mut().show();
}
