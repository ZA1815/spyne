use std::{fs::read, io::{Error, ErrorKind}, path::Path};

// Simple Glyph Flags (glyf table)
pub const ON_CURVE_POINT: u8 = 0x01;
pub const X_SHORT_VECTOR: u8 = 0x02;
pub const Y_SHORT_VECTOR: u8 = 0x04;
pub const REPEAT_FLAG: u8 = 0x08;
pub const X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR: u8 = 0x10;
pub const Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR: u8 = 0x20;

// Composite Glyph Flags (glyf table)
pub const ARG_1_AND_2_ARE_WORDS: u16 = 0x0001;
pub const ARGS_ARE_XY_VALUES: u16 = 0x0002;
pub const ROUND_XY_TO_GRID: u16 = 0x0004;
pub const WE_HAVE_A_SCALE: u16 = 0x0008;
pub const MORE_COMPONENTS: u16 = 0x0020;
pub const WE_HAVE_AN_X_AND_Y_SCALE: u16 = 0x0040;
pub const WE_HAVE_A_TWO_BY_TWO: u16 = 0x0080;
pub const WE_HAVE_INSTRUCTIONS: u16 = 0x0100;
pub const USE_MY_METRICS: u16 = 0x0200;
pub const OVERLAP_COMPOUND: u16 = 0x0400;
pub const SCALED_COMPONENT_OFFSET: u16 = 0x0800;
pub const UNSCALED_COMPONENT_OFFSET: u16 = 0x1000;

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
    
    pub fn parse_cmap(&self) -> Result<CmapTable, Error> {
        let cmap_bytes = self.get_table(b"cmap")?;
        
        let version = u16::from_be_bytes(cmap_bytes.get(0..2).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
        if version != 0 {
            return Err(Error::new(ErrorKind::InvalidData, "Version number isn't zero"));
        }
        let num_tables = u16::from_be_bytes(cmap_bytes.get(2..4).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
        let mut encoding_records: Vec<EncodingRecord> = Vec::new();
        let mut count = 4;
        for _ in 0..num_tables {
            let platform_id = u16::from_be_bytes(cmap_bytes.get(count..count + 2).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
            let encoding_id = u16::from_be_bytes(cmap_bytes.get(count + 2..count + 4).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
            let offset = u32::from_be_bytes(cmap_bytes.get(count + 4..count + 8).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
            encoding_records.push(EncodingRecord { platform_id, encoding_id, offset });
            count += 8;
        }
        
        let mut subtables: Vec<CmapSubtable> = Vec::new();
        for rec in encoding_records.iter() {
            let mut offset = rec.offset as usize;
            let format = u16::from_be_bytes(cmap_bytes.get(offset..offset + 2).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
            offset += 2;
            match format {
                0 => {
                    let length = u16::from_be_bytes(cmap_bytes.get(offset..offset + 2).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let language = u16::from_be_bytes(cmap_bytes.get(offset + 2..offset + 4).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let glyph_id_array: [u8; 256] = cmap_bytes.get(offset + 4..offset + 260).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap();
                    
                    subtables.push(CmapSubtable::Format0 { length, language, glyph_id_array });
                }
                2 => {
                    let length = u16::from_be_bytes(cmap_bytes.get(offset..offset + 2).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let language = u16::from_be_bytes(cmap_bytes.get(offset + 2..offset + 4).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let sub_header_keys: [u16; 256] = cmap_bytes.get(offset + 4..offset + 516).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        u16::from_be_bytes(ch.try_into().unwrap())
                    }).collect::<Vec<u16>>().try_into().unwrap();
                    offset += 516;
                    let sub_headers_num = (sub_header_keys.iter().max().unwrap() / 8) + 1;
                    let sub_headers: Vec<SubHeader> = cmap_bytes.get(offset..offset + sub_headers_num as usize * 8).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(8).map(|ch| {
                        let first_code = u16::from_be_bytes(ch[0..2].try_into().unwrap());
                        let entry_count = u16::from_be_bytes(ch[2..4].try_into().unwrap());
                        let id_delta = i16::from_be_bytes(ch[4..6].try_into().unwrap());
                        let id_range_offset = u16::from_be_bytes(ch[6..8].try_into().unwrap());
                        
                        SubHeader { first_code, entry_count, id_delta, id_range_offset }
                    }).collect();
                    offset += sub_headers_num as usize * 8;
                    let glyph_id_array: Vec<u16> = cmap_bytes.get(offset..).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        u16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    
                    subtables.push(CmapSubtable::Format2 { length, language, sub_header_keys, sub_headers, glyph_id_array });
                }
                4 => {
                    let length = u16::from_be_bytes(cmap_bytes.get(offset..offset + 2).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let language = u16::from_be_bytes(cmap_bytes.get(offset + 2..offset + 4).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let seg_count_x2 = u16::from_be_bytes(cmap_bytes.get(offset + 4..offset + 6).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let search_range = u16::from_be_bytes(cmap_bytes.get(offset + 6..offset + 8).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let entry_selector = u16::from_be_bytes(cmap_bytes.get(offset + 8..offset + 10).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let range_shift = u16::from_be_bytes(cmap_bytes.get(offset + 10..offset + 12).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    offset += 12;
                    let end_code: Vec<u16> = cmap_bytes.get(offset..offset + seg_count_x2 as usize).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        u16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    offset += seg_count_x2 as usize;
                    let _reserved_pad = u16::from_be_bytes(cmap_bytes.get(offset..offset + 2).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    offset += 2;
                    let start_code: Vec<u16> = cmap_bytes.get(offset..offset + seg_count_x2 as usize).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        u16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    offset += seg_count_x2 as usize / 2;
                    let id_delta: Vec<i16> = cmap_bytes.get(offset..offset + seg_count_x2 as usize).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        i16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    offset += seg_count_x2 as usize / 2;
                    let id_range_offset: Vec<u16> = cmap_bytes.get(offset..offset + seg_count_x2 as usize).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        u16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    offset += seg_count_x2 as usize / 2;
                    let glyph_id_array: Vec<u16> = cmap_bytes.get(offset..).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        u16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    
                    subtables.push(CmapSubtable::Format4 { length, language, seg_count_x2, search_range, entry_selector, range_shift, end_code, _reserved_pad, start_code, id_delta, id_range_offset, glyph_id_array });
                }
                6 => {
                    let length = u16::from_be_bytes(cmap_bytes.get(offset..offset + 2).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let language = u16::from_be_bytes(cmap_bytes.get(offset + 2..offset + 4).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let first_code = u16::from_be_bytes(cmap_bytes.get(offset + 4..offset + 6).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let entry_count = u16::from_be_bytes(cmap_bytes.get(offset + 6..offset + 8).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    offset += 8;
                    let glyph_id_array: Vec<u16> = cmap_bytes.get(offset..offset + entry_count as usize * 2).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        u16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    
                    subtables.push(CmapSubtable::Format6 { length, language, first_code, entry_count, glyph_id_array });
                }
                12 => {
                    let _reserved = u16::from_be_bytes(cmap_bytes.get(offset..offset + 2).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let length = u32::from_be_bytes(cmap_bytes.get(offset + 2..offset + 6).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let language = u32::from_be_bytes(cmap_bytes.get(offset + 6..offset + 10).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let num_groups = u32::from_be_bytes(cmap_bytes.get(offset + 10..offset + 14).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let groups: Vec<Group> = cmap_bytes.get(offset + 14..).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(num_groups as usize).map(|ch| {
                        let start_char_code = u32::from_be_bytes(ch[0..4].try_into().unwrap());
                        let end_char_code = u32::from_be_bytes(ch[4..8].try_into().unwrap());
                        let start_glyph_id = u32::from_be_bytes(ch[8..].try_into().unwrap());
                        
                        Group { start_char_code, end_char_code, start_glyph_id }
                    }).collect();
                    
                    subtables.push(CmapSubtable::Format12 { _reserved, length, language, num_groups, groups });
                }
                13 => {
                    let _reserved = u16::from_be_bytes(cmap_bytes.get(offset..offset + 2).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let length = u32::from_be_bytes(cmap_bytes.get(offset + 2..offset + 6).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let language = u32::from_be_bytes(cmap_bytes.get(offset + 6..offset + 10).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let num_groups = u32::from_be_bytes(cmap_bytes.get(offset + 10..offset + 14).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let groups: Vec<Group> = cmap_bytes.get(offset + 14..).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(num_groups as usize).map(|ch| {
                        let start_char_code = u32::from_be_bytes(ch[0..4].try_into().unwrap());
                        let end_char_code = u32::from_be_bytes(ch[4..8].try_into().unwrap());
                        let start_glyph_id = u32::from_be_bytes(ch[8..].try_into().unwrap());
                        
                        Group { start_char_code, end_char_code, start_glyph_id }
                    }).collect();
                    
                    subtables.push(CmapSubtable::Format13 { _reserved, length, language, num_groups, groups });
                }
                14 => {
                    let length = u32::from_be_bytes(cmap_bytes.get(offset..offset + 4).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let num_var_selector_records = u32::from_be_bytes(cmap_bytes.get(offset + 4..offset + 8).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    let var_selector: Vec<VariationSelectorRecord> = cmap_bytes.get(offset + 8..).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(num_var_selector_records as usize).map(|ch| {
                        let var_selector: [u8; 3] = ch[0..3].try_into().unwrap();
                        let default_uvs_offset = u32::from_be_bytes(ch[3..7].try_into().unwrap());
                        let non_default_uvs_offset = u32::from_be_bytes(ch[7..11].try_into().unwrap());
                        
                        VariationSelectorRecord { var_selector, default_uvs_offset, non_default_uvs_offset }
                    }).collect();
                    
                    subtables.push(CmapSubtable::Format14 { length, num_var_selector_records, var_selector });
                }
                8 | 10 => return Err(Error::new(ErrorKind::InvalidData, "Subtable formats 8 and 10 are considered deprecated")),
                _ => return Err(Error::new(ErrorKind::InvalidData, ""))
            }
        }
        
        Ok(CmapTable { version, num_tables, encoding_records, subtables })
    }
    
    pub fn parse_loca(&self, num_glyphs: u16, index_to_loc_format: i16) -> Result<Vec<u32>, Error> {
        let loca_bytes = self.get_table(b"loca")?;
        
        let mut indices: Vec<u32> = Vec::new();
        match index_to_loc_format {
            0 => {
                for i in (0..(num_glyphs as usize + 1) * 2).step_by(2) {
                    let offset = u16::from_be_bytes(loca_bytes.get(i..i + 2).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap()); 
                    indices.push((offset * 2) as u32);
                }
            },
            1 => {
                for i in (0..(num_glyphs as usize + 1) * 4).step_by(4) {
                    let offset = u32::from_be_bytes(loca_bytes.get(i..i + 4).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    indices.push(offset);
                }
            },
            _ => return Err(Error::new(ErrorKind::InvalidInput, "index_to_loc_format was not 0 or 1"))
        }
        
        Ok(indices)
    }
    
    pub fn parse_glyf(&self, loca_offsets: Vec<u32>) -> Result<Vec<Glyph>, Error> {
        let glyf_bytes = self.get_table(b"glyf")?;
        let mut loca_iter = loca_offsets.iter().peekable();
        
        // Could optimize by initializing with capacity based on offsets
        let mut glyphs: Vec<Glyph> = Vec::new();
        
        while let Some(offset) = loca_iter.next() {
            if let Some(next_offset) = loca_iter.peek() {
                if *next_offset - offset == 0 {
                    continue;
                }
                
                let current_glyph_bytes = glyf_bytes.get(*offset as usize..**next_offset as usize).ok_or(ErrorKind::UnexpectedEof)?;
                let mut current_offset: usize = 0;
                let number_of_contours = i16::from_be_bytes(current_glyph_bytes[0..2].try_into().unwrap());
                let x_min = i16::from_be_bytes(current_glyph_bytes.get(2..4).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                let y_min = i16::from_be_bytes(current_glyph_bytes.get(4..6).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                let x_max = i16::from_be_bytes(current_glyph_bytes.get(6..8).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                let y_max = i16::from_be_bytes(current_glyph_bytes.get(8..10).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                current_offset += 10;
                let header = GlyphHeader { number_of_contours, x_min, y_min, x_max, y_max };
                
                let mut are_instructions = false;
                if number_of_contours > 0 {
                    let end_pts_of_contours: Vec<u16> = current_glyph_bytes
                        .get(current_offset..current_offset + number_of_contours as usize * 2)
                        .ok_or(ErrorKind::UnexpectedEof)?
                        .chunks_exact(2)
                        .map(|ch| {
                            u16::from_be_bytes(ch.try_into().unwrap())
                        }).collect();
                    current_offset += number_of_contours as usize * 2;
                    let instruction_length = u16::from_be_bytes(current_glyph_bytes.get(current_offset..current_offset + 2).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap());
                    current_offset += 2;
                    let instructions: Vec<u8> = current_glyph_bytes.get(current_offset..current_offset + instruction_length as usize).ok_or(ErrorKind::UnexpectedEof)?.to_vec();
                    current_offset += instruction_length as usize;
                    let total_points = end_pts_of_contours.last().unwrap() + 1;
                    let mut flags: Vec<u8> = Vec::with_capacity(total_points as usize);
                    while flags.len() < total_points as usize {
                        let flag = current_glyph_bytes.get(current_offset).ok_or(ErrorKind::UnexpectedEof)?;
                        if flag & REPEAT_FLAG != 0 {
                            let repeat_count = current_glyph_bytes.get(current_offset + 1).ok_or(ErrorKind::UnexpectedEof)?;
                            let flag_vec = vec![flag; *repeat_count as usize + 1];
                            flags.extend(flag_vec);
                            current_offset += 2;
                        }
                        else {
                            flags.push(*flag);
                            current_offset += 1;
                        }
                    }
                    let mut x_coordinates: Vec<i16> = Vec::with_capacity(total_points as usize);
                    let mut current_x: i16 = 0;
                    for flag in flags.iter() {
                        if flag & X_SHORT_VECTOR != 0 {
                            let x = current_glyph_bytes.get(current_offset).ok_or(ErrorKind::UnexpectedEof)?;
                            if flag & X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR != 0 {
                                current_x += *x as i16;
                            }
                            else {
                                current_x -= *x as i16;
                            }
                            current_offset += 1;
                        }
                        else {
                            if flag & X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR == 0 {
                                current_x += i16::from_be_bytes(
                                    current_glyph_bytes
                                        .get(current_offset..current_offset + 2)
                                        .ok_or(ErrorKind::UnexpectedEof)?
                                        .try_into()
                                        .unwrap()
                                );
                                current_offset += 2;
                            }
                        }
                        
                        x_coordinates.push(current_x);
                    }
                    let mut y_coordinates: Vec<i16> = Vec::with_capacity(total_points as usize);
                    let mut current_y: i16 = 0;
                    for flag in flags.iter() {
                        if flag & Y_SHORT_VECTOR != 0 {
                            let y = current_glyph_bytes.get(current_offset).ok_or(ErrorKind::UnexpectedEof)?;
                            if flag & Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR != 0 {
                                current_y += *y as i16;
                            }
                            else {
                                current_y -= *y as i16;
                            }
                            current_offset += 1;
                        }
                        else {
                            if flag & Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR == 0 {
                                current_y += i16::from_be_bytes(
                                    current_glyph_bytes
                                        .get(current_offset..current_offset + 2)
                                        .ok_or(ErrorKind::UnexpectedEof)?
                                        .try_into()
                                        .unwrap()
                                );
                                current_offset += 2;
                            }
                        }
                        
                        y_coordinates.push(current_y);
                    }
                    
                    glyphs.push(Glyph::Simple { header, end_pts_of_contours, instruction_length, instructions, flags, x_coordinates, y_coordinates });
                }
                else if number_of_contours == -1 {
                    let mut components: Vec<Component> = Vec::new();
                    loop {
                        let flags = u16::from_be_bytes(
                            current_glyph_bytes
                                .get(current_offset..current_offset + 2)
                                .ok_or(ErrorKind::UnexpectedEof)?
                                .try_into()
                                .unwrap()
                        );
                        let glyph_index = u16::from_be_bytes(
                            current_glyph_bytes
                                .get(current_offset + 2..current_offset + 4)
                                .ok_or(ErrorKind::UnexpectedEof)?
                                .try_into()
                                .unwrap()
                        );
                        current_offset += 4;
                        let argument_1: i16;
                        let argument_2: i16;
                        if flags & ARG_1_AND_2_ARE_WORDS != 0 {
                            argument_1 = i16::from_be_bytes(
                                current_glyph_bytes
                                    .get(current_offset..current_offset + 2)
                                    .ok_or(ErrorKind::UnexpectedEof)?
                                    .try_into()
                                    .unwrap()
                            );
                            argument_2 = i16::from_be_bytes(
                                current_glyph_bytes
                                    .get(current_offset + 2..current_offset + 4)
                                    .ok_or(ErrorKind::UnexpectedEof)?
                                    .try_into()
                                    .unwrap()
                            );
                            current_offset += 4;
                        }
                        else {
                            argument_1 = *current_glyph_bytes
                                .get(current_offset)
                                .ok_or(ErrorKind::UnexpectedEof)? as i8 as i16;
                            argument_2 = *current_glyph_bytes
                                .get(current_offset + 1)
                                .ok_or(ErrorKind::UnexpectedEof)? as i8 as i16;
                            current_offset += 2;
                        }
                        let transformation: [i16; 4];
                        if flags & WE_HAVE_A_SCALE != 0 {
                            transformation = [
                                i16::from_be_bytes(
                                current_glyph_bytes
                                    .get(current_offset..current_offset + 2)
                                    .ok_or(ErrorKind::UnexpectedEof)?
                                    .try_into()
                                    .unwrap()
                            ), 0, 0, 0];
                            current_offset += 2;
                        }
                        else if flags & WE_HAVE_AN_X_AND_Y_SCALE != 0 {
                            transformation = [
                                i16::from_be_bytes(
                                    current_glyph_bytes
                                        .get(current_offset..current_offset + 2)
                                        .ok_or(ErrorKind::UnexpectedEof)?
                                        .try_into()
                                        .unwrap()
                                ),
                                i16::from_be_bytes(
                                    current_glyph_bytes
                                        .get(current_offset + 2..current_offset + 4)
                                        .ok_or(ErrorKind::UnexpectedEof)?
                                        .try_into()
                                        .unwrap()
                                ), 0, 0
                            ];
                            current_offset += 4;
                        }
                        else if flags & WE_HAVE_A_TWO_BY_TWO != 0 {
                            transformation = [
                                i16::from_be_bytes(
                                    current_glyph_bytes
                                        .get(current_offset..current_offset + 2)
                                        .ok_or(ErrorKind::UnexpectedEof)?
                                        .try_into()
                                        .unwrap()
                                ),
                                i16::from_be_bytes(
                                    current_glyph_bytes
                                        .get(current_offset + 2..current_offset + 4)
                                        .ok_or(ErrorKind::UnexpectedEof)?
                                        .try_into()
                                        .unwrap()
                                ),
                                i16::from_be_bytes(
                                    current_glyph_bytes
                                        .get(current_offset + 4..current_offset + 6)
                                        .ok_or(ErrorKind::UnexpectedEof)?
                                        .try_into()
                                        .unwrap()
                                ),
                                i16::from_be_bytes(
                                    current_glyph_bytes
                                        .get(current_offset + 6..current_offset + 8)
                                        .ok_or(ErrorKind::UnexpectedEof)?
                                        .try_into()
                                        .unwrap()
                                )
                            ];
                            current_offset += 8;
                        }
                        else {
                            transformation = [0, 0, 0, 0];
                        }
                        
                        components.push(Component { flags, glyph_index, argument_1, argument_2, transformation });
                        
                        are_instructions = flags & WE_HAVE_INSTRUCTIONS != 0;
                        
                        if flags & MORE_COMPONENTS == 0 { break; }
                    }
                    let mut instruction_length: Option<u16> = None;
                    let mut instructions: Option<Vec<u8>> = None;
                    if are_instructions {
                        instruction_length = Some(u16::from_be_bytes(
                            current_glyph_bytes
                                .get(current_offset..current_offset + 2)
                                .ok_or(ErrorKind::UnexpectedEof)?
                                .try_into()
                                .unwrap()
                        ));
                        current_offset += 2;
                        instructions = Some(
                            current_glyph_bytes
                                .get(current_offset..current_offset + instruction_length.unwrap() as usize)
                                .ok_or(ErrorKind::UnexpectedEof)?
                                .to_vec()
                        )
                    }
                    
                    glyphs.push(Glyph::Composite { header, components, instruction_length, instructions });
                }
            }
        }
        
        Ok(glyphs)
    }
    
    pub fn parse_hhea(&self) -> Result<HheaTable, Error> {
        let hhea_bytes = self.get_table(b"hhea")?;
        
        let version = u32::from_be_bytes(
            hhea_bytes.get(0..4)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        );
        if version != 0x00010000 {
            return Err(Error::new(ErrorKind::InvalidData, "Version number for hhea table is incorrect"));
        }
        let ascender = i16::from_be_bytes(
            hhea_bytes.get(4..6)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        );
        let descender = i16::from_be_bytes(
            hhea_bytes.get(6..8)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        );
        let line_gap = i16::from_be_bytes(
            hhea_bytes.get(8..10)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        );
        let advance_width_max = u16::from_be_bytes(
            hhea_bytes.get(10..12)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        );
        let min_left_side_bearing = i16::from_be_bytes(
            hhea_bytes.get(12..14)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        );
        let min_right_side_bearing = i16::from_be_bytes(
            hhea_bytes.get(14..16)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        );
        let x_max_extent = i16::from_be_bytes(
            hhea_bytes.get(16..18)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        );
        let caret_slope_rise = i16::from_be_bytes(
            hhea_bytes.get(18..20)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        );
        let caret_slope_run = i16::from_be_bytes(
            hhea_bytes.get(20..22)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        );
        let caret_offset = i16::from_be_bytes(
            hhea_bytes.get(22..24)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        );
        let _reserved1 = 0;
        let _reserved2 = 0;
        let _reserved3 = 0;
        let _reserved4 = 0;
        let metric_data_format = i16::from_be_bytes(
            hhea_bytes.get(32..34)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        );
        let number_of_h_metrics = u16::from_be_bytes(
            hhea_bytes.get(34..36)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        );
        
        Ok(HheaTable {
            version,
            ascender,
            descender,
            line_gap,
            advance_width_max,
            min_left_side_bearing,
            min_right_side_bearing,
            x_max_extent,
            caret_slope_rise,
            caret_slope_run,
            caret_offset,
            _reserved1,
            _reserved2,
            _reserved3,
            _reserved4,
            metric_data_format,
            number_of_h_metrics
        })
    }
    
    pub fn parse_hmtx(&self, num_glyphs: u16, number_of_h_metrics: u16) -> Result<HmtxTable, Error> {
        let hmtx_bytes = self.get_table(b"hmtx")?;
        
        let h_metrics: Vec<LongHorMetric> = hmtx_bytes
            .get(0..number_of_h_metrics as usize * 4)
            .ok_or(ErrorKind::UnexpectedEof)?
            .chunks_exact(4)
            .map(|ch| {
                let advance_width = u16::from_be_bytes(ch[0..2].try_into().unwrap());
                let lsb = i16::from_be_bytes(ch[2..4].try_into().unwrap());
                
                LongHorMetric { advance_width, lsb }
            }).collect();
        
        let leftovers = num_glyphs - number_of_h_metrics;
        
        let left_side_bearings: Vec<i16> = hmtx_bytes
            .get(number_of_h_metrics as usize * 4..(number_of_h_metrics as usize * 4) + (leftovers as usize * 2))
            .ok_or(ErrorKind::UnexpectedEof)?
            .chunks_exact(2)
            .map(|ch| {
                i16::from_be_bytes(ch.try_into().unwrap())
            }).collect();
        
        Ok(HmtxTable { h_metrics, left_side_bearings })
    }
}

pub enum FontFileType {
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

struct CmapTable {
    pub version: u16,
    pub num_tables: u16,
    pub encoding_records: Vec<EncodingRecord>,
    pub subtables: Vec<CmapSubtable>
}

struct EncodingRecord {
    pub platform_id: u16,
    pub encoding_id: u16,
    pub offset: u32
}

pub enum CmapSubtable {
    Format0 {
        length: u16,
        language: u16,
        glyph_id_array: [u8; 256]
    },
    Format2 {
        length: u16,
        language: u16,
        sub_header_keys: [u16; 256],
        sub_headers: Vec<SubHeader>,
        glyph_id_array: Vec<u16>
    },
    Format4 {
        length: u16,
        language: u16,
        seg_count_x2: u16,
        search_range: u16,
        entry_selector: u16,
        range_shift: u16,
        end_code: Vec<u16>,
        _reserved_pad: u16,
        start_code: Vec<u16>,
        id_delta: Vec<i16>,
        id_range_offset: Vec<u16>,
        glyph_id_array: Vec<u16>
    },
    Format6 {
        length: u16,
        language: u16,
        first_code: u16,
        entry_count: u16,
        glyph_id_array: Vec<u16>
    },
    Format12 {
        _reserved: u16,
        length: u32,
        language: u32,
        num_groups: u32,
        groups: Vec<Group>
    },
    Format13 {
        _reserved: u16,
        length: u32,
        language: u32,
        num_groups: u32,
        groups: Vec<Group>
    },
    Format14 {
        length: u32,
        num_var_selector_records: u32,
        var_selector: Vec<VariationSelectorRecord>
    }
}

struct SubHeader {
    pub first_code: u16,
    pub entry_count: u16,
    pub id_delta: i16,
    pub id_range_offset: u16
}

struct Group {
    pub start_char_code: u32,
    pub end_char_code: u32,
    pub start_glyph_id: u32
}

struct VariationSelectorRecord {
    pub var_selector: [u8; 3],
    pub default_uvs_offset: u32,
    pub non_default_uvs_offset: u32
}

pub enum Glyph {
    Simple {
        header: GlyphHeader,
        end_pts_of_contours: Vec<u16>,
        instruction_length: u16,
        instructions: Vec<u8>,
        flags: Vec<u8>,
        x_coordinates: Vec<i16>,
        y_coordinates: Vec<i16>
    },
    Composite {
        header: GlyphHeader,
        components: Vec<Component>,
        instruction_length: Option<u16>,
        instructions: Option<Vec<u8>>
    }
}

struct GlyphHeader {
    pub number_of_contours: i16,
    pub x_min: i16,
    pub y_min: i16,
    pub x_max: i16,
    pub y_max: i16
}

struct Component {
    pub flags: u16,
    pub glyph_index: u16,
    pub argument_1: i16,
    pub argument_2: i16,
    pub transformation: [i16; 4]
}

struct HheaTable {
    pub version: u32,
    pub ascender: i16,
    pub descender: i16,
    pub line_gap: i16,
    pub advance_width_max: u16,
    pub min_left_side_bearing: i16,
    pub min_right_side_bearing: i16,
    pub x_max_extent: i16,
    pub caret_slope_rise: i16,
    pub caret_slope_run: i16,
    pub caret_offset: i16,
    _reserved1: i16,
    _reserved2: i16,
    _reserved3: i16,
    _reserved4: i16,
    pub metric_data_format: i16,
    pub number_of_h_metrics: u16
}

struct HmtxTable {
    pub h_metrics: Vec<LongHorMetric>,
    pub left_side_bearings: Vec<i16>
}

struct LongHorMetric {
    pub advance_width: u16,
    pub lsb: i16
}