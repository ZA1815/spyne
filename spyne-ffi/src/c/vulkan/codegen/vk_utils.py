import re

def camel_to_snake(name: str) -> str:
    if name == "maxImageDimension1D":
        return "max_image_dimension_1d"
    elif name == "maxImageDimension2D":
        return "max_image_dimension_2d"
    elif name == "maxImageDimension3D":
        return "max_image_dimension_3d"
    return re.sub(r"[A-Z]", lambda c: "_" + c.group(0).lower(), name)

c_type_mapping = {
    "uint8_t": "u8",
    "uint32_t": "u32",
    "uint64_t": "u64",
    "size_t": "usize",
    "int32_t": "i32",
    "ssize_t": "isize",
    "void": "c_void",
    "float": "f32",
    "char": "c_char"
}