use clap::Clap;
use futures::executor::block_on;
use naga::{valid::{ValidationFlags, Validator}};
use notify::{RawEvent, RecommendedWatcher, Watcher};
use std::{borrow::Cow, fs::{read_to_string, OpenOptions}, io::Write, path::{Path, PathBuf}, sync::mpsc::channel, time::Instant};
use wgpu::{Adapter, BackendBit, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BufferBindingType, BufferUsage, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, Instance, Limits, LoadOp, Operations, PipelineLayout, PrimitiveState, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RequestAdapterOptions, ShaderFlags, ShaderModule, ShaderSource, ShaderStage, Surface, SwapChain, SwapChainDescriptor, TextureFormat, TextureUsage, util::{BufferInitDescriptor, DeviceExt}};
use winit::{
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::{Window, WindowBuilder},
};

#[derive(Debug)]
struct Reload;

#[derive(Clap)]
struct Opts {
    wgsl_file: PathBuf,

    #[clap(short)]
    create: bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
struct Uniforms {
    pub mouse: [f32; 2],
    pub time: f32,
}

impl Default for Uniforms {
    fn default() -> Uniforms {
        Uniforms {
            time: 0.,
            mouse: [0.0, 0.0],
        }
    }
}

impl Uniforms {
    fn as_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }
}

struct Playground {
    watch_path: PathBuf,
    render_pipeline: RenderPipeline,
    window: Window,
    device: Device,
    vertex_shader_module: ShaderModule,
    pipeline_layout: PipelineLayout,
    swapchain_format: TextureFormat,
    swap_chain: SwapChain,
    sc_desc: SwapChainDescriptor,
    surface: Surface,
    
    uniforms: Uniforms,
}

impl Playground {
    fn reload(&mut self) {
        println!("Reload.");

        self.recreate_pipeline();

        self.window.request_redraw();
    }

