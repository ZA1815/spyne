from xml.etree.ElementTree import Element
from pathlib import Path
from vk_utils import camel_to_snake, c_type_mapping
from vk_flags import flags_names

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
        "deps": [
            "use crate::c::vulkan::{constants::enums::result::VkResult, types::{base::VkDeviceSize, buffer::{VkBuffer, VkBufferCreateInfo}, device::VkDevice, instance::VkAllocationCallbacks, memory::{VkDeviceMemory, VkMemoryRequirements}}};"
        ]
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
        ],
        "deps": [
            "use crate::c::vulkan::{constants::{enums::{pipeline_bind_point::VkPipelineBindPoint, result::VkResult, subpass_contents::VkSubpassContents}, flags::command_buffer_reset::VkCommandBufferResetFlagBits}, types::{base::VkDeviceSize, buffer::VkBuffer, command_buffer::{VkCommandBuffer, VkCommandBufferAllocateInfo, VkCommandBufferBeginInfo, VkCommandPool, VkCommandPoolCreateInfo, VkRenderPassBeginInfo}, device::VkDevice, instance::VkAllocationCallbacks, pipeline::{VkPipeline, VkRect2D, VkViewport}}};"
        ]
    },
    {
        "path": device,
        "funcs": [
            "vkGetDeviceQueue",
            "vkDeviceWaitIdle",
            "vkDestroyDevice"
        ],
        "deps": [
            "use crate::c::vulkan::{constants::enums::result::VkResult, types::{device::VkDevice, instance::VkAllocationCallbacks, queue::VkQueue}};"
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
        ],
        "deps": [
            "use crate::c::vulkan::{constants::enums::result::VkResult, types::{base::VkDeviceSize, device::VkDevice, image::{VkImage, VkImageCreateInfo, VkImageView, VkImageViewCreateInfo}, instance::VkAllocationCallbacks, memory::{VkDeviceMemory, VkMemoryRequirements}}};"
        ]
    },
    {
        "path": instance,
        "funcs": [
            "vkCreateInstance",
            "vkGetInstanceProcAddr",
            "vkEnumeratePhysicalDevices",
            "vkCreateDevice",
            "vkGetDeviceProcAddr"
        ],
        "deps": [
            "use std::ffi::c_char;",
            "use crate::c::vulkan::{constants::enums::result::VkResult, functions::func_pointers::PfnVkVoidFunction, types::{device::{VkDevice, VkDeviceCreateInfo}, instance::{VkAllocationCallbacks, VkInstance, VkInstanceCreateInfo}, physical_device::VkPhysicalDevice}};"
        ]
    },
    {
        "path": memory,
        "funcs": [
            "vkAllocateMemory",
            "vkFreeMemory",
            "vkMapMemory",
            "vkUnmapMemory"
        ],
        "deps": [
            "use std::ffi::c_void;",
            "use crate::c::vulkan::{constants::{enums::result::VkResult, flags::memory_map::VkMemoryMapFlagBits}, types::{base::VkDeviceSize, device::VkDevice, instance::VkAllocationCallbacks, memory::{VkDeviceMemory, VkMemoryAllocateInfo}}};"
        ]
    },
    {
        "path": physical_device,
        "funcs": [
            "vkGetPhysicalDeviceProperties",
            "vkGetPhysicalDeviceFeatures",
            "vkGetPhysicalDeviceMemoryProperties",
            "vkGetPhysicalDeviceQueueFamilyProperties"
        ],
        "deps": [
            "use crate::c::vulkan::types::physical_device::{VkPhysicalDevice, VkPhysicalDeviceFeatures, VkPhysicalDeviceMemoryProperties, VkPhysicalDeviceProperties, VkQueueFamilyProperties};"
        ]
    },
    {
        "path": pipeline,
        "funcs": [
            "vkCreatePipelineLayout",
            "vkDestroyPipelineLayout",
            "vkCreateGraphicsPipelines",
            "vkDestroyPipeline"
        ],
        "deps": [
            "use crate::c::vulkan::{constants::enums::result::VkResult, types::{device::VkDevice, instance::VkAllocationCallbacks, pipeline::{VkGraphicsPipelineCreateInfo, VkPipeline, VkPipelineCache, VkPipelineLayout, VkPipelineLayoutCreateInfo}}};"
        ]
    },
    {
        "path": queue,
        "funcs": [
            "vkQueueSubmit",
            "vkQueueWaitIdle",
            "vkQueuePresentKHR"
        ],
        "deps": [
            "use crate::c::vulkan::{constants::enums::result::VkResult, types::{queue::{VkPresentInfoKHR, VkQueue, VkSubmitInfo}, sync::VkFence}};"
        ]
    },
    {
        "path": render_pass,
        "funcs": [
            "vkCreateRenderPass",
            "vkDestroyRenderPass",
            "vkCreateFramebuffer",
            "vkDestroyFramebuffer"
        ],
        "deps": [
            "use crate::c::vulkan::{constants::enums::result::VkResult, types::{device::VkDevice, instance::VkAllocationCallbacks, render_pass::{VkFramebuffer, VkFramebufferCreateInfo, VkRenderPass, VkRenderPassCreateInfo}}};"
        ]
    },
    {
        "path": shader,
        "funcs": [
            "vkCreateShaderModule",
            "vkDestroyShaderModule"
        ],
        "deps": [
            "use crate::c::vulkan::{constants::enums::result::VkResult, types::{device::VkDevice, instance::VkAllocationCallbacks, shader::{VkShaderModule, VkShaderModuleCreateInfo}}};"
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
        ],
        "deps": [
            "use crate::c::vulkan::{constants::enums::{khr::present_mode::VkPresentModeKHR, result::VkResult}, types::{base::VkBool32, instance::{VkAllocationCallbacks, VkInstance}, physical_device::VkPhysicalDevice, surface::{VkSurfaceCapabilitiesKHR, VkSurfaceFormatKHR, VkSurfaceKHR}}};"
        ]
    },
    {
        "path": swapchain,
        "funcs": [
            "vkCreateSwapchainKHR",
            "vkDestroySwapchainKHR",
            "vkGetSwapchainImagesKHR",
            "vkAcquireNextImageKHR"
        ],
        "deps": [
            "use crate::c::vulkan::{constants::enums::result::VkResult, types::{device::VkDevice, image::VkImage, instance::VkAllocationCallbacks, swapchain::{VkSwapchainCreateInfoKHR, VkSwapchainKHR}, sync::{VkFence, VkSemaphore}}};"
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
        ],
        "deps": [
            "use crate::c::vulkan::{constants::enums::result::VkResult, types::{base::VkBool32, device::VkDevice, instance::VkAllocationCallbacks, sync::{VkFence, VkFenceCreateInfo, VkSemaphore, VkSemaphoreCreateInfo}}};"
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
            if command.find("proto/type").tail.strip() != "":
                return_type = c_type_mapping.get(return_type, command.find("proto/type").text)
                return_type = f"*mut {return_type}"
        
        for param in command.findall("param"):
            hardcoded_flags = False
            type_prefix = ""
            type = ""
            type_suffix = ""
            if param.find("type") is not None:
                if param.text is not None:
                    type_prefix = param.text.strip()
                type = param.find("type").text.strip()
                type = c_type_mapping.get(type, param.find("type").text.strip())
                if type.endswith("EXT"):
                    temp_type = type
                else:
                    temp_type = type.removesuffix("s") + "Bits"
                if "Flags" in type and temp_type in flags_names:
                    type = type.removesuffix("s")
                    type += "Bits"
                elif "Flags" in type:
                    hardcoded_flags = True
                    type = "VkFlags"
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
            
            if hardcoded_flags:
               params_list.append("    // Hardcoded VkFlags here, make sure that the real flags type doesn't exist")
            params_list.append(f"    {snake_name}: {full_type},")
            
        if len(params_list) != 0:
            str_list = []
            formatted_func_name = func_name[:1].upper() + func_name[1:]
            if formatted_func_name == "VkCreateWaylandSurfaceKHR":
                str_list.append("#[cfg(target_os = \"linux\")]")
                str_list.append("pub use khr_create_wayland_surface::*;")
                str_list.append("")
                str_list.append("#[cfg(target_os = \"linux\")]")
                str_list.append("mod khr_create_wayland_surface {")
                str_list.append("   use crate::c::vulkan::types::surface::VkWaylandSurfaceCreateInfoKHR;")
                str_list.append("   use super::*;")
                str_list.append("")
                str_list.append(f"    pub type {formatted_func_name} = unsafe extern \"system\" fn(")
                for p in params_list:
                    p = "   " + p
                    str_list.append(p)
                str_list.append(f"    ) -> {return_type};")
                str_list.append("}")
            else:
                str_list.append(f"pub type {formatted_func_name} = unsafe extern \"system\" fn(")
                for p in params_list:
                    str_list.append(p)
                if return_type == "void":
                    str_list.append(");")
                else:
                    if return_type == "PFN_vkVoidFunction":
                        return_type = "PfnVkVoidFunction"
                    str_list.append(f") -> {return_type};")
            str_list.append("")
            funcs_dict[f"{func_name}"] = "\n".join(str_list)
    
    return funcs_dict

def funcs_parse_second(funcs_dict: dict[str, list[str]]):
    for group in functions:
        with open(group["path"], 'w') as f:
            for dep in group["deps"]:
                print(f"{dep}", file=f)
                print("", file=f)
            for func in group["funcs"]:
                print(funcs_dict[f"{func}"], file=f)

func_pointers = Path("/Users/zaidahmed/Desktop/spyne/spyne-ffi/src/c/vulkan/functions/func_pointers.rs")

def func_pointers_parse(root: Element[str]):
    with open(func_pointers, 'w') as f:
        print("use std::ffi::{c_char, c_void};", file=f)
        print("", file=f)
        print("use crate::c::vulkan::{constants::{enums::{debug_report_object_type_ext::VkDebugReportObjectTypeEXT, internal_allocation_type::VkInternalAllocationType, system_allocation_scope::VkSystemAllocationScope}, flags::{debug_report_ext::VkDebugReportFlagBitsEXT, debug_utils_message_severity_ext::VkDebugUtilsMessageSeverityFlagBitsEXT, debug_utils_message_type_ext::VkDebugUtilsMessageTypeFlagBitsEXT}}, types::{base::VkBool32, device::VkFaultData, instance::{VkDebugUtilsMessengerCallbackDataEXT, VkInstance}, memory::VkDeviceMemoryReportCallbackDataEXT}};", file=f)
        print("", file=f)
        for fptr in root.iter('type'):
            if fptr.attrib.get('category') == "funcpointer":
                params_list = []
                if fptr.find("proto/name") is not None:
                    func_name = fptr.find("proto/name").text
                if fptr.find("proto/type") is not None:
                    return_type = fptr.find("proto/type").text
                    if fptr.find("proto/type").tail.strip() != "":
                        return_type = c_type_mapping.get(return_type, fptr.find("proto/type").text)
                        return_type = f"*mut {return_type}"
                
                for param in fptr.findall(".//param"):
                    hardcoded_flags = False
                    type_prefix = ""
                    type = ""
                    type_suffix = ""
                    if param.find("type") is not None:
                        if param.text is not None:
                            type_prefix = param.text.strip()
                        type = param.find("type").text.strip()
                        type = c_type_mapping.get(type, param.find("type").text.strip())
                        if type.endswith("EXT"):
                            temp_type = type.removesuffix("EXT").removesuffix("s") + "Bits" + "EXT"
                        else:
                            temp_type = type.removesuffix("s") + "Bits"
                        if "Flags" in type and temp_type in flags_names:
                            if type.endswith("EXT"):
                                type = type.removesuffix("EXT").removesuffix("s") + "Bits" + "EXT"
                            else:
                                type = type.removesuffix("s")
                                type += "Bits"
                        elif "Flags" in type:
                            hardcoded_flags = True
                            type = "VkFlags"
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
                    
                    if hardcoded_flags:
                        params_list.append("    // Hardcoded VkFlags here, make sure that the real flags type doesn't exist")
                    params_list.append(f"    {snake_name}: {full_type},")
                    
                formatted_func_name = func_name[0:1] + func_name[1:3].lower() + func_name[4:5].upper() + func_name[5:]
                print(f"pub type {formatted_func_name} = unsafe extern \"system\" fn(", file=f)
                for p in params_list:
                    print(p, file=f)
                if return_type == "void":
                    print(");", file=f)
                else:
                    if return_type == "PFN_vkVoidFunction":
                        return_type = "PfnVkVoidFunction"
                    print(f") -> {return_type};", file=f)
                print("", file=f)