use crate::{camera::Camera, canvas::Canvas, photons::Photon};
use bytemuck::{Pod, Zeroable};
use std::borrow::Cow;
use wgpu::util::DeviceExt;

pub struct LightProcessor {
    device: wgpu::Device,
    cs_module: wgpu::ShaderModule,
    queue: wgpu::Queue,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Default)]
struct Params {
    size: u32,
    amount: u32,

    _pad: [u32; 10],
}

impl LightProcessor {
    pub async fn new() -> Self {
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

        LightProcessor {
            device,
            cs_module,
            queue,
        }
    }

    pub async fn process_light_for_group(
        &mut self,
        camera: Camera,
        canvas: &mut Canvas,
        photons: &Vec<Photon>,
    ) -> Vec<Photon> {
        // photons
        //     .iter()
        //     .map(|photon| {
        //         if let Some((uv, factor)) = camera.get_intersection(*photon) {
        //             canvas.update_pixel_by_uv(1. - uv, photon.get_wavelength(), 1.0 - factor);
        //         }

        //         photon.process()
        //     })
        //     .collect()

        for photon in photons.iter() {
            if let Some((uv, factor)) = camera.get_intersection(*photon) {
                canvas.update_pixel_by_uv(1. - uv, photon.get_wavelength(), 1.0 - factor);
            }
        }

        let slice_size = photons.len() * std::mem::size_of::<Photon>();
        let size = slice_size as wgpu::BufferAddress;

        let size_2 = (size as f32).sqrt().ceil() as u32;
        // let size_2 = 2u32.pow((size_2 as f32).log2().ceil() as u32);

        let staging_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let params_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Params Buffer"),
                contents: bytemuck::cast_slice(&[Params {
                    amount: photons.len() as u32,
                    size: size_2 as u32,
                    ..Default::default()
                }]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let photons_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Storage Buffer"),
                contents: bytemuck::cast_slice(photons),
                usage: wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::COPY_DST
                    | wgpu::BufferUsages::COPY_SRC,
            });

        let compute_pipeline =
            self.device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: None,
                    layout: None,
                    module: &self.cs_module,
                    entry_point: "main",
                });

        let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
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

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut compute_pass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            compute_pass.set_pipeline(&compute_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.insert_debug_marker("compute photons");
            compute_pass.dispatch_workgroups(size_2 / 16, size_2 / 16, 1);
        }

        encoder.copy_buffer_to_buffer(&photons_buffer, 0, &staging_buffer, 0, size);

        self.queue.submit(Some(encoder.finish()));

        let buffer_slice = staging_buffer.slice(..);

        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        self.device.poll(wgpu::Maintain::Wait);

        if let Some(Ok(())) = receiver.receive().await {
            let data = buffer_slice.get_mapped_range();
            let result = bytemuck::cast_slice(&data).to_vec();

            drop(data);
            staging_buffer.unmap();

            result
        } else {
            panic!("Failed to map buffer");
        }
    }
}
