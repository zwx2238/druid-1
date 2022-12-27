use serde::{Deserialize, Serialize};
use crate::CLIENT;

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Element {
    pub selector: String,
    pub layout: Layout,
}

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Layout {
    pub(crate) origin: Point,
    size: Size,
}

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Size {
    w: f64,
    h: f64,
}

use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;
use druid::{post, WidgetId};

lazy_static! {
    static ref CACHE :Mutex<HashMap<String,Element>>= Mutex::new(HashMap::new());
}

pub fn update_layout(id: WidgetId, origin: druid_shell::kurbo::Point, size: druid_shell::kurbo::Size) {
    if let Some(selector) = ID_SELECTOR.lock().unwrap().get(&id) {
        let element = Element {
            selector: selector.clone(),
            layout: Layout {
                origin: Point {
                    x: origin.x,
                    y: origin.y,
                },
                size: Size {
                    w: size.width,
                    h: size.height,
                },
            },
        };
        let mut cache = CACHE.lock().unwrap();
        match cache.get(&element.selector) {
            Some(entry) => {
                if *entry != element {
                    post("update_layout").json(&element).send().unwrap();
                }
            }
            None => {
                post("update_layout").json(&element).send().unwrap();
            }
        }
        cache.insert(element.selector.clone(), element.clone());
    }
}

use crate::ID_SELECTOR;

pub fn get_layout(selector: String) -> Layout {
    CACHE.lock().unwrap().get(&selector).unwrap().layout.clone()
}