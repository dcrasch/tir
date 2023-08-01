use std::fs;

use lyon::math::*;
use lyon::tessellation;
use lyon::tessellation::geometry_builder::*;
use lyon::tessellation::{FillOptions, FillTessellator};
use lyon::tessellation::{StrokeOptions, StrokeTessellator};
use palette::{FromColor, Hsl, Srgb};
use rand::prelude::*;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

// For create_buffer_init()
use wgpu::util::DeviceExt;

use futures::executor::block_on;

use tessellations::render::*;
use tessellations::tessellationfigure::{TessellationFigure, TessellationPlane};
use tessellations::tessellationline::PointIndexPath;

//use log;

const PRIM_BUFFER_LEN: usize = 1356;

#[repr(C)]
#[derive(Copy, Clone)]
struct Globals {
    resolution: [f32; 2],
    scroll_offset: [f32; 2],
    zoom: f32,
    _pad: f32,
}

unsafe impl bytemuck::Pod for Globals {}
unsafe impl bytemuck::Zeroable for Globals {}

#[repr(C)]
#[derive(Copy, Clone)]
struct GpuVertex {
    position: [f32; 2],
    normal: [f32; 2],
    prim_id: u32,
}
unsafe impl bytemuck::Pod for GpuVertex {}
unsafe impl bytemuck::Zeroable for GpuVertex {}

#[repr(C)]
#[derive(Copy, Clone)]
struct Primitive {
    color: [f32; 4],
    translate: [f32; 2],
    z_index: i32,
    width: f32,
    angle: f32,
    scale: f32,
    _pad1: i32,
    _pad2: i32,
}

impl Primitive {
    const DEFAULT: Self = Primitive {
        color: [0.0; 4],
        translate: [0.0; 2],
        z_index: 0,
        width: 0.0,
        angle: 0.0,
        scale: 1.0,
        _pad1: 0,
        _pad2: 0,
    };
}

unsafe impl bytemuck::Pod for Primitive {}
unsafe impl bytemuck::Zeroable for Primitive {}

#[repr(C)]
#[derive(Copy, Clone)]
struct BgPoint {
    point: [f32; 2],
}
unsafe impl bytemuck::Pod for BgPoint {}
unsafe impl bytemuck::Zeroable for BgPoint {}

const DEFAULT_WINDOW_WIDTH: f32 = 800.0;
const DEFAULT_WINDOW_HEIGHT: f32 = 800.0;

/// Creates a texture that uses MSAA and fits a given swap chain
fn create_multisampled_framebuffer(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    sample_count: u32,
) -> wgpu::TextureView {
    let multisampled_frame_descriptor = &wgpu::TextureDescriptor {
        label: Some("Multisampled frame descriptor"),
        size: wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count,
        dimension: wgpu::TextureDimension::D2,
        format: config.format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    };

    device
        .create_texture(multisampled_frame_descriptor)
        .create_view(&wgpu::TextureViewDescriptor::default())
}

