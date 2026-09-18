use std::mem;
use bytemuck::{Pod, Zeroable};
use easy_gpu::assets::{BufferLayout, GpuInstance, GpuVertex};
use easy_gpu::wgpu::VertexFormat;
use easy_gpu::wgpu::VertexFormat::{Float32, Float32x2};
use easy_gpu::wgpu::VertexStepMode::Instance;
use crate::math::Vec2;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(super) struct Vertex{
    position: [f32;2],
}
impl Vertex {
    pub fn new(position: [f32;2]) -> Self {
        Vertex{position}
    }
}
impl GpuVertex for Vertex {
    fn buffer_layout() -> BufferLayout {
        BufferLayout::new()
            .stride(size_of::<Self>() as u64)
            .attribute(0,0,VertexFormat::Float32x2)
    }
}

#[repr(C)]
#[derive(Clone,Copy,Pod,Zeroable)]
pub struct BoidSprite {
    pos: Vec2,
    rot: f32,
}

impl GpuInstance for BoidSprite {
    fn buffer_layout() -> BufferLayout {
        BufferLayout::new()
            .stride(mem::size_of::<BoidSprite>() as u64)
            .step_mode(Instance)
            .attribute(0,0,Float32x2)
            .attribute(1,8,Float32)
    }
}