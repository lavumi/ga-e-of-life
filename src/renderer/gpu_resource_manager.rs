use std::collections::HashMap;
use std::default::Default;
use std::sync::Arc;

use cgmath::SquareMatrix;
use wgpu::util::DeviceExt;
use wgpu::{BindGroup, BindGroupLayout, Buffer, Device, Queue, RenderPass};

use crate::renderer::builder::make_tile_mesh;
use crate::renderer::mesh::{InstanceTileRaw, Mesh};
use crate::renderer::texture::TextureViewAndSampler;

#[derive(Default)]
pub struct GPUResourceManager {
    bind_group_layouts: HashMap<String, Arc<BindGroupLayout>>,
    bind_groups: HashMap<String, HashMap<u32, Arc<BindGroup>>>,
    buffers: HashMap<String, Arc<Buffer>>,
    meshes_by_atlas: HashMap<String, Mesh>,
}

impl GPUResourceManager {
    pub fn initialize(&mut self, device: &Device) {
        self.init_base_layouts(device);
        self.init_camera_bind_group(device);
    }

    pub fn init_atlas(&mut self, device: &Device, queue: &Queue) {
        let diffuse_texture = TextureViewAndSampler::from_bytes(
            device,
            queue,
            include_bytes!("../../assets/img/agent.png"),
            "circle",
        )
        .unwrap();
        self.make_bind_group("agent", diffuse_texture, device);
    }

    pub fn init_meshes(&mut self, device: &Device) {
        self.add_mesh("agent", make_tile_mesh(device, "agent".to_string()));
        // self.add_mesh("bg", make_tile_mesh(device, "bg".to_string()));
        // self.add_mesh("player", make_tile_mesh(device, "player".to_string()));
    }

