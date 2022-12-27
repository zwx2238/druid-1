use std::f64::consts::PI;

use druid::kurbo::{Circle, Line};
use druid::widget::prelude::*;
use druid::{AppDelegate, AppLauncher, CLIENT, Color, Command, DelegateCtx, execute, export, get_druid, Handled, LocalizedString, Point, post, Selector, start_record, Target, Vec2, WidgetExt, WindowDesc};
use druid::commands::SELECT_ALL;
use druid::im::{Vector, vector};

use druid::widget::{Flex, Label, List, Scroll, TextBox};
use druid::widget::Button;
use druid::Lens;

#[derive(Clone, Data, Lens)]
struct AppData {
    path: String,
    recording: bool,
    paths: Vector<String>,
}

pub static SELECT_PATH: Selector<String> = Selector::new("select-path");
pub static DELETE_PATH: Selector<String> = Selector::new("delete-path");

struct Delegate;

impl AppDelegate<AppData> for Delegate {
    fn command(&mut self, ctx: &mut DelegateCtx, target: Target, cmd: &Command, data: &mut AppData, env: &Env) -> Handled {
        if cmd.is(SELECT_PATH) {
            let path = cmd.get_unchecked(SELECT_PATH);
            data.path = path.clone();
            Handled::Yes
        } else if cmd.is(DELETE_PATH) {
            let path = cmd.get_unchecked(DELETE_PATH);
            data.paths.retain(|item|{
                item != path
            });
            post(format!("delete_path?path={}",path)).send().unwrap();
            Handled::Yes
        } else {
            Handled::No
        }
    }
}

fn build_root() -> impl Widget<AppData> {
    Flex::column()
        .with_child(
            TextBox::new().lens(AppData::path))
        .with_child(
            Flex::row()
                .with_child(
                    Button::dynamic(|data: &AppData, _| {
                        format!("{}", if data.recording { "stop" } else { "start" })
                    }).on_click(|_, data: &mut AppData, _| {
                        start_record();
                        data.recording = !data.recording;
                    })
                )
                .with_child(
                    Button::new("execute").on_click(|_, data: &mut AppData, _| {
                        get_druid(format!("execute/{}", data.path)).send().unwrap();
                        data.recording = false;
                    })
                )
                .with_child(
                    Button::new("export").on_click(|_, data: &mut AppData, _| {
                        export(data.path.clone());
                        data.recording = false;
                        if !data.paths.contains(&data.path) {
                            data.paths.push_back(data.path.clone());
                        }
                    })
                )
        )
        .with_child(
            Scroll::new(
                List::new(|| {
                    Flex::row()
                        .with_child(
                            Button::dynamic(|data: &String, _| {
                                format!("{}", data)
                            }).on_click(|ctx: &mut EventCtx, data: &mut String, _| {
                                ctx.submit_command(SELECT_PATH.with(data.clone()));
                            }).fix_width(300.0)
                        )
                        .with_child(
                            Button::new("delete").on_click(|ctx: &mut EventCtx, data: &mut String, _| {
                                ctx.submit_command(DELETE_PATH.with(data.clone()));
                            }).fix_width(300.0)
                        )
                }).lens(AppData::paths)
            ).vertical().fix_height(500.0)
        )
}

pub fn main() {
    let window = WindowDesc::new(build_root()).title(
        LocalizedString::new("test")
            .with_placeholder(""),
    );

    let paths = post("list_paths").send().unwrap().json::<Vector<String>>().unwrap();

    AppLauncher::with_window(window)
        .log_to_console()
        .delegate(Delegate {})
        .launch(AppData {
            path: "".to_string(),
            recording: true,
            paths,
        })
        .expect("launch failed");
}