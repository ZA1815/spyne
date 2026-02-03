from xml.etree.ElementTree import Element
from vk_enums import enums_paths, enums_names, enums_types
from vk_flags import flags_paths, flags_names, flags_types

const_paths = enums_paths + flags_paths
const_names = enums_names + flags_names
const_types = enums_types + flags_types

def consts_parse(root: Element[str]):
    for (path, const_name, type) in zip(const_paths, const_names, const_types):
        with open(path, 'w') as f:
            print("#[repr(transparent)]", file=f)
            print("#[derive(Debug, Clone, Copy, PartialEq, Eq)]", file=f)
            print(f"pub struct {const_name}(pub {type});", file=f)
            print("", file=f)
            for enum in root.iter('enums'):
                if enum.attrib.get('name') == const_name:
                    for enum_var in enum:
                        name = enum_var.attrib.get('name')
                        value = enum_var.attrib.get('value')
                        bitpos = enum_var.attrib.get('bitpos')
                        if name is not None and bitpos is not None:
                            print(f"pub const {name}: {const_name} = {const_name}(1 << {bitpos});", file=f)
                        if name is not None and value is not None:
                            print(f"pub const {name}: {const_name} = {const_name}({value});", file=f)