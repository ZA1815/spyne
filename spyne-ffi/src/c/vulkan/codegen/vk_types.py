from xml.etree.ElementTree import Element
from pathlib import Path
from vk_utils import camel_to_snake, c_type_mapping

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
        "deps": ""
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
            "VkRenderPassBeginInfo",
            "VkClearValue",
            "VkClearColorValue"
        ],
        "deps": ""
    },
    {
        "path": device,
        "handles": [
            "VkDevice",
        ],
        "structs": [
            "VkDeviceCreateInfo",
            "VkDeviceQueueCreateInfo"
        ],
        "deps": ""
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
        "deps": ""
    },
    {
        "path": instance,
        "handles": [
            "VkInstance"
        ],
        "structs": [
            "VkAllocationCallbacks"
        ],
        "deps": ""
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
            "VkMemoryHeap"
        ],
        "deps": ""
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
        "deps": ""
    },
    {
        "path": pipeline,
        "handles": [
            "VkPipeline",
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
        "deps": ""
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
        "deps": ""
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
        "deps": ""
    },
    {
        "path": shader,
        "handles": [
            "VkShaderModule"
        ],
        "structs": [
            "VkShaderModuleCreateInfo"
        ],
        "deps": ""
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
        "deps": ""
    },
    {
        "path": swapchain,
        "handles": [
            "VkSwapchainKHR"
        ],
        "structs": [
            "VkSwapchainCreateInfoKHR"
        ],
        "deps": ""
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
        "deps": ""
    }
]

def types_parse(root: Element[str]):
    for type in types:
        with open(type["path"], 'w') as f:
            print("use std::ffi::c_void;", file=f)
            for handle in type["handles"]:
                print("", file=f)
                print("#[repr(transparent)]", file=f)
                print("#[derive(Clone, Copy, PartialEq, Eq)]", file=f)
                print(f"pub struct {handle}(pub *mut c_void);", file=f)
                print("", file=f)
            for struct in type["structs"]:
                for type in root.iter('type'):
                    attrs = type.attrib
                    if attrs.get('category') == "struct" and attrs.get('name') == struct:
                        print("#[repr(C)]", file=f)
                        print(f"pub struct {struct} {{", file=f)
                        field_name = ""
                        field_prefix = ""
                        field_type = ""
                        field_suffix = ""
                        for elem in type.iter():
                            if elem.tag == "member" and elem.text is not None:
                                field_prefix = elem.text.strip()
                            if elem.tag == "name":
                                field_name = elem.text.strip()
                                field_name = camel_to_snake(field_name)
                            if elem.tag == "type":
                                field_type = elem.text.strip()
                                field_type = c_type_mapping.get(field_type, elem.text)
                                if elem.tail is not None:
                                    field_suffix = elem.tail.strip()
                            if field_name != "" and field_type != "":
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
                            