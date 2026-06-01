use glium::glutin::surface::WindowSurface;
use glium::{ Display, Surface };
use imgui::{ Context, FontConfig, FontSource, Ui };
use imgui_glium_renderer::Renderer;
use imgui_winit_support::winit::dpi::LogicalSize;
use imgui_winit_support::winit::event::{ DeviceEvent, Event, WindowEvent };
use imgui_winit_support::winit::event_loop::{ DeviceEvents, EventLoop };
use imgui_winit_support::winit::window::WindowAttributes;
use imgui_winit_support::{ HiDpiMode, WinitPlatform };
use std::path::Path;
use std::time::Instant;

use crate::modules::input::raw_mouse;

pub const FONT_SIZE: f32 = 13.0;

#[allow(dead_code)] // annoyingly, RA yells that this is unusued
pub fn simple_init<F: FnMut(&mut bool, &mut Ui) + 'static>(title: &str, run_ui: F) {
    init_with_startup(title, |_, _, _| {}, run_ui);
}

pub fn init_with_startup<FInit, FUi>(title: &str, mut startup: FInit, mut run_ui: FUi)
    where
        FInit: FnMut(&mut Context, &mut Renderer, &Display<WindowSurface>) + 'static,
        FUi: FnMut(&mut bool, &mut Ui) + 'static
{
    let mut imgui = create_context();

    let title = match Path::new(&title).file_name() {
        Some(file_name) => file_name.to_str().unwrap(),
        None => title,
    };
    let event_loop = EventLoop::new().expect("Failed to create EventLoop");

    let window_attributes = WindowAttributes::default()
        .with_title(title)
        .with_inner_size(LogicalSize::new(1024, 768));
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder
        ::new()
        .set_window_builder(window_attributes)
        .build(&event_loop);
    let mut renderer = Renderer::new(&mut imgui, &display).expect("Failed to initialize renderer");

    let mut platform = WinitPlatform::new(&mut imgui);
    {
        let dpi_mode = if let Ok(factor) = std::env::var("IMGUI_EXAMPLE_FORCE_DPI_FACTOR") {
            match factor.parse::<f64>() {
                Ok(f) => HiDpiMode::Locked(f),
                Err(e) => panic!("Invalid scaling factor: {}", e),
            }
        } else {
            HiDpiMode::Default
        };

        platform.attach_window(imgui.io_mut(), &window, dpi_mode);
    }

    let mut last_frame = Instant::now();
    let mut device_events_enabled = false;

    startup(&mut imgui, &mut renderer, &display);

    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| {
            // Ask winit to deliver raw `DeviceEvent`s regardless of focus, so the
            // recoil-trace recorder keeps receiving mouse deltas while the game
            // is the foreground window.
            if !device_events_enabled {
                window_target.listen_device_events(DeviceEvents::Always);
                device_events_enabled = true;
            }

            match event {
                Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                    raw_mouse::add_delta(delta.0.round() as i32, delta.1.round() as i32);
                }
                Event::NewEvents(_) => {
                    let now = Instant::now();
                    imgui.io_mut().update_delta_time(now - last_frame);
                    last_frame = now;
                }
                Event::AboutToWait => {
                    platform
                        .prepare_frame(imgui.io_mut(), &window)
                        .expect("Failed to prepare frame");
                    window.request_redraw();
                }
                Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                    let ui = imgui.frame();

                    let mut run = true;
                    run_ui(&mut run, ui);
                    if !run {
                        window_target.exit();
                    }

                    let mut target = display.draw();
                    target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
                    platform.prepare_render(ui, &window);
                    let draw_data = imgui.render();
                    renderer.render(&mut target, draw_data).expect("Rendering failed");
                    target.finish().expect("Failed to swap buffers");
                }
                Event::WindowEvent { event: WindowEvent::Resized(new_size), .. } => {
                    if new_size.width > 0 && new_size.height > 0 {
                        display.resize((new_size.width, new_size.height));
                    }
                    platform.handle_event(imgui.io_mut(), &window, &event);
                }
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } =>
                    window_target.exit(),
                event => {
                    platform.handle_event(imgui.io_mut(), &window, &event);
                }
            }
        })
        .expect("EventLoop error");
}

pub fn simple_init_with_resize<F: FnMut(&mut bool, &mut Ui, &mut dyn FnMut([f32; 2])) + 'static>(
    title: &str,
    run_ui: F
) {
    init_with_startup_with_resize(title, |_, _, _| {}, run_ui);
}

