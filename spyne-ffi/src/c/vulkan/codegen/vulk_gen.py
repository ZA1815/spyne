import xml.etree.ElementTree as et
from pathlib import Path
from vk_constants import consts_parse
from vk_types import types_parse
from vk_functions import funcs_parse_first, funcs_parse_second

BASE_DIR = Path(__file__).resolve().parent
xml_path = BASE_DIR / "vk.xml"

tree = et.parse(xml_path)
root = tree.getroot()

consts_parse(root)
types_parse(root)
funcs_dict = funcs_parse_first(root)
funcs_parse_second(funcs_dict)