from xml.etree.ElementTree import Element
from pathlib import Path
from vk_utils import camel_to_snake, c_type_mapping
from vk_flags import flags_names

buffer = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/types/buffer.rs")
command_buffer = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/types/command_buffer.rs")
device = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/types/device.rs")
image = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/types/image.rs")
instance = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/types/instance.rs")
memory = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/types/memory.rs")
physical_device = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/types/physical_device.rs")
pipeline = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/types/pipeline.rs")
queue = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/types/queue.rs")
render_pass = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/types/render_pass.rs")
shader = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/types/shader.rs")
surface = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/types/surface.rs")
swapchain = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/types/swapchain.rs")
sync = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/types/sync.rs")
types = [
    {
        "path": buffer,
        "handles": [
            "VkBuffer",
        ],
        "structs": [
            "VkBufferCreateInfo"
        ],
        "deps": [
            "use crate::c::vulkan::{constants::{enums::{sharing_mode::VkSharingMode, structure_type::VkStructureType}, flags::{buffer_create::VkBufferCreateFlagBits, buffer_usage::VkBufferUsageFlagBits}}, types::base::VkDeviceSize};"
        ]
    },
    {
        "path": command_buffer,
        "handles": [
            "VkCommandPool",
            "VkCommandBuffer"
        ],
        "structs": [
            "VkCommandPoolCreateInfo",
            "VkCommandBufferAllocateInfo",
            "VkCommandBufferBeginInfo",
            "VkCommandBufferInheritanceInfo",
            "VkRenderPassBeginInfo",
            "VkClearValue",
            "VkClearColorValue",
            "VkClearDepthStencilValue",
            "VkDebugUtilsLabelEXT"
        ],
        "deps": [
            "use std::ffi::c_char;",
            "use crate::c::vulkan::{constants::{enums::{command_buffer_level::VkCommandBufferLevel, structure_type::VkStructureType}, flags::{command_buffer_usage::VkCommandBufferUsageFlagBits, command_pool_create::VkCommandPoolCreateFlagBits, query_control::VkQueryControlFlagBits, query_pipeline_statistic::VkQueryPipelineStatisticFlagBits}}, types::{base::VkBool32, pipeline::VkRect2D, render_pass::{VkFramebuffer, VkRenderPass}}};"
        ]
    },
    {
        "path": device,
        "handles": [
            "VkDevice",
        ],
        "structs": [
            "VkDeviceCreateInfo",
            "VkDeviceQueueCreateInfo",
            "VkFaultData"
        ],
        "deps": [
            "use std::ffi::c_char;",
            "use crate::c::vulkan::{constants::{enums::{fault_level::VkFaultLevel, fault_type::VkFaultType, structure_type::VkStructureType}, flags::device_queue_create::VkDeviceQueueCreateFlagBits}, types::{base::VkFlags, physical_device::VkPhysicalDeviceFeatures}};"
        ]
    },
    {
        "path": image,
        "handles": [
            "VkImage",
            "VkImageView"
        ],
        "structs": [
            "VkImageCreateInfo",
            "VkImageViewCreateInfo",
            "VkComponentMapping",
            "VkImageSubresourceRange",
            "VkExtent2D",
            "VkExtent3D"
        ],
        "deps": [
            "use crate::c::vulkan::constants::{enums::{component_swizzle::VkComponentSwizzle, format::VkFormat, image_layout::VkImageLayout, image_tiling::VkImageTiling, image_type::VkImageType, image_view_type::VkImageViewType, sharing_mode::VkSharingMode, structure_type::VkStructureType}, flags::{image_aspect::VkImageAspectFlagBits, image_create::VkImageCreateFlagBits, image_usage::VkImageUsageFlagBits, image_view_create::VkImageViewCreateFlagBits, sample_count::VkSampleCountFlagBits}};"
        ]
    },
    {
        "path": instance,
        "handles": [
            "VkInstance"
        ],
        "structs": [
            "VkInstanceCreateInfo",
            "VkApplicationInfo",
            "VkAllocationCallbacks",
            "VkDebugUtilsMessengerCallbackDataEXT",
            "VkDebugUtilsObjectNameInfoEXT"
        ],
        "deps": [
            "use std::ffi::c_char;",
            "use crate::c::vulkan::{constants::{enums::{object_type::VkObjectType, structure_type::VkStructureType}, flags::instance_create::VkInstanceCreateFlagBits}, functions::func_pointers::{PfnVkAllocationFunction, PfnVkFreeFunction, PfnVkInternalAllocationNotification, PfnVkInternalFreeNotification, PfnVkReallocationFunction}, types::{base::VkFlags, command_buffer::VkDebugUtilsLabelEXT}};"
        ]
    },
    {
        "path": memory,
        "handles": [
            "VkDeviceMemory"
        ],
        "structs": [
            "VkMemoryRequirements",
            "VkMemoryAllocateInfo",
            "VkMemoryType",
            "VkMemoryHeap",
            "VkDeviceMemoryReportCallbackDataEXT"
        ],
        "deps": [
            "use crate::c::vulkan::{constants::{enums::{device_memory_report_event_type_ext::VkDeviceMemoryReportEventTypeEXT, object_type::VkObjectType, structure_type::VkStructureType}, flags::{memory_heap::VkMemoryHeapFlagBits, memory_property::VkMemoryPropertyFlagBits}}, types::base::{VkDeviceSize, VkFlags}};"
        ]
    },
    {
        "path": physical_device,
        "handles": [
            "VkPhysicalDevice"
        ],
        "structs": [
            "VkPhysicalDeviceProperties",
            "VkPhysicalDeviceSparseProperties",
            "VkPhysicalDeviceFeatures",
            "VkPhysicalDeviceMemoryProperties",
            "VkPhysicalDeviceLimits",
            "VkQueueFamilyProperties"
        ],
        "deps": [
            "use std::ffi::c_char;",
            "use crate::c::vulkan::{constants::{api_constants::{VK_MAX_MEMORY_HEAPS, VK_MAX_MEMORY_TYPES, VK_MAX_PHYSICAL_DEVICE_NAME_SIZE, VK_UUID_SIZE}, enums::physical_device_type::VkPhysicalDeviceType, flags::{queue::VkQueueFlagBits, sample_count::VkSampleCountFlagBits}}, types::{base::{VkBool32, VkDeviceSize}, image::VkExtent3D, memory::{VkMemoryHeap, VkMemoryType}}};"
        ]
    },
    {
        "path": pipeline,
        "handles": [
            "VkPipeline",
            "VkPipelineCache",
            "VkPipelineLayout",
            "VkDescriptorSetLayout"
        ],
        "structs": [
            "VkPipelineLayoutCreateInfo",
            "VkPipelineShaderStageCreateInfo",
            "VkPipelineVertexInputStateCreateInfo",
            "VkPipelineInputAssemblyStateCreateInfo",
            "VkPipelineViewportStateCreateInfo",
            "VkPipelineRasterizationStateCreateInfo",
            "VkPipelineTessellationStateCreateInfo",
            "VkPipelineDepthStencilStateCreateInfo",
            "VkPipelineMultisampleStateCreateInfo",
            "VkPipelineColorBlendStateCreateInfo",
            "VkPipelineColorBlendAttachmentState",
            "VkPipelineDynamicStateCreateInfo",
            "VkGraphicsPipelineCreateInfo",
            "VkViewport",
            "VkRect2D",
            "VkOffset2D",
            "VkVertexInputBindingDescription",
            "VkVertexInputAttributeDescription",
            "VkPushConstantRange",
            "VkSpecializationInfo",
            "VkSpecializationMapEntry",
            "VkStencilOpState"
        ],
        "deps": [
            "use std::ffi::c_char;",
            "use crate::c::vulkan::{constants::{enums::{blend_factor::VkBlendFactor, blend_op::VkBlendOp, compare_op::VkCompareOp, dynamic_state::VkDynamicState, format::VkFormat, front_face::VkFrontFace, logic_op::VkLogicOp, polygon_mode::VkPolygonMode, primitive_topology::VkPrimitiveTopology, stencil_op::VkStencilOp, structure_type::VkStructureType, vertex_input_rate::VkVertexInputRate}, flags::{color_component::VkColorComponentFlagBits, cull_mode::VkCullModeFlagBits, pipeline_color_blend_state_create::VkPipelineColorBlendStateCreateFlagBits, pipeline_create::VkPipelineCreateFlagBits, pipeline_depth_stencil_state_create::VkPipelineDepthStencilStateCreateFlagBits, pipeline_layout_create::VkPipelineLayoutCreateFlagBits, pipeline_shader_stage_create::VkPipelineShaderStageCreateFlagBits, sample_count::VkSampleCountFlagBits, shader_stage::VkShaderStageFlagBits}}, types::{base::{VkBool32, VkFlags, VkSampleMask}, image::VkExtent2D, render_pass::VkRenderPass, shader::VkShaderModule}};"
        ]
    },
    {
        "path": queue,
        "handles": [
            "VkQueue"
        ],
        "structs": [
            "VkSubmitInfo",
            "VkPresentInfoKHR"
        ],
        "deps": [
            "use crate::c::vulkan::{constants::{enums::{result::VkResult, structure_type::VkStructureType}, flags::pipeline_stage::VkPipelineStageFlagBits}, types::{command_buffer::VkCommandBuffer, swapchain::VkSwapchainKHR, sync::VkSemaphore}};"
        ]
    },
    {
        "path": render_pass,
        "handles": [
            "VkRenderPass",
            "VkFramebuffer"
        ],
        "structs": [
            "VkRenderPassCreateInfo",
            "VkFramebufferCreateInfo",
            "VkAttachmentDescription",
            "VkAttachmentReference",
            "VkSubpassDescription",
            "VkSubpassDependency"
        ],
        "deps": [
            "use crate::c::vulkan::{constants::{enums::{attachment_load_op::VkAttachmentLoadOp, attachment_store_op::VkAttachmentStoreOp, format::VkFormat, image_layout::VkImageLayout, pipeline_bind_point::VkPipelineBindPoint, structure_type::VkStructureType}, flags::{access::VkAccessFlagBits, attachment_description::VkAttachmentDescriptionFlagBits, dependency::VkDependencyFlagBits, framebuffer_create::VkFramebufferCreateFlagBits, pipeline_stage::VkPipelineStageFlagBits, render_pass_create::VkRenderPassCreateFlagBits, sample_count::VkSampleCountFlagBits, subpass_description::VkSubpassDescriptionFlagBits}}, types::image::VkImageView};"
        ]
    },
    {
        "path": shader,
        "handles": [
            "VkShaderModule"
        ],
        "structs": [
            "VkShaderModuleCreateInfo"
        ],
        "deps": [
            "use crate::c::vulkan::{constants::enums::structure_type::VkStructureType, types::base::VkFlags};"
        ]
    },
    {
        "path": surface,
        "handles": [
            "VkSurfaceKHR"
        ],
        "structs": [
            "VkSurfaceCapabilitiesKHR",
            "VkSurfaceFormatKHR",
            "VkWaylandSurfaceCreateInfoKHR"
        ],
        "deps": [
            "use crate::c::vulkan::{constants::{enums::{format::VkFormat, khr::color_space::VkColorSpaceKHR, structure_type::VkStructureType}, flags::{image_usage::VkImageUsageFlagBits, khr::{composite_alpha::VkCompositeAlphaFlagBitsKHR, surface_transform::VkSurfaceTransformFlagBitsKHR}}}, types::{base::VkFlags, image::VkExtent2D}};"
        ]
    },
    {
        "path": swapchain,
        "handles": [
            "VkSwapchainKHR"
        ],
        "structs": [
            "VkSwapchainCreateInfoKHR"
        ],
        "deps": [
            "use crate::c::vulkan::{constants::{enums::{format::VkFormat, khr::{color_space::VkColorSpaceKHR, present_mode::VkPresentModeKHR}, sharing_mode::VkSharingMode, structure_type::VkStructureType}, flags::{image_usage::VkImageUsageFlagBits, khr::{composite_alpha::VkCompositeAlphaFlagBitsKHR, surface_transform::VkSurfaceTransformFlagBitsKHR, swapchain_create::VkSwapchainCreateFlagBitsKHR}}}, types::{base::VkBool32, image::VkExtent2D, surface::VkSurfaceKHR}};"
        ]
    },
    {
        "path": sync,
        "handles": [
            "VkFence",
            "VkSemaphore"
        ],
        "structs": [
            "VkFenceCreateInfo",
            "VkSemaphoreCreateInfo"
        ],
        "deps": [
            "use crate::c::vulkan::{constants::{enums::structure_type::VkStructureType, flags::fence_create::VkFenceCreateFlagBits}, types::base::VkFlags};"
        ]
    }
]

