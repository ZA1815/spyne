use std::{fs::read, io::{Error, ErrorKind}, path::Path};

use crate::text::fonts::constants::{ARG_1_AND_2_ARE_WORDS, MAC_ROMAN_LOOKUP, MAC_STANDARD_NAMES, MORE_COMPONENTS, REPEAT_FLAG, WE_HAVE_A_SCALE, WE_HAVE_A_TWO_BY_TWO, WE_HAVE_AN_X_AND_Y_SCALE, WE_HAVE_INSTRUCTIONS, X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR, X_SHORT_VECTOR, Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR, Y_SHORT_VECTOR};

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
        let bytes = self.get_table(b"head")?;
        
        if bytes[12..16] != [0x5F, 0x0F, 0x3C, 0xF5] {
            return Err(Error::new(ErrorKind::InvalidData, "Head table doesn't contain correct magic number"));
        }
        
        let units_per_em = get_u16(bytes, 18, 20)?;
        let created = get_i64(bytes, 20, 28)?;
        let modified = get_i64(bytes, 28, 36)?;
        let x_min = get_i16(bytes, 36, 38)?; 
        let y_min = get_i16(bytes, 38, 40)?;
        let x_max = get_i16(bytes, 40, 42)?;
        let y_max = get_i16(bytes, 42, 44)?;
        let mac_style = get_u16(bytes, 44, 46)?;
        let lowest_rec_ppem = get_u16(bytes, 46, 48)?;
        let font_direction_hint = get_i16(bytes, 48, 50)?;
        let index_to_loc_format = get_i16(bytes, 50, 52)?;
        
        Ok(HeadTable {
            units_per_em,
            created,
            modified,
            x_min,
            y_min,
            x_max,
            y_max,
            mac_style,
            lowest_rec_ppem,
            font_direction_hint,
            index_to_loc_format
        })
    }
    
    pub fn parse_maxp(&self) -> Result<MaxpTable, Error> {
        let bytes = self.get_table(b"maxp")?;
        
        let version = get_u32(bytes, 0, 4)?;
        match version {
            0x5000 => {
                let num_glyphs = get_u16(bytes, 4, 6)?;
                
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
                let num_glyphs = get_u16(bytes, 4, 6)?;
                let max_points = get_u16(bytes, 6, 8)?;
                let max_contours = get_u16(bytes, 8, 10)?;
                let max_composite_points = get_u16(bytes, 10, 12)?;
                let max_composite_contours = get_u16(bytes, 12, 14)?;
                let max_zones = get_u16(bytes, 14, 16)?;
                let max_twilight_points = get_u16(bytes, 16, 18)?;
                let max_storage = get_u16(bytes, 16, 18)?;
                let max_function_defs = get_u16(bytes, 18, 20)?;
                let max_instruction_defs = get_u16(bytes, 20, 22)?;
                let max_stack_elements = get_u16(bytes, 22, 24)?;
                let max_size_of_instructions = get_u16(bytes, 24, 26)?;
                let max_component_elements = get_u16(bytes, 26, 28)?;
                let max_component_depth = get_u16(bytes, 30, 32)?;
                
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
        let bytes = self.get_table(b"cmap")?;
        
        let version = get_u16(bytes, 0, 2)?;
        if version != 0 {
            return Err(Error::new(ErrorKind::InvalidData, "Version number isn't zero"));
        }
        let num_tables = get_u16(bytes, 2, 4)?;
        let mut encoding_records: Vec<EncodingRecord> = Vec::new();
        let mut count = 4;
        for _ in 0..num_tables {
            let platform_id = get_u16(bytes, count, count + 2)?;
            let encoding_id = get_u16(bytes, count + 2, count + 4)?;
            let offset = get_u32(bytes, count + 4, count + 8)?;
            encoding_records.push(EncodingRecord { platform_id, encoding_id, offset });
            count += 8;
        }
        
        let mut subtables: Vec<CmapSubtable> = Vec::new();
        for rec in encoding_records.iter() {
            let mut offset = rec.offset as usize;
            let format = get_u16(bytes, offset, offset + 2)?;
            offset += 2;
            match format {
                0 => {
                    let length = get_u16(bytes, offset, offset + 2)?;
                    let language = get_u16(bytes, offset + 2, offset + 4)?;
                    let glyph_id_array: [u8; 256] = bytes.get(offset + 4..offset + 260).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap();
                    
                    subtables.push(CmapSubtable::Format0 { length, language, glyph_id_array });
                }
                2 => {
                    let length = get_u16(bytes, offset, offset + 2)?;
                    let language = get_u16(bytes, offset + 2, offset + 4)?;
                    let sub_header_keys: [u16; 256] = bytes.get(offset + 4..offset + 516).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        u16::from_be_bytes(ch.try_into().unwrap())
                    }).collect::<Vec<u16>>().try_into().unwrap();
                    offset += 516;
                    let sub_headers_num = (sub_header_keys.iter().max().unwrap() / 8) + 1;
                    let sub_headers: Vec<SubHeader> = bytes.get(offset..offset + sub_headers_num as usize * 8).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(8).map(|ch| {
                        let first_code = u16::from_be_bytes(ch[0..2].try_into().unwrap());
                        let entry_count = u16::from_be_bytes(ch[2..4].try_into().unwrap());
                        let id_delta = i16::from_be_bytes(ch[4..6].try_into().unwrap());
                        let id_range_offset = u16::from_be_bytes(ch[6..8].try_into().unwrap());
                        
                        SubHeader { first_code, entry_count, id_delta, id_range_offset }
                    }).collect();
                    offset += sub_headers_num as usize * 8;
                    let glyph_id_array: Vec<u16> = bytes.get(offset..).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        u16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    
                    subtables.push(CmapSubtable::Format2 { length, language, sub_header_keys, sub_headers, glyph_id_array });
                }
                4 => {
                    let length = get_u16(bytes, offset, offset + 2)?;
                    let language = get_u16(bytes, offset + 2, offset + 4)?;
                    let seg_count_x2 = get_u16(bytes, offset + 4, offset + 6)?;
                    let search_range = get_u16(bytes, offset + 6, offset + 8)?;
                    let entry_selector = get_u16(bytes, offset + 8, offset + 10)?;
                    let range_shift = get_u16(bytes, offset + 10, offset + 12)?;
                    offset += 12;
                    let end_code: Vec<u16> = bytes.get(offset..offset + seg_count_x2 as usize).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        u16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    offset += seg_count_x2 as usize;
                    let _reserved_pad = get_u16(bytes, offset, offset + 2)?;
                    offset += 2;
                    let start_code: Vec<u16> = bytes.get(offset..offset + seg_count_x2 as usize).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        u16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    offset += seg_count_x2 as usize / 2;
                    let id_delta: Vec<i16> = bytes.get(offset..offset + seg_count_x2 as usize).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        i16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    offset += seg_count_x2 as usize / 2;
                    let id_range_offset: Vec<u16> = bytes.get(offset..offset + seg_count_x2 as usize).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        u16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    offset += seg_count_x2 as usize / 2;
                    let glyph_id_array: Vec<u16> = bytes.get(offset..).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        u16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    
                    subtables.push(CmapSubtable::Format4 { length, language, seg_count_x2, search_range, entry_selector, range_shift, end_code, _reserved_pad, start_code, id_delta, id_range_offset, glyph_id_array });
                }
                6 => {
                    let length = get_u16(bytes, offset, offset + 2)?;
                    let language = get_u16(bytes, offset + 2, offset + 4)?;
                    let first_code = get_u16(bytes, offset + 4, offset + 6)?;
                    let entry_count = get_u16(bytes, offset + 6, offset + 8)?;
                    offset += 8;
                    let glyph_id_array: Vec<u16> = bytes.get(offset..offset + entry_count as usize * 2).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        u16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    
                    subtables.push(CmapSubtable::Format6 { length, language, first_code, entry_count, glyph_id_array });
                }
                12 => {
                    let _reserved = get_u16(bytes, offset, offset + 2)?;
                    let length = get_u32(bytes, offset + 2, offset + 6)?;
                    let language = get_u32(bytes, offset + 6, offset + 10)?;
                    let num_groups = get_u32(bytes, offset + 10, offset + 14)?;
                    let groups: Vec<Group> = bytes.get(offset + 14..).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(num_groups as usize).map(|ch| {
                        let start_char_code = u32::from_be_bytes(ch[0..4].try_into().unwrap());
                        let end_char_code = u32::from_be_bytes(ch[4..8].try_into().unwrap());
                        let start_glyph_id = u32::from_be_bytes(ch[8..].try_into().unwrap());
                        
                        Group { start_char_code, end_char_code, start_glyph_id }
                    }).collect();
                    
                    subtables.push(CmapSubtable::Format12 { _reserved, length, language, num_groups, groups });
                }
                13 => {
                    let _reserved = get_u16(bytes, offset, offset + 2)?;
                    let length = get_u32(bytes, offset + 2, offset + 6)?;
                    let language = get_u32(bytes, offset + 6, offset + 10)?;
                    let num_groups = get_u32(bytes, offset + 10, offset + 14)?;
                    let groups: Vec<Group> = bytes.get(offset + 14..).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(num_groups as usize).map(|ch| {
                        let start_char_code = u32::from_be_bytes(ch[0..4].try_into().unwrap());
                        let end_char_code = u32::from_be_bytes(ch[4..8].try_into().unwrap());
                        let start_glyph_id = u32::from_be_bytes(ch[8..].try_into().unwrap());
                        
                        Group { start_char_code, end_char_code, start_glyph_id }
                    }).collect();
                    
                    subtables.push(CmapSubtable::Format13 { _reserved, length, language, num_groups, groups });
                }
                14 => {
                    let length = get_u32(bytes, offset, offset + 4)?;
                    let num_var_selector_records = get_u32(bytes, offset + 4, offset + 8)?;
                    let var_selector: Vec<VariationSelectorRecord> = bytes.get(offset + 8..).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(num_var_selector_records as usize).map(|ch| {
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
        let bytes = self.get_table(b"loca")?;
        
        let mut indices: Vec<u32> = Vec::new();
        match index_to_loc_format {
            0 => {
                for i in (0..(num_glyphs as usize + 1) * 2).step_by(2) {
                    let offset = get_u16(bytes, i, i + 2)?;
                    indices.push((offset * 2) as u32);
                }
            },
            1 => {
                for i in (0..(num_glyphs as usize + 1) * 4).step_by(4) {
                    let offset = get_u32(bytes, i, i + 4)?;
                    indices.push(offset);
                }
            },
            _ => return Err(Error::new(ErrorKind::InvalidInput, "index_to_loc_format was not 0 or 1"))
        }
        
        Ok(indices)
    }
    
    pub fn parse_glyf(&self, loca_offsets: Vec<u32>) -> Result<Vec<Glyph>, Error> {
        let bytes = self.get_table(b"glyf")?;
        let mut loca_iter = loca_offsets.iter().peekable();
        
        // Could optimize by initializing with capacity based on offsets
        let mut glyphs: Vec<Glyph> = Vec::new();
        
        while let Some(offset) = loca_iter.next() {
            if let Some(next_offset) = loca_iter.peek() {
                if *next_offset - offset == 0 {
                    continue;
                }
                
                let current_glyph_bytes = bytes.get(*offset as usize..**next_offset as usize).ok_or(ErrorKind::UnexpectedEof)?;
                let mut current_offset: usize = 0;
                let number_of_contours = get_i16(current_glyph_bytes, 0, 2)?;
                let x_min = get_i16(current_glyph_bytes, 2, 4)?;
                let y_min = get_i16(current_glyph_bytes, 4, 6)?;
                let x_max = get_i16(current_glyph_bytes, 6, 8)?;
                let y_max = get_i16(current_glyph_bytes, 8, 10)?;
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
                    let instruction_length = get_u16(bytes, current_offset, current_offset + 2)?;
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
                                current_x += get_i16(current_glyph_bytes, current_offset, current_offset + 2)?;
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
                                current_y += get_i16(current_glyph_bytes, current_offset, current_offset + 2)?;
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
                        let flags = get_u16(current_glyph_bytes, current_offset, current_offset + 2)?;
                        let glyph_index = get_u16(current_glyph_bytes, current_offset + 2, current_offset + 4)?;
                        current_offset += 4;
                        let argument_1: i16;
                        let argument_2: i16;
                        if flags & ARG_1_AND_2_ARE_WORDS != 0 {
                            argument_1 = get_i16(current_glyph_bytes, current_offset, current_offset + 2)?;
                            argument_2 = get_i16(current_glyph_bytes, current_offset + 2, current_offset + 4)?;
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
                            transformation = [get_i16(current_glyph_bytes, current_offset, current_offset + 2)?, 0, 0, 0];
                            current_offset += 2;
                        }
                        else if flags & WE_HAVE_AN_X_AND_Y_SCALE != 0 {
                            transformation = [
                                get_i16(current_glyph_bytes, current_offset, current_offset + 2)?,
                                get_i16(current_glyph_bytes, current_offset + 2, current_offset + 4)?,
                                0, 0
                            ];
                            current_offset += 4;
                        }
                        else if flags & WE_HAVE_A_TWO_BY_TWO != 0 {
                            transformation = [
                                get_i16(current_glyph_bytes, current_offset, current_offset + 2)?,
                                get_i16(current_glyph_bytes, current_offset + 2, current_offset + 4)?,
                                get_i16(current_glyph_bytes, current_offset + 4, current_offset + 6)?,
                                get_i16(current_glyph_bytes, current_offset + 6, current_offset + 8)?
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
                        instruction_length = Some(get_u16(current_glyph_bytes, current_offset, current_offset + 2)?);
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
        let bytes = self.get_table(b"hhea")?;
        
        let version = get_u32(bytes, 0, 4)?;
        if version != 0x00010000 {
            return Err(Error::new(ErrorKind::InvalidData, "Version number for hhea table is incorrect"));
        }
        let ascender = get_i16(bytes, 4, 6)?;
        let descender = get_i16(bytes, 6, 8)?;
        let line_gap = get_i16(bytes, 8, 10)?;
        let advance_width_max = get_u16(bytes, 10, 12)?;
        let min_left_side_bearing = get_i16(bytes, 12, 14)?;
        let min_right_side_bearing = get_i16(bytes, 14, 16)?;
        let x_max_extent = get_i16(bytes, 16, 18)?;
        let caret_slope_rise = get_i16(bytes, 18, 20)?;
        let caret_slope_run = get_i16(bytes, 20, 22)?;
        let caret_offset = get_i16(bytes, 22, 24)?;
        let _reserved1 = 0;
        let _reserved2 = 0;
        let _reserved3 = 0;
        let _reserved4 = 0;
        let metric_data_format = get_i16(bytes, 32, 34)?;
        let number_of_h_metrics = get_u16(bytes, 34, 36)?;
        
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
        let bytes = self.get_table(b"hmtx")?;
        
        let mut entries: Vec<HmtxEntry> = bytes
            .get(0..number_of_h_metrics as usize * 4)
            .ok_or(ErrorKind::UnexpectedEof)?
            .chunks_exact(4)
            .map(|ch| {
                let advance_width = u16::from_be_bytes(ch[0..2].try_into().unwrap());
                let lsb = i16::from_be_bytes(ch[2..4].try_into().unwrap());
                
                HmtxEntry::FullMetric { advance_width, lsb }
            }).collect();
        
        let shared_advance_var = entries.last().unwrap();
        let shared_advance_width = match shared_advance_var {
            HmtxEntry::FullMetric { advance_width, lsb: _ } => *advance_width,
            _ => unreachable!()
        };
        
        let leftovers = num_glyphs - number_of_h_metrics;
        
        entries.extend(
            bytes
                .get(number_of_h_metrics as usize * 4..(number_of_h_metrics as usize * 4) + (leftovers as usize * 2))
                .ok_or(ErrorKind::UnexpectedEof)?
                .chunks_exact(2)
                .map(|ch| {
                    HmtxEntry::LeftoverBearing(i16::from_be_bytes(ch.try_into().unwrap()))
                })
        );
        
        Ok(HmtxTable { entries, shared_advance_width })
    }
    
    pub fn parse_name(&self) -> Result<NameTable, Error> {
        let bytes = self.get_table(b"name")?;
        let mut offset = 0;
        
        let version = get_u16(bytes, 0, 2)?;
        if version != 0 && version != 1 {
            return Err(Error::new(ErrorKind::InvalidData, "Version number is not 0 or 1"));
        }
        let count = get_u16(bytes, 2, 4)?;
        let storage_offset = get_u16(bytes, 4, 6)?;
        offset += 6;
        let records: Vec<NameRecord> = bytes.get(offset..offset + count as usize * 12)
            .ok_or(ErrorKind::UnexpectedEof)?
            .chunks_exact(12)
            .map(|ch| {
                let platform_id = u16::from_be_bytes(ch[0..2].try_into().unwrap());
                let encoding_id = u16::from_be_bytes(ch[2..4].try_into().unwrap());
                let language_id = u16::from_be_bytes(ch[4..6].try_into().unwrap());
                let name_id = u16::from_be_bytes(ch[6..8].try_into().unwrap());
                let length = u16::from_be_bytes(ch[8..10].try_into().unwrap());
                let string_offset = u16::from_be_bytes(ch[10..12].try_into().unwrap());
                let string_bytes = bytes.get(storage_offset as usize + string_offset as usize..storage_offset as usize + string_offset as usize + length as usize)
                    .ok_or(ErrorKind::UnexpectedEof)?;
                let string = decode_name_bytes(string_bytes, platform_id, encoding_id)?;
                
                Ok(NameRecord { platform_id, encoding_id, language_id, name_id, length, string_offset, string })
            }).collect::<Result<Vec<_>, Error>>()?;
        offset += count as usize * 12;
        let mut lang_tag_count: Option<u16> = None;
        let mut lang_tag_records: Option<Vec<LangTagRecord>> = None;
        if version == 1 {
            lang_tag_count = Some(get_u16(bytes, offset, offset + 2)?);
            offset += 2;
            lang_tag_records = Some(
                bytes.get(offset..offset + lang_tag_count.unwrap() as usize * 4)
                    .ok_or(ErrorKind::UnexpectedEof)?
                    .chunks_exact(4)
                    .map(|ch| {
                        let length = u16::from_be_bytes(ch[0..2].try_into().unwrap());
                        let lang_tag_offset = u16::from_be_bytes(ch[2..4].try_into().unwrap());
                        let string_bytes: Vec<u16> = bytes.get(storage_offset as usize + lang_tag_offset as usize.. storage_offset as usize + lang_tag_offset as usize + length as usize)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch[0..2].try_into().unwrap())
                            }).collect();
                        let string = String::from_utf16(&string_bytes).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
                        
                        Ok(LangTagRecord { length, lang_tag_offset, string })
                    }).collect::<Result<Vec<_>, Error>>()?
            );
        }
        
        Ok(NameTable { version, count, storage_offset, records, lang_tag_count, lang_tag_records })
    }
    
    pub fn parse_os2(&self) -> Result<OS2Table, Error> {
        let bytes = self.get_table(b"OS/2")?;
        
        let version = get_u16(bytes, 0, 2)?;
        let x_avg_char_width = get_i16(bytes, 2, 4)?;
        let us_weight_class = get_u16(bytes, 4, 6)?;
        let us_width_class = get_u16(bytes, 6, 8)?;
        let fs_type = get_u16(bytes, 8, 10)?;
        let y_subscript_x_size = get_i16(bytes, 10, 12)?;
        let y_subscript_y_size = get_i16(bytes, 12, 14)?;
        let y_subscript_x_offset = get_i16(bytes, 14, 16)?;
        let y_subscript_y_offset = get_i16(bytes, 16, 18)?;
        let y_superscript_x_size = get_i16(bytes, 18, 20)?;
        let y_superscript_y_size = get_i16(bytes, 20, 22)?;
        let y_superscript_x_offset = get_i16(bytes, 22, 24)?;
        let y_superscript_y_offset = get_i16(bytes, 24, 26)?;
        let y_strikeout_size = get_i16(bytes, 26, 28)?;
        let y_strikeout_position = get_i16(bytes, 28, 30)?;
        let s_family_class = get_i16(bytes, 30, 32)?;
        let panose: [u8; 10] = bytes.get(32..42)
            .ok_or(ErrorKind::UnexpectedEof)?
            .try_into()
            .unwrap();
        let ul_unicode_range_1 = get_u32(bytes, 42, 46)?;
        let ul_unicode_range_2 = get_u32(bytes, 46, 50)?;
        let ul_unicode_range_3 = get_u32(bytes, 50, 54)?;
        let ul_unicode_range_4 = get_u32(bytes, 54, 58)?;
        let ach_vend_id: [u8; 4] = bytes.get(58..62)
            .ok_or(ErrorKind::UnexpectedEof)?
            .try_into()
            .unwrap();
        let fs_selection = get_u16(bytes, 62, 64)?;
        let us_first_char_index = get_u16(bytes, 64, 66)?;
        let us_last_char_index = get_u16(bytes, 66, 68)?;
        let s_typo_ascender = get_i16(bytes, 68, 70)?;
        let s_typo_descender = get_i16(bytes, 70, 72)?;
        let s_typo_line_gap = get_i16(bytes, 72, 74)?;
        let us_win_ascent = get_u16(bytes, 74, 76)?;
        let us_win_descent = get_u16(bytes, 76, 78)?;
        let mut ul_code_page_range_1: Option<u32> = None;
        let mut ul_code_page_range_2: Option<u32> = None;
        if version >= 1 {
            ul_code_page_range_1 = Some(get_u32(bytes, 78, 82)?);
            ul_code_page_range_2 = Some(get_u32(bytes, 82, 86)?);
        }
        let mut sx_height: Option<i16> = None;
        let mut s_cap_height: Option<i16> = None;
        let mut us_default_char: Option<u16> = None;
        let mut us_break_char: Option<u16> = None;
        let mut us_max_context: Option<u16> = None;
        if version >= 2 {
            sx_height = Some(get_i16(bytes, 86, 88)?);
            s_cap_height = Some(get_i16(bytes, 88, 90)?);
            us_default_char = Some(get_u16(bytes, 90, 92)?);
            us_break_char = Some(get_u16(bytes, 92, 94)?);
            us_max_context = Some(get_u16(bytes, 94, 96)?);
        }
        let mut us_lower_optical_point_size: Option<u16> = None;
        let mut us_upper_optical_point_size: Option<u16> = None;
        if version >= 5 {
            us_lower_optical_point_size = Some(get_u16(bytes, 96, 98)?);
            us_upper_optical_point_size = Some(get_u16(bytes, 98, 100)?);
        }
        
        Ok(OS2Table {
            version,
            x_avg_char_width,
            us_weight_class,
            us_width_class,
            fs_type,
            y_subscript_x_size,
            y_subscript_y_size,
            y_subscript_x_offset,
            y_subscript_y_offset,
            y_superscript_x_size,
            y_superscript_y_size,
            y_superscript_x_offset,
            y_superscript_y_offset,
            y_strikeout_size,
            y_strikeout_position,
            s_family_class,
            panose,
            ul_unicode_range_1,
            ul_unicode_range_2,
            ul_unicode_range_3,
            ul_unicode_range_4,
            ach_vend_id,
            fs_selection,
            us_first_char_index,
            us_last_char_index,
            s_typo_ascender,
            s_typo_descender,
            s_typo_line_gap,
            us_win_ascent,
            us_win_descent,
            ul_code_page_range_1,
            ul_code_page_range_2,
            sx_height,
            s_cap_height,
            us_default_char,
            us_break_char,
            us_max_context,
            us_lower_optical_point_size,
            us_upper_optical_point_size
        })
    }
    
    pub fn parse_post(&self) -> Result<PostTable, Error> {
        let bytes = self.get_table(b"post")?;
        
        let version = get_u32(bytes, 0, 4)?;
        let italic_angle = get_i32(bytes, 4, 8)?;
        let underline_position = get_i16(bytes, 8, 10)?;
        let underline_thickness = get_i16(bytes, 10, 12)?;
        let is_fixed_pitch = get_u32(bytes, 12, 16)?;
        let min_mem_type_42 = get_u32(bytes, 16, 20)?;
        let max_mem_type_42 = get_u32(bytes, 20, 24)?;
        let min_mem_type_1 = get_u32(bytes, 24, 28)?;
        let max_mem_type_1 = get_u32(bytes, 28, 32)?;
        let mut num_glyphs: Option<u16> = None;
        let mut glyph_name_index: Option<Vec<u16>> = None;
        let mut names: Option<Vec<String>> = None;
        if version == 0x00010000 {
            names = Some(MAC_STANDARD_NAMES.iter().map(|s| s.to_string()).collect());
        }
        else if version == 0x00020000 {
            let mut offset = 34;
            num_glyphs = Some(get_u16(bytes, 32, 34)?);
            glyph_name_index = Some(
                bytes.get(offset..offset + num_glyphs.unwrap() as usize * 2)
                    .ok_or(ErrorKind::UnexpectedEof)?
                    .chunks_exact(2)
                    .map(|ch| u16::from_be_bytes(ch[0..2].try_into().unwrap())).collect()
            );
            let max_idx = glyph_name_index.as_ref().unwrap().iter().max().unwrap();
            let mut extra_names: Vec<String> = Vec::with_capacity(*max_idx as usize - 257);
            if *max_idx >= 258 {
                offset += num_glyphs.unwrap() as usize * 2;
                for _ in 0..(max_idx - 257) {
                    let length = bytes.get(offset).ok_or(ErrorKind::UnexpectedEof)?;
                    let string_bytes = bytes.get(offset + 1..offset + *length as usize).ok_or(ErrorKind::UnexpectedEof)?;
                    extra_names.push(decode_name_bytes(string_bytes, 1, 0)?);
                }
            }
            names = Some(Vec::with_capacity(num_glyphs.unwrap() as usize));
            for idx in glyph_name_index.as_ref().unwrap() {
                if *idx <= 257 {
                    names.as_mut().unwrap().push(MAC_STANDARD_NAMES[*idx as usize].to_string());
                }
                else {
                    names.as_mut().unwrap().push(extra_names[*idx as usize].to_string());
                }
            }
        }
        else if version == 0x00025000 {
            return Err(Error::new(ErrorKind::InvalidData, "Version 2.5 is deprecated"))
        }
        else if version != 0x00030000 {
            return Err(Error::new(ErrorKind::InvalidData, format!("Version {} is not valid", version)))
        }
        
        Ok(PostTable {
            version,
            italic_angle,
            underline_position,
            underline_thickness,
            is_fixed_pitch,
            min_mem_type_42,
            max_mem_type_42,
            min_mem_type_1,
            max_mem_type_1,
            num_glyphs,
            glyph_name_index,
            names
        })
    }
    
    pub fn parse_vhea(&self) -> Result<VheaTable, Error> {
        let vhea_bytes = self.get_table(b"vhea")?;
        
        let version = get_u32(vhea_bytes, 0, 4)?;
        if version != 0x00010000 && version != 0x00011000 {
            return Err(Error::new(ErrorKind::InvalidData, format!("Invalid version number: {}", version)))
        }
        let vert_typo_ascender = get_i16(vhea_bytes, 4, 6)?;
        let vert_typo_descender = get_i16(vhea_bytes, 6, 8)?;
        let vert_typo_line_gap = get_i16(vhea_bytes, 8, 10)?;
        let advance_height_max = get_u16(vhea_bytes, 10, 12)?;
        let min_top_side_bearing = get_i16(vhea_bytes, 12, 14)?;
        let min_bottom_side_bearing = get_i16(vhea_bytes, 14, 16)?;
        let y_max_extent = get_i16(vhea_bytes, 16, 18)?;
        let caret_slope_rise = get_i16(vhea_bytes, 18, 20)?;
        let caret_slope_run = get_i16(vhea_bytes, 20, 22)?;
        let caret_offset = get_i16(vhea_bytes, 22, 24)?;
        let _reserved1 = get_i16(vhea_bytes, 24, 26)?;
        let _reserved2 = get_i16(vhea_bytes, 26, 28)?;
        let _reserved3 = get_i16(vhea_bytes, 28, 30)?;
        let _reserved4 = get_i16(vhea_bytes, 30, 32)?;
        let metric_data_format = get_i16(vhea_bytes, 32, 34)?;
        let num_of_long_ver_metrics = get_u16(vhea_bytes, 34, 36)?;
        
        Ok(VheaTable {
            version,
            vert_typo_ascender,
            vert_typo_descender,
            vert_typo_line_gap,
            advance_height_max,
            min_top_side_bearing,
            min_bottom_side_bearing,
            y_max_extent,
            caret_slope_rise,
            caret_slope_run,
            caret_offset,
            _reserved1,
            _reserved2,
            _reserved3,
            _reserved4,
            metric_data_format,
            num_of_long_ver_metrics
        })
    }
    
    pub fn parse_vmtx(&self, num_glyphs: u16, number_of_v_metrics: u16) -> Result<VmtxTable, Error> {
        let bytes = self.get_table(b"vmtx")?;
        
        let mut entries: Vec<VmtxEntry> = bytes
            .get(0..number_of_v_metrics as usize * 4)
            .ok_or(ErrorKind::UnexpectedEof)?
            .chunks_exact(4)
            .map(|ch| {
                let advance_height = u16::from_be_bytes(ch[0..2].try_into().unwrap());
                let tsb = i16::from_be_bytes(ch[2..4].try_into().unwrap());
                
                VmtxEntry::FullMetric { advance_height, tsb }
            }).collect();
        
        let shared_advance_var = entries.last().unwrap();
        let shared_advance_height = match shared_advance_var {
            VmtxEntry::FullMetric { advance_height, tsb: _ } => *advance_height,
            _ => unreachable!()
        };
        
        let leftovers = num_glyphs - number_of_v_metrics;
        
        entries.extend(
            bytes
                .get(number_of_v_metrics as usize * 4..(number_of_v_metrics as usize * 4) + (leftovers as usize * 2))
                .ok_or(ErrorKind::UnexpectedEof)?
                .chunks_exact(2)
                .map(|ch| {
                    VmtxEntry::LeftoverBearing(i16::from_be_bytes(ch.try_into().unwrap()))
                })
        );
        
        Ok(VmtxTable { entries, shared_advance_height })
    }
    
    pub fn parse_kern(&self) -> Result<KernTable, Error> {
        let bytes = self.get_table(b"kern")?;
        
        let version_test = get_u16(bytes, 0, 2)?;
        match version_test {
            0 => {
                let version = get_u16(bytes, 0, 2)?;
                let n_tables = get_u16(bytes, 2, 4)?;
                let mut subtables: Vec<WindowsSubtable> = Vec::with_capacity(n_tables as usize);
                let mut offset: usize = 4;
                let mut subtable_start: usize;
                for _ in 0..n_tables {
                    subtable_start = offset;
                    let version = get_u16(bytes, offset, offset + 2)?;
                    let length = get_u16(bytes, offset + 2, offset + 4)?;
                    let coverage = get_u16(bytes, offset + 4, offset + 6)?;
                    offset += 6;
                    let subtable = match coverage >> 8 {
                        0 => parse_kern_format0(&bytes[offset..]),
                        2 => parse_kern_format2(&bytes[offset..], subtable_start, length as usize),
                        _ => {
                            offset += length as usize - 6;
                            continue;
                        }
                    }?;
                    
                    subtables.push(WindowsSubtable {
                        version,
                        length,
                        coverage,
                        subtable
                    });
                }
                
                Ok(KernTable::Windows {
                    version,
                    n_tables,
                    subtables
                })
            }
            1 => {
                let version = get_u32(bytes, 0, 4)?;
                let n_tables = get_u32(bytes, 4, 8)?;
                let mut offset: usize = 8;
                let mut subtable_start: usize;
                let mut subtables: Vec<MacSubtable> = Vec::with_capacity(n_tables as usize);
                for _ in 0..n_tables {
                    subtable_start = offset;
                    let length = get_u32(bytes, offset, offset + 4)?;
                    let coverage = get_u16(bytes, offset + 4, offset + 6)?;
                    let tuple_index = get_u16(bytes, offset + 6, offset + 8)?;
                    offset += 8;
                    let subtable = match coverage & 0xFF {
                        0 => parse_kern_format0(&bytes[offset..]),
                        2 => parse_kern_format2(&bytes[offset..], subtable_start, length as usize),
                        _ => {
                            offset += length as usize - 8;
                            continue;
                        }
                    }?;
                    
                    subtables.push(MacSubtable {
                        length,
                        coverage,
                        tuple_index,
                        subtable
                    });
                }
                
                Ok(KernTable::Mac {
                    version,
                    n_tables,
                    subtables
                })
            }
            _ => Err(Error::new(ErrorKind::InvalidData, format!("Invalid version number: {}", version_test)))
        }
    }
    
    pub fn parse_gasp(&self) -> Result<GaspTable, Error> {
        let bytes = self.get_table(b"gasp")?;
        
        let version = get_u16(bytes, 0, 2)?;
        if version != 0 && version != 1 {
            return Err(Error::new(ErrorKind::InvalidData, format!("Version number is not 0 or 1: {}", version)))
        }
        let num_ranges = get_u16(bytes, 2, 4)?;
        let range_records: Vec<RangeRecord> = bytes.get(4..4 + num_ranges as usize * 4)
            .ok_or(ErrorKind::UnexpectedEof)?
            .chunks_exact(4)
            .map(|ch| {
                let range_max_ppem = get_u16(ch, 0, 2)?;
                let range_gasp_behavior = get_u16(ch, 2, 4)?;
                
                Ok(RangeRecord { range_max_ppem, range_gasp_behavior })
            }).collect::<Result<Vec<_>, Error>>()?;
        
        Ok(GaspTable { version, num_ranges, range_records })
    }
    
    pub fn parse_cvt(&self) -> Result<Vec<i16>, Error> {
        let cvt_bytes = self.get_table(b"cvt ")?;
        
        Ok(
            cvt_bytes.chunks_exact(2)
                .map(|ch| {
                    i16::from_be_bytes(ch[0..2].try_into().unwrap())
                }).collect()
        )
    }
    
    pub fn parse_fpgm(&self) -> Result<Vec<u8>, Error> {
        Ok(self.get_table(b"fpgm")?.to_vec())
    }
    
    pub fn parse_prep(&self) -> Result<Vec<u8>, Error> {
        Ok(self.get_table(b"prep")?.to_vec())
    }
}

fn parse_kern_format0(bytes: &[u8]) -> Result<KernSubtable, Error> {
    let n_pairs = get_u16(bytes, 0, 2)?;
    let search_range = get_u16(bytes, 2, 4)?;
    let entry_selector = get_u16(bytes, 4, 6)?;
    let range_shift = get_u16(bytes, 6, 8)?;
    let pairs: Vec<KernPair> = bytes.get(8..8 + n_pairs as usize * 6)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(6)
        .map(|ch| {
            let left = get_u16(ch, 0, 2)?;
            let right = get_u16(ch, 2, 4)?;
            let value = get_i16(ch, 4, 6)?;
            
            Ok(KernPair { left, right, value })
        }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(KernSubtable::Format0 {
        n_pairs,
        search_range,
        entry_selector,
        range_shift,
        pairs
    })
}

fn parse_kern_format2(bytes: &[u8], subtable_start: usize, length: usize) -> Result<KernSubtable, Error> {
    let mut offset = 0;
    let row_width = get_u16(bytes, offset, offset + 2)?;
    let left_offset = get_u16(bytes, offset + 2, offset + 4)?;
    let right_offset = get_u16(bytes, offset + 4, offset + 6)?;
    let array_offset = get_u16(bytes, offset + 6, offset + 8)?;
    offset = subtable_start + left_offset as usize;
    let left_class_format = get_u16(bytes, offset, offset + 2)?;
    let left_class_table = parse_kern_class(bytes, left_class_format, offset)?;
    offset = subtable_start + right_offset as usize;
    let right_class_format = get_u16(bytes, offset, offset + 2)?;
    let right_class_table = parse_kern_class(bytes, right_class_format, offset)?;
    offset = subtable_start + array_offset as usize;
    let kerning_array: Vec<i16> = bytes.get(offset..offset + length)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| Ok(get_i16(ch, 0, 2)?))
        .collect::<Result<Vec<_>, Error>>()?;
    
    Ok(KernSubtable::Format2 {
        row_width,
        left_offset,
        right_offset,
        array_offset,
        left_class_table,
        right_class_table,
        kerning_array
    })
}

fn parse_kern_class(bytes: &[u8], class_format: u16, offset: usize) -> Result<KernClassTable, Error> {
    let mut offset = offset;
    match class_format {
        1 => {
            let start_glyph = get_u16(bytes, offset + 2, offset + 4)?;
            let glyph_count = get_u16(bytes, offset + 4, offset + 6)?;
            offset += 6;
            let class_ids: Vec<u16> = bytes.get(offset..offset + glyph_count as usize * 2)
                .ok_or(ErrorKind::UnexpectedEof)?
                .chunks_exact(2)
                .map(|ch| Ok(get_u16(ch, 0, 2)?))
                .collect::<Result<Vec<_>, Error>>()?;
            
            Ok(KernClassTable::Format1 { start_glyph, glyph_count, class_ids })
        }
        2 => {
            let range_count = get_u16(bytes, offset, offset + 2)?;
            offset += 2;
            let ranges: Vec<Range> = bytes.get(offset..offset + range_count as usize * 6)
                .ok_or(ErrorKind::UnexpectedEof)?
                .chunks_exact(6)
                .map(|ch| {
                    let start_glyph = get_u16(bytes, 0, 2)?;
                    let end_glyph = get_u16(bytes, 2, 4)?;
                    let class = get_u16(bytes, 4, 6)?;
                    
                    Ok(Range { start_glyph, end_glyph, class })
                }).collect::<Result<Vec<_>, Error>>()?;
            
            Ok(KernClassTable::Format2 { range_count, ranges })
        }
        _ => Err(Error::new(ErrorKind::InvalidData, format!("Class table format invalid: {}", class_format)))
    }
}

fn get_u16(bytes: &[u8], start: usize, end: usize) -> Result<u16, Error> {
    Ok(
        u16::from_be_bytes(
        bytes.get(start..end)
            .ok_or(ErrorKind::UnexpectedEof)?
            .try_into()
            .unwrap()
        )
    )
}

fn get_u32(bytes: &[u8], start: usize, end: usize) -> Result<u32, Error> {
    Ok(
        u32::from_be_bytes(
            bytes.get(start..end)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        )
    )
}

fn get_u64(bytes: &[u8], start: usize, end: usize) -> Result<u64, Error> {
    Ok(
        u64::from_be_bytes(
            bytes.get(start..end)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        )
    )
}

fn get_i16(bytes: &[u8], start: usize, end: usize) -> Result<i16, Error> {
    Ok(
        i16::from_be_bytes(
            bytes.get(start..end)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        )
    )
}

fn get_i32(bytes: &[u8], start: usize, end: usize) -> Result<i32, Error> {
    Ok(
        i32::from_be_bytes(
            bytes.get(start..end)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        )
    )
}

fn get_i64(bytes: &[u8], start: usize, end: usize) -> Result<i64, Error> {
    Ok(
        i64::from_be_bytes(
            bytes.get(start..end)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        )
    )
}

// Add support for more once I create HTTP/HTTPS part of spyne
fn decode_name_bytes(bytes: &[u8], platform_id: u16, encoding_id: u16) -> Result<String, Error> {
    match platform_id {
        0 => {
            let name_bytes: Vec<u16> = bytes.chunks_exact(2)
                .map(|ch| u16::from_be_bytes(ch[0..2].try_into().unwrap()))
                .collect();
            
            Ok(String::from_utf16(&name_bytes).map_err(|e| Error::new(ErrorKind::InvalidData, e))?)
        }
        1 => {
            match encoding_id {
                0 => {
                    Ok(
                        bytes.iter()
                            .map(|idx| MAC_ROMAN_LOOKUP[*idx as usize])
                            .collect::<String>()
                    )
                }
                _ => Err(Error::new(ErrorKind::Unsupported, "Only encoding 0 (Mac Roman) currently supported for platform 1"))
            }
        }
        3 => {
            match encoding_id {
                0 | 1 | 10 => {
                    let name_bytes: Vec<u16> = bytes.chunks_exact(2)
                        .map(|ch| u16::from_be_bytes(ch[0..2].try_into().unwrap()))
                        .collect();
                    
                    Ok(String::from_utf16(&name_bytes).map_err(|e| Error::new(ErrorKind::InvalidData, e))?)
                }
                _ => Err(Error::new(ErrorKind::Unsupported, "Platform 3, Encodings 2 - 9 decoding currently unsupported"))
            }
        }
        _ => Err(Error::new(ErrorKind::Unsupported, "Platform 2 and 4 decoding currently unsupported"))
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
    entries: Vec<HmtxEntry>,
    shared_advance_width: u16
}

pub enum HmtxEntry {
    FullMetric {
        advance_width: u16,
        lsb: i16
    },
    LeftoverBearing(i16)
}

struct NameTable {
    pub version: u16,
    pub count: u16,
    pub storage_offset: u16,
    pub records: Vec<NameRecord>,
    pub lang_tag_count: Option<u16>,
    pub lang_tag_records: Option<Vec<LangTagRecord>>
}

struct NameRecord {
    pub platform_id: u16,
    pub encoding_id: u16,
    pub language_id: u16,
    pub name_id: u16,
    pub length: u16,
    pub string_offset: u16,
    pub string: String
}

struct LangTagRecord {
    pub length: u16,
    pub lang_tag_offset: u16,
    pub string: String
}

struct OS2Table {
    pub version: u16,
    pub x_avg_char_width: i16,
    pub us_weight_class: u16,
    pub us_width_class: u16,
    pub fs_type: u16,
    pub y_subscript_x_size: i16,
    pub y_subscript_y_size: i16,
    pub y_subscript_x_offset: i16,
    pub y_subscript_y_offset: i16,
    pub y_superscript_x_size: i16,
    pub y_superscript_y_size: i16,
    pub y_superscript_x_offset: i16,
    pub y_superscript_y_offset: i16,
    pub y_strikeout_size: i16,
    pub y_strikeout_position: i16,
    pub s_family_class: i16,
    pub panose: [u8; 10],
    pub ul_unicode_range_1: u32,
    pub ul_unicode_range_2: u32,
    pub ul_unicode_range_3: u32,
    pub ul_unicode_range_4: u32,
    pub ach_vend_id: [u8; 4],
    pub fs_selection: u16,
    pub us_first_char_index: u16,
    pub us_last_char_index: u16,
    pub s_typo_ascender: i16,
    pub s_typo_descender: i16,
    pub s_typo_line_gap: i16,
    pub us_win_ascent: u16,
    pub us_win_descent: u16,
    // Version 1 Additions
    pub ul_code_page_range_1: Option<u32>,
    pub ul_code_page_range_2: Option<u32>,
    // Version 2 Additions
    pub sx_height: Option<i16>,
    pub s_cap_height: Option<i16>,
    pub us_default_char: Option<u16>,
    pub us_break_char: Option<u16>,
    pub us_max_context: Option<u16>,
    // Version 5 Additions
    pub us_lower_optical_point_size: Option<u16>,
    pub us_upper_optical_point_size: Option<u16>
}

struct PostTable {
    pub version: u32,
    pub italic_angle: i32,
    pub underline_position: i16,
    pub underline_thickness: i16,
    pub is_fixed_pitch: u32,
    pub min_mem_type_42: u32,
    pub max_mem_type_42: u32,
    pub min_mem_type_1: u32,
    pub max_mem_type_1: u32,
    pub num_glyphs: Option<u16>,
    pub glyph_name_index: Option<Vec<u16>>,
    pub names: Option<Vec<String>>
}

struct VheaTable {
    pub version: u32,
    pub vert_typo_ascender: i16,
    pub vert_typo_descender: i16,
    pub vert_typo_line_gap: i16,
    pub advance_height_max: u16,
    pub min_top_side_bearing: i16,
    pub min_bottom_side_bearing: i16,
    pub y_max_extent: i16,
    pub caret_slope_rise: i16,
    pub caret_slope_run: i16,
    pub caret_offset: i16,
    _reserved1: i16,
    _reserved2: i16,
    _reserved3: i16,
    _reserved4: i16,
    pub metric_data_format: i16,
    pub num_of_long_ver_metrics: u16
}

struct VmtxTable {
    entries: Vec<VmtxEntry>,
    shared_advance_height: u16
}

pub enum VmtxEntry {
    FullMetric {
        advance_height: u16,
        tsb: i16
    },
    LeftoverBearing(i16)
}

pub enum KernTable {
    Windows {
        version: u16,
        n_tables: u16,
        subtables: Vec<WindowsSubtable>,
    },
    Mac {
        version: u32,
        n_tables: u32,
        subtables: Vec<MacSubtable>,
    }
}

struct WindowsSubtable {
    version: u16,
    length: u16,
    coverage: u16,
    subtable: KernSubtable
}

struct MacSubtable {
    length: u32,
    coverage: u16,
    tuple_index: u16,
    subtable: KernSubtable
}

pub enum KernSubtable {
    Format0 {
        n_pairs: u16,
        search_range: u16,
        entry_selector: u16,
        range_shift: u16,
        pairs: Vec<KernPair>
    },
    Format2 {
        row_width: u16,
        left_offset: u16,
        right_offset: u16,
        array_offset: u16,
        left_class_table: KernClassTable,
        right_class_table: KernClassTable,
        kerning_array: Vec<i16>
    }
}

struct KernPair {
    pub left: u16,
    pub right: u16,
    pub value: i16
}

pub enum KernClassTable {
    Format1 {
        start_glyph: u16,
        glyph_count: u16,
        class_ids: Vec<u16>
    },
    Format2 {
        range_count: u16,
        ranges: Vec<Range>
    }
}

struct Range {
    pub start_glyph: u16,
    pub end_glyph: u16,
    pub class: u16
}

struct GaspTable {
    pub version: u16,
    pub num_ranges: u16,
    pub range_records: Vec<RangeRecord>
}

struct RangeRecord {
    pub range_max_ppem: u16,
    pub range_gasp_behavior: u16
}