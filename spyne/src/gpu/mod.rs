mod vulkan;

use std::ops::BitOr;

trait Gpu {
    type PhysicalDevice;
    type Device;
    type CommandQueue;
    type Buffer;
    type Texture;
    type Shader;
    type GraphicsPipeline;
    type ComputePipeline;
    type CommandBuffer;
    type Swapchain;
    
    fn enumerate_devices(&self) -> Result<Vec<Self::PhysicalDevice>, GpuError>;
    fn open_device(&self, physical_device: &Self::PhysicalDevice, queues: &[QueueRequest]) -> Result<Self::Device, GpuError>;

    fn device_name(&self, physical_device: &Self::PhysicalDevice) -> String;
    fn supports_compute(&self, physical_device: &Self::PhysicalDevice) -> bool;
    fn has_unified_memory(&self, physical_device: &Self::PhysicalDevice) -> bool;
    fn max_texture_size(&self, physical_device: &Self::PhysicalDevice) -> usize;
    fn supported_msaa_samples(&self, physical_device: &Self::PhysicalDevice) -> MsaaSampleCount;

    fn create_buffer(&self, device: &Self::Device, size: usize, location: MemoryLocation) -> Self::Buffer;
    fn read_buffer(&self, buffer: &Self::Buffer, offset: usize, length: usize) -> Vec<u8>;
    fn write_buffer(&self, buffer: &Self::Buffer, offset: usize, data: &[u8]);

    fn create_texture(
        &self,
        device: &Self::Device,
        dimension: TextureDimension,
        format: TextureFormat,
        width: u32,
        height: u32,
        depth: u32,
        layers: u32,
        mip_levels: u32,
        usage: TextureUsage,
    ) -> Self::Texture;
    
    // Stub for now
    fn read_texture(
        &self,
        texture: &Self::Texture,
        mip_level: u32,
        layer: u32,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Vec<u8>;
    
    fn write_texture(
        &self,
        texture: &Self::Texture,
        mip_level: u32,
        layer: u32,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        data: &[u8],
    );
    
    fn create_shader(&self, device: &Self::Device, bytecode: &[u8], stage: ShaderStage) -> Self::Shader;
    // Stub for now, complete when I create a GLSL to SPIR-V Compiler
    fn compile_shader(&self, device: &Self::Device, source: &str, stage: ShaderStage) -> Result<Self::Shader, ShaderCompileError>;

    fn create_graphics_pipeline(&self, device: &Self::Device, desc: &GraphicsPipelineDesc<Self>) -> Self::GraphicsPipeline;
    fn create_compute_pipeline(&self, device: &Self::Device, shader: &Self::Shader) -> Self::ComputePipeline;

    fn create_command_buffer(&self, queue: &Self::CommandQueue) -> Self::CommandBuffer;
    fn begin_command_buffer(&self, cmd: &mut Self::CommandBuffer);
    fn end_command_buffer(&self, cmd: &mut Self::CommandBuffer);

    fn begin_render_pass(&self, cmd: &mut Self::CommandBuffer, desc: &RenderPassDesc<Self>) -> Result<(), GpuError>;
    fn end_render_pass(&self, cmd: &mut Self::CommandBuffer);

    fn begin_compute_pass(&self, cmd: &mut Self::CommandBuffer) -> Result<(), GpuError>;
    fn end_compute_pass(&self, cmd: &mut Self::CommandBuffer);

    fn cmd_bind_graphics_pipeline(&self, cmd: &mut Self::CommandBuffer, pipeline: &Self::GraphicsPipeline);
    fn cmd_bind_vertex_buffer(&self, cmd: &mut Self::CommandBuffer, buffer: &Self::Buffer, offset: usize);
    fn cmd_bind_index_buffer(&self, cmd: &mut Self::CommandBuffer, buffer: &Self::Buffer, offset: usize, index_type: IndexType);
    fn cmd_set_viewport(&self, cmd: &mut Self::CommandBuffer, x: f32, y: f32, width: f32, height: f32, min_depth: f32, max_depth: f32);
    fn cmd_set_scissor(&self, cmd: &mut Self::CommandBuffer, x: u32, y: u32, width: u32, height: u32);
    fn cmd_draw(&self, cmd: &mut Self::CommandBuffer, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32);
    fn cmd_draw_indexed(&self, cmd: &mut Self::CommandBuffer, index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32);

    fn cmd_bind_compute_pipeline(&self, cmd: &mut Self::CommandBuffer, pipeline: &Self::ComputePipeline);
    fn cmd_dispatch(&self, cmd: &mut Self::CommandBuffer, groups_x: u32, groups_y: u32, groups_z: u32);

    fn cmd_bind_buffer(&self, cmd: &mut Self::CommandBuffer, slot: u32, buffer: &Self::Buffer, offset: usize);
    fn cmd_bind_texture(&self, cmd: &mut Self::CommandBuffer, slot: u32, texture: &Self::Texture);

    fn submit(&self, queue: &Self::CommandQueue, cmd: &Self::CommandBuffer);
    
    fn create_swapchain(&self, device: &Self::Device, surface: SurfaceHandle, width: u32, height: u32, config: &SwapchainConfig) -> Self::Swapchain;
    fn resize_swapchain(&self, swapchain: &mut Self::Swapchain, width: u32, height: u32);
    fn acquire_next_image(&self, swapchain: &Self::Swapchain) -> Self::Texture;
    fn present(&self, queue: &Self::CommandQueue, swapchain: &Self::Swapchain);
}

pub enum GpuError {
    InitializationFailed(String),
    OutOfMemory(String),
    DeviceLost(String),
    FeatureNotSupported(String),
    QueueCapabilityMismatch(String),
    BackendError { code: i32, message: String }
}

pub struct QueueRequest {
    pub capabilities: QueueCapabilities,
    pub count: usize
}

#[derive(Debug, Clone, Copy)]
pub struct QueueCapabilities(u32);
impl BitOr for QueueCapabilities {
    type Output = Self;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl QueueCapabilities {
    pub const GRAPHICS: Self = Self(1 << 0);
    pub const COMPUTE: Self = Self(1 << 1);
    pub const TRANSFER: Self = Self(1 << 2);
}

pub enum MemoryLocation {
    Shared,
    Private
}

pub enum TextureDimension {
    D1,
    D2,
    D3,
    Cube
}

pub enum TextureFormat {
    RGBA8,
    BGRA8,
    R8,
    RGBA16F,
    RGBA32F,
    Depth24,
    Depth32F
}

pub struct TextureUsage(u32);
impl BitOr for TextureUsage {
    type Output = Self;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl TextureUsage {
    pub const SAMPLE: Self = Self(1 << 0);
    pub const RENDER_TARGET: Self = Self(1 << 1);
    pub const STORAGE: Self = Self(1 << 2);
}

pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute
}

pub struct ShaderCompileError {
    message: String
}
