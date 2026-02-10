use std::{fs::read, io::{Error, ErrorKind}, path::Path};

struct FontFile {
    pub file_type: FontFileType,
    pub bytes: Vec<u8>,
    pub table_records: Vec<TableRecord>
}

impl FontFile {
    pub fn parse_font_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let bytes = read(path)?;
        
        let file_type = match bytes.get(0..4).ok_or(ErrorKind::UnexpectedEof)? {
            [0x00, 0x01, 0x00, 0x00] => FontFileType::TrueType,
            [0x4F, 0x54, 0x54, 0x4F] => FontFileType::OpenType,
            _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid Magic Number"))
        };
        
        let num_tables = u16::from_be_bytes(bytes.get(4..6).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
        let mut table_records: Vec<TableRecord> = Vec::new();
        let mut current_offset = 12;
        for _ in 0..num_tables {
            let tag: [u8; 4] = bytes
                .get(current_offset..current_offset + 4)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap();
            
            let checksum = u32::from_be_bytes(
                bytes
                    .get(current_offset + 4..current_offset + 8)
                    .ok_or(ErrorKind::UnexpectedEof)?
                    .try_into()
                    .unwrap()
            );
            
            let offset = u32::from_be_bytes(
                bytes
                    .get(current_offset + 8..current_offset + 12)
                    .ok_or(ErrorKind::UnexpectedEof)?
                    .try_into()
                    .unwrap()
            );
            
            let length = u32::from_be_bytes(
                bytes
                    .get(current_offset + 12..current_offset + 16)
                    .ok_or(ErrorKind::UnexpectedEof)?
                    .try_into()
                    .unwrap()
            );
            
            table_records.push(TableRecord { tag, checksum, offset, length });
            current_offset += 16;
        }
        
        table_records.sort_by_key(|rec| rec.tag);
        
        Ok(Self { file_type, bytes, table_records })
    }
    
    pub fn get_table(&self, tag: &[u8; 4]) -> Result<&[u8], Error> {
        match self.table_records.binary_search_by_key(tag, |rec| rec.tag) {
            Ok(idx) => {
                let table = self.table_records[idx];
                let offset = table.offset as usize;
                let length = table.length as usize;
                
                Ok(&self.bytes[offset..offset + length])
            },
            Err(_) => Err(Error::new(ErrorKind::NotFound, "Given tag was not found in table records"))
        }
    }
    
