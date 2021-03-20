use glutin::PossiblyCurrent;
use imgui::{im_str, Condition, StyleVar, Ui, Window};
use std::cell::Cell;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};

mod gl;
mod widgets;

pub struct LmmpWindow {
    event_loop: Cell<Option<EventLoop<()>>>, // *sigh*
    gl_ctx: glutin::WindowedContext<PossiblyCurrent>,
    imgui: imgui::Context,
    backend: imgui_winit_support::WinitPlatform,
    renderer: imgui_opengl_renderer::Renderer,
}

impl LmmpWindow {
    pub fn new() -> LmmpWindow {
        use glutin::window::WindowBuilder;
        use glutin::{Api, ContextBuilder, GlProfile, GlRequest};
        use imgui_opengl_renderer::Renderer;
        use imgui_winit_support::{HiDpiMode, WinitPlatform};

        let wb = WindowBuilder::new()
            .with_visible(true)
            .with_resizable(true)
            .with_transparent(false);
        let event_loop = EventLoop::new();
        let gl_ctx = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 0)))
            .with_gl_profile(GlProfile::Core)
            .with_vsync(true)
            .build_windowed(wb, &event_loop)
            .unwrap();
        let gl_ctx = unsafe { gl_ctx.make_current().unwrap() };

        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);

        /* since winit won't do it's own decorations and let's be honest,
        why should we? there should probably be something here that
        detects gnome and then forces x11 (unfortunately ): */
        let dpi_mode = match std::env::var("WAYLAND_DISPLAY") {
            Ok(w) => {
                if !w.is_empty() {
                    HiDpiMode::Default
                } else {
                    HiDpiMode::Locked(1f64)
                }
            }
            Err(_) => HiDpiMode::Locked(1f64),
        };
        let mut backend = WinitPlatform::init(&mut imgui);
        backend.attach_window(imgui.io_mut(), gl_ctx.window(), dpi_mode);

        let renderer = Renderer::new(&mut imgui, |s| gl_ctx.get_proc_address(s) as _);
        gl::load_with(|s| gl_ctx.get_proc_address(s) as *const _);

        LmmpWindow {
            event_loop: Cell::new(Some(event_loop)),
            gl_ctx,
            imgui,
            backend,
            renderer,
        }
    }

    fn handle_window_event(
        &mut self,
        wev: &WindowEvent,
        targ: &EventLoopWindowTarget<()>,
        flow: &mut ControlFlow,
    ) {
        use WindowEvent::*;
        match wev {
            // the config pragma means that removing the brackets here is not syntactically equivalent
            #[rustfmt::skip]
            Resized(sz) => {
                #[cfg(target_os = "linux")]
                {
                    use glutin::platform::unix::EventLoopWindowTargetExtUnix;
                    if targ.is_wayland() {
                        self.gl_ctx.resize(*sz);
                    }
                }
            }
            CloseRequested => {
                *flow = ControlFlow::Exit;
            }
            _ => {}
        };
    }

    fn process(&mut self, ev: &Event<()>, redraw: bool) {
        let window = self.gl_ctx.window();
        self.backend.handle_event(self.imgui.io_mut(), window, ev);
        if redraw {
            window.request_redraw();
        }
    }

    fn redraw(&mut self) {
        let window = self.gl_ctx.window();
        self.backend
            .prepare_frame(self.imgui.io_mut(), window)
            .unwrap();
        let ui = self.imgui.frame();
        let size = window.inner_size();

        let bar_height = widgets::toolbar(&ui, size);
        widgets::statusbar(&ui, size, bar_height);

        let inner_height = size.height as f32 - 2f32 * bar_height;
        let half = inner_height / 2f32;

        Window::new(im_str!("art"))
            .position([0f32, bar_height], Condition::Always)
            .size([half, half], Condition::Always)
            .no_decoration()
            .build(&ui, || {
                ui.text("art");
            });

        Window::new(im_str!("library"))
            .position([half, bar_height], Condition::Always)
            .size([size.width as f32 - half, half], Condition::Always)
            .no_decoration()
            .build(&ui, || ui.text("library"));

        Window::new(im_str!("playlist"))
            .position([0f32, bar_height + half], Condition::Always)
            .size([size.width as f32, half], Condition::Always)
            .no_decoration()
            .build(&ui, || ui.text("playlist"));

        self.backend.prepare_render(&ui, window);
        unsafe {
            gl::ClearColor(0f32, 0f32, 0f32, 1f32);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        self.renderer.render(ui);
        self.gl_ctx.swap_buffers().unwrap();
    }

    pub fn run(mut self) -> ! {
        use std::time::Instant;
        // to avoid a partial move, we move the event loop out of the struct. silly, but it works
        let event_loop = self.event_loop.replace(None).unwrap();
        let mut last_frame = Instant::now();
        event_loop.run(move |ev, targ, flow| {
            use Event::*;

            *flow = ControlFlow::Wait;
            match ev {
                NewEvents(_) => {
                    let now = Instant::now();
                    self.imgui.io_mut().update_delta_time(now - last_frame);
                    last_frame = now;
                }
                RedrawRequested(_) => {
                    self.redraw();
                }
                WindowEvent { event: ref wev, .. } => {
                    self.handle_window_event(&wev, &targ, flow);
                    self.process(&ev, true);
                }
                DeviceEvent { .. } => {
                    self.process(&ev, true);
                }
                ev => self.process(&ev, false),
            };
        })
    }
}
