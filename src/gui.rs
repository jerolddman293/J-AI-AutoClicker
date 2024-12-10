#![allow(dead_code)]
#![allow(unused)]

use std::cell::RefCell;
use std::option;
use std::rc::Rc;

pub struct Window {
    title: String,
    width: u32,
    height: u32,
    components: Vec<Rc<RefCell<dyn GUIComponent>>>,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Window {
            title: title.to_string(),
            width,
            height,
            components: Vec::new(),
        }
    }

    
    pub fn add(&mut self, button: Option<Button>, slider: Option<&mut Slider>, textbox: Option<&mut TextBox>, checkbox: Option<CheckBox>) {
        self.components = Vec::new();
    }

    pub fn show(&self) {

    }
}

pub trait GUIComponent {
    fn draw(&self);
    fn set_position(&mut self, x: u32, y: u32);
}

#[derive(Clone)]
pub struct Slider {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    value: f32,
}

impl Slider {
    pub fn new(x: u32, y: u32, width: u32, height: u32, initial_value: f32) -> Self {
        Slider {
            x,
            y,
            width,
            height,
            value: initial_value,
        }
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    }
}

impl GUIComponent for Slider {
    fn draw(&self) {

    }

    fn set_position(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}

pub struct Button {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    label: String,
    action: Box<dyn Fn()>,
}

impl Button {
    pub fn new(x: u32, y: u32, width: u32, height: u32, label: &str, action: impl Fn() + 'static) -> Self {
        Button {
            x,
            y,
            width,
            height,
            label: label.to_string(),
            action: Box::new(action),
        }
    }

    pub fn click(&self) {
        (self.action)();
    }
}

impl GUIComponent for Button {
    fn draw(&self) {

    }

    fn set_position(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}

#[derive(Clone)]
pub struct TextBox {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    text: String,
}

impl TextBox {
    pub fn new(x: u32, y: u32, width: u32, height: u32, initial_text: &str) -> Self {
        TextBox {
            x,
            y,
            width,
            height,
            text: initial_text.to_string(),
        }
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}

impl GUIComponent for TextBox {
    fn draw(&self) {

    }

    fn set_position(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}

pub struct Label {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    text: String,
}

impl Label {
    pub fn new(x: u32, y: u32, width: u32, height: u32, text: &str) -> Self {
        Label {
            x,
            y,
            width,
            height,
            text: text.to_string(),
        }
    }
}

impl GUIComponent for Label {
    fn draw(&self) {

    }

    fn set_position(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}

#[derive(Clone)]
pub struct CheckBox {
    x: u32,
    y: u32,
    label: String,
    checked: bool,
}

impl CheckBox {
    pub fn new(x: u32, y: u32, label: &str, checked: bool) -> Self {
        CheckBox {
            x,
            y,
            label: label.to_string(),
            checked,
        }
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn toggle(&mut self) {
        self.checked = !self.checked;
    }
}

impl GUIComponent for CheckBox {
    fn draw(&self) {
        
    }

    fn set_position(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}
