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
            "VkDebugUtilsLabelEXT"
        ],
        "deps": [
            "use crate::c::vulkan::constants::{enums::structure_type::VkStructureType, flags::command_pool_create::VkCommandPoolCreateFlagBits};"
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
            "use crate::c::vulkan::{constants::enums::{fault_level::VkFaultLevel, fault_type::VkFaultType, structure_type::VkStructureType}, types::{base::VkFlags, physical_device::VkPhysicalDeviceFeatures}};"
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
            "use crate::c::vulkan::{constants::{enums::{device_memory_report_event_type_ext::VkDeviceMemoryReportEventTypeEXT, object_type::VkObjectType, structure_type::VkStructureType}, flags::memory_property::VkMemoryPropertyFlagBits}, types::base::{VkDeviceSize, VkFlags}};"
        ]
    },
    {
        "path": physical_device,
        "handles": [
            "VkPhysicalDevice"
        ],
        "structs": [
            "VkPhysicalDeviceProperties",
            "VkPhysicalDeviceFeatures",
            "VkPhysicalDeviceMemoryProperties",
            "VkQueueFamilyProperties"
        ],
        "deps": [
            ""
        ]
    },
    {
        "path": pipeline,
        "handles": [
            "VkPipeline",
            "VkPipelineCache",
            "VkPipelineLayout"
        ],
        "structs": [
            "VkPipelineLayoutCreateInfo",
            "VkPipelineShaderStageCreateInfo",
            "VkPipelineVertexInputStateCreateInfo",
            "VkPipelineInputAssemblyStateCreateInfo",
            "VkPipelineViewportStateCreateInfo",
            "VkPipelineRasterizationStateCreateInfo",
            "VkPipelineMultisampleStateCreateInfo",
            "VkPipelineColorBlendStateCreateInfo",
            "VkPipelineColorBlendAttachmentState",
            "VkPipelineDynamicStateCreateInfo",
            "VkGraphicsPipelineCreateInfo",
            "VkViewport",
            "VkRect2D",
            "VkOffset2D",
            "VkVertexInputBindingDescription",
            "VkVertexInputAttributeDescription"
        ],
        "deps": [
            ""
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
            ""
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
            ""
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
            ""
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
            ""
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
            ""
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
            ""
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
                        if attrs.get('category') == "struct":
                            print("#[repr(C)]", file=f)
                            print(f"pub struct {struct} {{", file=f)
                        elif attrs.get('category') == "union":
                            print("#[repr(C)]", file=f)
                            print(f"pub union {struct} {{", file=f)
                        if attrs.get('category') == "struct" or attrs.get('category') == "union":
                            field_name = ""
                            field_prefix = ""
                            field_type = ""
                            field_suffix = ""
                            for elem in type.iter():
                                hardcoded_flags = False
                                if elem.tag == "member" and elem.text is not None:
                                    field_prefix = elem.text.strip()
                                if elem.tag == "name":
                                    field_name = elem.text.strip()
                                    field_name = camel_to_snake(field_name)
                                    if field_name == "type":
                                        field_name = "r#type"
                                if elem.tag == "type":
                                    field_type = elem.text.strip()
                                    field_type = c_type_mapping.get(field_type, elem.text)
                                    if field_type.startswith("PFN"):
                                        field_type = field_type[0:1] + field_type[1:3].lower() + field_type[4:5].upper() + field_type[5:]
                                    if elem.tail is not None:
                                        field_suffix = elem.tail.strip()
                                    if field_type.endswith("EXT"):
                                        temp_type = field_type.removesuffix("EXT").removesuffix("s") + "Bits" + "EXT"
                                    else:
                                        temp_type = field_type.removesuffix("s") + "Bits"
                                    if "Flags" in field_type and temp_type in flags_names:
                                        if field_type.endswith("EXT"):
                                            field_type = field_type.removesuffix("EXT").removesuffix("s") + "Bits" + "EXT"
                                        else:
                                            field_type = field_type.removesuffix("s")
                                            field_type += "Bits"
                                    elif "Flags" in field_type:
                                        hardcoded_flags = True
                                        field_type = "VkFlags"
                                if field_name != "" and field_type != "":
                                    if hardcoded_flags:
                                        print("    // Hardcoded VkFlags here, make sure that the real flags type doesn't exist", file=f)
                                    if field_prefix != "":
                                        print(f"    pub {field_name}: {field_suffix}{field_prefix} {field_type},", file=f)
                                    elif field_suffix != "":
                                        print(f"    pub {field_name}: {field_suffix}mut {field_type},", file=f)
                                    else:
                                        print(f"    pub {field_name}: {field_type},", file=f)
                                    field_name = ""
                                    field_type = ""
                                    field_prefix = ""
                                    field_suffix = ""
                                    
                            print("}", file=f)
                            print("", file=f)