fn main() {
    env_logger::init();
    println!("== wgpu example ==");
    println!("Controls:");
    println!("  Arrow keys: scrolling");
    println!("  PgUp/PgDown: zoom in/out");
    println!("  b: toggle drawing the background");
    println!("  a/z: increase/decrease the stroke width");

    let mut palette: Vec<Srgb<f32>> = generate_palette();
    println!("{:?}", palette);
    // add tessellation square
    let mut f = TessellationFigure::triangle();
    let plane = TessellationPlane {};
    let m: Transform = Transform::scale(100.0, 100.0).then_translate(euclid::vec2(0.0, 0.0));

    // Number of samples for anti-aliasing
    // Set to 1 to disable
    let sample_count = 4;

    let tolerance = 0.02;

    let stroke_prim_id = 0;
    let fill_prim_id = 1;

    let mut scene = SceneParams {
        target_zoom: 2.0,
        zoom: 2.0,
        target_scroll: vector(0.0, 0.0),
        scroll: vector(0.0, 0.0),
        show_points: true,
        stroke_width: 1.0,
        target_stroke_width: 1.0,
        draw_background: true,
        window_size: PhysicalSize::new(DEFAULT_WINDOW_WIDTH as u32, DEFAULT_WINDOW_HEIGHT as u32),
        size_changed: true,
        render: false,

        drag_start: None,
        mouse_position: None,
        selected_point_index: None,
    };

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new().with_inner_size(scene.window_size);
    let window = window_builder.build(&event_loop).unwrap();

    // create an instance
    let instance = wgpu::Instance::default();

    // create an surface
    let surface = unsafe { instance.create_surface(&window) }.unwrap();

    // create an adapter
    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }))
    .unwrap();
    // create a device and a queue
    let (device, queue) = block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            features: wgpu::Features::default(),
            limits: wgpu::Limits::default(),
        },
        None,
    ))
    .unwrap();

    let prim_buffer_byte_size = (PRIM_BUFFER_LEN * std::mem::size_of::<Primitive>()) as u64;
    let globals_buffer_byte_size = std::mem::size_of::<Globals>() as u64;

    let prims_ubo = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Prims ubo"),
        size: prim_buffer_byte_size,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let globals_ubo = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Globals ubo"),
        size: globals_buffer_byte_size,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let vs_module = &device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Geometry vs"),
        source: wgpu::ShaderSource::Wgsl(include_str!("./../shaders/geometry.vs.wgsl").into()),
    });
    let fs_module = &device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Geometry fs"),
        source: wgpu::ShaderSource::Wgsl(include_str!("./../shaders/geometry.fs.wgsl").into()),
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Bind group layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(globals_buffer_byte_size),
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(prim_buffer_byte_size),
                },
                count: None,
            },
        ],
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(globals_ubo.as_entire_buffer_binding()),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer(prims_ubo.as_entire_buffer_binding()),
            },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
        label: None,
    });

    let depth_stencil_state = Some(wgpu::DepthStencilState {
        format: wgpu::TextureFormat::Depth32Float,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Greater,
        stencil: wgpu::StencilState {
            front: wgpu::StencilFaceState::IGNORE,
            back: wgpu::StencilFaceState::IGNORE,
            read_mask: 0,
            write_mask: 0,
        },
        bias: wgpu::DepthBiasState::default(),
    });

    let mut render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: vs_module,
            entry_point: "main",
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<GpuVertex>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttribute {
                        offset: 0,
                        format: wgpu::VertexFormat::Float32x2,
                        shader_location: 0,
                    },
                    wgpu::VertexAttribute {
                        offset: 8,
                        format: wgpu::VertexFormat::Float32x2,
                        shader_location: 1,
                    },
                    wgpu::VertexAttribute {
                        offset: 16,
                        format: wgpu::VertexFormat::Uint32,
                        shader_location: 2,
                    },
                ],
            }],
        },
        fragment: Some(wgpu::FragmentState {
            module: fs_module,
            entry_point: "main",
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Bgra8Unorm,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            polygon_mode: wgpu::PolygonMode::Fill,
            front_face: wgpu::FrontFace::Ccw,
            strip_index_format: None,
            cull_mode: Some(wgpu::Face::Back),
            conservative: false,
            unclipped_depth: false,
        },
        depth_stencil: depth_stencil_state.clone(),
        multisample: wgpu::MultisampleState {
            count: sample_count,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    };

    let render_pipeline = device.create_render_pipeline(&render_pipeline_descriptor);

    // TODO: this isn't what we want: we'd need the equivalent of VK_POLYGON_MODE_LINE,
    // but it doesn't seem to be exposed by wgpu?
    render_pipeline_descriptor.primitive.topology = wgpu::PrimitiveTopology::LineList;

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let size = window.inner_size();

    let mut surface_desc = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    };

    let mut multisampled_render_target = None;

    surface.configure(&device, &surface_desc);

    let mut depth_texture_view = None;

    window.request_redraw();

    event_loop.run(move |event, _, control_flow| {
        if !update_inputs(
            event,
            &window,
            control_flow,
            &mut scene,
            &mut f,
            &mut palette,
        ) {
            // keep polling inputs.
            return;
        }

        if scene.size_changed {
            scene.size_changed = false;
            let physical = scene.window_size;
            surface_desc.width = physical.width;
            surface_desc.height = physical.height;
            surface.configure(&device, &surface_desc);

            let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Depth texture"),
                size: wgpu::Extent3d {
                    width: surface_desc.width,
                    height: surface_desc.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });

            depth_texture_view =
                Some(depth_texture.create_view(&wgpu::TextureViewDescriptor::default()));

            multisampled_render_target = if sample_count > 1 {
                Some(create_multisampled_framebuffer(
                    &device,
                    &surface_desc,
                    sample_count,
                ))
            } else {
                None
            };
        }

        if !scene.render {
            return;
        }

        scene.render = false;

        let frame = match surface.get_current_texture() {
            Ok(texture) => texture,
            Err(e) => {
                println!("Swap-chain error: {e:?}");
                return;
            }
        };

        let frame_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Encoder"),
        });

        let mut geometry: VertexBuffers<GpuVertex, u16> = VertexBuffers::new();

        let mut fill_tess = FillTessellator::new();
        let mut stroke_tess = StrokeTessellator::new();

        let lb = Box::new(LyonBackend);
        let path = lb.build(&f).unwrap();
        let grid = lb.build_plane(&plane, &f, &palette);
        //println!("{}",grid.len());
        fill_tess
            .tessellate_path(
                &path,
                &FillOptions::tolerance(tolerance).with_fill_rule(tessellation::FillRule::NonZero),
                &mut BuffersBuilder::new(&mut geometry, WithId(fill_prim_id as u32)),
            )
            .unwrap();

        let fill_range = 0..(geometry.indices.len() as u32);

        stroke_tess
            .tessellate_path(
                &path,
                &StrokeOptions::tolerance(tolerance)
                    .with_line_cap(lyon::path::LineCap::Square)
                    .with_line_join(lyon::path::LineJoin::Bevel)
                    .with_miter_limit(1.0),
                &mut BuffersBuilder::new(&mut geometry, WithId(stroke_prim_id as u32)),
            )
            .unwrap();

        let stroke_range = fill_range.end..(geometry.indices.len() as u32);

        let mut cpu_primitives = Vec::with_capacity(PRIM_BUFFER_LEN);
        for _ in 0..PRIM_BUFFER_LEN {
            cpu_primitives.push(Primitive {
                color: [1.0, 0.0, 0.0, 1.0],
                z_index: 0,
                width: 0.0,
                translate: [0.0, 0.0],
                angle: 0.0,
                ..Primitive::DEFAULT
            });
        }

        // Stroke primitive
        cpu_primitives[stroke_prim_id] = Primitive {
            color: [0.0, 0.0, 0.0, 1.0],
            z_index: 1,
            width: scene.stroke_width,
            scale: 100.0,
            ..Primitive::DEFAULT
        };

        // Main fill primitive
        cpu_primitives[fill_prim_id] = Primitive {
            color: [0.0, 1.0, 1.0, 1.0],
            z_index: 1,
            ..Primitive::DEFAULT
        };

        if scene.draw_background {
            // grid stuff
            for (i, p) in grid.iter().enumerate() {
                cpu_primitives[fill_prim_id + i] = Primitive {
                    color: [p.r, p.g, p.b, 1.0],
                    translate: [p.x * 100.0, p.y * 100.0],
                    z_index: 1,
                    angle: p.angle,
                    scale: 100.0,
                    ..Primitive::DEFAULT
                };
            }
        }
        let figure_count = 1 + grid.len() as u32;

        let vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&geometry.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let ibo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&geometry.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // render figure

        queue.write_buffer(
            &globals_ubo,
            0,
            bytemuck::cast_slice(&[Globals {
                resolution: [
                    scene.window_size.width as f32,
                    scene.window_size.height as f32,
                ],
                zoom: scene.zoom,
                scroll_offset: scene.scroll.to_array(),
                _pad: 0.0,
            }]),
        );

        queue.write_buffer(&prims_ubo, 0, bytemuck::cast_slice(&cpu_primitives));

        {
            // A resolve target is only supported if the attachment actually uses anti-aliasing
            // So if sample_count == 1 then we must render directly to the surface's buffer
            let color_attachment = if let Some(msaa_target) = &multisampled_render_target {
                wgpu::RenderPassColorAttachment {
                    view: msaa_target,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: true,
                    },
                    resolve_target: Some(&frame_view),
                }
            } else {
                wgpu::RenderPassColorAttachment {
                    view: &frame_view,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: true,
                    },
                    resolve_target: None,
                }
            };

            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(color_attachment)],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: depth_texture_view.as_ref().unwrap(),
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0.0),
                        store: true,
                    }),
                    stencil_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0),
                        store: true,
                    }),
                }),
            });

            pass.set_pipeline(&render_pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.set_index_buffer(ibo.slice(..), wgpu::IndexFormat::Uint16);
            pass.set_vertex_buffer(0, vbo.slice(..));

            pass.draw_indexed(stroke_range.clone(), 0, 0..1);
            pass.draw_indexed(fill_range.clone(), 0, 0..figure_count);
        }

        queue.submit(Some(encoder.finish()));
        frame.present();
    });
}

