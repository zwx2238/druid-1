use druid::{store_event, Widget};
use crate::{BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, WidgetId};

pub struct DebugContainer<T> {
    inner: Box<dyn Widget<T>>,
    selector: String,
    id: WidgetId,
}

impl<T> Widget<T> for DebugContainer<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::MouseDown(mouse_event) => {
                if !mouse_event.from_command {
                    store_event(self.selector.clone(), event, ctx.window_id());
                }
            }
            Event::MouseUp(mouse_event) => {
                if !mouse_event.from_command {
                    store_event(self.selector.clone(), event, ctx.window_id());
                }
            }
            Event::MouseMove(mouse_event) => {
                if !mouse_event.from_command {
                    store_event(self.selector.clone(), event, ctx.window_id());
                }
            }
            Event::Wheel(mouse_event) => {
                if !mouse_event.from_command {
                    store_event(self.selector.clone(), event, ctx.window_id());
                }
            }
            Event::KeyDown(key_event) => {
                if !key_event.from_command {
                    store_event(self.selector.clone(), event, ctx.window_id());
                }
            }
            Event::KeyUp(key_event) => {
                if !key_event.from_command {
                    store_event(self.selector.clone(), event, ctx.window_id());
                }
            }
            _ => {}
        }
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.inner.paint(ctx, data, env);
    }

    fn id(&self) -> Option<WidgetId> {
        Some(self.id)
    }
}

pub trait DebugExt<T> {
    fn debug_str(self, selector: &'static str) -> DebugContainer<T>;

    fn debug(self, selector: String) -> DebugContainer<T>;
}

use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;

lazy_static! {
    pub static ref ID_SELECTOR: Mutex<HashMap<WidgetId,String>> = Mutex::new(HashMap::new());
}

impl<T, W: Widget<T> + 'static> DebugExt<T> for W {
    fn debug_str(self, selector: &'static str) -> DebugContainer<T> {
        let id = WidgetId::next();
        ID_SELECTOR.lock().unwrap().insert(id, selector.to_string());
        DebugContainer {
            id,
            inner: Box::new(self),
            selector: selector.to_string(),
        }
    }

    fn debug(self, selector: String) -> DebugContainer<T> {
        let id = WidgetId::next();
        ID_SELECTOR.lock().unwrap().insert(id, selector.clone());
        DebugContainer {
            id,
            inner: Box::new(self),
            selector,
        }
    }
}