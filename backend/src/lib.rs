use clipboard::{ClipboardContext, ClipboardProvider};
use imgui::{
    ClipboardBackend, Context, FontConfig, FontGlyphRanges, FontSource, ImStr, ImString, Ui,
};
use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::{
    panic, thread,
    time::{Duration, Instant},
};
use tokio::runtime::Handle;
use wgpu::{Device, Queue, Surface, SwapChain};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
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
pub enum Backend {
    Default,
    Vulkan,
    DirectX11,
    DirectX12,
    Metal,
}

// System
pub fn init(title: &str, width: f64, height: f64, backend: Backend) -> System {
    let backend = match backend {
        Backend::Default => wgpu::BackendBit::PRIMARY,
        Backend::Vulkan => wgpu::BackendBit::VULKAN,
        Backend::DirectX11 => wgpu::BackendBit::DX11,
        Backend::DirectX12 => wgpu::BackendBit::DX12,
        Backend::Metal => wgpu::BackendBit::METAL,
    };

    #[cfg(target_os = "windows")]
    {
        panic::catch_unwind(|| init_with_backend(title, width, height, backend)).unwrap_or_else(
            |_| {
                eprintln!("Fallback to DirectX 11");
                eprintln!("If it works for you, you can ignore this error or run TSE with --dx11 argument");
                init_with_backend(title, width, height, wgpu::BackendBit::DX11)
            },
        )
    }

    #[cfg(not(target_os = "windows"))]
    init_with_backend(title, width, height, backend)
}

fn init_with_backend(title: &str, width: f64, height: f64, backend: wgpu::BackendBit) -> System {
    let rt = Handle::current();

    let instance = wgpu::Instance::new(backend);

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

    const FONT: &[u8] = include_bytes!("mplus-1p-regular.ttf");
    imgui.fonts().add_font(&[
        FontSource::DefaultFontData {
            config: Some(FontConfig { size_pixels: font_size, ..Default::default() }),
        },
        FontSource::TtfData {
            data: FONT,
            size_pixels: font_size * 1.4,
            config: Some(FontConfig {
                glyph_ranges: FontGlyphRanges::cyrillic(),
                ..Default::default()
            }),
        },
        FontSource::TtfData {
            data: FONT,
            size_pixels: font_size * 1.4,
            config: Some(FontConfig {
                glyph_ranges: FontGlyphRanges::japanese(),
                ..Default::default()
            }),
        },
    ]);

    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    let renderer_config = RendererConfig { texture_format: sc_desc.format, ..Default::default() };
    let renderer = Renderer::new(&mut imgui, &device, &queue, renderer_config);

    System { window, event_loop, surface, device, queue, swap_chain, imgui, platform, renderer }
}

pub struct System {
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

impl System {
    pub fn main_loop<F>(self, mut ui_builder: F)
    where
        F: FnMut(&mut bool, &mut Ui, &mut Option<String>) + 'static,
    {
        let System {
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
        let mut next_frame_factor = 0.0;

        let mut last_cursor = None;
        let mut run = true;
        let mut dropped_file = None;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event: WindowEvent::Resized(_), .. } => {
                    let PhysicalSize { width, height } = window.inner_size();

                    // Prevent a crash when minimizing because it triggers `WindowEvent::Resized`
                    if width == 0 || height == 0 {
                        return;
                    }

                    let sc_desc = wgpu::SwapChainDescriptor {
                        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        width: width as u32,
                        height: height as u32,
                        present_mode: wgpu::PresentMode::Fifo,
                    };

                    swap_chain = device.create_swap_chain(&surface, &sc_desc);
                }
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent { event: WindowEvent::DroppedFile(ref path), .. } => {
                    dropped_file = Some(path.to_string_lossy().into());
                }
                Event::MainEventsCleared => window.request_redraw(),
                Event::RedrawEventsCleared => {
                    // Prevent CPU tanking when minimized
                    let PhysicalSize { width, height } = window.inner_size();
                    if width == 0 || height == 0 {
                        const SLEEP_DURATION: Duration = Duration::from_millis(100); // 10 fps
                        thread::sleep(SLEEP_DURATION);
                        return;
                    }

                    // Delta time
                    {
                        let now = Instant::now();
                        let delta = now - last_frame;

                        // FPS limit to 60
                        const FRAME_TARGET: f32 = 1.0 / 60.0;
                        let frame_duration_target =
                            Duration::from_secs_f32((FRAME_TARGET + next_frame_factor).max(0.0));
                        if delta < frame_duration_target {
                            const SLEEP_DURATION: Duration = Duration::from_millis(1);
                            thread::sleep(SLEEP_DURATION);
                            return;
                        }
                        next_frame_factor =
                            frame_duration_target.as_secs_f32() - delta.as_secs_f32();

                        imgui.io_mut().update_delta_time(delta);
                        last_frame = now;
                    }

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
                    ui_builder(&mut run, &mut ui, &mut dropped_file);

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