    fn listen(watch_path: PathBuf, proxy: EventLoopProxy<Reload>) {
        let (tx, rx) = channel();

        let mut watcher: RecommendedWatcher = Watcher::new_raw(tx).unwrap();

        watcher
            .watch(&watch_path, notify::RecursiveMode::NonRecursive)
            .unwrap();

        loop {
            match rx.recv() {
                Ok(RawEvent {
                    path: Some(_),
                    op: Ok(_),
                    ..
                }) => {
                    proxy.send_event(Reload).unwrap();
                }
                Ok(event) => println!("broken event: {:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    }

    async fn get_async_stuff(instance: &Instance, surface: &Surface) -> (Adapter, Device, Queue) {
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    features: Features::default(),
                    limits: Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        (adapter, device, queue)
    }

    fn recreate_pipeline(&mut self) {
        match Self::create_pipeline(
            &self.device,
            &self.vertex_shader_module,
            &self.pipeline_layout,
            self.swapchain_format,
            &self.watch_path,
        ) {
            Ok(render_pipeline) => self.render_pipeline = render_pipeline,
            Err(e) => println!("{}", e),
        }
    }

    fn create_pipeline(
        device: &Device,
        vertex_shader_module: &ShaderModule,
        pipeline_layout: &PipelineLayout,
        swapchain_format: TextureFormat,
        frag_shader_path: &Path,
    ) -> Result<RenderPipeline, String> {
        let frag_wgsl = read_to_string(&frag_shader_path).unwrap();

        let module =
            naga::front::wgsl::parse_str(&frag_wgsl).map_err(|e| format!("Parse Error: {}", &e))?;

        Validator::new(ValidationFlags::all())
            .validate(&module)
            .map_err(|e| format!("Validation Error: {}", &e))?;

        let fragement_shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Fragment shader"),
            source: ShaderSource::Wgsl(Cow::Owned(frag_wgsl)),
            flags: ShaderFlags::all(),
        });

        Ok(
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vertex_shader_module,
                    entry_point: "vs_main",
                    buffers: &[],
                },
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &fragement_shader_module,
                    entry_point: "fs_main",
                    targets: &[swapchain_format.into()],
                }),
            }),
        )
    }

    pub fn resize(&mut self, new_size: &PhysicalSize<u32>) {
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;

        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        self.window.request_redraw();
    }

    pub fn run(watch_path: PathBuf) {
        let event_loop: EventLoop<Reload> = EventLoop::with_user_event();
        let proxy = event_loop.create_proxy();

        {
            let watch_path = watch_path.clone();
            std::thread::spawn(move || Self::listen(watch_path, proxy));
        }

        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(600, 600))
            .with_title("WGSL Playground")
            .build(&event_loop)
            .unwrap();
        let size = window.inner_size();
        let instance = wgpu::Instance::new(BackendBit::all());
        let surface = unsafe { instance.create_surface(&window) };
        let (adapter, device, queue) = block_on(Self::get_async_stuff(&instance, &surface));
        let vertex_shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Vertex shader"),
            source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("vertex.wgsl"))),
            flags: ShaderFlags::all(),
        });

        let uniforms = Uniforms::default();

        let uniforms_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: uniforms.as_bytes(),
            usage: BufferUsage::UNIFORM | BufferUsage::COPY_DST,
        });

        let uniforms_buffer_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStage::FRAGMENT,
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                }
            ]
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&uniforms_buffer_layout],
            push_constant_ranges: &[],
        });

        let swapchain_format = adapter.get_swap_chain_preferred_format(&surface).unwrap();

        let render_pipeline = match Self::create_pipeline(
            &device,
            &vertex_shader_module,
            &pipeline_layout,
            swapchain_format,
            &watch_path,
        ) {
            Ok(render_pipeline) => render_pipeline,
            Err(e) => {
                println!("Could not start due to error: {}", &e);
                return;
            }
        };

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: TextureUsage::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let uniforms_buffer_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &uniforms_buffer_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: uniforms_buffer.as_entire_binding(),
                }
            ],
        });

        let mut playground = Playground {
            watch_path,
            render_pipeline,
            window,
            device,
            swapchain_format,
            pipeline_layout,
            vertex_shader_module,
            swap_chain,
            sc_desc,
            surface,
            uniforms,
        };

        let instant = Instant::now();

        event_loop.run(move |event, _, control_flow| match event {
            winit::event::Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(new_size) => playground.resize(new_size),
                WindowEvent::CursorMoved {position, ..} => {
                    let size = playground.window.inner_size();
                    let normalized_x = position.x as f32 / size.width as f32;
                    let normalized_y = position.y as f32 / size.height as f32;
                    playground.uniforms.mouse = [normalized_x * 2. - 1., -normalized_y * 2. + 1.];
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    playground.resize(new_inner_size)
                }
                _ => {}
            },
            winit::event::Event::RedrawRequested(_) => {
                playground.uniforms.time = instant.elapsed().as_secs_f32();
                queue.write_buffer(&uniforms_buffer, 0, playground.uniforms.as_bytes());
                let frame = playground.swap_chain.get_current_frame().unwrap().output;
                let mut encoder = playground
                    .device
                    .create_command_encoder(&CommandEncoderDescriptor { label: None });

                {
                    let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                        label: None,
                        color_attachments: &[RenderPassColorAttachment {
                            view: &frame.view,
                            resolve_target: None,
                            ops: Operations {
                                load: LoadOp::Clear(wgpu::Color::BLACK),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                    render_pass.set_pipeline(&playground.render_pipeline);
                    render_pass.set_bind_group(0, &uniforms_buffer_bind_group, &[]);
                    render_pass.draw(0..3, 0..1);
                }

                queue.submit(Some(encoder.finish()));
            }
            winit::event::Event::UserEvent(Reload) => {
                playground.reload();
            }
            winit::event::Event::MainEventsCleared => {
                playground.window.request_redraw();
            }
            _ => {}
        });
    }
}

fn main() {
    let opts = Opts::parse();

    if opts.create {
        let mut file = if let Ok(file) = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(opts.wgsl_file.clone())
        {
            file
        } else {
            println!(
                "Couldn't create file {:?}, make sure it doesn't already exist.",
                &opts.wgsl_file
            );
            return;
        };
        file.write_all(include_bytes!("frag.default.wgsl")).unwrap();
    }

    Playground::run(opts.wgsl_file);
}
