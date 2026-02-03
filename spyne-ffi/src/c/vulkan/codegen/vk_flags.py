from pathlib import Path

access = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/flags/access.rs")
buffer_usage = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/flags/buffer_usage.rs")
cull_mode = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/flags/cull_mode.rs")
fence_create = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/flags/fence_create.rs")
image_aspect = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/flags/image_aspect.rs")
image_usage = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/flags/image_usage.rs")
memory_property = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/flags/memory_property.rs")
pipeline_stage = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/flags/pipeline_stage.rs")
queue = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/flags/queue.rs")
sample_count = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/flags/sample_count.rs")
shader_stage = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/flags/shader_stage.rs")
flags_paths = [
    access,
    buffer_usage,
    cull_mode,
    fence_create,
    image_aspect,
    image_usage,
    memory_property,
    pipeline_stage,
    queue,
    sample_count,
    shader_stage
]
flags_names = [
    "VkAccessFlagBits",
    "VkBufferUsageFlagBits",
    "VkCullModeFlagBits",
    "VkFenceCreateFlagBits"
    "VkImageAspectFlagBits",
    "VkImageUsageFlagBits",
    "VkMemoryPropertyFlagBits",
    "VkPipelineStageFlagBits",
    "VkQueueFlagBits",
    "VkSampleCountFlagBits",
    "VkShaderStageFlagBits",
]
flags_types = [
    "u32",
    "u32",
    "u32",
    "u32",
    "u32",
    "u32",
    "u32",
    "u32",
    "u32",
    "u32",
    "u32"
]