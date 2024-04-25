// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![allow(unused)]
use std::rc::Rc;

use dpi::{LogicalPosition, LogicalSize};
use muda::{
    accelerator::{Accelerator, Code, Modifiers},
    dpi::Position,
    AboutMetadata, CheckMenuItem, ContextMenu, IconMenuItem, Menu, MenuEvent, MenuItem,
    PredefinedMenuItem, Submenu,
};
#[cfg(target_os = "macos")]
use tao::platform::macos::WindowExtMacOS;
#[cfg(target_os = "linux")]
use tao::platform::unix::WindowExtUnix;
#[cfg(target_os = "windows")]
use tao::platform::windows::{EventLoopBuilderExtWindows, WindowExtWindows};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::{Window, WindowBuilder},
};
#[cfg(target_os = "linux")]
use wry::WebViewBuilderExtUnix;
use wry::{http::Request, Rect, WebViewBuilder};

fn main() -> wry::Result<()> {
    let mut event_loop_builder = EventLoopBuilder::new();
    let event_loop = event_loop_builder.build();

    /* Menu */

    let menu_bar = Menu::new();

    let app_m = Submenu::new("App", true);
    app_m
        .append_items(&[
            &PredefinedMenuItem::about(None, None),
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::services(None),
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::hide(None),
            &PredefinedMenuItem::hide_others(None),
            &PredefinedMenuItem::show_all(None),
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::quit(None),
        ])
        .unwrap();

    let edit_m = Submenu::new("&Edit", true);
    edit_m
        .append_items(&[
            &PredefinedMenuItem::undo(None),
            &PredefinedMenuItem::copy(None),
            &PredefinedMenuItem::cut(None),
            &PredefinedMenuItem::paste(None),
        ])
        .unwrap();

    menu_bar.append_items(&[&app_m, &edit_m]).unwrap();
    menu_bar.init_for_nsapp();

    /* Window */

    let window = WindowBuilder::new()
        .with_inner_size(dpi::LogicalSize::new(800, 800))
        .with_title("Window")
        .build(&event_loop)
        .unwrap();

    let size = window.inner_size().to_logical::<u32>(window.scale_factor());

    let webview = WebViewBuilder::new_as_child(&window)
        .with_bounds(Rect {
            position: LogicalPosition::new(0, 0).into(),
            size: LogicalSize::new(size.width / 2, size.height / 2).into(),
        })
        .with_url("https://www.slatejs.org/examples/richtext")
        .build()?;
    let webview2 = WebViewBuilder::new_as_child(&window)
        .with_bounds(Rect {
            position: LogicalPosition::new(size.width / 2, 0).into(),
            size: LogicalSize::new(size.width / 2, size.height / 2).into(),
        })
        .with_url("https://www.slatejs.org/examples/richtext")
        .build()?;
    let webview3 = WebViewBuilder::new_as_child(&window)
        .with_bounds(Rect {
            position: LogicalPosition::new(0, size.height / 2).into(),
            size: LogicalSize::new(size.width / 2, size.height / 2).into(),
        })
        .with_url("https://www.google.com")
        .build()?;
    let webview4 = WebViewBuilder::new_as_child(&window)
        .with_bounds(Rect {
            position: LogicalPosition::new(size.width / 2, size.height / 2).into(),
            size: LogicalSize::new(size.width / 2, size.height / 2).into(),
        })
        .with_url("https://www.slatejs.org/examples/richtext")
        .build()?;

    // let menu_channel = MenuEvent::receiver();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    })
}