def types_parse(root: Element[str]):
    for vk_type in types:
        with open(vk_type["path"], 'w') as f:
            print("use std::ffi::c_void;", file=f)
            print("", file=f)
            for dep in vk_type["deps"]:
                print(f"{dep}", file=f)
                print("", file=f)
            for handle in vk_type["handles"]:
                print("", file=f)
                print("#[repr(transparent)]", file=f)
                print("#[derive(Clone, Copy, PartialEq, Eq)]", file=f)
                print(f"pub struct {handle}(pub *mut c_void);", file=f)
                print("", file=f)
            for struct in vk_type["structs"]:
                for type in root.iter('type'):
                    attrs = type.attrib
                    if attrs.get('name') == struct:
                        if attrs.get('category') == "struct" and struct == "VkWaylandSurfaceCreateInfoKHR":
                            print("#[cfg(target_os = \"linux\")]", file=f)
                            print("pub use wayland_surface_create_info_khr::*;", file=f)
                            print("", file=f)
                            print("#[cfg(target_os = \"linux\")]", file=f)
                            print("mod wayland_surface_create_info_khr {", file=f)
                            print("    use crate::c::linux::wayland::{wl_display, wl_surface};", file=f)
                            print("    use super::*;", file=f)
                            print("", file=f)
                            print("    #[repr(C)]", file=f)
                            print("    pub struct VkWaylandSurfaceCreateInfoKHR {", file=f)
                        elif attrs.get('category') == "struct":
                            print("#[repr(C)]", file=f)
                            if struct == "VkClearDepthStencilValue":
                                print("#[derive(Clone, Copy)]", file=f)
                            print(f"pub struct {struct} {{", file=f)
                        elif attrs.get('category') == "union":
                            print("#[repr(C)]", file=f)
                            if struct == "VkClearValue" or struct == "VkClearColorValue":
                                print("#[derive(Clone, Copy)]", file=f)
                            print(f"pub union {struct} {{", file=f)
                        if attrs.get('category') == "struct" or attrs.get('category') == "union":
                            field_name = ""
                            field_prefix = ""
                            field_type = ""
                            field_suffix = ""
                            constant_val = ""
                            stripped_api = False
                            hardcoded_flags = False
                            array_syntax = False
                            already_set_constant = False
                            for elem in type.iter():
                                if elem.tag == "member" and elem.attrib.get('api') == "vulkansc":
                                    stripped_api = True
                                if elem.tag == "member" and not already_set_constant:
                                    enum = elem.find('enum')
                                    if enum is not None:
                                        already_set_constant = True
                                        array_syntax = True
                                        constant_val = enum.text.strip()
                                if elem.tag == "member" and elem.text is not None:
                                    field_prefix = elem.text.strip()
                                if elem.tag == "name":
                                    field_name = elem.text.strip()
                                    field_name = camel_to_snake(field_name)
                                    if field_name == "type":
                                        field_name = "r#type"
                                    if elem.tail is not None and not already_set_constant:
                                        array_syntax = True
                                        already_set_constant = True
                                        constant_val = elem.tail.strip()
                                        constant_val = constant_val[1:-1]
                                if elem.tag == "type":
                                    field_type = elem.text.strip()
                                    field_type = c_type_mapping.get(field_type, elem.text)
                                    if field_type.startswith("PFN"):
                                        field_type = field_type[0:1] + field_type[1:3].lower() + field_type[4:5].upper() + field_type[5:]
                                    if elem.tail is not None:
                                        field_suffix = elem.tail.strip()
                                    if field_type.endswith("EXT"):
                                        temp_type = field_type.removesuffix("EXT").removesuffix("s") + "Bits" + "EXT"
                                    elif field_type.endswith("KHR"):
                                        temp_type = field_type.removesuffix("KHR").removesuffix("s") + "Bits" + "KHR"
                                    else:
                                        temp_type = field_type.removesuffix("s") + "Bits"
                                    if "Flags" in field_type and temp_type in flags_names:
                                        if field_type.endswith("EXT"):
                                            field_type = field_type.removesuffix("EXT").removesuffix("s") + "Bits" + "EXT"
                                        elif field_type.endswith("KHR"):
                                            field_type = field_type.removesuffix("KHR").removesuffix("s") + "Bits" + "KHR"
                                        else:
                                            field_type = field_type.removesuffix("s") + "Bits"
                                    elif "Flags" in field_type:
                                        hardcoded_flags = True
                                        field_type = "VkFlags"
                                if field_name != "" and field_type != "":
                                    if stripped_api:
                                        stripped_api = False
                                        field_name = ""
                                        field_type = ""
                                        field_prefix = ""
                                        field_suffix = ""
                                        array_syntax = False
                                        already_set_constant = False
                                        continue
                                    if hardcoded_flags and struct == "VkWaylandSurfaceCreateInfoKHR":
                                        hardcoded_flags = False
                                        print("        // Hardcoded VkFlags here, make sure that the real flags type doesn't exist", file=f)
                                    elif hardcoded_flags:
                                        hardcoded_flags = False
                                        print("    // Hardcoded VkFlags here, make sure that the real flags type doesn't exist", file=f)
                                    if field_prefix == "struct" and struct == "VkWaylandSurfaceCreateInfoKHR":
                                        print(f"        pub {field_name}: {field_suffix}mut {field_type},", file=f)
                                    elif field_prefix == "struct":
                                        print(f"    pub {field_name}: {field_suffix}mut {field_type},", file=f)
                                    elif field_prefix != "" and struct == "VkWaylandSurfaceCreateInfoKHR":
                                        print(f"        pub {field_name}: {field_suffix}{field_prefix} {field_type},", file=f)
                                    elif field_prefix != "":
                                        print(f"    pub {field_name}: {field_suffix}{field_prefix} {field_type},", file=f)
                                    elif field_suffix != "" and struct == "VkWaylandSurfaceCreateInfoKHR":
                                        print(f"        pub {field_name}: {field_suffix}mut {field_type},", file=f)
                                    elif field_suffix != "":
                                        print(f"    pub {field_name}: {field_suffix}mut {field_type},", file=f)
                                    elif struct == "VkWaylandSurfaceCreateInfoKHR":
                                        print(f"        pub {field_name}: {field_type},", file=f)
                                    elif array_syntax and constant_val != "":
                                        print(f"    pub {field_name}: [{field_type}; {constant_val}],", file=f)
                                    else:
                                        print(f"    pub {field_name}: {field_type},", file=f)
                                    field_name = ""
                                    field_type = ""
                                    field_prefix = ""
                                    field_suffix = ""
                                    array_syntax = False
                                    already_set_constant = False
                                    
                        if attrs.get('category') == "struct" and struct == "VkWaylandSurfaceCreateInfoKHR":
                            print("    }", file=f)
                            print("}", file=f)
                            print("", file=f)
                        elif attrs.get('category') == "struct" or attrs.get('category') == "union":
                            print("}", file=f)
                            print("", file=f)