pub fn init_with_startup_with_resize<FInit, FUi>(title: &str, mut startup: FInit, mut run_ui: FUi)
    where
        FInit: FnMut(&mut Context, &mut Renderer, &Display<WindowSurface>) + 'static,
        FUi: FnMut(&mut bool, &mut Ui, &mut dyn FnMut([f32; 2])) + 'static
{
    let mut imgui = create_context();

    let title = match Path::new(&title).file_name() {
        Some(file_name) => file_name.to_str().unwrap(),
        None => title,
    };
    let event_loop = EventLoop::new().expect("Failed to create EventLoop");

    let window_attributes = WindowAttributes::default()
        .with_title(title)
        .with_inner_size(LogicalSize::new(1024, 768));
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder
        ::new()
        .set_window_builder(window_attributes)
        .build(&event_loop);
    let mut renderer = Renderer::new(&mut imgui, &display).expect("Failed to initialize renderer");

    let mut platform = WinitPlatform::new(&mut imgui);
    {
        let dpi_mode = if let Ok(factor) = std::env::var("IMGUI_EXAMPLE_FORCE_DPI_FACTOR") {
            match factor.parse::<f64>() {
                Ok(f) => HiDpiMode::Locked(f),
                Err(e) => panic!("Invalid scaling factor: {}", e),
            }
        } else {
            HiDpiMode::Default
        };

        platform.attach_window(imgui.io_mut(), &window, dpi_mode);
    }

    let mut last_frame = Instant::now();
    let mut device_events_enabled = false;

    startup(&mut imgui, &mut renderer, &display);

    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| {
            // Ask winit to deliver raw `DeviceEvent`s regardless of focus, so the
            // recoil-trace recorder keeps receiving mouse deltas while the game
            // is the foreground window.
            if !device_events_enabled {
                window_target.listen_device_events(DeviceEvents::Always);
                device_events_enabled = true;
            }

            match event {
                Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                    raw_mouse::add_delta(delta.0.round() as i32, delta.1.round() as i32);
                }
                Event::NewEvents(_) => {
                    let now = Instant::now();
                    imgui.io_mut().update_delta_time(now - last_frame);
                    last_frame = now;
                }
                Event::AboutToWait => {
                    platform
                        .prepare_frame(imgui.io_mut(), &window)
                        .expect("Failed to prepare frame");
                    window.request_redraw();
                }
                Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                    let ui = imgui.frame();

                    let mut run = true;
                    let mut requested_size: Option<[f32; 2]> = None;

                    run_ui(
                        &mut run,
                        ui,
                        &mut (|size: [f32; 2]| {
                            requested_size = Some(size);
                        })
                    );
                    if !run {
                        window_target.exit();
                    }

                    if let Some(size) = requested_size {
                        use imgui_winit_support::winit::dpi::LogicalSize;
                        let _ = window.request_inner_size(
                            LogicalSize::new(size[0] as f64, size[1] as f64)
                        );
                        display.resize((size[0] as u32, size[1] as u32));
                    }

                    let mut target = display.draw();
                    target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
                    platform.prepare_render(ui, &window);
                    let draw_data = imgui.render();
                    renderer.render(&mut target, draw_data).expect("Rendering failed");
                    target.finish().expect("Failed to swap buffers");
                }
                Event::WindowEvent { event: WindowEvent::Resized(new_size), .. } => {
                    if new_size.width > 0 && new_size.height > 0 {
                        display.resize((new_size.width, new_size.height));
                    }
                    platform.handle_event(imgui.io_mut(), &window, &event);
                }
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } =>
                    window_target.exit(),
                event => {
                    platform.handle_event(imgui.io_mut(), &window, &event);
                }
            }
        })
        .expect("EventLoop error");
}

pub fn create_context() -> imgui::Context {
    let mut imgui = Context::create();
    imgui.fonts().add_font(
        &[
            FontSource::DefaultFontData {
                config: Some(FontConfig {
                    size_pixels: FONT_SIZE,
                    rasterizer_multiply: 1.5,
                    oversample_h: 4,
                    oversample_v: 4,
                    ..FontConfig::default()
                }),
            },
        ]
    );
    imgui.set_ini_filename(None);

    apply_theme(&mut imgui);

    imgui
}

/// A single coherent dark theme with a red accent, so every tab/control shares
/// the same look instead of raw default imgui grey.
fn apply_theme(imgui: &mut imgui::Context) {
    use imgui::StyleColor::*;

    let style = imgui.style_mut();
    style.window_rounding = 6.0;
    style.child_rounding = 6.0;
    style.frame_rounding = 4.0;
    style.popup_rounding = 4.0;
    style.grab_rounding = 4.0;
    style.tab_rounding = 4.0;
    style.scrollbar_rounding = 4.0;
    style.window_padding = [14.0, 14.0];
    style.frame_padding = [9.0, 5.0];
    style.item_spacing = [9.0, 9.0];
    style.item_inner_spacing = [7.0, 5.0];
    style.grab_min_size = 11.0;
    style.window_border_size = 0.0;
    style.frame_border_size = 0.0;
    style.indent_spacing = 16.0;

    let accent = [0.85, 0.22, 0.28, 1.0];
    let accent_hover = [0.95, 0.31, 0.37, 1.0];
    let accent_active = [0.72, 0.16, 0.22, 1.0];
    let bg = [0.085, 0.088, 0.10, 1.0];
    let panel = [0.15, 0.155, 0.18, 1.0];
    let panel_hover = [0.21, 0.215, 0.25, 1.0];
    let border = [0.24, 0.24, 0.28, 1.0];

    style[WindowBg] = bg;
    style[ChildBg] = bg;
    style[PopupBg] = [0.11, 0.115, 0.13, 1.0];
    style[Border] = border;
    style[FrameBg] = panel;
    style[FrameBgHovered] = panel_hover;
    style[FrameBgActive] = panel_hover;
    style[TitleBg] = bg;
    style[TitleBgActive] = panel;
    style[Button] = panel;
    style[ButtonHovered] = accent_hover;
    style[ButtonActive] = accent_active;
    style[Header] = panel;
    style[HeaderHovered] = accent_hover;
    style[HeaderActive] = accent_active;
    style[SliderGrab] = accent;
    style[SliderGrabActive] = accent_hover;
    style[CheckMark] = accent;
    style[Tab] = panel;
    style[TabHovered] = accent_hover;
    style[TabActive] = accent;
    style[TabUnfocused] = panel;
    style[TabUnfocusedActive] = accent_active;
    style[Separator] = border;
    style[SeparatorHovered] = accent_hover;
    style[Text] = [0.91, 0.91, 0.93, 1.0];
    style[TextDisabled] = [0.48, 0.48, 0.53, 1.0];
    style[ScrollbarBg] = bg;
    style[ScrollbarGrab] = panel;
    style[ScrollbarGrabHovered] = panel_hover;
    style[ScrollbarGrabActive] = accent;
}
