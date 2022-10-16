use crate::{camera::Camera, canvas::Canvas, photons::Photon, physics_constants::TIME_SPEED};
use bytemuck::{Pod, Zeroable};
use std::{
    borrow::Cow,
    sync::{Arc, Mutex},
};
use wgpu::util::DeviceExt;

pub struct LightProcessor {
    device: wgpu::Device,
    queue: wgpu::Queue,

    // amount of photons to be emitted per frame
    photons_group_size: wgpu::BufferAddress,
    size_sq: u32,

    staging_buffer: wgpu::Buffer,
    photons_buffer: wgpu::Buffer,

    compute_pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Default)]
struct Params {
    size: u32,
    amount: u32,
    time_speed: f32,

    _pad: [u32; 9],
}

impl LightProcessor {
    pub async fn new(photons: Vec<Photon>) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_defaults(),
                },
                None,
            )
            .await
            .unwrap();

        let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let slice_size = photons.len() * std::mem::size_of::<Photon>();
        let size = slice_size as wgpu::BufferAddress;

        let size_sq = (size as f32).sqrt().ceil() as u32;
        // let size_2 = 2u32.pow((size_2 as f32).log2().ceil() as u32);

        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let params_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Params Buffer"),
            contents: bytemuck::cast_slice(&[Params {
                amount: photons.len() as u32,
                size: size_sq as u32,
                time_speed: TIME_SPEED,
                ..Default::default()
            }]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let photons_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Storage Buffer"),
            contents: bytemuck::cast_slice(&photons),
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &cs_module,
            entry_point: "main",
        });

        let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: params_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: photons_buffer.as_entire_binding(),
                },
            ],
        });

        LightProcessor {
            device,
            queue,
            photons_group_size: size,
            size_sq,
            photons_buffer,
            staging_buffer,
            bind_group,
            compute_pipeline,
        }
    }

    pub async fn process_light_for_group(
        &mut self,
        camera: Camera,
        canvas: Arc<Mutex<&mut Canvas>>,
    ) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut compute_pass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &self.bind_group, &[]);
            compute_pass.insert_debug_marker("compute photons");
            compute_pass.dispatch_workgroups(self.size_sq / 16, self.size_sq / 16, 1);
        }

        encoder.copy_buffer_to_buffer(
            &self.photons_buffer,
            0,
            &self.staging_buffer,
            0,
            self.photons_group_size,
        );

        self.queue.submit(Some(encoder.finish()));

        let buffer_slice = self.staging_buffer.slice(..);

        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        self.device.poll(wgpu::Maintain::Wait);

        if let Some(Ok(())) = receiver.receive().await {
            let data = buffer_slice.get_mapped_range();
            let result: Vec<Photon> = bytemuck::cast_slice(&data).to_vec();

            drop(data);
            self.staging_buffer.unmap();

            for photon in result.iter() {
                if let Some((uv, factor)) = camera.get_intersection(*photon) {
                    let mut canvas = canvas.lock().unwrap();
                    canvas.update_pixel_by_uv(1. - uv, photon.get_wavelength(), 1.0 - factor);
                }
            }
        } else {
            panic!("Failed to map buffer");
        }
    }
}
