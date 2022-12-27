// Copyright 2018 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Simple calculator.

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]
#![feature(async_closure)]

use std::thread::{sleep, spawn};
use std::time::Duration;
use lazy_static::lazy_static;
use druid::{theme, AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt, WindowDesc, DebugExt, AppDelegate, DelegateCtx, WindowId, Event, Env, execute, start_record, export, Target, Command, Handled, WindowConfig};
use druid::commands::{EXECUTE_EVENT, EXECUTE_EVENTS};
use druid::Target::Auto;

use druid::widget::{CrossAxisAlignment, Flex, Label, Painter, TextBox};
use druid_shell::KbKey;

#[derive(Clone, Data, Lens)]
struct CalcState {
    /// The number displayed. Generally a valid float.
    value: String,
    operand: f64,
    operator: char,
    in_num: bool,
    text: String,
}

impl CalcState {
    fn digit(&mut self, digit: u8) {
        if !self.in_num {
            self.value.clear();
            self.in_num = true;
        }
        let ch = (b'0' + digit) as char;
        self.value.push(ch);
    }

    fn display(&mut self) {
        // TODO: change hyphen-minus to actual minus
        self.value = self.operand.to_string();
    }

    fn compute(&mut self) {
        if self.in_num {
            let operand2 = self.value.parse().unwrap_or(0.0);
            let result = match self.operator {
                '+' => Some(self.operand + operand2),
                '−' => Some(self.operand - operand2),
                '×' => Some(self.operand * operand2),
                '÷' => Some(self.operand / operand2),
                _ => None,
            };
            if let Some(result) = result {
                self.operand = result;
                self.display();
                self.in_num = false;
            }
        }
    }

