use std::{sync::Arc, time::Instant};

use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, OwnedDisplayHandle},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

use crate::{camera::Camera, cube::RubiksCube, renderer::Renderer};

mod camera;
mod cube;
mod mesh;
mod renderer;
mod solver;

struct State {
    instance: wgpu::Instance,
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    depth_texture_view: wgpu::TextureView,
    renderer: Renderer,
    camera: Camera,
    cube: RubiksCube,
    last_time: Instant,
    mouse_pressed: bool,
    last_mouse_pos: Option<(f64, f64)>,
    modifiers: winit::keyboard::ModifiersState,
}

impl State {
    async fn new(display: OwnedDisplayHandle, window: Arc<Window>) -> State {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_with_display_handle(
            Box::new(display),
        ));
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();

        let size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];

        let depth_texture_view = Self::create_depth_texture(&device, size.width, size.height);

        let cube = RubiksCube::new();
        let aspect = size.width as f32 / size.height as f32;
        let camera = Camera::new(aspect);
        let renderer = Renderer::new(&device, surface_format, aspect, &cube);

        let state = State {
            instance,
            window,
            device,
            queue,
            size,
            surface,
            surface_format,
            depth_texture_view,
            renderer,
            camera,
            cube,
            last_time: Instant::now(),
            mouse_pressed: false,
            last_mouse_pos: None,
            modifiers: winit::keyboard::ModifiersState::default(),
        };

        state.configure_surface();

        state
    }

    fn get_window(&self) -> &Window {
        &self.window
    }

    fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    fn create_depth_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;

        self.configure_surface();
        self.depth_texture_view =
            Self::create_depth_texture(&self.device, new_size.width, new_size.height);
    }

    fn on_key(&mut self, key: KeyCode, shift: bool) {
        match key {
            KeyCode::KeyR => self.cube.rotate_face(cube::Face::Right, !shift),
            KeyCode::KeyL => self.cube.rotate_face(cube::Face::Left, !shift),
            KeyCode::KeyU => self.cube.rotate_face(cube::Face::Up, !shift),
            KeyCode::KeyD => self.cube.rotate_face(cube::Face::Down, !shift),
            KeyCode::KeyF => self.cube.rotate_face(cube::Face::Front, !shift),
            KeyCode::KeyB => self.cube.rotate_face(cube::Face::Back, !shift),
            KeyCode::KeyS => self.cube.shuffle(),
            _ => {}
        }
    }

    fn on_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        if button == MouseButton::Left {
            self.mouse_pressed = state == ElementState::Pressed;

            if !self.mouse_pressed {
                self.last_mouse_pos = None;
            }
        }
    }

    fn on_mouse_moved(&mut self, x: f64, y: f64) {
        if !self.mouse_pressed {
            return;
        }

        if let Some((lx, ly)) = self.last_mouse_pos {
            let dx = (x - lx) as f32;
            let dy = (y - ly) as f32;

            self.camera.yaw -= dx * 0.005;

            self.camera.pitch += dy * 0.005;
            self.camera.pitch = self.camera.pitch.clamp(
                -std::f32::consts::FRAC_PI_2 + 0.05,
                std::f32::consts::FRAC_PI_2 - 0.05,
            );
        }

        self.last_mouse_pos = Some((x, y));
    }

    fn on_scroll(&mut self, delta: MouseScrollDelta) {
        let scroll = match delta {
            MouseScrollDelta::LineDelta(_x, y) => y,
            MouseScrollDelta::PixelDelta(pos) => pos.y as f32 * 0.05,
        };

        self.camera.distance -= scroll * 0.5;
        self.camera.distance = self.camera.distance.clamp(3.0, 30.0);
    }

    fn update(&mut self) {
        let now = Instant::now();
        let dt = (now - self.last_time).as_secs_f32();
        self.last_time = now;

        self.cube.update(dt);
        self.renderer.update_camera(&self.queue, &self.camera);
        self.renderer.update_cubies(&self.queue, &self.cube);
    }

    fn render(&mut self) {
        self.renderer.update_camera(&self.queue, &self.camera);

        let surface_texture = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,
            wgpu::CurrentSurfaceTexture::Occluded | wgpu::CurrentSurfaceTexture::Timeout => return,
            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                drop(texture);
                self.configure_surface();
                return;
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.configure_surface();
                return;
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                unreachable!("No error scope registered, so validation errors will panic")
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                self.surface = self.instance.create_surface(self.window.clone()).unwrap();
                self.configure_surface();
                return;
            }
        };
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.15,
                            g: 0.15,
                            b: 0.20,
                            a: 1.00,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            self.renderer.draw(&mut pass);
        }

        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
    }
}

#[derive(Default)]
struct App {
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let state = pollster::block_on(State::new(
            event_loop.owned_display_handle(),
            window.clone(),
        ));
        self.state = Some(state);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                state.update();
                state.render();
                state.get_window().request_redraw();
            }
            WindowEvent::Resized(size) => {
                state.resize(size);
            }
            WindowEvent::MouseInput {
                state: btn_state,
                button,
                ..
            } => {
                state.on_mouse_button(button, btn_state);
            }
            WindowEvent::CursorMoved { position, .. } => {
                state.on_mouse_moved(position.x, position.y);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                state.on_scroll(delta);
            }
            WindowEvent::ModifiersChanged(mods) => {
                state.modifiers = mods.state();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                let shift = state.modifiers.shift_key();
                state.on_key(key, shift);
            }
            _ => (),
        }
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
