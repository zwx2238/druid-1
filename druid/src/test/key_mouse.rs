use druid::{MouseEvent, post};
use druid_shell::{KbKey, KeyEvent, MouseButton, MouseButtons};
use serde::{Deserialize, Serialize};
use crate::{CLIENT, keyboard_types, Vec2, WidgetId, WindowId};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotEvent {
    pub selector: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    KeyDown(KeyEvent),
    KeyUp(KeyEvent),
    Wheel(MouseEvent),
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
    MouseMove(MouseEvent),
    Screenshot(ScreenshotEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetEvent {
    pub event: Event,
    pub selector: String,
    pub window_id: WindowId,
}


pub fn store_event(selector: String, event: &druid::Event, window_id: WindowId) {
    let event = match event {
        druid::Event::KeyDown(key_event) => {
            Event::KeyDown(key_event.clone())
        }
        druid::Event::KeyUp(key_event) => {
            Event::KeyUp(key_event.clone())
        }
        druid::Event::Wheel(mouse_event) => {
            Event::Wheel(mouse_event.clone())
        }
        druid::Event::MouseDown(mouse_event) => {
            Event::MouseDown(mouse_event.clone())
        }
        druid::Event::MouseUp(mouse_event) => {
            Event::MouseUp(mouse_event.clone())
        }
        druid::Event::MouseMove(mouse_event) => {
            Event::MouseMove(mouse_event.clone())
        }
        _ => {
            unreachable!()
        }
    };
    post("store_event").json(&WidgetEvent { event, selector, window_id }).send().unwrap();
}

pub fn start_record() {
    post("start_record").send().unwrap();
}

pub fn execute(path: String) -> Vec<WidgetEvent> {
    let res = post(format!("execute?path={}", path)).send().unwrap();
    let events = res.json::<Vec<WidgetEvent>>().unwrap();
    events
}


pub fn export(path: String) {
    post(format!("export?path={}", path)).send().unwrap();
}

pub fn screenshot(selector: String) {
    post(format!("screenshot?selector={}", selector)).send().unwrap();
}