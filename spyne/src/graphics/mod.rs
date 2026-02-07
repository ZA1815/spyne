mod vulkan;

use std::ops::BitOr;

trait Graphics {
    type PhysicalDevice;
    type Device<'a>;
    type CommandQueue;
    type Buffer;
    type Texture;
    type Shader;
    type GraphicsPipeline;
    type ComputePipeline;
    type CommandBuffer;
    type Swapchain;
    
    fn enumerate_devices(&self) -> Result<Vec<Self::PhysicalDevice>, GraphicsError>;
    fn open_device<'a>(&self, physical_device: &'a Self::PhysicalDevice, queues: &[QueueRequest]) -> Result<Self::Device<'a>, GraphicsError>;

    fn device_name(&self, physical_device: &Self::PhysicalDevice) -> String;
    fn supports_compute(&self, physical_device: &Self::PhysicalDevice) -> bool;
    fn has_unified_memory(&self, physical_device: &Self::PhysicalDevice) -> bool;
    fn max_texture_size_1d(&self, physical_device: &Self::PhysicalDevice) -> usize;
    fn max_texture_size_2d(&self, physical_device: &Self::PhysicalDevice) -> usize;
    fn max_texture_size_3d(&self, physical_device: &Self::PhysicalDevice) -> usize;
    fn max_texture_size_cube(&self, physical_device: &Self::PhysicalDevice) -> usize;
    fn supported_msaa_samples(&self, physical_device: &Self::PhysicalDevice) -> MsaaSampleCount;

    fn create_buffer<'a>(&self, device: &Self::Device<'a>, size: usize, usage: BufferUsage, location: MemoryLocation) -> Result<Self::Buffer, GraphicsError>;
    fn read_buffer<'a>(&self, device: &Self::Device<'a>, buffer: &Self::Buffer, offset: usize, size: usize) -> Result<Vec<u8>, GraphicsError>;
    fn write_buffer<'a>(&self, device: &Self::Device<'a>, buffer: &Self::Buffer, offset: usize, data: &[u8]) -> Result<(), GraphicsError>;

    fn create_texture<'a>(
        &self,
        device: &Self::Device<'a>,
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
    
    fn create_shader<'a>(&self, device: &Self::Device<'a>, bytecode: &[u8], stage: ShaderStage) -> Self::Shader;
    // Stub for now, complete when I create a GLSL to SPIR-V Compiler
    fn compile_shader<'a>(&self, device: &Self::Device<'a>, source: &str, stage: ShaderStage) -> Result<Self::Shader, ShaderCompileError>;

    fn create_graphics_pipeline<'a>(&self, device: &Self::Device<'a>, desc: &GraphicsPipelineDesc<Self>) -> Self::GraphicsPipeline;
    fn create_compute_pipeline<'a>(&self, device: &Self::Device<'a>, shader: &Self::Shader) -> Self::ComputePipeline;

    fn create_command_buffer(&self, queue: &Self::CommandQueue) -> Self::CommandBuffer;
    fn begin_command_buffer(&self, cmd: &mut Self::CommandBuffer);
    fn end_command_buffer(&self, cmd: &mut Self::CommandBuffer);

    fn begin_render_pass(&self, cmd: &mut Self::CommandBuffer, desc: &RenderPassDesc<Self>) -> Result<(), GraphicsError>;
    fn end_render_pass(&self, cmd: &mut Self::CommandBuffer);

    fn begin_compute_pass(&self, cmd: &mut Self::CommandBuffer) -> Result<(), GraphicsError>;
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
    
    fn create_swapchain<'a>(&self, device: &Self::Device<'a>, surface: SurfaceHandle, width: u32, height: u32, config: &SwapchainConfig) -> Self::Swapchain;
    fn resize_swapchain(&self, swapchain: &mut Self::Swapchain, width: u32, height: u32);
    fn acquire_next_image(&self, swapchain: &Self::Swapchain) -> Self::Texture;
    fn present(&self, queue: &Self::CommandQueue, swapchain: &Self::Swapchain);
}

pub enum GraphicsError {
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

pub struct MsaaSampleCount(u32);
impl BitOr for MsaaSampleCount {
    type Output = Self;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl MsaaSampleCount {
    pub const ONE: Self = Self(1 << 0);
    pub const TWO: Self = Self(1 << 1);
    pub const FOUR: Self = Self(1 << 2);
    pub const EIGHT: Self = Self(1 << 3);
    pub const SIXTEEN: Self = Self(1 << 4);
    pub const THIRTY_TWO: Self = Self(1 << 5);
    pub const SIXTY_FOUR: Self = Self(1 << 6);
}

pub struct BufferUsage(u32);
impl BitOr for BufferUsage {
    type Output = Self;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl BufferUsage {
    pub const TRANSFER_SRC: Self = Self(1 << 0);
    pub const TRANSFER_DST: Self = Self(1 << 1);
    pub const UNIFORM_TEXEL: Self = Self(1 << 2);
    pub const STORAGE_TEXEL: Self = Self(1 << 3);
    pub const UNIFORM: Self = Self(1 << 4);
    pub const STORAGE: Self = Self(1 << 5);
    pub const INDEX: Self = Self(1 << 6);
    pub const VERTEX: Self = Self(1 << 7);
    pub const INDIRECT: Self = Self(1 << 8);
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