/// This vertex constructor forwards the positions and normals provided by the
/// tessellators and add a shape id.
pub struct WithId(pub u32);

impl FillVertexConstructor<GpuVertex> for WithId {
    fn new_vertex(&mut self, vertex: tessellation::FillVertex) -> GpuVertex {
        GpuVertex {
            position: vertex.position().to_array(),
            normal: [0.0, 0.0],
            prim_id: self.0,
        }
    }
}

impl StrokeVertexConstructor<GpuVertex> for WithId {
    fn new_vertex(&mut self, vertex: tessellation::StrokeVertex) -> GpuVertex {
        GpuVertex {
            position: vertex.position_on_path().to_array(),
            normal: vertex.normal().to_array(),
            prim_id: self.0,
        }
    }
}

pub struct Custom;

impl FillVertexConstructor<BgPoint> for Custom {
    fn new_vertex(&mut self, vertex: tessellation::FillVertex) -> BgPoint {
        BgPoint {
            point: vertex.position().to_array(),
        }
    }
}

struct SceneParams {
    target_zoom: f32,
    zoom: f32,
    target_scroll: Vector,
    scroll: Vector,
    show_points: bool,
    stroke_width: f32,
    target_stroke_width: f32,
    draw_background: bool,
    window_size: PhysicalSize<u32>,
    size_changed: bool,
    render: bool,
    drag_start: Option<(f32, f32)>,
    mouse_position: Option<(f32, f32)>,
    selected_point_index: Option<PointIndexPath>,
}

