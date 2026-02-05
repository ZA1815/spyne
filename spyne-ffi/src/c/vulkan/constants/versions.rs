pub const fn make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | patch
}

pub const VKSC_API_VARIANT: u32 = 1;
pub const VK_API_VERSION: u32 = make_api_version(0, 1, 0, 0);
pub const VK_API_VERSION_1_0: u32 = make_api_version(0, 1, 0, 0);
pub const VK_API_VERSION_1_1: u32 = make_api_version(0, 1, 1, 0);
pub const VK_API_VERSION_1_2: u32 = make_api_version(0, 1, 2, 0);
pub const VK_API_VERSION_1_3: u32 = make_api_version(0, 1, 3, 0);
pub const VK_API_VERSION_1_4: u32 = make_api_version(0, 1, 4, 0);
pub const VKSC_API_VERSION_1_0: u32 = make_api_version(VKSC_API_VARIANT, 1, 0, 0);