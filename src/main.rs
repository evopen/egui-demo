#![allow(unused)]

struct Engine {
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    swap_chain_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    ui_instance: egui_winit::Instance,
    ui_render_pass: egui_wgpu::RenderPass,
    scale_factor: f64,
}

impl Engine {
    pub async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        log::info!("using {}", adapter.get_info().name);
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None,
            )
            .await
            .unwrap();

        let swap_chain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        let ui_instance = egui_winit::Instance::new(size, window.scale_factor());
        let ui_render_pass = egui_wgpu::RenderPass::new(&device, swap_chain_desc.format);

        let scale_factor = window.scale_factor();

        Self {
            size,
            surface,
            device,
            queue,
            swap_chain_desc,
            swap_chain,
            ui_instance,
            ui_render_pass,
            scale_factor,
        }
    }

    fn resize(&mut self, new_size: &winit::dpi::PhysicalSize<u32>) {
        self.size.clone_from(new_size);
        self.swap_chain_desc.width = self.size.width;
        self.swap_chain_desc.height = self.size.height;
        self.swap_chain = self
            .device
            .create_swap_chain(&self.surface, &self.swap_chain_desc);
        log::info!(
            "swap chain resized to {}, {}",
            self.size.width,
            self.size.height
        );
    }

    fn input(&mut self, event: &winit::event::WindowEvent) {
        self.ui_instance.input(event);
        match event {
            winit::event::WindowEvent::Resized(new_inner_size) => {
                self.resize(new_inner_size);
            }
            winit::event::WindowEvent::Moved(_) => {}
            winit::event::WindowEvent::CloseRequested => {}
            winit::event::WindowEvent::Destroyed => {}
            winit::event::WindowEvent::DroppedFile(_) => {}
            winit::event::WindowEvent::HoveredFile(_) => {}
            winit::event::WindowEvent::HoveredFileCancelled => {}
            winit::event::WindowEvent::ReceivedCharacter(_) => {}
            winit::event::WindowEvent::Focused(_) => {}
            winit::event::WindowEvent::KeyboardInput {
                device_id,
                input,
                is_synthetic,
            } => {}
            winit::event::WindowEvent::ModifiersChanged(_) => {}
            winit::event::WindowEvent::CursorMoved {
                device_id,
                position,
                ..
            } => {}
            winit::event::WindowEvent::CursorEntered { device_id } => {}
            winit::event::WindowEvent::CursorLeft { device_id } => {}
            winit::event::WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
                ..
            } => {}
            winit::event::WindowEvent::MouseInput {
                device_id,
                state,
                button,
                ..
            } => {}
            winit::event::WindowEvent::TouchpadPressure {
                device_id,
                pressure,
                stage,
            } => {}
            winit::event::WindowEvent::AxisMotion {
                device_id,
                axis,
                value,
            } => {}
            winit::event::WindowEvent::Touch(_) => {}
            winit::event::WindowEvent::ScaleFactorChanged {
                scale_factor,
                new_inner_size,
            } => {}
            winit::event::WindowEvent::ThemeChanged(_) => {}
        }
    }

    fn draw_ui(&mut self) {
        self.ui_instance.begin_frame();
        egui::CentralPanel::default().show(self.ui_instance.context(), |ui| {
            ui.button("1234567890");
            ui.button("numerous");
            ui.button("1234567890");
        });
        egui::Window::new("hello").show(self.ui_instance.context(), |ui| {
            if ui.button("fuckyou").clicked {
                println!("this");
            }
        });

        self.ui_instance.end_frame();
    }

    fn update(&mut self) {
        self.ui_instance.update_time();
        self.draw_ui();
        self.ui_render_pass.upload_buffers(
            &mut self.device,
            &mut self.queue,
            egui::Vec2::new(self.size.width as f32, self.size.height as f32),
            self.ui_instance.paint_jobs(),
        );
        self.ui_render_pass.upload_texture(
            &self.device,
            &self.queue,
            self.ui_instance.context().texture(),
        );
    }

    fn render(&mut self) {
        let frame = self.swap_chain.get_current_frame().unwrap().output;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Main Encoder"),
            });
        self.ui_render_pass
            .encode(&mut encoder, &frame.view, Some(wgpu::Color::BLUE));
        self.queue.submit(std::iter::once(encoder.finish()));
    }
}

fn main() {
    env_logger::builder().format_timestamp(None).init();

    log::info!("initializing");
    let time = std::time::Instant::now();

    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
        .with_title(env!("CARGO_PKG_NAME"))
        .build(&event_loop)
        .unwrap();

    let mut engine = futures::executor::block_on(Engine::new(&window));

    log::info!("initialized, took {} ms", time.elapsed().as_millis());
    drop(time);

    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::NewEvents(_) => {}
        winit::event::Event::WindowEvent { window_id, event } => {
            engine.input(&event);
            match event {
                winit::event::WindowEvent::Resized(_) => {}
                winit::event::WindowEvent::Moved(_) => {}
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                winit::event::WindowEvent::Destroyed => {}
                winit::event::WindowEvent::DroppedFile(_) => {}
                winit::event::WindowEvent::HoveredFile(_) => {}
                winit::event::WindowEvent::HoveredFileCancelled => {}
                winit::event::WindowEvent::ReceivedCharacter(_) => {}
                winit::event::WindowEvent::Focused(_) => {}
                winit::event::WindowEvent::KeyboardInput {
                    device_id,
                    input,
                    is_synthetic,
                } => match input {
                    winit::event::KeyboardInput {
                        virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                        state: winit::event::ElementState::Pressed,
                        ..
                    } => {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    }
                    _ => {}
                },
                winit::event::WindowEvent::ModifiersChanged(_) => {}
                winit::event::WindowEvent::CursorMoved {
                    device_id,
                    position,
                    ..
                } => {}
                winit::event::WindowEvent::CursorEntered { device_id } => {}
                winit::event::WindowEvent::CursorLeft { device_id } => {}
                winit::event::WindowEvent::MouseWheel {
                    device_id,
                    delta,
                    phase,
                    ..
                } => {}
                winit::event::WindowEvent::MouseInput {
                    device_id,
                    state,
                    button,
                    ..
                } => {}
                winit::event::WindowEvent::TouchpadPressure {
                    device_id,
                    pressure,
                    stage,
                } => {}
                winit::event::WindowEvent::AxisMotion {
                    device_id,
                    axis,
                    value,
                } => {}
                winit::event::WindowEvent::Touch(_) => {}
                winit::event::WindowEvent::ScaleFactorChanged {
                    scale_factor,
                    new_inner_size,
                } => {}
                winit::event::WindowEvent::ThemeChanged(_) => {}
            }
        }
        winit::event::Event::DeviceEvent { device_id, event } => {}
        winit::event::Event::UserEvent(_) => {}
        winit::event::Event::Suspended => {}
        winit::event::Event::Resumed => {}
        winit::event::Event::MainEventsCleared => {
            window.request_redraw();
        }
        winit::event::Event::RedrawRequested(_) => {
            engine.update();
            engine.render();
        }
        winit::event::Event::RedrawEventsCleared => {}
        winit::event::Event::LoopDestroyed => {}
    });
}
