// Copyright (c) 2019 Steven Wittens
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// From https://github.com/Yatekii/imgui-wgpu-rs/blob/master/examples/hello_world.rs

use clipboard::{ClipboardContext, ClipboardProvider};
use imgui::{ClipboardBackend, Context, FontConfig, FontSource, ImStr, ImString, Ui};
use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;
use wgpu::{Device, Queue, Surface, SwapChain};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

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

const MIN_WIDTH: u32 = 480;
const MIN_HEIGHT: u32 = 270;

// Backend
pub fn init(title: &str, width: f64, height: f64) -> Backend {
    let rt = Runtime::new().unwrap();

    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY | wgpu::BackendBit::DX11);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(true)
        .with_title(title)
        .with_min_inner_size(LogicalSize::new(MIN_WIDTH, MIN_HEIGHT))
        .with_inner_size(LogicalSize::new(width, height))
        .with_visible(false)
        .build(&event_loop)
        .expect("Failed to create a window");

    let surface = unsafe { instance.create_surface(&window) };

    let adapter = rt
        .block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
        }))
        .unwrap();

    let (device, queue) =
        rt.block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None)).unwrap();

    // Set up swap chain
    let size = window.inner_size();
    let sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: size.width as u32,
        height: size.height as u32,
        present_mode: wgpu::PresentMode::Fifo,
    };

    let swap_chain = device.create_swap_chain(&surface, &sc_desc);

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
    platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Default);

    let hidpi_factor = platform.hidpi_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.fonts().add_font(&[FontSource::DefaultFontData {
        config: Some(FontConfig {
            oversample_h: 1,
            pixel_snap_h: true,
            size_pixels: font_size,
            ..Default::default()
        }),
    }]);

    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    let renderer_config = RendererConfig { texture_format: sc_desc.format, ..Default::default() };
    let renderer = Renderer::new(&mut imgui, &device, &queue, renderer_config);

    Backend { window, event_loop, surface, device, queue, swap_chain, imgui, platform, renderer }
}

pub struct Backend {
    window: Window,
    event_loop: EventLoop<()>,
    surface: Surface,
    device: Device,
    queue: Queue,
    swap_chain: SwapChain,
    imgui: Context,
    platform: WinitPlatform,
    pub renderer: Renderer,
}

impl Backend {
    pub fn main_loop<F>(self, mut ui_builder: F)
    where
        F: FnMut(&mut bool, &mut Ui) + 'static,
    {
        let Backend {
            window,
            event_loop,
            surface,
            device,
            queue,
            mut swap_chain,
            mut imgui,
            mut platform,
            mut renderer,
        } = self;

        window.set_visible(true);

        let mut last_frame = Instant::now();
        let mut last_cursor = None;
        let mut run = true;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event: WindowEvent::Resized(_), .. } => {
                    let size = window.inner_size();

                    // Prevent a crash when minimizing because it triggers `WindowEvent::Resized`
                    if size.width == 0 || size.height == 0 {
                        return;
                    }

                    let sc_desc = wgpu::SwapChainDescriptor {
                        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        width: size.width as u32,
                        height: size.height as u32,
                        present_mode: wgpu::PresentMode::Fifo,
                    };

                    swap_chain = device.create_swap_chain(&surface, &sc_desc);
                }
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::MainEventsCleared => window.request_redraw(),
                Event::RedrawEventsCleared => {
                    // Prevent CPU tanking when minimized
                    let size = window.inner_size();
                    if size.width == 0 || size.height == 0 {
                        std::thread::sleep(Duration::from_secs_f32(1.0 / 60.0)); // 60 fps
                        return;
                    }

                    let now = Instant::now();
                    imgui.io_mut().update_delta_time(now - last_frame);
                    last_frame = now;

                    let frame = match swap_chain.get_current_frame() {
                        Ok(frame) => frame,
                        Err(_) => {
                            // Frame dropped
                            return;
                        }
                    };

                    platform
                        .prepare_frame(imgui.io_mut(), &window)
                        .expect("Failed to prepare frame");

                    let mut ui = imgui.frame();
                    ui_builder(&mut run, &mut ui);

                    if !run {
                        *control_flow = ControlFlow::Exit;
                    }

                    let mut encoder: wgpu::CommandEncoder = device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                    if last_cursor != Some(ui.mouse_cursor()) {
                        last_cursor = Some(ui.mouse_cursor());
                        platform.prepare_render(&ui, &window);
                    }

                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[wgpu::RenderPassColorAttachment {
                            view: &frame.output.view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 1.0,
                                }),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });

                    renderer
                        .render(ui.render(), &queue, &device, &mut rpass)
                        .expect("Rendering failed");

                    drop(rpass);

                    queue.submit(Some(encoder.finish()));
                }
                _ => (),
            }
            platform.handle_event(imgui.io_mut(), &window, &event)
        });
    }
}
