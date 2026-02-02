import xml.etree.ElementTree as et
from pathlib import Path
from constants import constants_parse

BASE_DIR = Path(__file__).resolve().parent
xml_path = BASE_DIR / "vk.xml"

tree = et.parse(xml_path)
root = tree.getroot()

constants_parse(root)