    fn init_base_layouts(&mut self, device: &Device) {
        self.add_bind_group_layout(
            "texture_bind_group_layout",
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            }),
        );
        self.add_bind_group_layout(
            "camera_bind_group_layout",
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            }),
        );
    }

    fn init_camera_bind_group(&mut self, device: &Device) {
        let camera_uniform: [[f32; 4]; 4] = cgmath::Matrix4::identity().into();
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let resources = camera_buffer.as_entire_binding();
        let camera_bind_group_layout = self
            .get_bind_group_layout("camera_bind_group_layout")
            .unwrap();
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: resources,
            }],
            label: Some("camera_bind_group"),
        });
        self.add_buffer("camera_matrix", camera_buffer);
        self.add_bind_group("camera", 0, camera_bind_group);
    }

    fn make_bind_group<T: Into<String> + Copy>(
        &mut self,
        name: T,
        diffuse_texture: TextureViewAndSampler,
        device: &Device,
    ) {
        let texture_bind_group_layout = self
            .get_bind_group_layout("texture_bind_group_layout")
            .unwrap();
        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        self.add_bind_group(name.into(), 1, diffuse_bind_group);
    }

    fn add_bind_group<T: Into<String>>(
        &mut self,
        name: T,
        bind_group_index: u32,
        bind_group: BindGroup,
    ) {
        let key = name.into();
        if self.bind_groups.contains_key(&key) {
            let bind_groups = self.bind_groups.get_mut(&key).unwrap();
            bind_groups.insert(bind_group_index, Arc::new(bind_group));
        } else {
            let mut hash_map = HashMap::new();
            hash_map.insert(bind_group_index, Arc::new(bind_group));
            self.bind_groups.insert(key.clone(), hash_map);
        }
    }

    pub fn set_bind_group<'a, T: Into<String>>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        name: T,
    ) {
        let key = name.into();
        if !self.bind_groups.contains_key(&key) {
            panic!("Resource Manager: Couldn't find any bind groups! {key}");
        }
        let bind_groups = self.bind_groups.get(&key).unwrap();

        for (key, val) in bind_groups.iter() {
            render_pass.set_bind_group(*key, val, &[]);
        }
    }

    fn add_bind_group_layout<T: Into<String>>(
        &mut self,
        name: T,
        bind_group_layout: BindGroupLayout,
    ) {
        let key = name.into();
        if self.bind_group_layouts.contains_key(&key) {
            panic!(
                "Bind group layout already exists use `get_bind_group_layout` or a different key."
            );
        }
        self.bind_group_layouts
            .insert(key, Arc::new(bind_group_layout));
    }

    pub fn get_bind_group_layout<T: Into<String>>(&self, name: T) -> Option<Arc<BindGroupLayout>> {
        let key = name.into();
        self.bind_group_layouts.get(&key).cloned()
    }

    fn add_mesh<T: Into<String>>(&mut self, name: T, mesh: Mesh) {
        let name = name.into();
        if self.meshes_by_atlas.contains_key(&name) {
            panic!("Buffer already exists use `get_buffer` or use a different key.");
        }
        self.meshes_by_atlas.insert(name, mesh);
    }

    // fn render_meshes_all<'a>(
    //     &'a self,
    //     render_pass: &mut RenderPass<'a>,
    // ){
    //     for (_, mesh) in self.meshes_by_atlas.iter() {
    //         match mesh.instance_buffer {
    //             //todo instance buffer 가 없어도 뭔가 렌더링 하게 해줘야지...
    //             None => {}
    //             Some(_) => {
    //                 //코드가 좀 안예쁘군...
    //                 self.set_bind_group(render_pass, mesh.atlas_name.clone());
    //
    //                 render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
    //                 render_pass.set_vertex_buffer(1, mesh.instance_buffer.as_ref().unwrap().slice(..));
    //                 render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    //                 render_pass.draw_indexed(0..mesh.num_indices, 0, 0..mesh.num_instances);
    //             }
    //         }
    //     }
    // }

    fn render_meshes<'a, T: Into<String>>(&'a self, render_pass: &mut RenderPass<'a>, name: T) {
        match self.meshes_by_atlas.get(&name.into()) {
            None => {}
            Some(mesh) => {
                match mesh.instance_buffer {
                    //todo instance buffer 가 없어도 뭔가 렌더링 하게 해줘야지...
                    None => {}
                    Some(_) => {
                        //코드가 좀 안예쁘군...
                        self.set_bind_group(render_pass, mesh.atlas_name.clone());

                        render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                        render_pass
                            .set_vertex_buffer(1, mesh.instance_buffer.as_ref().unwrap().slice(..));
                        render_pass.set_index_buffer(
                            mesh.index_buffer.slice(..),
                            wgpu::IndexFormat::Uint16,
                        );
                        render_pass.draw_indexed(0..mesh.num_indices, 0, 0..mesh.num_instances);
                    }
                }
            }
        }
    }

    fn add_buffer<T: Into<String>>(&mut self, name: T, buffer: Buffer) {
        let name = name.into();
        if self.buffers.contains_key(&name) {
            panic!("Buffer already exists use `get_buffer` or use a different key.");
        }
        self.buffers.insert(name, Arc::new(buffer));
    }

    pub fn get_buffer<T: Into<String>>(&self, name: T) -> Arc<Buffer> {
        self.buffers.get(&name.into()).unwrap().clone()
    }

    pub fn update_mesh_instance<T, I>(
        &mut self,
        name: T,
        device: &Device,
        queue: &Queue,
        tile_instance: Vec<I>,
    ) where
        T: Into<String>,
        I: InstanceTileRaw,
    {
        let name_str = name.into();
        if self.meshes_by_atlas.get_mut(&name_str).is_none() {
            log::info!("{}", name_str);
            return;
        }
        let mesh = self.meshes_by_atlas.get_mut(&name_str).unwrap();
        if tile_instance.is_empty() {
            mesh.num_instances = 0;
            return;
        }
        if mesh.num_instances == tile_instance.len() as u32 {
            queue.write_buffer(
                mesh.instance_buffer.as_ref().unwrap(),
                0,
                bytemuck::cast_slice(&tile_instance),
            );
        } else {
            log::info!(
                "update_mesh_instance {} before : {} , after : {}",
                name_str,
                mesh.num_instances,
                tile_instance.len()
            );
            let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(format!("Instance Buffer {}", name_str).as_str()),
                contents: bytemuck::cast_slice(&tile_instance),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
            mesh.replace_instance(instance_buffer, tile_instance.len() as u32);
        }
    }

    pub fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        self.set_bind_group(render_pass, "camera");

        //이 형태 좀 많이 구린걸?
        //좀 알아서 랜더링 하고 싶은데
        //font 는 파이프라인이 달라서  많이 번거롭네
        self.render_meshes(render_pass, "agent");
    }

    pub fn init_ui_atlas(&mut self, device: &Device, font_texture: wgpu::Texture) {
        let diffuse_texture =
            TextureViewAndSampler::from_wgpu_texture(device, font_texture).unwrap();
        self.make_bind_group("font", diffuse_texture, device);
    }

    pub fn init_ui_meshes(&mut self, device: &Device) {
        self.add_mesh("font", make_tile_mesh(device, "font".to_string()));
    }

    pub fn render_ui<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        self.render_meshes(render_pass, "font");
    }
}