fn update_inputs(
    event: Event<()>,
    window: &Window,
    control_flow: &mut ControlFlow,
    scene: &mut SceneParams,
    figure: &mut TessellationFigure,
    palette: &mut Vec<Srgb<f32>>,
) -> bool {
    let mpx = (window.inner_size().width as f32) / 2.0;
    let mpy = (window.inner_size().height as f32) / 2.0;
    match event {
        Event::RedrawRequested(_) => {
            scene.render = true;
        }
        Event::RedrawEventsCleared => {
            window.request_redraw();
        }
        Event::WindowEvent {
            event: WindowEvent::Destroyed,
            ..
        }
        | Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
            return false;
        }
        Event::WindowEvent {
            event: WindowEvent::Resized(size),
            ..
        } => {
            scene.window_size = size;
            scene.size_changed = true;
        }
        Event::WindowEvent {
            event:
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    button: MouseButton::Left,
                    ..
                },
            ..
        } => {
            if let Some(mouse) = scene.mouse_position {
                //println!("pressed");
                scene.drag_start = scene.mouse_position;
                let mi: Transform = Transform::translation(
                    scene.scroll.x * scene.zoom - mpx,
                    scene.scroll.y * scene.zoom - mpy,
                )
                .then_scale(1.0 / scene.zoom / 100.0, 1.0 / scene.zoom / 100.0);

                let p = mi.transform_point(Point::new(mouse.0, mouse.1));
                match figure.hitpoints(p, 0.05) {
                    Some(h) => {
                        //println!("hit ");
                        scene.selected_point_index = Some(h)
                    }
                    _ => match figure.hitline(p, 0.05) {
                        Some(h) => {
                            //println!("breakline");
                            figure.insert(h, p);
                            scene.selected_point_index = Some(PointIndexPath {
                                line_index: h.line_index,
                                point_index: h.point_index + 1,
                                corrp: h.corrp,
                            });
                        }
                        _ => scene.selected_point_index = None,
                    },
                };
            }
        }
        Event::WindowEvent {
            event:
                WindowEvent::MouseInput {
                    state: ElementState::Released,
                    button: MouseButton::Left,
                    ..
                },
            ..
        } => {
            scene.drag_start = None;
        }
        Event::WindowEvent {
            event: WindowEvent::CursorMoved { position, .. },
            ..
        } => {
            let mouse_point = Some((position.x as f32, position.y as f32));
            if let Some(mouse) = mouse_point {
                let mi: Transform = Transform::translation(
                    scene.scroll.x * scene.zoom - mpx,
                    scene.scroll.y * scene.zoom - mpy,
                )
                .then_scale(1.0 / scene.zoom / 100.0, 1.0 / scene.zoom / 100.0);

                let p = mi.transform_point(Point::new(mouse.0, mouse.1));
                if let Some(d) = scene.drag_start {
                    //println!("dragging");
                    if d != mouse {
                        if let Some(h) = scene.selected_point_index {
                            figure.update(h, p);
                            window.request_redraw();
                        }
                    }
                };
                scene.mouse_position = Some(mouse);
            }
        }
        Event::WindowEvent {
            event:
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(key),
                            ..
                        },
                    ..
                },
            ..
        } => match key {
            VirtualKeyCode::Escape => {
                *control_flow = ControlFlow::Exit;
                return false;
            }
            VirtualKeyCode::PageDown => {
                scene.target_zoom *= 0.8;
            }
            VirtualKeyCode::PageUp => {
                scene.target_zoom *= 1.25;
            }
            VirtualKeyCode::Left => {
                scene.target_scroll.x -= 50.0 / scene.target_zoom;
            }
            VirtualKeyCode::Right => {
                scene.target_scroll.x += 50.0 / scene.target_zoom;
            }
            VirtualKeyCode::Up => {
                scene.target_scroll.y -= 50.0 / scene.target_zoom;
            }
            VirtualKeyCode::Down => {
                scene.target_scroll.y += 50.0 / scene.target_zoom;
            }
            VirtualKeyCode::P => {
                scene.show_points = !scene.show_points;
            }
            VirtualKeyCode::B => {
                scene.draw_background = !scene.draw_background;
            }
            VirtualKeyCode::A => {
                scene.target_stroke_width /= 0.8;
            }
            VirtualKeyCode::Z => {
                scene.target_stroke_width *= 0.8;
            }
            VirtualKeyCode::R => {
                *palette = generate_palette();
                window.request_redraw();
            }
            VirtualKeyCode::S => {
                fs::write(
                    "figure.json",
                    serde_json::to_string(figure)
                        .expect("json error")
                        .as_bytes(),
                )
                .expect("file error");
            }
            VirtualKeyCode::L => {
                *figure = serde_json::from_str(
                    fs::read_to_string("figure.json")
                        .expect("file error")
                        .as_str(),
                )
                .expect("json error");
                window.request_redraw();
            }
            VirtualKeyCode::W => {
                let p = TessellationPlane {};
                let svgbackend = Box::new(SVGBackend);
                let m: Transform =
                    Transform::scale(100.0, 100.0).then_translate(euclid::vec2(100.0, 100.0));

                let svg = svgbackend.compose_plane(&p, &figure, &m).unwrap();
                svg.save_svg(std::path::Path::new("out.svg"));
            }
            VirtualKeyCode::Key1 => {
                *figure = TessellationFigure::square();
                window.request_redraw();
            }
            VirtualKeyCode::Key2 => {
                *figure = TessellationFigure::triangle();
                window.request_redraw();
            }
            VirtualKeyCode::Key3 => {
                *figure = TessellationFigure::square90();
                window.request_redraw();
            }
            VirtualKeyCode::Key4 => {
                *figure = TessellationFigure::diamond();
                window.request_redraw();
            }
            VirtualKeyCode::Key5 => {
                *figure = TessellationFigure::brick();
                window.request_redraw();
            }
            VirtualKeyCode::Key6 => {
                *figure = TessellationFigure::hexagon();
                window.request_redraw();
            }
            _key => {}
        },
        _evt => {
            //println!("{:?}", _evt);
        }
    }
    //println!(" -- zoom: {}, scroll: {:?}", scene.target_zoom, scene.target_scroll);

    scene.zoom += (scene.target_zoom - scene.zoom) / 3.0;
    scene.scroll = scene.scroll + (scene.target_scroll - scene.scroll) / 3.0;
    scene.stroke_width =
        scene.stroke_width + (scene.target_stroke_width - scene.stroke_width) / 5.0;

    *control_flow = ControlFlow::Poll;

    true
}

fn generate_palette() -> Vec<Srgb<f32>> {
    let mut rng: ThreadRng = rand::thread_rng();

    // Generate a random base hue
    let base_hue = rng.gen_range(0..360) as f32;
    let mut palette = Vec::<Srgb<f32>>::new();
    // Calculate three analogous hues
    let hue_shift = 30.0; // You can adjust this value for different analogous colors
    let analogous_hues = vec![
        (base_hue + hue_shift) % 360.0,
        (base_hue + 2.0 * hue_shift) % 360.0,
        (base_hue + 3.0 * hue_shift) % 360.0,
    ];

    // Convert analogous hues to RGB colors and add to the palette
    for hue in &analogous_hues {
        let color = Hsl::new(*hue, 0.7, 0.6).into_format();
        palette.push(Srgb::from_color(color));
    }

    // Add a complementary color to the palette
    let complementary_hue = (base_hue + 180.0) % 360.0;
    let complementary_color = Hsl::new(complementary_hue, 0.7, 0.6).into_format();
    palette.push(Srgb::from_color(complementary_color));
    return palette;
}
