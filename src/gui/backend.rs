// Copyright (c) 2015-2020 The imgui-rs Developers

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// From imgui-rs examples

use clipboard::{ClipboardContext, ClipboardProvider};
use glium::{
    glutin::{
        self,
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        platform::windows::EventLoopExtWindows,
        window::WindowBuilder,
    },
    Display, Surface,
};
use imgui::{ClipboardBackend, Context, ImStr, ImString, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;

// Clipboard
pub struct ClipboardSupport(ClipboardContext);

pub fn clipboard_init() -> Option<ClipboardSupport> {
    ClipboardContext::new().ok().map(ClipboardSupport)
}

impl ClipboardBackend for ClipboardSupport {
    fn get(&mut self) -> Option<ImString> {
        self.0.get_contents().ok().map(|text| text.into())
    }
    fn set(&mut self, text: &ImStr) {
        let _ = self.0.set_contents(text.to_str().to_owned());
    }
}

// Backend
pub struct System {
    pub event_loop: EventLoop<()>,
    pub display: glium::Display,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub font_size: f32,
}

pub fn init(title: &str, width: f64, height: f64) -> System {
    let event_loop = EventLoop::new_any_thread();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let builder = WindowBuilder::new()
        .with_resizable(true)
        .with_title(title)
        .with_min_inner_size(glutin::dpi::LogicalSize::new(480.0, 270.0))
        .with_inner_size(glutin::dpi::LogicalSize::new(width, height));
    let display =
        Display::new(builder, context, &event_loop).expect("Failed to initialize display");

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    if let Some(backend) = clipboard_init() {
        imgui.set_clipboard_backend(Box::new(backend));
    } else {
        tokio::spawn(async {
            panic!("Failed to initialize clipboard");
        });
    }

    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let gl_window = display.gl_window();
        let window = gl_window.window();
        platform.attach_window(imgui.io_mut(), window, HiDpiMode::Rounded);
    }

    let hidpi_factor = platform.hidpi_factor();
    let font_size = (14.0 * hidpi_factor) as f32;

    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    let renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    System { event_loop, display, imgui, platform, renderer, font_size }
}

impl System {
    pub fn main_loop<F>(self, mut run_ui: F)
    where
        F: FnMut(&mut bool, &mut Ui) + 'static,
    {
        let System { event_loop, display, mut imgui, mut platform, mut renderer, .. } = self;
        let mut last_frame = Instant::now();

        event_loop.run(move |event, _, control_flow| match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::MainEventsCleared => {
                let gl_window = display.gl_window();
                platform
                    .prepare_frame(imgui.io_mut(), gl_window.window())
                    .expect("Failed to prepare frame");
                gl_window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                let mut run = true;
                let mut ui = imgui.frame();

                run_ui(&mut run, &mut ui);
                if !run {
                    *control_flow = ControlFlow::Exit;
                }

                let gl_window = display.gl_window();
                let mut target = display.draw();
                target.clear_color_srgb(0.0, 0.0, 0.0, 1.0);
                platform.prepare_render(&ui, gl_window.window());
                let draw_data = ui.render();
                renderer.render(&mut target, draw_data).expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit
            }
            event => {
                let gl_window = display.gl_window();
                platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
            }
        })
    }
}
