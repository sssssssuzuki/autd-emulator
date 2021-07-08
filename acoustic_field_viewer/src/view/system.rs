/*
 * File: system.rs
 * Project: view
 * Created Date: 08/07/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/07/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

use glutin::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource};
use imgui_winit_support::{HiDpiMode, WinitPlatform};

use crate::view::render_system::RenderSystem;

type EventsLoop = EventLoop<()>;

pub struct System {
    pub events_loop: EventsLoop,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub render_sys: RenderSystem,
    pub font_size: f32,
}

impl System {
    pub fn init(title: &str) -> Self {
        let events_loop = EventsLoop::new();
        let builder = WindowBuilder::new()
            .with_title(title.to_owned())
            .with_inner_size(LogicalSize::new(1024f64, 768f64));

        let mut imgui = Context::create();
        imgui.set_ini_filename(None);

        let mut platform = WinitPlatform::init(&mut imgui);

        let hidpi_factor = platform.hidpi_factor();
        let font_size = (16.0 * hidpi_factor) as f32;
        imgui.fonts().add_font(&[FontSource::TtfData {
            data: include_bytes!("../../../assets/fonts/NotoSans-Regular.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                rasterizer_multiply: 1.75,
                glyph_ranges: FontGlyphRanges::japanese(),
                ..FontConfig::default()
            }),
        }]);

        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        let render_sys = RenderSystem::init(&mut imgui, builder, &events_loop);
        platform.attach_window(imgui.io_mut(), render_sys.window(), HiDpiMode::Rounded);
        System {
            events_loop,
            imgui,
            platform,
            render_sys,
            font_size,
        }
    }
}
