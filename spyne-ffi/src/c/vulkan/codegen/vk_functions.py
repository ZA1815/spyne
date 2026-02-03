from xml.etree.ElementTree import Element
from pathlib import Path
from vk_utils import camel_to_snake, c_type_mapping

buffer = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/functions/buffer.rs")
command_buffer = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/functions/command_buffer.rs")
device = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/functions/device.rs")
image = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/functions/image.rs")
instance = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/functions/instance.rs")
memory = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/functions/memory.rs")
physical_device = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/functions/physical_device.rs")
pipeline = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/functions/pipeline.rs")
queue = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/functions/queue.rs")
render_pass = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/functions/render_pass.rs")
shader = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/functions/shader.rs")
surface = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/functions/surface.rs")
swapchain = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/functions/swapchain.rs")
sync = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/functions/sync.rs")
functions = [
    {
        "path": buffer,
        "funcs": [
            "vkCreateBuffer",
            "vkDestroyBuffer",
            "vkGetBufferMemoryRequirements",
            "vkBindBufferMemory"
        ],
        "deps": "use crate::c::vulkan::{constants::enums::result::VkResult, types::{base::VkDeviceSize, buffer::{VkBuffer, VkBufferCreateInfo}, device::VkDevice, instance::VkAllocationCallbacks, memory::{VkDeviceMemory, VkMemoryRequirements}}};"
    },
    {
        "path": command_buffer,
        "funcs": [
            "vkCreateCommandPool",
            "vkDestroyCommandPool",
            "vkAllocateCommandBuffers",
            "vkFreeCommandBuffers",
            "vkBeginCommandBuffer",
            "vkEndCommandBuffer",
            "vkResetCommandBuffer",
            "vkCmdBeginRenderPass",
            "vkCmdEndRenderPass",
            "vkCmdBindPipeline",
            "vkCmdBindVertexBuffers",
            "vkCmdSetViewport",
            "vkCmdSetScissor",
            "vkCmdDraw"
        ]
    },
    {
        "path": device,
        "funcs": [
            "vkGetDeviceQueue",
            "vkDeviceWaitIdle",
            "vkDestroyDevice"
        ]
    },
    {
        "path": image,
        "funcs": [
            "vkCreateImage",
            "vkDestroyImage",
            "vkGetImageMemoryRequirements",
            "vkBindImageMemory",
            "vkCreateImageView",
            "vkDestroyImageView"
        ]
    },
    {
        "path": instance,
        "funcs": [
            "vkEnumeratePhysicalDevices",
            "vkCreateDevice"
        ]
    },
    {
        "path": memory,
        "funcs": [
            "vkAllocateMemory",
            "vkFreeMemory",
            "vkMapMemory",
            "vkUnmapMemory"
        ]
    },
    {
        "path": physical_device,
        "funcs": [
            "vkGetPhysicalDeviceProperties",
            "vkGetPhysicalDeviceFeatures",
            "vkGetPhysicalDeviceMemoryProperties",
            "vkGetPhysicalDeviceQueueFamilyProperties"
        ]
    },
    {
        "path": pipeline,
        "funcs": [
            "vkCreatePipelineLayout",
            "vkDestroyPipelineLayout",
            "vkCreateGraphicsPipelines",
            "vkDestroyPipeline"
        ]
    },
    {
        "path": queue,
        "funcs": [
            "vkQueueSubmit",
            "vkQueueWaitIdle",
            "vkQueuePresentKHR"
        ]
    },
    {
        "path": render_pass,
        "funcs": [
            "vkCreateRenderPass",
            "vkDestroyRenderPass",
            "vkCreateFramebuffer",
            "vkDestroyFramebuffer"
        ]
    },
    {
        "path": shader,
        "funcs": [
            "vkCreateShaderModule",
            "vkDestroyShaderModule"
        ]
    },
    {
        "path": surface,
        "funcs": [
            "vkDestroySurfaceKHR",
            "vkGetPhysicalDeviceSurfaceSupportKHR",
            "vkGetPhysicalDeviceSurfaceCapabilitiesKHR",
            "vkGetPhysicalDeviceSurfaceFormatsKHR",
            "vkGetPhysicalDeviceSurfacePresentModesKHR",
            "vkCreateWaylandSurfaceKHR"
        ]
    },
    {
        "path": swapchain,
        "funcs": [
            "vkCreateSwapchainKHR",
            "vkDestroySwapchainKHR",
            "vkGetSwapchainImagesKHR",
            "vkAcquireNextImageKHR"
        ]
    },
    {
        "path": sync,
        "funcs": [
            "vkCreateFence",
            "vkDestroyFence",
            "vkWaitForFences",
            "vkResetFences",
            "vkCreateSemaphore",
            "vkDestroySemaphore"
        ]
    }
]

def funcs_parse_first(root: Element[str]) -> dict[str, list[str]]:
    funcs_dict = {}
    for command in root.findall(".//command"):
        params_list = []
        if command.find("proto/name") is not None:
            func_name = command.find("proto/name").text
        if command.find("proto/type") is not None:
            return_type = command.find("proto/type").text
        
        for param in command.findall(".//param"):
            type_prefix = ""
            type = ""
            type_suffix = ""
            if param.find("type") is not None:
                if param.text is not None:
                    type_prefix = param.text.strip()
                type = param.find("type").text.strip()
                type = c_type_mapping.get(type, param.find("type").text.strip())
                type_suffix = param.find("type").tail.strip()
                if type_prefix != "" and type != "" and type_suffix != "":
                    full_type = f"{type_suffix}{type_prefix} {type}"
                elif type_prefix != "" and type != "":
                    full_type = f"{type_prefix} {type}"
                elif type != "" and type_suffix == "**":
                    full_type = f"*mut *mut {type}"
                elif type != "" and type_suffix != "":
                    full_type = f"{type_suffix}mut {type}"
                else:
                    full_type = f"{type}"
                    
                type_prefix = ""
                type = ""
                type_suffix = ""
                    
            if param.find("name") is not None:
                camel_name = param.find("name").text.strip()
                snake_name = camel_to_snake(camel_name)
            params_list.append(f"    {snake_name}: {full_type},")
            
        if len(params_list) != 0:
            str_list = []
            formatted_func_name = func_name[:1].upper() + func_name[1:]
            str_list.append(f"pub type {formatted_func_name} = unsafe extern \"system\" fn(")
            for p in params_list:
                str_list.append(p)
            if return_type == "void":
                str_list.append(");")
            else:
                str_list.append(f") -> {return_type};")
            str_list.append("")
            funcs_dict[f"{func_name}"] = "\n".join(str_list)
    
    return funcs_dict

def funcs_parse_second(funcs_dict: dict[str, list[str]]):
    for group in functions:
        with open(group["path"], 'w') as f:
            for func in group["funcs"]:
                print(funcs_dict[f"{func}"], file=f)