    pub fn parse_head(&self) -> Result<HeadTable, Error> {
        let head_bytes = self.get_table(b"head")?;
        
        if head_bytes[12..16] != [0x5F, 0x0F, 0x3C, 0xF5] {
            return Err(Error::new(ErrorKind::InvalidData, "Head table doesn't contain correct magic number"));
        }
        
        let units_per_em = u16::from_be_bytes(head_bytes.get(18..20).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
        let created = i64::from_be_bytes(head_bytes.get(20..28).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
        let modified = i64::from_be_bytes(head_bytes.get(28..36).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
        let x_min = i16::from_be_bytes(head_bytes.get(36..38).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
        let y_min = i16::from_be_bytes(head_bytes.get(38..40).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
        let x_max = i16::from_be_bytes(head_bytes.get(40..42).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
        let y_max = i16::from_be_bytes(head_bytes.get(42..44).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
        let mac_style = u16::from_be_bytes(head_bytes.get(44..46).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
        let lowest_rec_ppem = u16::from_be_bytes(head_bytes.get(46..48).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
        let font_direction_hint = i16::from_be_bytes(head_bytes.get(48..50).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
        let index_to_loc_format = i16::from_be_bytes(head_bytes.get(50..52).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
        
        Ok(HeadTable { units_per_em, created, modified, x_min, y_min, x_max, y_max, mac_style, lowest_rec_ppem, font_direction_hint, index_to_loc_format })
    }
    
    pub fn parse_maxp(&self) -> Result<MaxpTable, Error> {
        let maxp_bytes = self.get_table(b"maxp")?;
        
        let version = u32::from_be_bytes(maxp_bytes.get(0..4).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
        match version {
            0x5000 => {
                let num_glyphs = u16::from_be_bytes(maxp_bytes.get(4..6).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                
                Ok(MaxpTable {
                    version,
                    num_glyphs,
                    max_points: None,
                    max_contours: None,
                    max_composite_points: None,
                    max_composite_contours: None,
                    max_zones: None,
                    max_twilight_points: None,
                    max_storage: None,
                    max_function_defs: None,
                    max_instruction_defs: None,
                    max_stack_elements: None,
                    max_size_of_instructions: None,
                    max_components_elements: None,
                    max_component_depth: None
                })
            }
            0x10000 => {
                let num_glyphs = u16::from_be_bytes(maxp_bytes.get(4..6).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                let max_points = u16::from_be_bytes(maxp_bytes.get(6..8).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                let max_contours = u16::from_be_bytes(maxp_bytes.get(8..10).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                let max_composite_points = u16::from_be_bytes(maxp_bytes.get(10..12).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                let max_composite_contours = u16::from_be_bytes(maxp_bytes.get(12..14).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                let max_zones = u16::from_be_bytes(maxp_bytes.get(14..16).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                let max_twilight_points = u16::from_be_bytes(maxp_bytes.get(16..18).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                let max_storage = u16::from_be_bytes(maxp_bytes.get(18..20).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                let max_function_defs = u16::from_be_bytes(maxp_bytes.get(20..22).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                let max_instruction_defs = u16::from_be_bytes(maxp_bytes.get(22..24).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                let max_stack_elements = u16::from_be_bytes(maxp_bytes.get(24..26).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                let max_size_of_instructions = u16::from_be_bytes(maxp_bytes.get(26..28).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                let max_component_elements = u16::from_be_bytes(maxp_bytes.get(28..30).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                let max_component_depth = u16::from_be_bytes(maxp_bytes.get(30..32).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                
                Ok(MaxpTable {
                    version,
                    num_glyphs,
                    max_points: Some(max_points),
                    max_contours: Some(max_contours),
                    max_composite_points: Some(max_composite_points),
                    max_composite_contours: Some(max_composite_contours),
                    max_zones: Some(max_zones),
                    max_twilight_points: Some(max_twilight_points),
                    max_storage: Some(max_storage),
                    max_function_defs: Some(max_function_defs),
                    max_instruction_defs: Some(max_instruction_defs),
                    max_stack_elements: Some(max_stack_elements),
                    max_size_of_instructions: Some(max_size_of_instructions),
                    max_components_elements: Some(max_component_elements),
                    max_component_depth: Some(max_component_depth)
                })
            }
            _ => Err(Error::new(ErrorKind::InvalidData, "Version number invalid"))
        }
    }
    
    pub fn parse_loca(&self, num_glyphs: u16, index_to_loc_format: i16) -> Result<Vec<u32>, Error> {
        let loca_bytes = self.get_table(b"loca")?;
        
        let mut indices: Vec<u32> = Vec::new();
        match index_to_loc_format {
            0 => {
                for i in 0..num_glyphs + 1 {
                    indices.push(loca_bytes[i as usize + 1] - loca_bytes[i as usize]);
                }
            },
            1 => {
                for i in 0..num_glyphs + 1 {
                    
                }
            },
            _ => return Err(Error::new(ErrorKind::InvalidInput, "index_to_loc_format was not 0 or 1"))
        }
        
        Ok(indices)
    }
}

enum FontFileType {
    TrueType,
    OpenType
}

#[derive(Clone, Copy)]
struct TableRecord {
    pub tag: [u8; 4],
    pub checksum: u32,
    pub offset: u32,
    pub length: u32
}

struct HeadTable {
    pub units_per_em: u16,
    pub created: i64,
    pub modified: i64,
    pub x_min: i16,
    pub y_min: i16,
    pub x_max: i16,
    pub y_max: i16,
    pub mac_style: u16,
    pub lowest_rec_ppem: u16,
    pub font_direction_hint: i16,
    pub index_to_loc_format: i16
}

struct MaxpTable {
    pub version: u32,
    pub num_glyphs: u16,
    pub max_points: Option<u16>,
    pub max_contours: Option<u16>,
    pub max_composite_points: Option<u16>,
    pub max_composite_contours: Option<u16>,
    pub max_zones: Option<u16>,
    pub max_twilight_points: Option<u16>,
    pub max_storage: Option<u16>,
    pub max_function_defs: Option<u16>,
    pub max_instruction_defs: Option<u16>,
    pub max_stack_elements: Option<u16>,
    pub max_size_of_instructions: Option<u16>,
    pub max_components_elements: Option<u16>,
    pub max_component_depth: Option<u16>
}