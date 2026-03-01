use std::{fs::read, io::{Error, ErrorKind}, path::Path};

use crate::text::fonts::parse::{constants::{ARG_1_AND_2_ARE_WORDS, MAC_ROMAN_LOOKUP, MAC_STANDARD_NAMES, MORE_COMPONENTS, REPEAT_FLAG, USE_MARK_FILTERING_SET, WE_HAVE_A_SCALE, WE_HAVE_A_TWO_BY_TWO, WE_HAVE_AN_X_AND_Y_SCALE, WE_HAVE_INSTRUCTIONS, X_ADVANCE, X_ADVANCE_DEVICE, X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR, X_PLACEMENT, X_PLACEMENT_DEVICE, X_SHORT_VECTOR, Y_ADVANCE, Y_ADVANCE_DEVICE, Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR, Y_PLACEMENT, Y_PLACEMENT_DEVICE, Y_SHORT_VECTOR}, structures::{AlternateSet, Anchor, BaseArray, BaseRecord, Class1Record, Class2Record, ClassDef, ClassRangeRecord, CmapSubtable, CmapTable, Component, ComponentRecord, Condition, ConditionSet, Coverage, CoverageRangeRecord, Device, DeviceOrVariationIndex, EncodingRecord, EntryExitRecord, Feature, FeatureList, FeatureParams, FeatureRecord, FeatureTableSubstitution, FeatureTableSubstitutionRecord, FeatureVariationRecord, FeatureVariations, FontFile, FontFileType, GaspRangeRecord, GaspTable, Glyph, GlyphHeader, GposChainSubClassRule, GposChainSubClassSet, GposChainSubRule, GposChainSubRuleSet, GposSubClassRule, GposSubClassSet, GposSubRule, GposSubRuleSet, GposSubtable, GposTable, GposType1Format, GposType2Format, GposType3Format, GposType4Format, GposType5Format, GposType6Format, GposType7Format, GposType8Format, GposType9Format, Group, GsubChainSubClassRule, GsubChainSubClassSet, GsubChainSubRule, GsubChainSubRuleSet, GsubSubClassRule, GsubSubClassSet, GsubSubRule, GsubSubRuleSet, GsubSubtable, GsubTable, GsubType1Format, GsubType2Format, GsubType3Format, GsubType4Format, GsubType5Format, GsubType6Format, GsubType7Format, GsubType8Format, HeadTable, HheaTable, HmtxEntry, HmtxTable, KernClassTable, KernPair, KernSubtable, KernTable, LangSys, LangSysRecord, LangTagRecord, Ligature, LigatureArray, LigatureAttach, LigatureSet, Lookup, LookupList, MacSubtable, Mark2Array, Mark2Record, MarkArray, MarkRecord, MaxpTable, NameRecord, NameTable, OS2Table, PairSet, PairValueRecord, PosLookupRecord, PostTable, Range, Script, ScriptList, ScriptRecord, Sequence, SubHeader, SubstLookupRecord, TableHeader, TableRecord, ValueRecord, VariationIndex, VariationSelectorRecord, VheaTable, VmtxEntry, VmtxTable, WindowsSubtable}};

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
            
            table_records.push(TableRecord::new(tag, checksum, offset, length));
            current_offset += 16;
        }
        
        table_records.sort_by_key(|rec| rec.tag());
        
        Ok(FontFile::new(file_type, bytes, table_records))
    }
    
    pub fn get_table(&self, tag: &[u8; 4]) -> Result<&[u8], Error> {
        match self.table_records().binary_search_by_key(tag, |rec| rec.tag()) {
            Ok(idx) => {
                let table = self.table_records()[idx];
                let offset = table.offset() as usize;
                let length = table.length() as usize;
                
                Ok(&self.bytes().get(offset..offset + length).ok_or(ErrorKind::UnexpectedEof)?)
            },
            Err(_) => Err(Error::new(ErrorKind::NotFound, "Given tag was not found in table records"))
        }
    }
    
    pub fn parse_head(&self) -> Result<HeadTable, Error> {
        let bytes = self.get_table(b"head")?;
        
        if bytes[12..16] != [0x5F, 0x0F, 0x3C, 0xF5] {
            return Err(Error::new(ErrorKind::InvalidData, "Head table doesn't contain correct magic number"));
        }
        
        let units_per_em = get_u16(bytes, 18)?;
        let created = get_i64(bytes, 20)?;
        let modified = get_i64(bytes, 28)?;
        let x_min = get_i16(bytes, 36)?; 
        let y_min = get_i16(bytes, 38)?;
        let x_max = get_i16(bytes, 40)?;
        let y_max = get_i16(bytes, 42)?;
        let mac_style = get_u16(bytes, 44)?;
        let lowest_rec_ppem = get_u16(bytes, 46)?;
        let font_direction_hint = get_i16(bytes, 48)?;
        let index_to_loc_format = get_i16(bytes, 50)?;
        
        Ok(HeadTable::new(
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
        ))
    }
    
    pub fn parse_maxp(&self) -> Result<MaxpTable, Error> {
        let bytes = self.get_table(b"maxp")?;
        
        let version = get_u32(bytes, 0)?;
        match version {
            0x5000 => {
                let num_glyphs = get_u16(bytes, 4)?;
                let max_points = None;
                let max_contours = None;
                let max_composite_points = None;
                let max_composite_contours = None;
                let max_zones = None;
                let max_twilight_points = None;
                let max_storage = None;
                let max_function_defs = None;
                let max_instruction_defs = None;
                let max_stack_elements = None;
                let max_size_of_instructions = None;
                let max_components_elements = None;
                let max_component_depth = None;
                
                Ok(MaxpTable::new(
                    version,
                    num_glyphs,
                    max_points,
                    max_contours,
                    max_composite_points,
                    max_composite_contours,
                    max_zones,
                    max_twilight_points,
                    max_storage,
                    max_function_defs,
                    max_instruction_defs,
                    max_stack_elements,
                    max_size_of_instructions,
                    max_components_elements,
                    max_component_depth
                ))
            }
            0x10000 => {
                let num_glyphs = get_u16(bytes, 4)?;
                let max_points = get_u16(bytes, 6)?;
                let max_contours = get_u16(bytes, 8)?;
                let max_composite_points = get_u16(bytes, 10)?;
                let max_composite_contours = get_u16(bytes, 12)?;
                let max_zones = get_u16(bytes, 14)?;
                let max_twilight_points = get_u16(bytes, 16)?;
                let max_storage = get_u16(bytes, 18)?;
                let max_function_defs = get_u16(bytes, 20)?;
                let max_instruction_defs = get_u16(bytes, 22)?;
                let max_stack_elements = get_u16(bytes, 24)?;
                let max_size_of_instructions = get_u16(bytes, 26)?;
                let max_component_elements = get_u16(bytes, 28)?;
                let max_component_depth = get_u16(bytes, 30)?;
                
                Ok(MaxpTable::new(
                    version,
                    num_glyphs,
                    Some(max_points),
                    Some(max_contours),
                    Some(max_composite_points),
                    Some(max_composite_contours),
                    Some(max_zones),
                    Some(max_twilight_points),
                    Some(max_storage),
                    Some(max_function_defs),
                    Some(max_instruction_defs),
                    Some(max_stack_elements),
                    Some(max_size_of_instructions),
                    Some(max_component_elements),
                    Some(max_component_depth)
                ))
            }
            _ => Err(Error::new(ErrorKind::InvalidData, "Version number invalid"))
        }
    }
    
    pub fn parse_cmap(&self) -> Result<CmapTable, Error> {
        let bytes = self.get_table(b"cmap")?;
        
        let version = get_u16(bytes, 0)?;
        if version != 0 {
            return Err(Error::new(ErrorKind::InvalidData, "Version number isn't zero"));
        }
        let num_tables = get_u16(bytes, 2)?;
        let mut encoding_records: Vec<EncodingRecord> = Vec::new();
        let mut count = 4;
        for _ in 0..num_tables {
            let platform_id = get_u16(bytes, count)?;
            let encoding_id = get_u16(bytes, count + 2)?;
            let offset = get_u32(bytes, count + 4)?;
            encoding_records.push(EncodingRecord::new(platform_id, encoding_id, offset));
            count += 8;
        }
        
        let mut subtables: Vec<CmapSubtable> = Vec::new();
        for rec in encoding_records.iter() {
            let mut offset = rec.offset() as usize;
            let format = get_u16(bytes, offset)?;
            offset += 2;
            match format {
                0 => {
                    let length = get_u16(bytes, offset)?;
                    let language = get_u16(bytes, offset + 2)?;
                    let glyph_id_array: [u8; 256] = bytes.get(offset + 4..offset + 260).ok_or(ErrorKind::UnexpectedEof)?.try_into().unwrap();
                    
                    subtables.push(CmapSubtable::Format0 { length, language, glyph_id_array });
                }
                2 => {
                    let length = get_u16(bytes, offset)?;
                    let language = get_u16(bytes, offset + 2)?;
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
                        
                        SubHeader::new(first_code, entry_count, id_delta, id_range_offset)
                    }).collect();
                    offset += sub_headers_num as usize * 8;
                    let glyph_id_array: Vec<u16> = bytes.get(offset..).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        u16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    
                    subtables.push(CmapSubtable::Format2 { length, language, sub_header_keys, sub_headers, glyph_id_array });
                }
                4 => {
                    let length = get_u16(bytes, offset)?;
                    let language = get_u16(bytes, offset + 2)?;
                    let seg_count_x2 = get_u16(bytes, offset + 4)?;
                    let search_range = get_u16(bytes, offset + 6)?;
                    let entry_selector = get_u16(bytes, offset + 8)?;
                    let range_shift = get_u16(bytes, offset + 10)?;
                    offset += 12;
                    let end_code: Vec<u16> = bytes.get(offset..offset + seg_count_x2 as usize).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        u16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    offset += seg_count_x2 as usize;
                    let _reserved_pad = get_u16(bytes, offset)?;
                    offset += 2;
                    let start_code: Vec<u16> = bytes.get(offset..offset + seg_count_x2 as usize).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        u16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    offset += seg_count_x2 as usize;
                    let id_delta: Vec<i16> = bytes.get(offset..offset + seg_count_x2 as usize).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        i16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    offset += seg_count_x2 as usize;
                    let id_range_offset: Vec<u16> = bytes.get(offset..offset + seg_count_x2 as usize).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        u16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    offset += seg_count_x2 as usize;
                    let glyph_id_array: Vec<u16> = bytes.get(offset..).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        u16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    
                    subtables.push(CmapSubtable::Format4 { length, language, seg_count_x2, search_range, entry_selector, range_shift, end_code, _reserved_pad, start_code, id_delta, id_range_offset, glyph_id_array });
                }
                6 => {
                    let length = get_u16(bytes, offset)?;
                    let language = get_u16(bytes, offset + 2)?;
                    let first_code = get_u16(bytes, offset + 4)?;
                    let entry_count = get_u16(bytes, offset + 6)?;
                    offset += 8;
                    let glyph_id_array: Vec<u16> = bytes.get(offset..offset + entry_count as usize * 2).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(2).map(|ch| {
                        u16::from_be_bytes(ch.try_into().unwrap())
                    }).collect();
                    
                    subtables.push(CmapSubtable::Format6 { length, language, first_code, entry_count, glyph_id_array });
                }
                12 => {
                    let _reserved = get_u16(bytes, offset)?;
                    let length = get_u32(bytes, offset + 2)?;
                    let language = get_u32(bytes, offset + 6)?;
                    let num_groups = get_u32(bytes, offset + 10)?;
                    let groups: Vec<Group> = bytes.get(offset + 14..).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(12).map(|ch| {
                        let start_char_code = u32::from_be_bytes(ch[0..4].try_into().unwrap());
                        let end_char_code = u32::from_be_bytes(ch[4..8].try_into().unwrap());
                        let start_glyph_id = u32::from_be_bytes(ch[8..].try_into().unwrap());
                        
                        Group::new(start_char_code, end_char_code, start_glyph_id)
                    }).collect();
                    
                    subtables.push(CmapSubtable::Format12 { _reserved, length, language, num_groups, groups });
                }
                13 => {
                    let _reserved = get_u16(bytes, offset)?;
                    let length = get_u32(bytes, offset + 2)?;
                    let language = get_u32(bytes, offset + 6)?;
                    let num_groups = get_u32(bytes, offset + 10)?;
                    let groups: Vec<Group> = bytes.get(offset + 14..).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(12).map(|ch| {
                        let start_char_code = u32::from_be_bytes(ch[0..4].try_into().unwrap());
                        let end_char_code = u32::from_be_bytes(ch[4..8].try_into().unwrap());
                        let start_glyph_id = u32::from_be_bytes(ch[8..].try_into().unwrap());
                        
                        Group::new(start_char_code, end_char_code, start_glyph_id)
                    }).collect();
                    
                    subtables.push(CmapSubtable::Format13 { _reserved, length, language, num_groups, groups });
                }
                14 => {
                    let length = get_u32(bytes, offset)?;
                    let num_var_selector_records = get_u32(bytes, offset + 4)?;
                    let var_selector: Vec<VariationSelectorRecord> = bytes.get(offset + 8..).ok_or(ErrorKind::UnexpectedEof)?.chunks_exact(11).map(|ch| {
                        let var_selector: [u8; 3] = ch[0..3].try_into().unwrap();
                        let default_uvs_offset = u32::from_be_bytes(ch[3..7].try_into().unwrap());
                        let non_default_uvs_offset = u32::from_be_bytes(ch[7..11].try_into().unwrap());
                        
                        VariationSelectorRecord::new(var_selector, default_uvs_offset, non_default_uvs_offset)
                    }).collect();
                    
                    subtables.push(CmapSubtable::Format14 { length, num_var_selector_records, var_selector });
                }
                8 | 10 => return Err(Error::new(ErrorKind::InvalidData, "Subtable formats 8 and 10 are considered deprecated")),
                _ => return Err(Error::new(ErrorKind::InvalidData, ""))
            }
        }
        
        Ok(CmapTable::new(version, num_tables, encoding_records, subtables))
    }
    
    pub fn parse_loca(&self, num_glyphs: u16, index_to_loc_format: i16) -> Result<Vec<u32>, Error> {
        let bytes = self.get_table(b"loca")?;
        
        let mut indices: Vec<u32> = Vec::new();
        match index_to_loc_format {
            0 => {
                for i in (0..(num_glyphs as usize + 1) * 2).step_by(2) {
                    let offset = get_u16(bytes, i)?;
                    indices.push((offset * 2) as u32);
                }
            },
            1 => {
                for i in (0..(num_glyphs as usize + 1) * 4).step_by(4) {
                    let offset = get_u32(bytes, i)?;
                    indices.push(offset);
                }
            },
            _ => return Err(Error::new(ErrorKind::InvalidInput, "index_to_loc_format was not 0 or 1"))
        }
        
        Ok(indices)
    }
    
    pub fn parse_glyf(&self, loca_offsets: Vec<u32>) -> Result<Vec<Option<Glyph>>, Error> {
        let bytes = self.get_table(b"glyf")?;
        let mut loca_iter = loca_offsets.iter().peekable();
        
        // Could optimize by initializing with capacity based on offsets
        let mut glyphs: Vec<Option<Glyph>> = Vec::new();
        
        while let Some(offset) = loca_iter.next() {
            if let Some(next_offset) = loca_iter.peek() {
                if *next_offset - offset == 0 {
                    glyphs.push(None);
                    continue;
                }
                
                let current_glyph_bytes = bytes.get(*offset as usize..**next_offset as usize).ok_or(ErrorKind::UnexpectedEof)?;
                let mut current_offset: usize = 0;
                let number_of_contours = get_i16(current_glyph_bytes, 0)?;
                let x_min = get_i16(current_glyph_bytes, 2)?;
                let y_min = get_i16(current_glyph_bytes, 4)?;
                let x_max = get_i16(current_glyph_bytes, 6)?;
                let y_max = get_i16(current_glyph_bytes, 8)?;
                current_offset += 10;
                let header = GlyphHeader::new(number_of_contours, x_min, y_min, x_max, y_max);
                let mut are_instructions;
                if number_of_contours > 0 {
                    let end_pts_of_contours: Vec<u16> = current_glyph_bytes
                        .get(current_offset..current_offset + number_of_contours as usize * 2)
                        .ok_or(ErrorKind::UnexpectedEof)?
                        .chunks_exact(2)
                        .map(|ch| {
                            u16::from_be_bytes(ch.try_into().unwrap())
                        }).collect();
                    current_offset += number_of_contours as usize * 2;
                    let instruction_length = get_u16(current_glyph_bytes, current_offset)?;
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
                                current_x += get_i16(current_glyph_bytes, current_offset)?;
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
                                current_y += get_i16(current_glyph_bytes, current_offset)?;
                                current_offset += 2;
                            }
                        }
                        
                        y_coordinates.push(current_y);
                    }
                    
                    glyphs.push(Some(Glyph::Simple { header, end_pts_of_contours, instruction_length, instructions, flags, x_coordinates, y_coordinates }));
                }
                else if number_of_contours == -1 {
                    let mut components: Vec<Component> = Vec::new();
                    loop {
                        let flags = get_u16(current_glyph_bytes, current_offset)?;
                        let glyph_index = get_u16(current_glyph_bytes, current_offset + 2)?;
                        current_offset += 4;
                        let argument_1: i16;
                        let argument_2: i16;
                        if flags & ARG_1_AND_2_ARE_WORDS != 0 {
                            argument_1 = get_i16(current_glyph_bytes, current_offset)?;
                            argument_2 = get_i16(current_glyph_bytes, current_offset + 2)?;
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
                            transformation = [get_i16(current_glyph_bytes, current_offset)?, 0, 0, 0];
                            current_offset += 2;
                        }
                        else if flags & WE_HAVE_AN_X_AND_Y_SCALE != 0 {
                            transformation = [
                                get_i16(current_glyph_bytes, current_offset)?,
                                get_i16(current_glyph_bytes, current_offset + 2)?,
                                0, 0
                            ];
                            current_offset += 4;
                        }
                        else if flags & WE_HAVE_A_TWO_BY_TWO != 0 {
                            transformation = [
                                get_i16(current_glyph_bytes, current_offset)?,
                                get_i16(current_glyph_bytes, current_offset + 2)?,
                                get_i16(current_glyph_bytes, current_offset + 4)?,
                                get_i16(current_glyph_bytes, current_offset + 6)?
                            ];
                            current_offset += 8;
                        }
                        else {
                            transformation = [0, 0, 0, 0];
                        }
                        
                        components.push(Component::new(flags, glyph_index, argument_1, argument_2, transformation));
                        
                        are_instructions = flags & WE_HAVE_INSTRUCTIONS != 0;
                        
                        if flags & MORE_COMPONENTS == 0 { break; }
                    }
                    let mut instruction_length: Option<u16> = None;
                    let mut instructions: Option<Vec<u8>> = None;
                    if are_instructions {
                        instruction_length = Some(get_u16(current_glyph_bytes, current_offset)?);
                        current_offset += 2;
                        instructions = Some(
                            current_glyph_bytes
                                .get(current_offset..current_offset + instruction_length.unwrap() as usize)
                                .ok_or(ErrorKind::UnexpectedEof)?
                                .to_vec()
                        )
                    }
                    
                    glyphs.push(Some(Glyph::Composite { header, components, instruction_length, instructions }));
                }
                else if number_of_contours == 0 {
                    glyphs.push(None);
                }
            }
        }
        
        Ok(glyphs)
    }
    
    pub fn parse_hhea(&self) -> Result<HheaTable, Error> {
        let bytes = self.get_table(b"hhea")?;
        
        let version = get_u32(bytes, 0)?;
        if version != 0x00010000 {
            return Err(Error::new(ErrorKind::InvalidData, "Version number for hhea table is incorrect"));
        }
        let ascender = get_i16(bytes, 4)?;
        let descender = get_i16(bytes, 6)?;
        let line_gap = get_i16(bytes, 8)?;
        let advance_width_max = get_u16(bytes, 10)?;
        let min_left_side_bearing = get_i16(bytes, 12)?;
        let min_right_side_bearing = get_i16(bytes, 14)?;
        let x_max_extent = get_i16(bytes, 16)?;
        let caret_slope_rise = get_i16(bytes, 18)?;
        let caret_slope_run = get_i16(bytes, 20)?;
        let caret_offset = get_i16(bytes, 22)?;
        let _reserved1 = 0;
        let _reserved2 = 0;
        let _reserved3 = 0;
        let _reserved4 = 0;
        let metric_data_format = get_i16(bytes, 32)?;
        let number_of_h_metrics = get_u16(bytes, 34)?;
        
        Ok(HheaTable::new(
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
        ))
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
        
        Ok(HmtxTable::new(entries, shared_advance_width))
    }
    
    pub fn parse_name(&self) -> Result<NameTable, Error> {
        let bytes = self.get_table(b"name")?;
        let mut offset = 0;
        
        let version = get_u16(bytes, 0)?;
        if version != 0 && version != 1 {
            return Err(Error::new(ErrorKind::InvalidData, "Version number is not 0 or 1"));
        }
        let count = get_u16(bytes, 2)?;
        let storage_offset = get_u16(bytes, 4)?;
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
                
                Ok(NameRecord::new(platform_id, encoding_id, language_id, name_id, length, string_offset, string))
            }).collect::<Result<Vec<_>, Error>>()?;
        offset += count as usize * 12;
        let mut lang_tag_count: Option<u16> = None;
        let mut lang_tag_records: Option<Vec<LangTagRecord>> = None;
        if version == 1 {
            lang_tag_count = Some(get_u16(bytes, offset)?);
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
                        
                        Ok(LangTagRecord::new(length, lang_tag_offset, string))
                    }).collect::<Result<Vec<_>, Error>>()?
            );
        }
        
        Ok(NameTable::new(version, count, storage_offset, records, lang_tag_count, lang_tag_records))
    }
    
    pub fn parse_os2(&self) -> Result<OS2Table, Error> {
        let bytes = self.get_table(b"OS/2")?;
        
        let version = get_u16(bytes, 0)?;
        let x_avg_char_width = get_i16(bytes, 2)?;
        let us_weight_class = get_u16(bytes, 4)?;
        let us_width_class = get_u16(bytes, 6)?;
        let fs_type = get_u16(bytes, 8)?;
        let y_subscript_x_size = get_i16(bytes, 10)?;
        let y_subscript_y_size = get_i16(bytes, 12)?;
        let y_subscript_x_offset = get_i16(bytes, 14)?;
        let y_subscript_y_offset = get_i16(bytes, 16)?;
        let y_superscript_x_size = get_i16(bytes, 18)?;
        let y_superscript_y_size = get_i16(bytes, 20)?;
        let y_superscript_x_offset = get_i16(bytes, 22)?;
        let y_superscript_y_offset = get_i16(bytes, 24)?;
        let y_strikeout_size = get_i16(bytes, 26)?;
        let y_strikeout_position = get_i16(bytes, 28)?;
        let s_family_class = get_i16(bytes, 30)?;
        let panose: [u8; 10] = bytes.get(32..42)
            .ok_or(ErrorKind::UnexpectedEof)?
            .try_into()
            .unwrap();
        let ul_unicode_range_1 = get_u32(bytes, 42)?;
        let ul_unicode_range_2 = get_u32(bytes, 46)?;
        let ul_unicode_range_3 = get_u32(bytes, 50)?;
        let ul_unicode_range_4 = get_u32(bytes, 54)?;
        let ach_vend_id: [u8; 4] = bytes.get(58..62)
            .ok_or(ErrorKind::UnexpectedEof)?
            .try_into()
            .unwrap();
        let fs_selection = get_u16(bytes, 62)?;
        let us_first_char_index = get_u16(bytes, 64)?;
        let us_last_char_index = get_u16(bytes, 66)?;
        let s_typo_ascender = get_i16(bytes, 68)?;
        let s_typo_descender = get_i16(bytes, 70)?;
        let s_typo_line_gap = get_i16(bytes, 72)?;
        let us_win_ascent = get_u16(bytes, 74)?;
        let us_win_descent = get_u16(bytes, 76)?;
        let mut ul_code_page_range_1: Option<u32> = None;
        let mut ul_code_page_range_2: Option<u32> = None;
        if version >= 1 {
            ul_code_page_range_1 = Some(get_u32(bytes, 78)?);
            ul_code_page_range_2 = Some(get_u32(bytes, 82)?);
        }
        let mut sx_height: Option<i16> = None;
        let mut s_cap_height: Option<i16> = None;
        let mut us_default_char: Option<u16> = None;
        let mut us_break_char: Option<u16> = None;
        let mut us_max_context: Option<u16> = None;
        if version >= 2 {
            sx_height = Some(get_i16(bytes, 86)?);
            s_cap_height = Some(get_i16(bytes, 88)?);
            us_default_char = Some(get_u16(bytes, 90)?);
            us_break_char = Some(get_u16(bytes, 92)?);
            us_max_context = Some(get_u16(bytes, 94)?);
        }
        let mut us_lower_optical_point_size: Option<u16> = None;
        let mut us_upper_optical_point_size: Option<u16> = None;
        if version >= 5 {
            us_lower_optical_point_size = Some(get_u16(bytes, 96)?);
            us_upper_optical_point_size = Some(get_u16(bytes, 98)?);
        }
        
        Ok(OS2Table::new(
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
        ))
    }
    
    pub fn parse_post(&self) -> Result<PostTable, Error> {
        let bytes = self.get_table(b"post")?;
        
        let version = get_u32(bytes, 0)?;
        let italic_angle = get_i32(bytes, 4)?;
        let underline_position = get_i16(bytes, 8)?;
        let underline_thickness = get_i16(bytes, 10)?;
        let is_fixed_pitch = get_u32(bytes, 12)?;
        let min_mem_type_42 = get_u32(bytes, 16)?;
        let max_mem_type_42 = get_u32(bytes, 20)?;
        let min_mem_type_1 = get_u32(bytes, 24)?;
        let max_mem_type_1 = get_u32(bytes, 28)?;
        let mut num_glyphs: Option<u16> = None;
        let mut glyph_name_index: Option<Vec<u16>> = None;
        let mut names: Option<Vec<String>> = None;
        if version == 0x00010000 {
            names = Some(MAC_STANDARD_NAMES.iter().map(|s| s.to_string()).collect());
        }
        else if version == 0x00020000 {
            let mut offset = 34;
            num_glyphs = Some(get_u16(bytes, 32)?);
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
                    offset += 1;
                    let string_bytes = bytes.get(offset..offset + *length as usize).ok_or(ErrorKind::UnexpectedEof)?;
                    offset += *length as usize;
                    extra_names.push(decode_name_bytes(string_bytes, 1, 0)?);
                }
            }
            names = Some(Vec::with_capacity(num_glyphs.unwrap() as usize));
            for idx in glyph_name_index.as_ref().unwrap() {
                if *idx <= 257 {
                    names.as_mut().unwrap().push(MAC_STANDARD_NAMES[*idx as usize].to_string());
                }
                else {
                    names.as_mut().unwrap().push(extra_names[*idx as usize - 258].to_string());
                }
            }
        }
        else if version == 0x00025000 {
            return Err(Error::new(ErrorKind::InvalidData, "Version 2.5 is deprecated"))
        }
        else if version != 0x00030000 {
            return Err(Error::new(ErrorKind::InvalidData, format!("Version {} is not valid", version)))
        }
        
        Ok(PostTable::new(
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
        ))
    }
    
    pub fn parse_vhea(&self) -> Result<VheaTable, Error> {
        let vhea_bytes = self.get_table(b"vhea")?;
        
        let version = get_u32(vhea_bytes, 0)?;
        if version != 0x00010000 && version != 0x00011000 {
            return Err(Error::new(ErrorKind::InvalidData, format!("Invalid version number: {}", version)))
        }
        let vert_typo_ascender = get_i16(vhea_bytes, 4)?;
        let vert_typo_descender = get_i16(vhea_bytes, 6)?;
        let vert_typo_line_gap = get_i16(vhea_bytes, 8)?;
        let advance_height_max = get_u16(vhea_bytes, 10)?;
        let min_top_side_bearing = get_i16(vhea_bytes, 12)?;
        let min_bottom_side_bearing = get_i16(vhea_bytes, 14)?;
        let y_max_extent = get_i16(vhea_bytes, 16)?;
        let caret_slope_rise = get_i16(vhea_bytes, 18)?;
        let caret_slope_run = get_i16(vhea_bytes, 20)?;
        let caret_offset = get_i16(vhea_bytes, 22)?;
        let _reserved1 = get_i16(vhea_bytes, 24)?;
        let _reserved2 = get_i16(vhea_bytes, 26)?;
        let _reserved3 = get_i16(vhea_bytes, 28)?;
        let _reserved4 = get_i16(vhea_bytes, 30)?;
        let metric_data_format = get_i16(vhea_bytes, 32)?;
        let num_of_long_ver_metrics = get_u16(vhea_bytes, 34)?;
        
        Ok(VheaTable::new(
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
        ))
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
        
        Ok(VmtxTable::new(entries, shared_advance_height))
    }
    
    pub fn parse_kern(&self) -> Result<KernTable, Error> {
        let bytes = self.get_table(b"kern")?;
        
        let version_test = get_u16(bytes, 0)?;
        match version_test {
            0 => {
                let version = get_u16(bytes, 0)?;
                let n_tables = get_u16(bytes, 2)?;
                let mut subtables: Vec<WindowsSubtable> = Vec::with_capacity(n_tables as usize);
                let mut offset: usize = 4;
                let mut subtable_start: usize;
                for _ in 0..n_tables {
                    subtable_start = offset;
                    let version = get_u16(bytes, offset)?;
                    let length = get_u16(bytes, offset + 2)?;
                    let coverage = get_u16(bytes, offset + 4)?;
                    offset += 6;
                    let subtable = match coverage >> 8 {
                        0 => parse_kern_format0(&bytes[offset..]),
                        2 => parse_kern_format2(&bytes[offset..], subtable_start, length as usize),
                        _ => {
                            offset += length as usize - 6;
                            continue;
                        }
                    }?;
                    
                    subtables.push(WindowsSubtable::new(version, length, coverage, subtable));
                }
                
                Ok(KernTable::Windows {
                    version,
                    n_tables,
                    subtables
                })
            }
            1 => {
                let version = get_u32(bytes, 0)?;
                let n_tables = get_u32(bytes, 4)?;
                let mut offset: usize = 8;
                let mut subtable_start: usize;
                let mut subtables: Vec<MacSubtable> = Vec::with_capacity(n_tables as usize);
                for _ in 0..n_tables {
                    subtable_start = offset;
                    let length = get_u32(bytes, offset)?;
                    let coverage = get_u16(bytes, offset + 4)?;
                    let tuple_index = get_u16(bytes, offset + 6)?;
                    offset += 8;
                    let subtable = match coverage & 0xFF {
                        0 => parse_kern_format0(&bytes[offset..]),
                        2 => parse_kern_format2(&bytes[offset..], subtable_start, length as usize),
                        _ => {
                            offset += length as usize - 8;
                            continue;
                        }
                    }?;
                    
                    subtables.push(MacSubtable::new(length, coverage, tuple_index, subtable));
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
        
        let version = get_u16(bytes, 0)?;
        if version != 0 && version != 1 {
            return Err(Error::new(ErrorKind::InvalidData, format!("Version number is not 0 or 1: {}", version)))
        }
        let num_ranges = get_u16(bytes, 2)?;
        let range_records: Vec<GaspRangeRecord> = bytes.get(4..4 + num_ranges as usize * 4)
            .ok_or(ErrorKind::UnexpectedEof)?
            .chunks_exact(4)
            .map(|ch| {
                let range_max_ppem = get_u16(ch, 0)?;
                let range_gasp_behavior = get_u16(ch, 2)?;
                
                Ok(GaspRangeRecord::new(range_max_ppem, range_gasp_behavior))
            }).collect::<Result<Vec<_>, Error>>()?;
        
        Ok(GaspTable::new(version, num_ranges, range_records))
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
    
    pub fn parse_gpos(&self) -> Result<GposTable<GposSubtable>, Error> {
        let bytes = self.get_table(b"GPOS")?;
        let mut offset: usize = 0;
        
        let major_version = get_u16(bytes, 0)?;
        let minor_version = get_u16(bytes, 2)?;
        let script_list_offset = get_u16(bytes, 4)?;
        let feature_list_offset = get_u16(bytes, 6)?;
        let lookup_list_offset = get_u16(bytes, 8)?;
        offset += 10;
        let feature_variations_offset: Option<u32>;
        if minor_version >= 1 {
            feature_variations_offset = Some(get_u32(bytes, offset)?);
        }
        else {
            feature_variations_offset = None;
        }
        let header = TableHeader::new(
            major_version,
            minor_version,
            script_list_offset,
            feature_list_offset,
            lookup_list_offset,
            feature_variations_offset
        );
        let script_list = parse_script_list(bytes, script_list_offset)?;
        let feature_list = parse_feature_list(bytes, feature_list_offset)?;
        let lookup_list = parse_lookup_list(bytes, lookup_list_offset)?;
        let feature_variations: Option<FeatureVariations>;
        if feature_variations_offset != None {
            feature_variations = Some(parse_feature_variations(bytes, feature_variations_offset.unwrap(), feature_list.clone())?);
        }
        else {
            feature_variations = None;
        }
        
        Ok(GposTable::new(
            header,
            script_list,
            feature_list,
            lookup_list,
            feature_variations
        ))
    }
    
    pub fn parse_gsub(&self) -> Result<GsubTable<GsubSubtable>, Error> {
        let bytes = self.get_table(b"GSUB")?;
        let mut offset: usize = 0;
        
        let major_version = get_u16(bytes, 0)?;
        let minor_version = get_u16(bytes, 2)?;
        let script_list_offset = get_u16(bytes, 4)?;
        let feature_list_offset = get_u16(bytes, 6)?;
        let lookup_list_offset = get_u16(bytes, 8)?;
        offset += 10;
        let feature_variations_offset: Option<u32>;
        if minor_version >= 1 {
            feature_variations_offset = Some(get_u32(bytes, offset)?);
        }
        else {
            feature_variations_offset = None;
        }
        let header = TableHeader::new(
            major_version,
            minor_version,
            script_list_offset,
            feature_list_offset,
            lookup_list_offset,
            feature_variations_offset
        );
        let script_list = parse_script_list(bytes, script_list_offset)?;
        let feature_list = parse_feature_list(bytes, feature_list_offset)?;
        let lookup_list = parse_lookup_list(bytes, lookup_list_offset)?;
        let feature_variations: Option<FeatureVariations>;
        if feature_variations_offset != None {
            feature_variations = Some(parse_feature_variations(bytes, feature_variations_offset.unwrap(), feature_list.clone())?);
        }
        else {
            feature_variations = None;
        }
        
        Ok(GsubTable::new(
            header,
            script_list,
            feature_list,
            lookup_list,
            feature_variations
        ))
    }
}

fn parse_kern_format0(bytes: &[u8]) -> Result<KernSubtable, Error> {
    let n_pairs = get_u16(bytes, 0)?;
    let search_range = get_u16(bytes, 2)?;
    let entry_selector = get_u16(bytes, 4)?;
    let range_shift = get_u16(bytes, 6)?;
    let pairs: Vec<KernPair> = bytes.get(8..8 + n_pairs as usize * 6)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(6)
        .map(|ch| {
            let left = get_u16(ch, 0)?;
            let right = get_u16(ch, 2)?;
            let value = get_i16(ch, 4)?;
            
            Ok(KernPair::new(left, right, value))
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
    let row_width = get_u16(bytes, offset)?;
    let left_offset = get_u16(bytes, offset + 2)?;
    let right_offset = get_u16(bytes, offset + 4)?;
    let array_offset = get_u16(bytes, offset + 6)?;
    offset = subtable_start + left_offset as usize;
    let left_class_format = get_u16(bytes, offset)?;
    let left_class_table = parse_kern_class(bytes, left_class_format, offset)?;
    offset = subtable_start + right_offset as usize;
    let right_class_format = get_u16(bytes, offset)?;
    let right_class_table = parse_kern_class(bytes, right_class_format, offset)?;
    offset = subtable_start + array_offset as usize;
    let kerning_array: Vec<i16> = bytes.get(offset..offset + length)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| Ok(get_i16(ch, 0)?))
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
            let start_glyph = get_u16(bytes, offset + 2)?;
            let glyph_count = get_u16(bytes, offset + 4)?;
            offset += 6;
            let class_ids: Vec<u16> = bytes.get(offset..offset + glyph_count as usize * 2)
                .ok_or(ErrorKind::UnexpectedEof)?
                .chunks_exact(2)
                .map(|ch| Ok(get_u16(ch, 0)?))
                .collect::<Result<Vec<_>, Error>>()?;
            
            Ok(KernClassTable::Format1 { start_glyph, glyph_count, class_ids })
        }
        2 => {
            let range_count = get_u16(bytes, offset)?;
            offset += 2;
            let ranges: Vec<Range> = bytes.get(offset..offset + range_count as usize * 6)
                .ok_or(ErrorKind::UnexpectedEof)?
                .chunks_exact(6)
                .map(|ch| {
                    let start_glyph = get_u16(ch, 0)?;
                    let end_glyph = get_u16(ch, 2)?;
                    let class = get_u16(ch, 4)?;
                    
                    Ok(Range::new(start_glyph, end_glyph, class))
                }).collect::<Result<Vec<_>, Error>>()?;
            
            Ok(KernClassTable::Format2 { range_count, ranges })
        }
        _ => Err(Error::new(ErrorKind::InvalidData, format!("Class table format invalid: {}", class_format)))
    }
}

fn parse_script_list(bytes: &[u8], script_list_offset: u16) -> Result<ScriptList, Error> {
    let mut offset = script_list_offset as usize;
    
    let script_count = get_u16(bytes, offset)?;
    offset += 2;
    let script_records: Vec<ScriptRecord> = bytes.get(offset..offset + script_count as usize * 6)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(6)
        .map(|ch| {
            let script_tag: [u8; 4] = ch[0..4].try_into().unwrap();
            let script_offset = get_u16(ch, 4)?;
            
            Ok(ScriptRecord::new(script_tag, script_offset))
        }).collect::<Result<Vec<_>, Error>>()?;
    let scripts: Vec<Script> = script_records.iter()
        .map(|sr| {
            let mut offset = (script_list_offset + sr.script_offset()) as usize;
            let test_offset = get_u16(bytes, offset)?;
            offset += 2;
            let default_lang_sys_offset = if test_offset != 0 {
                Some(test_offset)
            }
            else {
                None
            };
            let default_lang_sys = if default_lang_sys_offset != None {
                let mut offset = (script_list_offset + sr.script_offset() + default_lang_sys_offset.unwrap()) as usize;
                let _lookup_order_offset = get_u16(bytes, offset)?;
                let required_feature_index = get_u16(bytes, offset + 2)?;
                let feature_index_count = get_u16(bytes, offset + 4)?;
                offset += 6;
                let feature_indices = bytes.get(offset..offset + feature_index_count as usize * 2)
                    .ok_or(ErrorKind::UnexpectedEof)?
                    .chunks_exact(2)
                    .map(|ch| {
                        Ok(get_u16(ch, 0)?)
                    }).collect::<Result<Vec<_>, Error>>()?;
                
                Some(LangSys::new(
                    _lookup_order_offset,
                    required_feature_index,
                    feature_index_count,
                    feature_indices
                ))
            }
            else {
                None
            };
            let lang_sys_count = get_u16(bytes, offset)?;
            offset += 2;
            let lang_sys_records: Vec<LangSysRecord> = bytes.get(offset..offset + lang_sys_count as usize * 6)
                .ok_or(ErrorKind::UnexpectedEof)?
                .chunks_exact(6)
                .map(|ch| {
                    let lang_sys_tag: [u8; 4] = ch.get(0..4)
                        .ok_or(ErrorKind::UnexpectedEof)?
                        .try_into()
                        .unwrap();
                    let lang_sys_offset = get_u16(ch, 4)?;
                    
                    Ok(LangSysRecord::new(lang_sys_tag, lang_sys_offset))
                }).collect::<Result<Vec<_>, Error>>()?;
            let lang_syses: Vec<LangSys> = lang_sys_records.iter()
                .map(|lsr| {
                    let mut offset = (script_list_offset + sr.script_offset() + lsr.lang_sys_offset()) as usize;
                    let _lookup_order_offset = get_u16(bytes, offset)?;
                    let required_feature_index = get_u16(bytes, offset + 2)?;
                    let feature_index_count = get_u16(bytes, offset + 4)?;
                    offset += 6;
                    let feature_indices = bytes.get(offset..offset + feature_index_count as usize * 2)
                        .ok_or(ErrorKind::UnexpectedEof)?
                        .chunks_exact(2)
                        .map(|ch| {
                            Ok(get_u16(ch, 0)?)
                        }).collect::<Result<Vec<_>, Error>>()?;
                    
                    Ok(LangSys::new(
                        _lookup_order_offset,
                        required_feature_index,
                        feature_index_count,
                        feature_indices
                    ))
                }).collect::<Result<Vec<_>, Error>>()?;
            
            Ok(Script::new(
                default_lang_sys_offset,
                default_lang_sys,
                lang_sys_count,
                lang_sys_records,
                lang_syses
            ))
        }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(ScriptList::new(
        script_count,
        script_records,
        scripts
    ))
}

fn parse_feature_list(bytes: &[u8], feature_list_offset: u16) -> Result<FeatureList, Error> {
    let mut offset = feature_list_offset as usize;
    
    let feature_count = get_u16(bytes, offset)?;
    offset += 2;
    let feature_records: Vec<FeatureRecord> = bytes.get(offset..offset + feature_count as usize * 6)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(6)
        .map(|ch| {
            let feature_tag: [u8; 4] = ch[0..4].try_into().unwrap();
            let feature_offset = get_u16(ch, 4)?;
            
            Ok(FeatureRecord::new(feature_tag, feature_offset))
        }).collect::<Result<Vec<_>, Error>>()?;
    let features: Vec<Feature> = feature_records.iter()
        .map(|fr| {
            let mut offset = (feature_list_offset + fr.feature_offset()) as usize;
            let test_offset = get_u16(bytes, offset)?;
            offset += 2;
            let feature_params_offset: Option<u16> = if test_offset != 0 {
                Some(test_offset)
            }
            else {
                None
            };
            let feature_params: Option<FeatureParams> = if feature_params_offset != None {
                let offset = (feature_list_offset + fr.feature_offset() + feature_params_offset.unwrap()) as usize;
                match &fr.feature_tag() {
                    b"size" => {
                        let design_size = get_u16(bytes, offset)?;
                        let subfamily_id = get_u16(bytes, offset + 2)?;
                        let subfamily_name_id = get_u16(bytes, offset + 4)?;
                        let range_start = get_u16(bytes, offset + 6)?;
                        let range_end = get_u16(bytes, offset + 8)?;
                        
                        Some(FeatureParams::Size {
                            design_size,
                            subfamily_id,
                            subfamily_name_id,
                            range_start,
                            range_end
                        })
                    },
                    tag if tag.starts_with(b"ss") => {
                        let version = get_u16(bytes, offset)?;
                        let ui_name_id = get_u16(bytes, offset + 2)?;
                        
                        Some(FeatureParams::StylisticSet {
                            version,
                            ui_name_id
                        })
                    },
                    tag if tag.starts_with(b"cv") => {
                        let format = get_u16(bytes, offset)?;
                        let feat_ui_label_name_id = get_u16(bytes, offset + 2)?;
                        let feat_tooltip_text_name_id = get_u16(bytes, offset + 4)?;
                        let sample_text_name_id = get_u16(bytes, offset + 6)?;
                        let num_named_parameters = get_u16(bytes, offset + 8)?;
                        let first_param_ui_label_name_id = get_u16(bytes, offset + 10)?;
                        let char_count = get_u16(bytes, offset + 12)?;
                        let character: [u8; 3] = bytes.get(offset + 14..offset + 17)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .try_into()
                            .unwrap();
                        
                        Some(FeatureParams::CharacterVariant {
                            format,
                            feat_ui_label_name_id,
                            feat_tooltip_text_name_id,
                            sample_text_name_id,
                            num_named_parameters,
                            first_param_ui_label_name_id,
                            char_count,
                            character
                        })
                    }
                    _ => return Err(Error::new(ErrorKind::InvalidData, format!("Feature tag is invalid: {:#?}", fr.feature_tag())))
                }
            }
            else {
                None
            };
            let lookup_index_count = get_u16(bytes, offset)?;
            offset += 2;
            let lookup_list_indices: Vec<u16> = bytes.get(offset..offset + lookup_index_count as usize * 2)
                .ok_or(ErrorKind::UnexpectedEof)?
                .chunks_exact(2)
                .map(|ch| {
                    u16::from_be_bytes(ch.try_into().unwrap())
                }).collect();
            
            Ok(Feature::new(
                feature_params_offset,
                feature_params,
                lookup_index_count,
                lookup_list_indices
            ))
        }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(FeatureList::new(feature_count, feature_records, features))
}

fn parse_lookup_list<T: SubtableParser>(bytes: &[u8], lookup_list_offset: u16) -> Result<LookupList<T>, Error> {
    let mut offset = lookup_list_offset as usize;
    
    let lookup_count = get_u16(bytes, offset)?;
    offset += 2;
    let lookup_offsets: Vec<u16> = bytes.get(offset..offset + lookup_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    let mut lookups: Vec<Lookup<T>> = Vec::with_capacity(lookup_offsets.len());
    for offset in lookup_offsets.iter() {
        let lookup_offset = lookup_list_offset as usize + *offset as usize;
        let mut offset = lookup_offset;
        let lookup_type = get_u16(bytes, offset)?;
        let lookup_flag = get_u16(bytes, offset + 2)?;
        let subtable_count = get_u16(bytes, offset + 4)?;
        offset += 6;
        let subtable_offsets: Vec<u16> = bytes.get(offset..offset + subtable_count as usize * 2)
            .ok_or(ErrorKind::UnexpectedEof)?
            .chunks_exact(2)
            .map(|ch| {
                u16::from_be_bytes(ch.try_into().unwrap())
            }).collect();
        offset += subtable_count as usize * 2;
        let subtables: Vec<T> = subtable_offsets.iter()
            .map(|offset| {
                Ok(T::parse(bytes, lookup_offset + *offset as usize, 0, lookup_type)?)
            }).collect::<Result<Vec<_>, Error>>()?;
        let mark_filtering_set = if lookup_flag & USE_MARK_FILTERING_SET != 0 {
            Some(get_u16(bytes, offset)?)
        }
        else {
            None
        };
        
        lookups.push(Lookup::new(
            lookup_type,
            lookup_flag,
            subtable_count,
            subtable_offsets,
            subtables,
            mark_filtering_set
        ));
    }
    
    Ok(LookupList::new(
        lookup_count,
        lookup_offsets,
        lookups
    ))
}

trait SubtableParser: Sized {
    fn parse(bytes: &[u8], subtable_offset: usize, extension_offset: u32, lookup_type: u16) -> Result<Self, Error>;
}

impl SubtableParser for GposSubtable {
    fn parse(bytes: &[u8], subtable_offset: usize, extension_offset: u32, lookup_type: u16) -> Result<Self, Error> {
        let mut offset = (subtable_offset as u32 + extension_offset) as usize;
        let format = get_u16(bytes, offset)?;
        offset += 2;
        match lookup_type {
            1 => {
                match format {
                    1 => {
                        let coverage_offset = get_u16(bytes, offset)?;
                        offset += 2;
                        let coverage = parse_coverage(bytes, subtable_offset, coverage_offset)?;
                        let value_format = get_u16(bytes, offset)?;
                        offset += 2;
                        let value_record = parse_value_record(bytes, value_format, &mut offset, subtable_offset)?;
                        
                        Ok(GposSubtable::Type1(GposType1Format::Format1 {
                            coverage_offset,
                            coverage,
                            value_format,
                            value_record
                        }))
                    }
                    2 => {
                        let coverage_offset = get_u16(bytes, offset)?;
                        offset += 2;
                        let coverage = parse_coverage(bytes, subtable_offset, coverage_offset)?;
                        let value_format = get_u16(bytes, offset)?;
                        let value_count = get_u16(bytes, offset + 2)?;
                        offset += 4;
                        let value_records: Vec<ValueRecord> = (0..value_count).map(|_| {
                            Ok(parse_value_record(bytes, value_format, &mut offset, subtable_offset)?)
                        }).collect::<Result<Vec<_>, Error>>()?;
                        
                        Ok(GposSubtable::Type1(GposType1Format::Format2 {
                            coverage_offset,
                            coverage,
                            value_format,
                            value_count,
                            value_records
                        }))
                    }
                    _ => Err(Error::new(ErrorKind::InvalidData, format!("Format for Lookup Type 1 is invalid: {}", format)))
                }
            }
            2 => {
                match format {
                    1 => {
                        let coverage_offset = get_u16(bytes, offset)?;
                        let coverage = parse_coverage(bytes, subtable_offset, coverage_offset)?;
                        let value_format1 = get_u16(bytes, offset + 2)?;
                        let value_format2 = get_u16(bytes, offset + 4)?;
                        let pair_set_count = get_u16(bytes, offset + 6)?;
                        offset += 6;
                        let pair_set_offsets: Vec<u16> = bytes.get(offset..offset + pair_set_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        let pair_sets: Vec<PairSet> = pair_set_offsets.iter()
                            .map(|offset| {
                                let mut offset = subtable_offset + *offset as usize;
                                let pair_value_count = get_u16(bytes, offset)?;
                                offset += 2;
                                let pair_value_records: Vec<PairValueRecord> = (0..pair_value_count).map(|_| {
                                    let second_glyph = get_u16(bytes, offset)?;
                                    offset += 2;
                                    let value_record1 = parse_value_record(bytes, value_format1, &mut offset, subtable_offset)?;
                                    let value_record2 = parse_value_record(bytes, value_format2, &mut offset, subtable_offset)?;
                                    
                                    Ok(PairValueRecord::new(
                                        second_glyph,
                                        value_record1,
                                        value_record2
                                    ))
                                }).collect::<Result<Vec<_>, Error>>()?;
                                
                                Ok(PairSet::new(
                                    pair_value_count,
                                    pair_value_records
                                ))
                            }).collect::<Result<Vec<_>, Error>>()?;
                        
                        Ok(GposSubtable::Type2(GposType2Format::Format1 {
                            coverage_offset,
                            coverage,
                            value_format1,
                            value_format2,
                            pair_set_count,
                            pair_set_offsets,
                            pair_sets
                        }))
                    }
                    2 => {
                        let coverage_offset = get_u16(bytes, offset)?;
                        let coverage = parse_coverage(bytes, subtable_offset, coverage_offset)?;
                        let value_format1 = get_u16(bytes, offset + 2)?;
                        let value_format2 = get_u16(bytes, offset + 4)?;
                        let class_def1_offset = get_u16(bytes, offset + 6)?;
                        let class_def1 = parse_class_def(bytes, subtable_offset, class_def1_offset)?;
                        let class_def2_offset = get_u16(bytes, offset + 8)?;
                        let class_def2 = parse_class_def(bytes, subtable_offset, class_def2_offset)?;
                        let class1_count = get_u16(bytes, offset + 10)?;
                        let class2_count = get_u16(bytes, offset + 12)?;
                        offset += 12;
                        let class1_records: Vec<Class1Record> = (0..class1_count).map(|_| {
                            Ok(parse_class1_record(bytes, subtable_offset, value_format1, value_format2, class2_count, &mut offset)?)
                        }).collect::<Result<Vec<_>, Error>>()?;
                        
                        Ok(GposSubtable::Type2(GposType2Format::Format2 {
                            coverage_offset,
                            coverage,
                            value_format1,
                            value_format2,
                            class_def1_offset,
                            class_def1,
                            class_def2_offset,
                            class_def2,
                            class1_count,
                            class2_count,
                            class1_records
                        }))
                    }
                    _ => Err(Error::new(ErrorKind::InvalidData, format!("Format for Lookup Type 2 is invalid: {}", format)))
                }
            }
            3 => {
                match format {
                    1 => {
                        let coverage_offset = get_u16(bytes, offset)?;
                        let coverage = parse_coverage(bytes, subtable_offset, coverage_offset)?;
                        let entry_exit_count = get_u16(bytes, offset + 2)?;
                        offset += 4;
                        let entry_exit_records: Vec<EntryExitRecord> = (0..entry_exit_count).map(|_| {
                            Ok(parse_entry_exit_record(bytes, subtable_offset, &mut offset)?)
                        }).collect::<Result<Vec<_>, Error>>()?;
                        
                        Ok(GposSubtable::Type3(GposType3Format::Format1 {
                            coverage_offset,
                            coverage,
                            entry_exit_count,
                            entry_exit_records
                        }))
                    }
                    _ => Err(Error::new(ErrorKind::InvalidData, format!("Format for Lookup Type 3 is invalid: {}", format)))
                }
            }
            4 => {
                match format {
                    1 => {
                        let mark_coverage_offset = get_u16(bytes, offset)?;
                        let mark_coverage = parse_coverage(bytes, subtable_offset, mark_coverage_offset)?;
                        let base_coverage_offset = get_u16(bytes, offset + 2)?;
                        let base_coverage = parse_coverage(bytes, subtable_offset, base_coverage_offset)?;
                        let mark_class_count = get_u16(bytes, offset + 4)?;
                        let mark_array_offset = get_u16(bytes, offset + 6)?;
                        let mark_array = parse_mark_array(bytes, subtable_offset, mark_array_offset)?;
                        let base_array_offset = get_u16(bytes, offset + 8)?;
                        let base_array = parse_base_array(bytes, subtable_offset, base_array_offset)?;
                        
                        Ok(GposSubtable::Type4(GposType4Format::Format1 {
                            mark_coverage_offset,
                            mark_coverage,
                            base_coverage_offset,
                            base_coverage,
                            mark_class_count,
                            mark_array_offset,
                            mark_array,
                            base_array_offset,
                            base_array
                        }))
                    }
                    _ => Err(Error::new(ErrorKind::InvalidData, format!("Format for Lookup Type 4 is invalid: {}", format)))
                }
            }
            5 => {
                match format {
                    1 => {
                        let mark_coverage_offset = get_u16(bytes, offset)?;
                        let mark_coverage = parse_coverage(bytes, subtable_offset, mark_coverage_offset)?;
                        let ligature_coverage_offset = get_u16(bytes, offset + 2)?;
                        let ligature_coverage = parse_coverage(bytes, subtable_offset, ligature_coverage_offset)?;
                        let mark_class_count = get_u16(bytes, offset + 4)?;
                        let mark_array_offset = get_u16(bytes, offset + 6)?;
                        let mark_array = parse_mark_array(bytes, subtable_offset, mark_array_offset)?;
                        let ligature_array_offset = get_u16(bytes, offset + 8)?;
                        let ligature_array = parse_ligature_array(bytes, subtable_offset, ligature_array_offset, mark_class_count)?;
                        
                        Ok(GposSubtable::Type5(GposType5Format::Format1 {
                            mark_coverage_offset,
                            mark_coverage,
                            ligature_coverage_offset,
                            ligature_coverage,
                            mark_class_count,
                            mark_array_offset,
                            mark_array,
                            ligature_array_offset,
                            ligature_array
                        }))
                    }
                    _ => Err(Error::new(ErrorKind::InvalidData, format!("Format for Lookup Type 5 is invalid: {}", format)))
                }
            }
            6 => {
                match format {
                    1 => {
                        let mark1_coverage_offset = get_u16(bytes, offset)?;
                        let mark1_coverage = parse_coverage(bytes, subtable_offset, mark1_coverage_offset)?;
                        let mark2_coverage_offset = get_u16(bytes, offset + 2)?;
                        let mark2_coverage = parse_coverage(bytes, subtable_offset, mark2_coverage_offset)?;
                        let mark_class_count = get_u16(bytes, offset + 4)?;
                        let mark1_array_offset = get_u16(bytes, offset + 6)?;
                        let mark1_array = parse_mark_array(bytes, subtable_offset, mark1_array_offset)?;
                        let mark2_array_offset = get_u16(bytes, offset + 8)?;
                        let mark2_array = parse_mark2_array(bytes, subtable_offset, mark2_array_offset, mark_class_count)?;
                        
                        Ok(GposSubtable::Type6(GposType6Format::Format1 {
                            mark1_coverage_offset,
                            mark1_coverage,
                            mark2_coverage_offset,
                            mark2_coverage,
                            mark_class_count,
                            mark1_array_offset,
                            mark1_array,
                            mark2_array_offset,
                            mark2_array
                        }))
                    }
                    _ => Err(Error::new(ErrorKind::InvalidData, format!("Format for Lookup Type 6 is invalid: {}", format)))
                }
            }
            7 => {
                match format {
                    1 => {
                        let coverage_offset = get_u16(bytes, offset)?;
                        let coverage = parse_coverage(bytes, subtable_offset, coverage_offset)?;
                        let sub_rule_set_count = get_u16(bytes, offset + 2)?;
                        offset += 2;
                        let sub_rule_set_offsets: Vec<u16> = bytes.get(offset..offset + sub_rule_set_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        let sub_rule_sets: Vec<GposSubRuleSet> = sub_rule_set_offsets.iter()
                            .map(|offset| {
                                Ok(parse_gpos_sub_rule_set(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        
                        Ok(GposSubtable::Type7(GposType7Format::Format1 {
                            coverage_offset,
                            coverage,
                            sub_rule_set_count,
                            sub_rule_set_offsets,
                            sub_rule_sets
                        }))
                    }
                    2 => {
                        let coverage_offset = get_u16(bytes, offset)?;
                        let coverage = parse_coverage(bytes, subtable_offset, coverage_offset)?;
                        let class_def_offset = get_u16(bytes, offset + 2)?;
                        let class_def = parse_class_def(bytes, subtable_offset, class_def_offset)?;
                        let sub_class_set_count = get_u16(bytes, offset + 4)?;
                        offset += 6;
                        let sub_class_set_offsets: Vec<u16> = bytes.get(offset..offset + sub_class_set_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        let sub_class_sets: Vec<GposSubClassSet> = sub_class_set_offsets.iter()
                            .map(|offset| {
                                Ok(parse_gpos_sub_class_set(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        
                        Ok(GposSubtable::Type7(GposType7Format::Format2 {
                            coverage_offset,
                            coverage,
                            class_def_offset,
                            class_def,
                            sub_class_set_count,
                            sub_class_set_offsets,
                            sub_class_sets
                        }))
                    }
                    3 => {
                        let glyph_count = get_u16(bytes, offset)?;
                        let sub_count = get_u16(bytes, offset + 2)?;
                        offset += 4;
                        let coverage_offsets: Vec<u16> = bytes.get(offset..offset + glyph_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        offset += glyph_count as usize * 2;
                        let coverages: Vec<Coverage> = coverage_offsets.iter()
                            .map(|offset| {
                                Ok(parse_coverage(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        let pos_lookup_records: Vec<PosLookupRecord> = (0..sub_count).map(|_| {
                            Ok(parse_pos_lookup_record(bytes, &mut offset)?)
                        }).collect::<Result<Vec<_>, Error>>()?;
                        
                        Ok(GposSubtable::Type7(GposType7Format::Format3 {
                            glyph_count,
                            sub_count,
                            coverage_offsets,
                            coverages,
                            pos_lookup_records
                        }))
                    }
                    _ => Err(Error::new(ErrorKind::InvalidData, format!("Format for Lookup Type 7 is invalid: {}", format)))
                }
            }
            8 => {
                match format {
                    1 => {
                        let coverage_offset = get_u16(bytes, offset)?;
                        let coverage = parse_coverage(bytes, subtable_offset, coverage_offset)?;
                        let chain_sub_rule_set_count = get_u16(bytes, offset + 2)?;
                        offset += 4;
                        let chain_sub_rule_set_offsets: Vec<u16> = bytes.get(offset..offset + chain_sub_rule_set_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        let chain_sub_rule_sets: Vec<GposChainSubRuleSet> = chain_sub_rule_set_offsets.iter()
                            .map(|offset| {
                                Ok(parse_gpos_chain_sub_rule_set(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        
                        Ok(GposSubtable::Type8(GposType8Format::Format1 {
                            coverage_offset,
                            coverage,
                            chain_sub_rule_set_count,
                            chain_sub_rule_set_offsets,
                            chain_sub_rule_sets
                        }))
                    }
                    2 => {
                        let coverage_offset = get_u16(bytes, offset)?;
                        let coverage = parse_coverage(bytes, subtable_offset, coverage_offset)?;
                        let backtrack_class_def_offset = get_u16(bytes, offset + 2)?;
                        let backtrack_class_def = parse_class_def(bytes, subtable_offset, backtrack_class_def_offset)?;
                        let input_class_def_offset = get_u16(bytes, offset + 4)?;
                        let input_class_def = parse_class_def(bytes, subtable_offset, input_class_def_offset)?;
                        let lookahead_class_def_offset = get_u16(bytes, offset + 6)?;
                        let lookahead_class_def = parse_class_def(bytes, subtable_offset, lookahead_class_def_offset)?;
                        let chain_sub_class_set_count = get_u16(bytes, offset + 8)?;
                        offset += 8;
                        let chain_sub_class_set_offsets: Vec<u16> = bytes.get(offset..offset + chain_sub_class_set_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        let chain_sub_class_sets: Vec<GposChainSubClassSet> = chain_sub_class_set_offsets.iter()
                            .map(|offset| {
                                Ok(parse_gpos_chain_sub_class_set(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        
                        Ok(GposSubtable::Type8(GposType8Format::Format2 {
                            coverage_offset,
                            coverage,
                            backtrack_class_def_offset,
                            backtrack_class_def,
                            input_class_def_offset,
                            input_class_def,
                            lookahead_class_def_offset,
                            lookahead_class_def,
                            chain_sub_class_set_count,
                            chain_sub_class_set_offsets,
                            chain_sub_class_sets
                        }))
                    }
                    3 => {
                        let backtrack_glyph_count = get_u16(bytes, offset)?;
                        offset += 2;
                        let backtrack_coverage_offsets: Vec<u16> = bytes.get(offset..offset + backtrack_glyph_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        offset += backtrack_glyph_count as usize * 2;
                        let backtrack_coverages: Vec<Coverage> = backtrack_coverage_offsets.iter()
                            .map(|offset| {
                                Ok(parse_coverage(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        let input_glyph_count = get_u16(bytes, offset)?;
                        offset += 2;
                        let input_coverage_offsets: Vec<u16> = bytes.get(offset..offset + input_glyph_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        offset += input_glyph_count as usize * 2;
                        let input_coverages: Vec<Coverage> = input_coverage_offsets.iter()
                            .map(|offset| {
                                Ok(parse_coverage(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        let lookahead_glyph_count = get_u16(bytes, offset)?;
                        offset += 2;
                        let lookahead_coverage_offsets: Vec<u16> = bytes.get(offset..offset + lookahead_glyph_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        offset += lookahead_glyph_count as usize * 2;
                        let lookahead_coverages: Vec<Coverage> = lookahead_coverage_offsets.iter()
                            .map(|offset| {
                                Ok(parse_coverage(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        let sub_count = get_u16(bytes, offset)?;
                        offset += 2;
                        let pos_lookup_records: Vec<PosLookupRecord> = (0..sub_count).map(|_| {
                            Ok(parse_pos_lookup_record(bytes, &mut offset)?)
                        }).collect::<Result<Vec<_>, Error>>()?;
                        
                        Ok(GposSubtable::Type8(GposType8Format::Format3 {
                            backtrack_glyph_count,
                            backtrack_coverage_offsets,
                            backtrack_coverages,
                            input_glyph_count,
                            input_coverage_offsets,
                            input_coverages,
                            lookahead_glyph_count,
                            lookahead_coverage_offsets,
                            lookahead_coverages,
                            sub_count,
                            pos_lookup_records
                        }))
                    }
                    _ => Err(Error::new(ErrorKind::InvalidData, format!("Format for Lookup Type 8 is invalid: {}", format)))
                }
            }
            9 => {
                match format {
                    1 => {
                        let extension_lookup_type = get_u16(bytes, offset)?;
                        let extension_offset = get_u32(bytes, offset + 2)?;
                        let extension = Box::new(GposSubtable::parse(bytes, subtable_offset, extension_offset, extension_lookup_type)?);
                        
                        Ok(GposSubtable::Type9(GposType9Format::Format1 {
                            extension_lookup_type,
                            extension_offset,
                            extension
                        }))
                    }
                    _ => Err(Error::new(ErrorKind::InvalidData, format!("Format for Lookup Type 9 is invalid: {}", format)))
                }
            }
            _ => Err(Error::new(ErrorKind::InvalidData, format!("Lookup Type is invalid: {}", lookup_type)))
        }
    }
}

impl SubtableParser for GsubSubtable {
    fn parse(bytes: &[u8], subtable_offset: usize, extension_offset: u32, lookup_type: u16) -> Result<Self, Error> {
        let mut offset = (subtable_offset as u32 + extension_offset) as usize;
        let format = get_u16(bytes, offset)?;
        offset += 2;
        match lookup_type {
            1 => {
                match format {
                    1 => {
                        let coverage_offset = get_u16(bytes, offset)?;
                        let coverage = parse_coverage(bytes, subtable_offset, coverage_offset)?;
                        let delta_glyph_id = get_i16(bytes, offset + 2)?;
                        
                        Ok(GsubSubtable::Type1(GsubType1Format::Format1 {
                            coverage_offset,
                            coverage,
                            delta_glyph_id
                        }))
                    }
                    2 => {
                        let coverage_offset = get_u16(bytes, offset)?;
                        let coverage = parse_coverage(bytes, subtable_offset, coverage_offset)?;
                        let glyph_count = get_u16(bytes, offset + 2)?;
                        offset += 4;
                        let substitute_glyph_ids = bytes.get(offset..offset + glyph_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        
                        Ok(GsubSubtable::Type1(GsubType1Format::Format2 {
                            coverage_offset,
                            coverage,
                            glyph_count,
                            substitute_glyph_ids
                        }))
                    }
                    _ => Err(Error::new(ErrorKind::InvalidData, format!("Format for Lookup Type 1 is invalid: {}", format)))
                }
            }
            2 => {
                match format {
                    1 => {
                        let coverage_offset = get_u16(bytes, offset)?;
                        let coverage = parse_coverage(bytes, subtable_offset, coverage_offset)?;
                        let sequence_count = get_u16(bytes, offset + 2)?;
                        offset += 4;
                        let sequence_offsets: Vec<u16> = bytes.get(offset..offset + sequence_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        let sequences: Vec<Sequence> = sequence_offsets.iter()
                            .map(|offset| {
                                Ok(parse_sequence(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        
                        Ok(GsubSubtable::Type2(GsubType2Format::Format1 {
                            coverage_offset,
                            coverage,
                            sequence_count,
                            sequence_offsets,
                            sequences
                        }))
                    }
                    _ => Err(Error::new(ErrorKind::InvalidData, format!("Format for Lookup Type 2 is invalid: {}", format)))
                }
            }
            3 => {
                match format {
                    1 => {
                        let coverage_offset = get_u16(bytes, offset)?;
                        let coverage = parse_coverage(bytes, subtable_offset, coverage_offset)?;
                        let alternate_set_count = get_u16(bytes, offset + 2)?;
                        offset += 4;
                        let alternate_set_offsets: Vec<u16> = bytes.get(offset..offset + alternate_set_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        let alternate_sets: Vec<AlternateSet> = alternate_set_offsets.iter()
                            .map(|offset| {
                                Ok(parse_alternate_set(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        
                        Ok(GsubSubtable::Type3(GsubType3Format::Format1 {
                            coverage_offset,
                            coverage,
                            alternate_set_count,
                            alternate_set_offsets,
                            alternate_sets
                        }))
                    }
                    _ => Err(Error::new(ErrorKind::InvalidData, format!("Format for Lookup Type 3 is invalid: {}", format)))
                }
            }
            4 => {
                match format {
                    1 => {
                        let coverage_offset = get_u16(bytes, offset)?;
                        let coverage = parse_coverage(bytes, subtable_offset, coverage_offset)?;
                        let ligature_set_count = get_u16(bytes, offset + 2)?;
                        offset += 4;
                        let ligature_set_offsets: Vec<u16> = bytes.get(offset..offset + ligature_set_count as usize)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        let ligature_sets: Vec<LigatureSet> = ligature_set_offsets.iter()
                            .map(|offset| {
                                Ok(parse_ligature_set(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        
                        Ok(GsubSubtable::Type4(GsubType4Format::Format1 {
                            coverage_offset,
                            coverage,
                            ligature_set_count,
                            ligature_set_offsets,
                            ligature_sets
                        }))
                    }
                    _ => Err(Error::new(ErrorKind::InvalidData, format!("Format for Lookup Type 4 is invalid: {}", format)))
                }
            }
            5 => {
                match format {
                    1 => {
                        let coverage_offset = get_u16(bytes, offset)?;
                        let coverage = parse_coverage(bytes, subtable_offset, coverage_offset)?;
                        let sub_rule_set_count = get_u16(bytes, offset + 2)?;
                        offset += 4;
                        let sub_rule_set_offsets: Vec<u16> = bytes.get(offset..offset + sub_rule_set_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        let sub_rule_sets: Vec<GsubSubRuleSet> = sub_rule_set_offsets.iter()
                            .map(|offset| {
                                Ok(parse_gsub_sub_rule_set(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        
                        Ok(GsubSubtable::Type5(GsubType5Format::Format1 {
                            coverage_offset,
                            coverage,
                            sub_rule_set_count,
                            sub_rule_set_offsets,
                            sub_rule_sets
                        }))
                    }
                    2 => {
                        let coverage_offset = get_u16(bytes, offset)?;
                        let coverage = parse_coverage(bytes, subtable_offset, coverage_offset)?;
                        let class_def_offset = get_u16(bytes, offset + 2)?;
                        let class_def = parse_class_def(bytes, subtable_offset, class_def_offset)?;
                        let sub_class_set_count = get_u16(bytes, offset + 4)?;
                        offset += 6;
                        let sub_class_set_offsets: Vec<u16> = bytes.get(offset..offset + sub_class_set_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        let sub_class_sets: Vec<GsubSubClassSet> = sub_class_set_offsets.iter()
                            .map(|offset| {
                                Ok(parse_gsub_sub_class_set(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        
                        Ok(GsubSubtable::Type5(GsubType5Format::Format2 {
                            coverage_offset,
                            coverage,
                            class_def_offset,
                            class_def,
                            sub_class_set_count,
                            sub_class_set_offsets,
                            sub_class_sets
                        }))
                    }
                    3 => {
                        let glyph_count = get_u16(bytes, offset)?;
                        let sub_count = get_u16(bytes, offset + 2)?;
                        offset += 4;
                        let coverage_offsets: Vec<u16> = bytes.get(offset..offset + glyph_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        offset += glyph_count as usize * 2;
                        let coverages: Vec<Coverage> = coverage_offsets.iter()
                            .map(|offset| {
                                Ok(parse_coverage(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        let subst_lookup_records: Vec<SubstLookupRecord> = (0..sub_count).map(|_| {
                            Ok(parse_subst_lookup_record(bytes, &mut offset)?)
                        }).collect::<Result<Vec<_>, Error>>()?;
                        
                        Ok(GsubSubtable::Type5(GsubType5Format::Format3 {
                            glyph_count,
                            sub_count,
                            coverage_offsets,
                            coverages,
                            subst_lookup_records
                        }))
                    }
                    _ => Err(Error::new(ErrorKind::InvalidData, format!("Format for Lookup Type 5 is invalid: {}", format)))
                }
            }
            6 => {
                match format {
                    1 => {
                        let coverage_offset = get_u16(bytes, offset)?;
                        let coverage = parse_coverage(bytes, subtable_offset, coverage_offset)?;
                        let chain_sub_rule_set_count = get_u16(bytes, offset + 2)?;
                        offset += 4;
                        let chain_sub_rule_set_offsets: Vec<u16> = bytes.get(offset..offset + chain_sub_rule_set_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        let chain_sub_rule_sets: Vec<GsubChainSubRuleSet> = chain_sub_rule_set_offsets.iter()
                            .map(|offset| {
                                Ok(parse_gsub_chain_sub_rule_set(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        
                        Ok(GsubSubtable::Type6(GsubType6Format::Format1 {
                            coverage_offset,
                            coverage,
                            chain_sub_rule_set_count,
                            chain_sub_rule_set_offsets,
                            chain_sub_rule_sets
                        }))
                    }
                    2 => {
                        let coverage_offset = get_u16(bytes, offset)?;
                        let coverage = parse_coverage(bytes, subtable_offset, coverage_offset)?;
                        let backtrack_class_def_offset = get_u16(bytes, offset + 2)?;
                        let backtrack_class_def = parse_class_def(bytes, subtable_offset, backtrack_class_def_offset)?;
                        let input_class_def_offset = get_u16(bytes, offset + 4)?;
                        let input_class_def = parse_class_def(bytes, subtable_offset, input_class_def_offset)?;
                        let lookahead_class_def_offset = get_u16(bytes, offset + 6)?;
                        let lookahead_class_def = parse_class_def(bytes, subtable_offset, lookahead_class_def_offset)?;
                        let chain_sub_class_set_count = get_u16(bytes, offset + 8)?;
                        offset += 8;
                        let chain_sub_class_set_offsets: Vec<u16> = bytes.get(offset..offset + chain_sub_class_set_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        let chain_sub_class_sets: Vec<GsubChainSubClassSet> = chain_sub_class_set_offsets.iter()
                            .map(|offset| {
                                Ok(parse_gsub_chain_sub_class_set(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        
                        Ok(GsubSubtable::Type6(GsubType6Format::Format2 {
                            coverage_offset,
                            coverage,
                            backtrack_class_def_offset,
                            backtrack_class_def,
                            input_class_def_offset,
                            input_class_def,
                            lookahead_class_def_offset,
                            lookahead_class_def,
                            chain_sub_class_set_count,
                            chain_sub_class_set_offsets,
                            chain_sub_class_sets
                        }))
                    }
                    3 => {
                        let backtrack_glyph_count = get_u16(bytes, offset)?;
                        offset += 2;
                        let backtrack_coverage_offsets: Vec<u16> = bytes.get(offset..offset + backtrack_glyph_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        offset += backtrack_glyph_count as usize * 2;
                        let backtrack_coverages: Vec<Coverage> = backtrack_coverage_offsets.iter()
                            .map(|offset| {
                                Ok(parse_coverage(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        let input_glyph_count = get_u16(bytes, offset)?;
                        offset += 2;
                        let input_coverage_offsets: Vec<u16> = bytes.get(offset..offset + input_glyph_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        offset += input_glyph_count as usize * 2;
                        let input_coverages: Vec<Coverage> = input_coverage_offsets.iter()
                            .map(|offset| {
                                Ok(parse_coverage(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        let lookahead_glyph_count = get_u16(bytes, offset)?;
                        offset += 2;
                        let lookahead_coverage_offsets: Vec<u16> = bytes.get(offset..offset + lookahead_glyph_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        offset += lookahead_glyph_count as usize * 2;
                        let lookahead_coverages: Vec<Coverage> = lookahead_coverage_offsets.iter()
                            .map(|offset| {
                                Ok(parse_coverage(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        let sub_count = get_u16(bytes, offset)?;
                        offset += 2;
                        let subst_lookup_records: Vec<SubstLookupRecord> = (0..sub_count).map(|_| {
                            Ok(parse_subst_lookup_record(bytes, &mut offset)?)
                        }).collect::<Result<Vec<_>, Error>>()?;
                        
                        Ok(GsubSubtable::Type6(GsubType6Format::Format3 {
                            backtrack_glyph_count,
                            backtrack_coverage_offsets,
                            backtrack_coverages,
                            input_glyph_count,
                            input_coverage_offsets,
                            input_coverages,
                            lookahead_glyph_count,
                            lookahead_coverage_offsets,
                            lookahead_coverages,
                            sub_count,
                            subst_lookup_records
                        }))
                    }
                    _ => Err(Error::new(ErrorKind::InvalidData, format!("Format for Lookup Type 6 is invalid: {}", format)))
                }
            }
            7 => {
                match format {
                    1 => {
                        let extension_lookup_type = get_u16(bytes, offset)?;
                        let extension_offset = get_u32(bytes, offset + 2)?;
                        let extension = Box::new(GsubSubtable::parse(bytes, subtable_offset, extension_offset, extension_lookup_type)?);
                                                
                        Ok(GsubSubtable::Type7(GsubType7Format::Format1 {
                            extension_lookup_type,
                            extension_offset,
                            extension
                        }))
                    }
                    _ => Err(Error::new(ErrorKind::InvalidData, format!("Format for Lookup Type 7 is invalid: {}", format)))
                }
            }
            8 => {
                match format {
                    1 => {
                        let coverage_offset = get_u16(bytes, offset)?;
                        let coverage = parse_coverage(bytes, subtable_offset, coverage_offset)?;
                        let backtrack_glyph_count = get_u16(bytes, offset + 2)?;
                        offset += 4;
                        let backtrack_coverage_offsets: Vec<u16> = bytes.get(offset..offset + backtrack_glyph_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        offset += backtrack_glyph_count as usize * 2;
                        let backtrack_coverages: Vec<Coverage> = backtrack_coverage_offsets.iter()
                            .map(|offset| {
                                Ok(parse_coverage(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        let lookahead_glyph_count = get_u16(bytes, offset)?;
                        offset += 2;
                        let lookahead_coverage_offsets: Vec<u16> = bytes.get(offset..offset + lookahead_glyph_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        offset += lookahead_glyph_count as usize * 2;
                        let lookahead_coverages: Vec<Coverage> = lookahead_coverage_offsets.iter()
                            .map(|offset| {
                                Ok(parse_coverage(bytes, subtable_offset, *offset)?)
                            }).collect::<Result<Vec<_>, Error>>()?;
                        let glyph_count = get_u16(bytes, offset)?;
                        offset += 2;
                        let substitute_glyph_ids: Vec<u16> = bytes.get(offset..offset + glyph_count as usize * 2)
                            .ok_or(ErrorKind::UnexpectedEof)?
                            .chunks_exact(2)
                            .map(|ch| {
                                u16::from_be_bytes(ch.try_into().unwrap())
                            }).collect();
                        
                        Ok(GsubSubtable::Type8(GsubType8Format::Format1 {
                            coverage_offset,
                            coverage,
                            backtrack_glyph_count,
                            backtrack_coverage_offsets,
                            backtrack_coverages,
                            lookahead_glyph_count,
                            lookahead_coverage_offsets,
                            lookahead_coverages,
                            glyph_count,
                            substitute_glyph_ids
                        }))
                    }
                    _ => Err(Error::new(ErrorKind::InvalidData, format!("Format for Lookup Type 8 is invalid: {}", format)))
                }
            }
            _ => Err(Error::new(ErrorKind::InvalidData, format!("Lookup Type is invalid: {}", lookup_type)))
        }
    }
}

fn parse_feature_variations(bytes: &[u8], feature_variations_offset: u32, feature_list: FeatureList) -> Result<FeatureVariations, Error> {
    let mut offset = feature_variations_offset as usize;
    
    let major_version = get_u16(bytes, offset)?;
    let minor_version = get_u16(bytes, offset + 2)?;
    let feature_variation_record_count = get_u32(bytes, offset + 4)?;
    offset += 8;
    let mut feature_variation_records: Vec<FeatureVariationRecord> = Vec::with_capacity(feature_variation_record_count as usize);
    for _ in 0..feature_variation_record_count {
        let condition_set_offset = get_u32(bytes, offset)?;
        offset += 4;
        let condition_set: ConditionSet = {
            let mut offset = (feature_variations_offset + condition_set_offset) as usize;
            let condition_count = get_u16(bytes, offset)?;
            offset += 2;
            let condition_offsets: Vec<u32> = bytes.get(offset..offset + condition_count as usize * 4)
                .ok_or(ErrorKind::UnexpectedEof)?
                .chunks_exact(4)
                .map(|ch| {
                    u32::from_be_bytes(ch.try_into().unwrap())
                }).collect();
            let conditions: Vec<Condition> = condition_offsets.iter()
                .map(|off| {
                    let offset = *off as usize;
                    let format = get_u16(bytes, offset)?;
                    match format {
                        1 => {
                            let axis_index = get_u16(bytes, offset)?;
                            let filter_range_min_value = get_i16(bytes, offset + 2)?;
                            let filter_range_max_value = get_i16(bytes, offset + 4)?;
                            
                            Ok(Condition::Format1 {
                                axis_index,
                                filter_range_min_value,
                                filter_range_max_value
                            })
                        }
                        _ => return Err(Error::new(ErrorKind::InvalidData, format!("Condition format invalid: {}", format)))
                    }
                }).collect::<Result<Vec<_>, Error>>()?;
            
            ConditionSet::new(
                condition_count,
                condition_offsets,
                conditions
            )
        };
        let feature_table_substitution_offset = get_u32(bytes, offset)?;
        let feature_table_substitution: FeatureTableSubstitution = {
            let mut offset = (feature_variations_offset + feature_table_substitution_offset) as usize;
            let major_version = get_u16(bytes, offset)?;
            let minor_version = get_u16(bytes, offset + 2)?;
            let substitution_count = get_u16(bytes, offset + 4)?;
            offset += 4;
            let mut substitution_records: Vec<FeatureTableSubstitutionRecord> = Vec::with_capacity(substitution_count as usize);
            for _ in 0..substitution_count {
                let feature_index = get_u16(bytes, offset)?;
                let alternate_feature_table_offset = get_u32(bytes, offset + 2)?;
                offset += 6;
                let alternate_feature_table: Feature = {
                    let mut offset = (feature_variations_offset + feature_table_substitution_offset + alternate_feature_table_offset) as usize;
                    let test_offset = get_u16(bytes, offset)?;
                    offset += 2;
                    let feature_params_offset = if test_offset != 0 {
                        Some(test_offset)
                    }
                    else {
                        None
                    };
                    let feature_params = if feature_params_offset != None {
                        let feature_tag = feature_list.feature_records()
                            .get(feature_index as usize)
                            .ok_or(ErrorKind::NotFound)?
                            .feature_tag();
                        match &feature_tag {
                            b"size" => {
                                let design_size = get_u16(bytes, offset)?;
                                let subfamily_id = get_u16(bytes, offset + 2)?;
                                let subfamily_name_id = get_u16(bytes, offset + 4)?;
                                let range_start = get_u16(bytes, offset + 6)?;
                                let range_end = get_u16(bytes, offset + 8)?;
                                
                                Some(FeatureParams::Size {
                                    design_size,
                                    subfamily_id,
                                    subfamily_name_id,
                                    range_start,
                                    range_end
                                })
                            },
                            tag if tag.starts_with(b"ss") => {
                                let version = get_u16(bytes, offset)?;
                                let ui_name_id = get_u16(bytes, offset + 2)?;
                                
                                Some(FeatureParams::StylisticSet {
                                    version,
                                    ui_name_id
                                })
                            },
                            tag if tag.starts_with(b"cv") => {
                                let format = get_u16(bytes, offset)?;
                                let feat_ui_label_name_id = get_u16(bytes, offset + 2)?;
                                let feat_tooltip_text_name_id = get_u16(bytes, offset + 4)?;
                                let sample_text_name_id = get_u16(bytes, offset + 6)?;
                                let num_named_parameters = get_u16(bytes, offset + 8)?;
                                let first_param_ui_label_name_id = get_u16(bytes, offset + 10)?;
                                let char_count = get_u16(bytes, offset + 12)?;
                                let character: [u8; 3] = bytes.get(offset + 14..offset + 17)
                                    .ok_or(ErrorKind::UnexpectedEof)?
                                    .try_into()
                                    .unwrap();
                                
                                Some(FeatureParams::CharacterVariant {
                                    format,
                                    feat_ui_label_name_id,
                                    feat_tooltip_text_name_id,
                                    sample_text_name_id,
                                    num_named_parameters,
                                    first_param_ui_label_name_id,
                                    char_count,
                                    character
                                })
                            }
                            _ => return Err(Error::new(ErrorKind::InvalidData, format!("Feature tag is invalid: {:#?}", feature_tag)))
                        }
                    }
                    else {
                        None
                    };
                    let lookup_index_count = get_u16(bytes, offset)?;
                    offset += 2;
                    let lookup_list_indices: Vec<u16> = bytes.get(offset..offset + lookup_index_count as usize  * 2)
                        .ok_or(ErrorKind::UnexpectedEof)?
                        .chunks_exact(2)
                        .map(|ch| {
                            u16::from_be_bytes(ch.try_into().unwrap())
                        }).collect();
                    
                    Feature::new(
                        feature_params_offset,
                        feature_params,
                        lookup_index_count,
                        lookup_list_indices
                    )
                };
                
                substitution_records.push(FeatureTableSubstitutionRecord::new(
                    feature_index,
                    alternate_feature_table_offset,
                    alternate_feature_table
                ))
            }
            
            FeatureTableSubstitution::new(
                major_version,
                minor_version,
                substitution_count,
                substitution_records
            )
        };
        
        feature_variation_records.push(FeatureVariationRecord::new(
            condition_set_offset,
            condition_set,
            feature_table_substitution_offset,
            feature_table_substitution
        ));
    }
    
    Ok(FeatureVariations::new(
        major_version,
        minor_version,
        feature_variation_record_count,
        feature_variation_records
    ))
}

fn parse_coverage(bytes: &[u8], subtable_offset: usize, coverage_offset: u16) -> Result<Coverage, Error> {
    let mut offset = subtable_offset + coverage_offset as usize;
    let format = get_u16(bytes, offset)?;
    offset += 2;
    match format {
        1 => {
            let glyph_count = get_u16(bytes, offset)?;
            offset += 2;
            let glyph_array = bytes.get(offset..offset + glyph_count as usize * 2)
                .ok_or(ErrorKind::UnexpectedEof)?
                .chunks_exact(2)
                .map(|ch| {
                    u16::from_be_bytes(ch.try_into().unwrap())
                }).collect();
            
            Ok(Coverage::Format1 {
                glyph_count,
                glyph_array
            })
        }
        2 => {
            let range_count = get_u16(bytes, offset)?;
            offset += 2;
            let range_records: Vec<CoverageRangeRecord> = bytes.get(offset..offset + range_count as usize * 6)
                .ok_or(ErrorKind::UnexpectedEof)?
                .chunks_exact(6)
                .map(|ch| {
                    let start_glyph_id = u16::from_be_bytes(ch[0..2].try_into().unwrap());
                    let end_glyph_id = u16::from_be_bytes(ch[2..4].try_into().unwrap());
                    let start_coverage_index = u16::from_be_bytes(ch[4..6].try_into().unwrap());
                    
                    CoverageRangeRecord::new(
                        start_glyph_id,
                        end_glyph_id,
                        start_coverage_index
                    )
                }).collect();
            
            Ok(Coverage::Format2 {
                range_count,
                range_records
            })
        }
        _ => return Err(Error::new(ErrorKind::InvalidData, format!("Coverage format invalid: {}", format)))
    }
}

fn parse_value_record(bytes: &[u8], value_format: u16, offset: &mut usize, subtable_offset: usize) -> Result<ValueRecord, Error> {
    let x_placement = if value_format & X_PLACEMENT != 0 {
        *offset += 2;
        Some(get_i16(bytes, *offset - 2)?)
    }
    else { None };
    let y_placement = if value_format & Y_PLACEMENT != 0 {
        *offset += 2;
        Some(get_i16(bytes, *offset - 2)?)
    }
    else { None };
    let x_advance = if value_format & X_ADVANCE != 0 {
        *offset += 2;
        Some(get_i16(bytes, *offset - 2)?)
    }
    else { None };
    let y_advance = if value_format & Y_ADVANCE != 0 {
        *offset += 2;
        Some(get_i16(bytes, *offset - 2)?)
    }
    else { None };
    let x_pla_device_offset = if value_format & X_PLACEMENT_DEVICE != 0 {
        *offset += 2;
        Some(get_u16(bytes, *offset - 2)?)
    }
    else { None };
    let x_pla_device = if x_pla_device_offset != None {
        Some(parse_device_or_variation_index(bytes, subtable_offset, x_pla_device_offset.unwrap())?)
    }
    else { None };
    let y_pla_device_offset = if value_format & Y_PLACEMENT_DEVICE != 0 {
        *offset += 2;
        Some(get_u16(bytes, *offset - 2)?)
    }
    else { None };
    let y_pla_device = if y_pla_device_offset != None {
        Some(parse_device_or_variation_index(bytes, subtable_offset, y_pla_device_offset.unwrap())?)
    }
    else { None };
    let x_adv_device_offset = if value_format & X_ADVANCE_DEVICE != 0 {
        *offset += 2;
        Some(get_u16(bytes, *offset - 2)?)
    }
    else { None };
    let x_adv_device = if x_adv_device_offset != None {
        Some(parse_device_or_variation_index(bytes, subtable_offset, x_adv_device_offset.unwrap())?)
    }
    else { None };
    let y_adv_device_offset = if value_format & Y_ADVANCE_DEVICE != 0 {
        *offset += 2;
        Some(get_u16(bytes, *offset - 2)?)
    }
    else { None };
    let y_adv_device = if y_adv_device_offset != None {
        Some(parse_device_or_variation_index(bytes, subtable_offset, y_adv_device_offset.unwrap())?)
    }
    else { None };
    
    Ok(ValueRecord::new(
        x_placement,
        y_placement,
        x_advance,
        y_advance,
        x_pla_device_offset,
        x_pla_device,
        y_pla_device_offset,
        y_pla_device,
        x_adv_device_offset,
        x_adv_device,
        y_adv_device_offset,
        y_adv_device
    ))
}

fn parse_device_or_variation_index(bytes: &[u8], subtable_offset: usize, device_offset: u16) -> Result<DeviceOrVariationIndex, Error> {
    let offset = subtable_offset + device_offset as usize;
    let delta_format = get_u16(bytes, offset + 4)?;
    if delta_format == 1 || delta_format == 2 || delta_format == 3 {
        Ok(parse_device(bytes, subtable_offset, device_offset)?)
    }
    else if delta_format == 0x8000 {
        Ok(parse_variation_index(bytes, subtable_offset, device_offset)?)
    }
    else {
        Err(Error::new(ErrorKind::InvalidData, "DeltaFormat value invalid: {}".to_string()))
    }
}

fn parse_device(bytes: &[u8], subtable_offset: usize, device_offset: u16) -> Result<DeviceOrVariationIndex, Error> {
    let offset = subtable_offset + device_offset as usize;
    let start_size = get_u16(bytes, offset)?;
    let end_size = get_u16(bytes, offset + 2)?;
    let delta_format = get_u16(bytes, offset + 4)?;
    let count = end_size - start_size + 1;
    let length = match delta_format {
        1 => (count * 2).div_ceil(16),
        2 => (count * 4).div_ceil(16),
        3 => (count * 8).div_ceil(16),
        _ => return Err(Error::new(ErrorKind::InvalidData, format!("DeltaFormat value invalid: {}", delta_format)))
    };
    let delta_values: Vec<u16> = bytes.get(offset..offset + length as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    
    Ok(DeviceOrVariationIndex::Device(
        Device::new(
            start_size,
            end_size,
            delta_format,
            delta_values
        )
    ))
}

fn parse_variation_index(bytes: &[u8], subtable_offset: usize, device_offset: u16) -> Result<DeviceOrVariationIndex, Error> {
    let offset = subtable_offset + device_offset as usize;
    let delta_set_outer_index = get_u16(bytes, offset)?;
    let delta_set_inner_index = get_u16(bytes, offset + 2)?;
    let delta_format = get_u16(bytes, offset + 4)?;
    
    Ok(DeviceOrVariationIndex::VariationIndex(
        VariationIndex::new(
            delta_set_outer_index,
            delta_set_inner_index,
            delta_format
        )
    ))
}

fn parse_class_def(bytes: &[u8], subtable_offset: usize, class_def_offset: u16) -> Result<ClassDef, Error> {
    let mut offset = subtable_offset + class_def_offset as usize;
    let format = get_u16(bytes, offset)?;
    offset += 2;
    match format {
        1 => {
            let start_glyph_id = get_u16(bytes, offset)?;
            let glyph_count = get_u16(bytes, offset + 2)?;
            offset += 4;
            let class_value_array: Vec<u16> = bytes.get(offset..offset + glyph_count as usize * 2)
                .ok_or(ErrorKind::UnexpectedEof)?
                .chunks_exact(2)
                .map(|ch| {
                    u16::from_be_bytes(ch.try_into().unwrap())
                }).collect();
            
            Ok(ClassDef::Format1 {
                start_glyph_id,
                glyph_count,
                class_value_array
            })
        }
        2 => {
            let class_range_count = get_u16(bytes, offset)?;
            offset += 2;
            let class_range_records: Vec<ClassRangeRecord> = bytes.get(offset..offset + class_range_count as usize * 6)
                .ok_or(ErrorKind::UnexpectedEof)?
                .chunks_exact(6)
                .map(|ch| {
                    let start_glyph_id = u16::from_be_bytes(ch[0..2].try_into().unwrap());
                    let end_glyph_id = u16::from_be_bytes(ch[2..4].try_into().unwrap());
                    let class = u16::from_be_bytes(ch[4..6].try_into().unwrap());
                    
                    ClassRangeRecord::new(
                        start_glyph_id,
                        end_glyph_id,
                        class
                    )
                }).collect();
            
            Ok(ClassDef::Format2 {
                class_range_count,
                class_range_records
            })
        }
        _ => return Err(Error::new(ErrorKind::InvalidData, format!("ClassDef format invalid: {}", format)))
    }
}

fn parse_class1_record(
    bytes: &[u8],
    subtable_offset: usize,
    value_format1: u16,
    value_format2: u16,
    class2_count: u16,
    offset: &mut usize
) -> Result<Class1Record, Error> {
    let class2_records: Vec<Class2Record> = (0..class2_count).map(|_| {
        let value_record1 = parse_value_record(bytes, value_format1, offset, subtable_offset)?;
        let value_record2 = parse_value_record(bytes, value_format2, offset, subtable_offset)?;
        
        Ok(Class2Record::new(value_record1, value_record2))
    }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(Class1Record::new(class2_records))
}

fn parse_entry_exit_record(bytes: &[u8], subtable_offset: usize, offset: &mut usize) -> Result<EntryExitRecord, Error> {
    let test_offset = get_u16(bytes, *offset)?;
    let entry_anchor_offset = if test_offset != 0 {
        Some(test_offset)
    }
    else { None };
    let entry_anchor = if entry_anchor_offset != None {
        Some(parse_anchor(bytes, subtable_offset, entry_anchor_offset.unwrap())?)
    }
    else { None };
    let test_offset = get_u16(bytes, *offset + 2)?;
    let exit_anchor_offset = if test_offset != 0 {
        Some(test_offset)
    }
    else { None };
    let exit_anchor = if exit_anchor_offset != None {
        Some(parse_anchor(bytes, subtable_offset, exit_anchor_offset.unwrap())?)
    }
    else { None };
    *offset += 4;
    
    Ok(EntryExitRecord::new(
        entry_anchor_offset,
        entry_anchor,
        exit_anchor_offset,
        exit_anchor
    ))
}

fn parse_anchor(bytes: &[u8], subtable_offset: usize, anchor_offset: u16) -> Result<Anchor, Error> {
    let mut offset = subtable_offset + anchor_offset as usize;
    let format = get_u16(bytes, offset)?;
    offset += 2;
    match format {
        1 => {
            let x_coordinate = get_i16(bytes, offset)?;
            let y_coordinate = get_i16(bytes, offset + 2)?;
            
            Ok(Anchor::Format1 {
                x_coordinate,
                y_coordinate
            })
        }
        2 => {
            let x_coordinate = get_i16(bytes, offset)?;
            let y_coordinate = get_i16(bytes, offset + 2)?;
            let anchor_point = get_u16(bytes, offset + 4)?;
            
            Ok(Anchor::Format2 {
                x_coordinate,
                y_coordinate,
                anchor_point
            })
        }
        3 => {
            let x_coordinate = get_i16(bytes, offset)?;
            let y_coordinate = get_i16(bytes, offset + 2)?;
            let x_device_offset = get_u16(bytes, offset + 4)?;
            let x_device = parse_device_or_variation_index(bytes, subtable_offset, x_device_offset)?;
            let y_device_offset = get_u16(bytes, offset + 6)?;
            let y_device = parse_device_or_variation_index(bytes, subtable_offset, y_device_offset)?;
            
            Ok(Anchor::Format3 {
                x_coordinate,
                y_coordinate,
                x_device_offset,
                x_device,
                y_device_offset,
                y_device
            })
        }
        _ => Err(Error::new(ErrorKind::InvalidData, format!("Anchor invalid format: {}", format)))
    }
}

fn parse_mark_array(bytes: &[u8], subtable_offset: usize, mark_array_offset: u16) -> Result<MarkArray, Error> {
    let mut offset = subtable_offset + mark_array_offset as usize;
    let mark_count = get_u16(bytes, offset)?;
    offset += 2;
    let mark_records: Vec<MarkRecord> =  (0..mark_count).map(|_| {
        let mark_class = get_u16(bytes, offset)?;
        let mark_anchor_offset = get_u16(bytes, offset + 2)?;
        offset += 4;
        let mark_anchor = parse_anchor(bytes, subtable_offset, mark_anchor_offset)?;
        
        Ok(MarkRecord::new(
            mark_class,
            mark_anchor_offset,
            mark_anchor
        ))
    }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(MarkArray::new(
        mark_count,
        mark_records
    ))
}

fn parse_base_array(bytes: &[u8], subtable_offset: usize, base_array_offset: u16) -> Result<BaseArray, Error> {
    let mut offset = subtable_offset + base_array_offset as usize;
    let base_count = get_u16(bytes, offset)?;
    offset += 2;
    let base_records: Vec<BaseRecord> = (0..base_count).map(|_| {
        let base_anchor_offsets: Vec<u16> = bytes.get(offset..offset + base_count as usize * 2)
            .ok_or(ErrorKind::UnexpectedEof)?
            .chunks_exact(2)
            .map(|ch| {
                u16::from_be_bytes(ch.try_into().unwrap())
            }).collect();
        offset += base_count as usize * 2;
        let base_anchors: Vec<Anchor> = base_anchor_offsets.iter()
            .map(|offset| {
                Ok(parse_anchor(bytes, subtable_offset, *offset)?)
            }).collect::<Result<Vec<_>, Error>>()?;
        
        Ok(BaseRecord::new(base_anchor_offsets, base_anchors))
    }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(BaseArray::new(
        base_count,
        base_records
    ))
}

fn parse_ligature_array(bytes: &[u8], subtable_offset: usize, ligature_array_offset: u16, mark_class_count: u16) -> Result<LigatureArray, Error> {
    let mut offset = subtable_offset + ligature_array_offset as usize;
    let ligature_count = get_u16(bytes, offset)?;
    offset += 2;
    let ligature_attach_offsets: Vec<u16> = bytes.get(offset..offset + ligature_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    let ligature_attaches: Vec<LigatureAttach> = ligature_attach_offsets.iter()
        .map(|offset| {
            Ok(parse_ligature_attach(bytes, subtable_offset, *offset, mark_class_count)?)
        }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(LigatureArray::new(
        ligature_count,
        ligature_attach_offsets,
        ligature_attaches
    ))
}

fn parse_ligature_attach(
    bytes: &[u8],
    subtable_offset: usize,
    ligature_attach_offset: u16,
    mark_class_count: u16
) -> Result<LigatureAttach, Error> {
    let mut offset = subtable_offset + ligature_attach_offset as usize;
    let component_count = get_u16(bytes, offset)?;
    offset += 2;
    let component_records: Vec<ComponentRecord> = (0..component_count).map(|_| {
        Ok(parse_component_record(bytes, subtable_offset, offset, mark_class_count)?)
    }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(LigatureAttach::new(
        component_count,
        component_records
    ))
}

fn parse_component_record(
    bytes: &[u8],
    subtable_offset: usize,
    offset: usize,
    mark_class_count: u16
) -> Result<ComponentRecord, Error> {
    let ligature_anchor_offsets: Vec<u16> = bytes.get(offset..offset + mark_class_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    let ligature_anchors: Vec<Anchor> = ligature_anchor_offsets.iter()
        .map(|offset| {
            Ok(parse_anchor(bytes, subtable_offset, *offset)?)
        }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(ComponentRecord::new(
        ligature_anchor_offsets,
        ligature_anchors
    ))
}

fn parse_mark2_array(bytes: &[u8], subtable_offset: usize, mark2_array_offset: u16, mark_class_count: u16) -> Result<Mark2Array, Error> {
    let mut offset = subtable_offset + mark2_array_offset as usize;
    let mark2_count = get_u16(bytes, offset)?;
    offset += 2;
    let mark2_records: Vec<Mark2Record> = (0..mark2_count).map(|_| {
        Ok(parse_mark2_record(bytes, subtable_offset, &mut offset, mark_class_count)?)
    }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(Mark2Array::new(
        mark2_count,
        mark2_records
    ))
}

fn parse_mark2_record(bytes: &[u8], subtable_offset: usize, offset: &mut usize, mark_class_count: u16) -> Result<Mark2Record, Error> {
    let mark2_anchor_offsets: Vec<u16> = bytes.get(*offset..*offset + mark_class_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    *offset += mark_class_count as usize * 2;
    let mark2_anchors: Vec<Anchor> = mark2_anchor_offsets.iter()
        .map(|offset| {
            Ok(parse_anchor(bytes, subtable_offset, *offset)?)
        }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(Mark2Record::new(
        mark2_anchor_offsets,
        mark2_anchors
    ))
}

fn parse_gpos_sub_rule_set(bytes: &[u8], subtable_offset: usize, sub_rule_set_offset: u16) -> Result<GposSubRuleSet, Error> {
    let mut offset = subtable_offset + sub_rule_set_offset as usize;
    let sub_rule_count = get_u16(bytes, offset)?;
    offset += 2;
    let sub_rule_offsets: Vec<u16> = bytes.get(offset..offset + sub_rule_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    let sub_rules: Vec<GposSubRule> = sub_rule_offsets.iter()
        .map(|offset| {
            Ok(parse_gpos_sub_rule(bytes, subtable_offset, *offset)?)
        }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(GposSubRuleSet::new(
        sub_rule_count,
        sub_rule_offsets,
        sub_rules
    ))
}

fn parse_gpos_sub_rule(bytes: &[u8], subtable_offset: usize, sub_rule_offset: u16) -> Result<GposSubRule, Error> {
    let mut offset = subtable_offset + sub_rule_offset as usize;
    let glyph_count = get_u16(bytes, offset)?;
    let sub_count = get_u16(bytes, offset + 2)?;
    offset += 4;
    let input_glyph_ids: Vec<u16> = bytes.get(offset..offset + (glyph_count as usize - 1) * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    offset += glyph_count as usize * 2;
    let pos_lookup_records: Vec<PosLookupRecord> = (0..sub_count).map(|_| {
        Ok(parse_pos_lookup_record(bytes, &mut offset)?)
    }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(GposSubRule::new(
        glyph_count,
        sub_count,
        input_glyph_ids,
        pos_lookup_records
    ))
}

fn parse_pos_lookup_record(bytes: &[u8], offset: &mut usize) -> Result<PosLookupRecord, Error> {
    let glyph_sequence_index = get_u16(bytes, *offset)?;
    let lookup_list_index = get_u16(bytes, *offset + 2)?;
    *offset += 4;
    
    Ok(PosLookupRecord::new(
        glyph_sequence_index,
        lookup_list_index
    ))
}

fn parse_gpos_sub_class_set(bytes: &[u8], subtable_offset: usize, sub_class_set_offset: u16) -> Result<GposSubClassSet, Error> {
    let mut offset = subtable_offset + sub_class_set_offset as usize;
    let sub_class_rule_count = get_u16(bytes, offset)?;
    offset += 2;
    let sub_class_rule_offsets: Vec<u16> = bytes.get(offset..offset + sub_class_rule_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    let sub_class_rules: Vec<GposSubClassRule> = sub_class_rule_offsets.iter()
        .map(|offset| {
            Ok(parse_gpos_sub_class_rule(bytes, subtable_offset, *offset)?)
        }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(GposSubClassSet::new(
        sub_class_rule_count,
        sub_class_rule_offsets,
        sub_class_rules
    ))
}

fn parse_gpos_sub_class_rule(bytes: &[u8], subtable_offset: usize, sub_class_rule_offset: u16) -> Result<GposSubClassRule, Error> {
    let mut offset = subtable_offset + sub_class_rule_offset as usize;
    let glyph_count = get_u16(bytes, offset)?;
    let sub_count = get_u16(bytes, offset + 2)?;
    offset += 4;
    let class_ids = bytes.get(offset..offset + glyph_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    offset += glyph_count as usize * 2;
    let pos_lookup_records: Vec<PosLookupRecord> = (0..sub_count).map(|_| {
        Ok(parse_pos_lookup_record(bytes, &mut offset)?)
    }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(GposSubClassRule::new(
        glyph_count,
        sub_count,
        class_ids,
        pos_lookup_records
    ))
}

fn parse_gpos_chain_sub_rule_set(bytes: &[u8], subtable_offset: usize, chain_sub_rule_set_offset: u16) -> Result<GposChainSubRuleSet, Error> {
    let mut offset = subtable_offset + chain_sub_rule_set_offset as usize;
    let chain_sub_rule_count = get_u16(bytes, offset)?;
    offset += 2;
    let chain_sub_rule_offsets: Vec<u16> = bytes.get(offset..offset + chain_sub_rule_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    let chain_sub_rules: Vec<GposChainSubRule> = chain_sub_rule_offsets.iter()
        .map(|offset| {
            Ok(parse_gpos_chain_sub_rule(bytes, subtable_offset, *offset)?)
        }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(GposChainSubRuleSet::new(
        chain_sub_rule_count,
        chain_sub_rule_offsets,
        chain_sub_rules
    ))
}

fn parse_gpos_chain_sub_class_set(bytes: &[u8], subtable_offset: usize, chain_sub_class_set_offset: u16) -> Result<GposChainSubClassSet, Error> {
    let mut offset = subtable_offset + chain_sub_class_set_offset as usize;
    let chain_sub_class_rule_count = get_u16(bytes, offset)?;
    offset += 2;
    let chain_sub_class_rule_offsets: Vec<u16> = bytes.get(offset..offset + chain_sub_class_rule_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    let chain_sub_class_rules: Vec<GposChainSubClassRule> = chain_sub_class_rule_offsets.iter()
        .map(|offset| {
            Ok(parse_gpos_chain_sub_class_rule(bytes, subtable_offset, *offset)?)
        }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(GposChainSubClassSet::new(
        chain_sub_class_rule_count,
        chain_sub_class_rule_offsets,
        chain_sub_class_rules
    ))
}

fn parse_gpos_chain_sub_class_rule(bytes: &[u8], subtable_offset: usize, chain_sub_class_rule_offset: u16) -> Result<GposChainSubClassRule, Error> {
    let mut offset = subtable_offset + chain_sub_class_rule_offset as usize;
    let backtrack_glyph_count = get_u16(bytes, offset)?;
    offset += 2;
    let backtrack_class_ids = bytes.get(offset..offset + backtrack_glyph_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    offset += backtrack_glyph_count as usize * 2;
    let input_glyph_count = get_u16(bytes, offset)?;
    offset += 2;
    let input_class_ids = bytes.get(offset..offset + input_glyph_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    offset += input_glyph_count as usize * 2;
    let lookahead_glyph_count = get_u16(bytes, offset)?;
    offset += 2;
    let lookahead_class_ids = bytes.get(offset..offset + lookahead_glyph_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    offset += lookahead_glyph_count as usize * 2;
    let sub_count = get_u16(bytes, offset)?;
    offset += 2;
    let pos_lookup_records = (0..sub_count).map(|_| {
        Ok(parse_pos_lookup_record(bytes, &mut offset)?)
    }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(GposChainSubClassRule::new(
        backtrack_glyph_count,
        backtrack_class_ids,
        input_glyph_count,
        input_class_ids,
        lookahead_glyph_count,
        lookahead_class_ids,
        sub_count,
        pos_lookup_records
    ))
}

fn parse_gpos_chain_sub_rule(bytes: &[u8], subtable_offset: usize, chain_sub_rule_offset: u16) -> Result<GposChainSubRule, Error> {
    let mut offset = subtable_offset + chain_sub_rule_offset as usize;
    let backtrack_glyph_count = get_u16(bytes, offset)?;
    offset += 2;
    let backtrack_glyph_ids = bytes.get(offset..offset + backtrack_glyph_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    offset += backtrack_glyph_count as usize * 2;
    let input_glyph_count = get_u16(bytes, offset)?;
    offset += 2;
    let input_glyph_ids = bytes.get(offset..offset + input_glyph_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    offset += input_glyph_count as usize * 2;
    let lookahead_glyph_count = get_u16(bytes, offset)?;
    offset += 2;
    let lookahead_glyph_ids = bytes.get(offset..offset + lookahead_glyph_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    offset += lookahead_glyph_count as usize * 2;
    let sub_count = get_u16(bytes, offset)?;
    offset += 2;
    let pos_lookup_records: Vec<PosLookupRecord> = (0..sub_count).map(|_| {
        Ok(parse_pos_lookup_record(bytes, &mut offset)?)
    }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(GposChainSubRule::new(
        backtrack_glyph_count,
        backtrack_glyph_ids,
        input_glyph_count,
        input_glyph_ids,
        lookahead_glyph_count,
        lookahead_glyph_ids,
        sub_count,
        pos_lookup_records
    ))
}

fn parse_sequence(bytes: &[u8], subtable_offset: usize, sequence_offset: u16) -> Result<Sequence, Error> {
    let mut offset = subtable_offset + sequence_offset as usize;
    let glyph_count = get_u16(bytes, offset)?;
    offset += 2;
    let substitute_glyph_ids: Vec<u16> = bytes.get(offset..offset + glyph_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    
    Ok(Sequence::new(
        glyph_count,
        substitute_glyph_ids
    ))
}

fn parse_alternate_set(bytes: &[u8], subtable_offset: usize, alternate_set_offset: u16) -> Result<AlternateSet, Error> {
    let mut offset = subtable_offset + alternate_set_offset as usize;
    let glyph_count = get_u16(bytes, offset)?;
    offset += 2;
    let alternate_glyph_ids = bytes.get(offset..offset + glyph_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    
    Ok(AlternateSet::new(
        glyph_count,
        alternate_glyph_ids
    ))
}

fn parse_ligature_set(bytes: &[u8], subtable_offset: usize, ligature_set_offset: u16) -> Result<LigatureSet, Error> {
    let mut offset = subtable_offset + ligature_set_offset as usize;
    let ligature_count = get_u16(bytes, offset)?;
    offset += 2;
    let ligature_offsets: Vec<u16> = bytes.get(offset..offset + ligature_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    let ligatures: Vec<Ligature> = ligature_offsets.iter()
        .map(|offset| {
            Ok(parse_ligature(bytes, subtable_offset, *offset)?)
        }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(LigatureSet::new(
        ligature_count,
        ligature_offsets,
        ligatures
    ))
}

fn parse_ligature(bytes: &[u8], subtable_offset: usize, ligature_offset: u16) -> Result<Ligature, Error> {
    let mut offset = subtable_offset + ligature_offset as usize;
    let ligature_glyph_id = get_u16(bytes, offset)?;
    let component_count = get_u16(bytes, offset + 2)?;
    offset += 4;
    let component_glyph_ids: Vec<u16> = bytes.get(offset..offset + component_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    
    Ok(Ligature::new(
        ligature_glyph_id,
        component_count,
        component_glyph_ids
    ))
}

fn parse_gsub_sub_rule_set(bytes: &[u8], subtable_offset: usize, sub_rule_set_offset: u16) -> Result<GsubSubRuleSet, Error> {
    let mut offset = subtable_offset + sub_rule_set_offset as usize;
    let sub_rule_count = get_u16(bytes, offset)?;
    offset += 2;
    let sub_rule_offsets: Vec<u16> = bytes.get(offset..offset + sub_rule_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    let sub_rules: Vec<GsubSubRule> = sub_rule_offsets.iter()
        .map(|offset| {
            Ok(parse_gsub_sub_rule(bytes, subtable_offset, *offset)?)
        }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(GsubSubRuleSet::new(
        sub_rule_count,
        sub_rule_offsets,
        sub_rules
    ))
}

fn parse_gsub_sub_rule(bytes: &[u8], subtable_offset: usize, sub_rule_offset: u16) -> Result<GsubSubRule, Error> {
    let mut offset = subtable_offset + sub_rule_offset as usize;
    let glyph_count = get_u16(bytes, offset)?;
    let sub_count = get_u16(bytes, offset + 2)?;
    offset += 4;
    let input_glyph_ids: Vec<u16> = bytes.get(offset..offset + (glyph_count as usize - 1) * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    offset += glyph_count as usize * 2;
    let subst_lookup_records: Vec<SubstLookupRecord> = (0..sub_count).map(|_| {
        Ok(parse_subst_lookup_record(bytes, &mut offset)?)
    }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(GsubSubRule::new(
        glyph_count,
        sub_count,
        input_glyph_ids,
        subst_lookup_records
    ))
}

fn parse_subst_lookup_record(bytes: &[u8], offset: &mut usize) -> Result<SubstLookupRecord, Error> {
    let glyph_sequence_index = get_u16(bytes, *offset)?;
    let lookup_list_index = get_u16(bytes, *offset + 2)?;
    *offset += 4;
    
    Ok(SubstLookupRecord::new(
        glyph_sequence_index,
        lookup_list_index
    ))
}

fn parse_gsub_sub_class_set(bytes: &[u8], subtable_offset: usize, sub_class_set_offset: u16) -> Result<GsubSubClassSet, Error> {
    let mut offset = subtable_offset + sub_class_set_offset as usize;
    let sub_class_rule_count = get_u16(bytes, offset)?;
    offset += 2;
    let sub_class_rule_offsets: Vec<u16> = bytes.get(offset..offset + sub_class_rule_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    let sub_class_rules: Vec<GsubSubClassRule> = sub_class_rule_offsets.iter()
        .map(|offset| {
            Ok(parse_gsub_sub_class_rule(bytes, subtable_offset, *offset)?)
        }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(GsubSubClassSet::new(
        sub_class_rule_count,
        sub_class_rule_offsets,
        sub_class_rules
    ))
}

fn parse_gsub_sub_class_rule(bytes: &[u8], subtable_offset: usize, sub_class_rule_offset: u16) -> Result<GsubSubClassRule, Error> {
    let mut offset = subtable_offset + sub_class_rule_offset as usize;
    let glyph_count = get_u16(bytes, offset)?;
    let sub_count = get_u16(bytes, offset + 2)?;
    offset += 4;
    let class_ids = bytes.get(offset..offset + glyph_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    offset += glyph_count as usize * 2;
    let subst_lookup_records: Vec<SubstLookupRecord> = (0..sub_count).map(|_| {
        Ok(parse_subst_lookup_record(bytes, &mut offset)?)
    }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(GsubSubClassRule::new(
        glyph_count,
        sub_count,
        class_ids,
        subst_lookup_records
    ))
}

fn parse_gsub_chain_sub_rule_set(bytes: &[u8], subtable_offset: usize, chain_sub_rule_set_offset: u16) -> Result<GsubChainSubRuleSet, Error> {
    let mut offset = subtable_offset + chain_sub_rule_set_offset as usize;
    let chain_sub_rule_count = get_u16(bytes, offset)?;
    offset += 2;
    let chain_sub_rule_offsets: Vec<u16> = bytes.get(offset..offset + chain_sub_rule_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    let chain_sub_rules: Vec<GsubChainSubRule> = chain_sub_rule_offsets.iter()
        .map(|offset| {
            Ok(parse_gsub_chain_sub_rule(bytes, subtable_offset, *offset)?)
        }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(GsubChainSubRuleSet::new(
        chain_sub_rule_count,
        chain_sub_rule_offsets,
        chain_sub_rules
    ))
}

fn parse_gsub_chain_sub_class_set(bytes: &[u8], subtable_offset: usize, chain_sub_class_set_offset: u16) -> Result<GsubChainSubClassSet, Error> {
    let mut offset = subtable_offset + chain_sub_class_set_offset as usize;
    let chain_sub_class_rule_count = get_u16(bytes, offset)?;
    offset += 2;
    let chain_sub_class_rule_offsets: Vec<u16> = bytes.get(offset..offset + chain_sub_class_rule_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    let chain_sub_class_rules: Vec<GsubChainSubClassRule> = chain_sub_class_rule_offsets.iter()
        .map(|offset| {
            Ok(parse_gsub_chain_sub_class_rule(bytes, subtable_offset, *offset)?)
        }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(GsubChainSubClassSet::new(
        chain_sub_class_rule_count,
        chain_sub_class_rule_offsets,
        chain_sub_class_rules
    ))
}

fn parse_gsub_chain_sub_class_rule(bytes: &[u8], subtable_offset: usize, chain_sub_class_rule_offset: u16) -> Result<GsubChainSubClassRule, Error> {
    let mut offset = subtable_offset + chain_sub_class_rule_offset as usize;
    let backtrack_glyph_count = get_u16(bytes, offset)?;
    offset += 2;
    let backtrack_class_ids = bytes.get(offset..offset + backtrack_glyph_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    offset += backtrack_glyph_count as usize * 2;
    let input_glyph_count = get_u16(bytes, offset)?;
    offset += 2;
    let input_class_ids = bytes.get(offset..offset + input_glyph_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    offset += input_glyph_count as usize * 2;
    let lookahead_glyph_count = get_u16(bytes, offset)?;
    offset += 2;
    let lookahead_class_ids = bytes.get(offset..offset + lookahead_glyph_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    offset += lookahead_glyph_count as usize * 2;
    let sub_count = get_u16(bytes, offset)?;
    offset += 2;
    let subst_lookup_records = (0..sub_count).map(|_| {
        Ok(parse_subst_lookup_record(bytes, &mut offset)?)
    }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(GsubChainSubClassRule::new(
        backtrack_glyph_count,
        backtrack_class_ids,
        input_glyph_count,
        input_class_ids,
        lookahead_glyph_count,
        lookahead_class_ids,
        sub_count,
        subst_lookup_records
    ))
}

fn parse_gsub_chain_sub_rule(bytes: &[u8], subtable_offset: usize, chain_sub_rule_offset: u16) -> Result<GsubChainSubRule, Error> {
    let mut offset = subtable_offset + chain_sub_rule_offset as usize;
    let backtrack_glyph_count = get_u16(bytes, offset)?;
    offset += 2;
    let backtrack_glyph_ids = bytes.get(offset..offset + backtrack_glyph_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    offset += backtrack_glyph_count as usize * 2;
    let input_glyph_count = get_u16(bytes, offset)?;
    offset += 2;
    let input_glyph_ids = bytes.get(offset..offset + input_glyph_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    offset += input_glyph_count as usize * 2;
    let lookahead_glyph_count = get_u16(bytes, offset)?;
    offset += 2;
    let lookahead_glyph_ids = bytes.get(offset..offset + lookahead_glyph_count as usize * 2)
        .ok_or(ErrorKind::UnexpectedEof)?
        .chunks_exact(2)
        .map(|ch| {
            u16::from_be_bytes(ch.try_into().unwrap())
        }).collect();
    offset += lookahead_glyph_count as usize * 2;
    let sub_count = get_u16(bytes, offset)?;
    offset += 2;
    let subst_lookup_records: Vec<SubstLookupRecord> = (0..sub_count).map(|_| {
        Ok(parse_subst_lookup_record(bytes, &mut offset)?)
    }).collect::<Result<Vec<_>, Error>>()?;
    
    Ok(GsubChainSubRule::new(
        backtrack_glyph_count,
        backtrack_glyph_ids,
        input_glyph_count,
        input_glyph_ids,
        lookahead_glyph_count,
        lookahead_glyph_ids,
        sub_count,
        subst_lookup_records
    ))
}

fn get_u16(bytes: &[u8], start: usize) -> Result<u16, Error> {
    Ok(
        u16::from_be_bytes(
        bytes.get(start..start + 2)
            .ok_or(ErrorKind::UnexpectedEof)?
            .try_into()
            .unwrap()
        )
    )
}

fn get_u32(bytes: &[u8], start: usize) -> Result<u32, Error> {
    Ok(
        u32::from_be_bytes(
            bytes.get(start..start + 4)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        )
    )
}

fn get_i16(bytes: &[u8], start: usize) -> Result<i16, Error> {
    Ok(
        i16::from_be_bytes(
            bytes.get(start..start + 2)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        )
    )
}

fn get_i32(bytes: &[u8], start: usize) -> Result<i32, Error> {
    Ok(
        i32::from_be_bytes(
            bytes.get(start..start + 4)
                .ok_or(ErrorKind::UnexpectedEof)?
                .try_into()
                .unwrap()
        )
    )
}

fn get_i64(bytes: &[u8], start: usize) -> Result<i64, Error> {
    Ok(
        i64::from_be_bytes(
            bytes.get(start..start + 8)
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