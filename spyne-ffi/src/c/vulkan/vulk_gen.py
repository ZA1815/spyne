import xml.etree.ElementTree as et
from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent
xml_path = BASE_DIR / "vk.xml"
constants_attachment_load_op = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/attachment_load_op.rs")
constants_attachment_store_op = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/attachment_store_op.rs")
constants_blend_factor = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/blend_factor.rs")
constants_blend_op = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/blend_op.rs")
constants_component_swizzle = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/component_swizzle.rs")
constants_dynamic_state = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/dynamic_state.rs")
constants_khr_color_space = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/khr/color_space.rs")
constants_khr_present_mode = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/khr/present_mode.rs")
constants_format = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/format.rs")
constants_front_face = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/front_face.rs")
constants_image_layout = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/image_layout.rs")
constants_image_tiling = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/image_tiling.rs")
constants_image_type = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/image_type.rs")
constants_image_view_type = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/image_view_type.rs")
constants_index_type = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/index_type.rs")
constants_pipeline_bind_point = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/pipeline_bind_point.rs")
constants_polygon_mode = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/polygon_mode.rs")
constants_primitive_topology = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/primitive_topology.rs")
constants_result = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/result.rs")
constants_sharing_mode = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/sharing_mode.rs")
constants_structure_type = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/constants/structure_type.rs")

tree = et.parse(xml_path)
root = tree.getroot()

def constants_parse(enum_name: str, type: str):
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

with open(constants_attachment_load_op, 'w') as f:
    constants_parse("VkAttachmentLoadOp", "u32")

with open(constants_attachment_store_op, 'w') as f:
    constants_parse("VkAttachmentStoreOp", "u32")

with open(constants_blend_factor, 'w') as f:
    constants_parse("VkBlendFactor", "u32")

with open(constants_blend_op, 'w') as f:
    constants_parse("VkBlendOp", "u32")

with open(constants_component_swizzle, 'w') as f:
    constants_parse("VkComponentSwizzle", "u32")

with open(constants_dynamic_state, 'w') as f:
    constants_parse("VkDynamicState", "u32")

with open(constants_khr_color_space, 'w') as f:
    constants_parse("VkColorSpaceKHR", "u32")

with open(constants_khr_present_mode, 'w') as f:
    constants_parse("VkPresentModeKHR", "u32")

with open(constants_format, 'w') as f:
    constants_parse("VkFormat", "u32")

with open(constants_front_face, 'w') as f:
    constants_parse("VkFrontFace", "u32")

with open(constants_image_layout, 'w') as f:
    constants_parse("VkImageLayout", "u32")

with open(constants_image_tiling, 'w') as f:
    constants_parse("VkImageTiling", "u32")

with open(constants_image_type, 'w') as f:
    constants_parse("VkImageType", "u32")

with open(constants_image_view_type, 'w') as f:
    constants_parse("VkImageViewType", "u32")

with open(constants_index_type, 'w') as f:
    constants_parse("VkIndexType", "u32")

with open(constants_pipeline_bind_point, 'w') as f:
    constants_parse("VkPipelineBindPoint", "u32")

with open(constants_polygon_mode, 'w') as f:
    constants_parse("VkPolygonMode", "u32")

with open(constants_primitive_topology, 'w') as f:
    constants_parse("VkPrimitiveTopology", "u32")

with open(constants_result, 'w') as f:
    constants_parse("VkResult", "i32")

with open(constants_sharing_mode, 'w') as f:
    constants_parse("VkSharingMode", "u32")

with open(constants_structure_type, 'w') as f:
    constants_parse("VkStructureType", "u32")