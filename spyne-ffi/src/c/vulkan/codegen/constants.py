from xml.etree.ElementTree import Element
from pathlib import Path

attachment_load_op = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/attachment_load_op.rs")
attachment_store_op = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/attachment_store_op.rs")
blend_factor = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/blend_factor.rs")
blend_op = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/blend_op.rs")
command_buffer_level = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/command_buffer_level.rs")
component_swizzle = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/component_swizzle.rs")
dynamic_state = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/dynamic_state.rs")
khr_color_space = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/khr/color_space.rs")
khr_present_mode = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/khr/present_mode.rs")
format = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/format.rs")
front_face = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/front_face.rs")
image_layout = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/image_layout.rs")
image_tiling = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/image_tiling.rs")
image_type = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/image_type.rs")
image_view_type = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/image_view_type.rs")
index_type = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/index_type.rs")
pipeline_bind_point = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/pipeline_bind_point.rs")
polygon_mode = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/polygon_mode.rs")
primitive_topology = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/primitive_topology.rs")
result = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/result.rs")
sharing_mode = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/sharing_mode.rs")
structure_type = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/structure_type.rs")
subpass_contents = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/subpass_contents.rs")
constant_paths = [
    attachment_load_op,
    attachment_store_op,
    blend_factor,
    blend_op,
    command_buffer_level,
    component_swizzle,
    dynamic_state,
    khr_color_space,
    khr_present_mode,
    format,
    front_face,
    image_layout,
    image_tiling,
    image_type,
    image_view_type,
    index_type,
    pipeline_bind_point,
    polygon_mode,
    primitive_topology,
    result,
    sharing_mode,
    structure_type,
    subpass_contents
]
constant_names = [
    "VkAttachmentLoadOp",
    "VkAttachmentStoreOp",
    "VkBlendFactor",
    "VkBlendOp",
    "VkCommandBufferLevel",
    "VkComponentSwizzle",
    "VkDynamicState",
    "VkColorSpaceKHR",
    "VkPresentModeKHR",
    "VkFormat",
    "VkFrontFace",
    "VkImageLayout",
    "VkImageTiling",
    "VkImageType",
    "VkImageViewType",
    "VkIndexType",
    "VkPipelineBindPoint",
    "VkPolygonMode",
    "VkPrimitiveTopology",
    "VkResult",
    "VkSharingMode",
    "VkStructureType",
    "VkSubpassContents"
]
constant_types = [
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
    "u32",
    "u32",
    "u32",
    "u32",
    "u32",
    "u32",
    "u32",
    "u32",
    "u32",
    "i32",
    "u32",
    "u32",
    "u32"
]

def constants_parse(root: Element[str]):
    print(len(constant_paths))
    print(len(constant_names))
    print(len(constant_types))
    for (path, enum_name, type) in zip(constant_paths, constant_names, constant_types):
        with open(path, 'w') as f:
            print("#[repr(transparent)]", file=f)
            print("#[derive(Debug, Clone, Copy, PartialEq, Eq)]", file=f)
            print(f"pub struct {enum_name}({type});", file=f)
            print("", file=f)
            for enum in root.iter('enums'):
                if enum.attrib.get('name') == enum_name:
                    for enum_var in enum:
                        name = enum_var.attrib.get('name')
                        value = enum_var.attrib.get('value')
                        if name is not None and value is not None:
                            print(f"pub const {name}: {enum_name} = {enum_name}({value});", file=f)