    fn op(&mut self, op: char) {
        match op {
            '+' | '−' | '×' | '÷' | '=' => {
                self.compute();
                self.operand = self.value.parse().unwrap_or(0.0);
                self.operator = op;
                self.in_num = false;
            }
            '±' => {
                if self.in_num {
                    if self.value.starts_with('−') {
                        self.value = self.value[3..].to_string();
                    } else {
                        self.value = ["−", &self.value].concat();
                    }
                } else {
                    self.operand = -self.operand;
                    self.display();
                }
            }
            '.' => {
                if !self.in_num {
                    self.value = "0".to_string();
                    self.in_num = true;
                }
                if self.value.find('.').is_none() {
                    self.value.push('.');
                }
            }
            'c' => {
                self.value = "0".to_string();
                self.in_num = false;
                self.text = "".to_string();
            }
            'C' => {
                self.value = "0".to_string();
                self.operator = 'C';
                self.in_num = false;
            }
            '⌫' => {
                if self.in_num {
                    self.value.pop();
                    if self.value.is_empty() || self.value == "−" {
                        self.value = "0".to_string();
                        self.in_num = false;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

fn op_button_label(op: char, label: String) -> impl Widget<CalcState> {
    let painter = Painter::new(|ctx, _, env| {
        let bounds = ctx.size().to_rect();

        ctx.fill(bounds, &env.get(theme::PRIMARY_DARK));

        if ctx.is_hot() {
            ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
        }

        if ctx.is_active() {
            ctx.fill(bounds, &env.get(theme::PRIMARY_LIGHT));
        }
    });

    Label::new(label.clone())
        .with_text_size(24.)
        .center()
        .background(painter)
        .expand()
        .on_click(move |_ctx, data: &mut CalcState, _env| data.op(op)).debug(format!("option-{}", label))
}

fn op_button(op: char) -> impl Widget<CalcState> {
    op_button_label(op, op.to_string())
}

fn digit_button(digit: u8) -> impl Widget<CalcState> {
    let painter = Painter::new(|ctx, _, env| {
        let bounds = ctx.size().to_rect();

        ctx.fill(bounds, &env.get(theme::BACKGROUND_LIGHT));

        if ctx.is_hot() {
            ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
        }

        if ctx.is_active() {
            ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
        }
    });

    Label::new(format!("{}", digit))
        .with_text_size(24.)
        .center()
        .background(painter)
        .expand()
        .on_click(move |_ctx, data: &mut CalcState, _env| data.digit(digit)).debug(format!("digit-{}", digit))
}

fn flex_row<T: Data>(
    w1: impl Widget<T> + 'static,
    w2: impl Widget<T> + 'static,
    w3: impl Widget<T> + 'static,
    w4: impl Widget<T> + 'static,
) -> impl Widget<T> {
    Flex::row()
        .with_flex_child(w1, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w2, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w3, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w4, 1.0)
}

fn build_calc() -> impl Widget<CalcState> {
    let display = Label::new(|data: &String, _env: &_| data.clone())
        .with_text_size(32.0)
        .lens(CalcState::value)
        .padding(5.0)
        .debug_str("result");
    Flex::column()
        .with_flex_spacer(0.2)
        .with_child(display)
        .with_flex_spacer(0.2)
        .cross_axis_alignment(CrossAxisAlignment::End)
        .with_flex_child(
            flex_row(
                op_button_label('c', "CE".to_string()),
                op_button('C'),
                op_button('⌫'),
                op_button('÷'),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                digit_button(7),
                digit_button(8),
                digit_button(9),
                op_button('×'),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                digit_button(4),
                digit_button(5),
                digit_button(6),
                op_button('−'),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                digit_button(1),
                digit_button(2),
                digit_button(3),
                op_button('+'),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                op_button('±'),
                digit_button(0),
                op_button('.'),
                op_button('='),
            ),
            1.0,
        )
        .with_child(
            TextBox::multiline().lens(CalcState::text).debug_str("text")
        )
}

#[derive(Default)]
struct DebugDelegate;

impl AppDelegate<CalcState> for DebugDelegate {
    fn command(&mut self, ctx: &mut DelegateCtx, target: Target, cmd: &Command, data: &mut CalcState, env: &Env) -> Handled {
        if let Some(path) = cmd.get(EXECUTE_EVENTS) {
            let sink = ctx.get_external_handle();
            let path = path.clone();
            spawn(move || {
                let events = execute(path);
                for event in events {
                    sink.submit_command(EXECUTE_EVENT, (
                        event.selector,
                        event.event,
                        event.window_id
                    ), Auto).unwrap();
                    sleep(Duration::from_secs_f64(1.0 / 30.0));
                }
            });
            Handled::Yes
        } else {
            Handled::No
        }
    }
}

use poem::{get, handler, listener::TcpListener, Route, Server, web::Path};


#[handler]
fn execute_ext(Path(id): Path<String>) {
    SINK.lock().unwrap().as_ref().unwrap().submit_command(EXECUTE_EVENTS, Box::new(id), Auto).unwrap();
}

#[handler]
fn hello() -> String {
    format!("hello")
}

use std::sync::Mutex;
use druid::ExtEventSink;

lazy_static! {
    static ref SINK : Mutex<Option<ExtEventSink>> = Mutex::new(None);
}

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    spawn(|| {
        let window = WindowDesc::new(build_calc())
            .with_config(WindowConfig::default().show_titlebar(false).window_size((1920.,1080.)))
            .title(
                LocalizedString::new("calc-demo-window-title").with_placeholder("Simple Calculator"),
            );
        let calc_state = CalcState {
            value: "0".to_string(),
            operand: 0.0,
            operator: 'C',
            in_num: false,
            text: "".to_string()
        };
        let launcher = AppLauncher::with_window(window)
            .log_to_console()
            .delegate(DebugDelegate::default());
        let sink = launcher.get_external_handle();
        *SINK.lock().unwrap() = Some(sink);
        launcher
            .launch(calc_state)
            .expect("launch failed");
    });
    let app = Route::new().at("/execute/:id", get(execute_ext)).at("/hello", get(hello));
    Server::new(TcpListener::bind("127.0.0.1:12480"))
        .run(app)
        .await
}
