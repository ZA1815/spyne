pub struct FontFile {
    file_type: FontFileType,
    bytes: Vec<u8>,
    table_records: Vec<TableRecord>
}

impl FontFile {
    pub(super) fn new(file_type: FontFileType, bytes: Vec<u8>, table_records: Vec<TableRecord>) -> Self {
        Self {
            file_type,
            bytes,
            table_records
        }
    }
    
    pub fn file_type(&self) -> FontFileType {
        self.file_type
    }
    
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    
    pub fn table_records(&self) -> &[TableRecord] {
        &self.table_records
    }
}

#[derive(Clone, Copy)]
pub enum FontFileType {
    TrueType,
    OpenType
}

#[derive(Clone, Copy)]
pub struct TableRecord {
    tag: [u8; 4],
    checksum: u32,
    offset: u32,
    length: u32
}

impl TableRecord {
    pub(super) fn new(tag: [u8; 4], checksum: u32, offset: u32, length: u32) -> Self {
        Self { tag, checksum, offset, length }
    }
    
    pub fn tag(&self) -> &[u8; 4] {
        &self.tag
    }
    
    pub fn checksum(&self) -> u32 {
        self.checksum
    }
    
    pub fn offset(&self) -> u32 {
        self.offset
    }
    
    pub fn length(&self) -> u32 {
        self.length
    }
}

pub struct HeadTable {
    units_per_em: u16,
    created: i64,
    modified: i64,
    x_min: i16,
    y_min: i16,
    x_max: i16,
    y_max: i16,
    mac_style: u16,
    lowest_rec_ppem: u16,
    font_direction_hint: i16,
    index_to_loc_format: i16
}

impl HeadTable {
    pub(super) fn new(
        units_per_em: u16,
        created: i64,
        modified: i64,
        x_min: i16,
        y_min: i16,
        x_max: i16,
        y_max: i16,
        mac_style: u16,
        lowest_rec_ppem: u16,
        font_direction_hint: i16,
        index_to_loc_format: i16
    ) -> Self {
        Self {
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
        }
    }
    
    pub fn units_per_em(&self) -> u16 {
        self.units_per_em
    }
    
    pub fn created(&self) -> i64 {
        self.created
    }
    
    pub fn modified(&self) -> i64 {
        self.modified
    }
    
    pub fn x_min(&self) -> i16 {
        self.x_min
    }
    
    pub fn y_min(&self) -> i16 {
        self.y_min
    }
    
    pub fn x_max(&self) -> i16 {
        self.x_max
    }
    
    pub fn y_max(&self) -> i16 {
        self.y_max
    }
    
    pub fn mac_style(&self) -> u16 {
        self.mac_style
    }
    
    pub fn lowest_rec_ppem(&self) -> u16 {
        self.lowest_rec_ppem
    }
    
    pub fn font_direction_hint(&self) -> i16 {
        self.font_direction_hint
    }
    
    pub fn index_to_loc_format(&self) -> i16 {
        self.index_to_loc_format
    }
}

pub struct MaxpTable {
    version: u32,
    num_glyphs: u16,
    max_points: Option<u16>,
    max_contours: Option<u16>,
    max_composite_points: Option<u16>,
    max_composite_contours: Option<u16>,
    max_zones: Option<u16>,
    max_twilight_points: Option<u16>,
    max_storage: Option<u16>,
    max_function_defs: Option<u16>,
    max_instruction_defs: Option<u16>,
    max_stack_elements: Option<u16>,
    max_size_of_instructions: Option<u16>,
    max_components_elements: Option<u16>,
    max_component_depth: Option<u16>
}

impl MaxpTable {
    pub(super) fn new(
        version: u32,
        num_glyphs: u16,
        max_points: Option<u16>,
        max_contours: Option<u16>,
        max_composite_points: Option<u16>,
        max_composite_contours: Option<u16>,
        max_zones: Option<u16>,
        max_twilight_points: Option<u16>,
        max_storage: Option<u16>,
        max_function_defs: Option<u16>,
        max_instruction_defs: Option<u16>,
        max_stack_elements: Option<u16>,
        max_size_of_instructions: Option<u16>,
        max_components_elements: Option<u16>,
        max_component_depth: Option<u16>
    ) -> Self {
        Self {
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
        }
    }
    
    pub fn version(&self) -> u32 {
        self.version
    }
    
    pub fn num_glyphs(&self) -> u16 {
        self.num_glyphs
    }
    
    pub fn max_points(&self) -> Option<u16> {
        self.max_points
    }
    
    pub fn max_contours(&self) -> Option<u16> {
        self.max_contours
    }
    
    pub fn max_composite_points(&self) -> Option<u16> {
        self.max_composite_points
    }
    
    pub fn max_composite_contours(&self) -> Option<u16> {
        self.max_composite_contours
    }
    
    pub fn max_zones(&self) -> Option<u16> {
        self.max_zones
    }
    
    pub fn max_twilight_points(&self) -> Option<u16> {
        self.max_twilight_points
    }
    
    pub fn max_storage(&self) -> Option<u16> {
        self.max_storage
    }
    
    pub fn max_function_defs(&self) -> Option<u16> {
        self.max_function_defs
    }
    
    pub fn max_instruction_defs(&self) -> Option<u16> {
        self.max_instruction_defs
    }
    
    pub fn max_stack_elements(&self) -> Option<u16> {
        self.max_stack_elements
    }
    
    pub fn max_size_of_instructions(&self) -> Option<u16> {
        self.max_size_of_instructions
    }
    
    pub fn max_component_elements(&self) -> Option<u16> {
        self.max_components_elements
    }
    
    pub fn max_component_depth(&self) -> Option<u16> {
        self.max_component_depth
    }
}

pub struct CmapTable {
    version: u16,
    num_tables: u16,
    encoding_records: Vec<EncodingRecord>,
    subtables: Vec<CmapSubtable>
}

impl CmapTable {
    pub(super) fn new(
        version: u16,
        num_tables: u16,
        encoding_records: Vec<EncodingRecord>,
        subtables: Vec<CmapSubtable>
    ) -> Self {
        Self {
            version,
            num_tables,
            encoding_records,
            subtables
        }
    }
    
    pub fn version(&self) -> u16 {
        self.version
    }
    
    pub fn num_tables(&self) -> u16 {
        self.num_tables
    }
    
    pub fn encoding_records(&self) -> &[EncodingRecord] {
        &self.encoding_records
    }
    
    pub fn subtables(&self) -> &[CmapSubtable] {
        &self.subtables
    }
}

pub struct EncodingRecord {
    platform_id: u16,
    encoding_id: u16,
    offset: u32
}

impl EncodingRecord {
    pub(super) fn new(
        platform_id: u16,
        encoding_id: u16,
        offset: u32
    ) -> Self {
        Self {
            platform_id,
            encoding_id,
            offset
        }
    }
    
    pub fn platform_id(&self) -> u16 {
        self.platform_id
    }
    
    pub fn encoding_id(&self) -> u16 {
        self.encoding_id
    }
    
    pub fn offset(&self) -> u32 {
        self.offset
    }
}

#[derive(Clone, PartialEq, Eq)]
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SubHeader {
    first_code: u16,
    entry_count: u16,
    id_delta: i16,
    id_range_offset: u16
}

impl SubHeader {
    pub(super) fn new(
        first_code: u16,
        entry_count: u16,
        id_delta: i16,
        id_range_offset: u16
    ) -> Self {
        Self {
            first_code,
            entry_count,
            id_delta,
            id_range_offset
        }
    }
    
    pub fn first_code(&self) -> u16 {
        self.first_code
    }
    
    pub fn entry_count(&self) -> u16 {
        self.entry_count
    }
    
    pub fn id_delta(&self) -> i16 {
        self.id_delta
    }
    
    pub fn id_range_offset(&self) -> u16 {
        self.id_range_offset
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Group {
    start_char_code: u32,
    end_char_code: u32,
    start_glyph_id: u32
}

impl Group {
    pub(super) fn new(
        start_char_code: u32,
        end_char_code: u32,
        start_glyph_id: u32
    ) -> Self {
        Self {
            start_char_code,
            end_char_code,
            start_glyph_id
        }
    }
    
    pub fn start_char_code(&self) -> u32 {
        self.start_char_code
    }
    
    pub fn end_char_code(&self) -> u32 {
        self.end_char_code
    }
    
    pub fn start_glyph_id(&self) -> u32 {
        self.start_glyph_id
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VariationSelectorRecord {
    var_selector: [u8; 3],
    default_uvs_offset: u32,
    non_default_uvs_offset: u32
}

impl VariationSelectorRecord {
    pub(super) fn new(
        var_selector: [u8; 3],
        default_uvs_offset: u32,
        non_default_uvs_offset: u32
    ) -> Self {
        Self {
            var_selector,
            default_uvs_offset,
            non_default_uvs_offset
        }
    }
    
    pub fn var_selector(&self) -> &[u8; 3] {
        &self.var_selector
    }
    
    pub fn default_uvs_offset(&self) -> u32 {
        self.default_uvs_offset
    }
    
    pub fn non_default_uvs_offset(&self) -> u32 {
        self.non_default_uvs_offset
    }
}

#[derive(Clone)]
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

#[derive(Clone, Copy, Default)]
pub struct GlyphHeader {
    number_of_contours: i16,
    x_min: i16,
    y_min: i16,
    x_max: i16,
    y_max: i16
}

impl GlyphHeader {
    pub(super) fn new(
        number_of_contours: i16,
        x_min: i16,
        y_min: i16,
        x_max: i16,
        y_max: i16
    ) -> Self {
        Self {
            number_of_contours,
            x_min,
            y_min,
            x_max,
            y_max
        }
    }
    
    pub fn number_of_contours(&self) -> i16 {
        self.number_of_contours
    }
    
    pub fn x_min(&self) -> i16 {
        self.x_min
    }
    
    pub fn y_min(&self) -> i16 {
        self.y_min
    }
    
    pub fn x_max(&self) -> i16 {
        self.x_max
    }
    
    pub fn y_max(&self) -> i16 {
        self.y_max
    }
}

#[derive(Clone, Copy)]
pub struct Component {
    flags: u16,
    glyph_index: u16,
    argument_1: i16,
    argument_2: i16,
    transformation: [i16; 4]
}

impl Component {
    pub(super) fn new(
        flags: u16,
        glyph_index: u16,
        argument_1: i16,
        argument_2: i16,
        transformation: [i16; 4]
    ) -> Self {
        Self {
            flags,
            glyph_index,
            argument_1,
            argument_2,
            transformation
        }
    }
    
    pub fn flags(&self) -> u16 {
        self.flags
    }
    
    pub fn glyph_index(&self) -> u16 {
        self.glyph_index
    }
    
    pub fn argument_1(&self) -> i16 {
        self.argument_1
    }
    
    pub fn argument_2(&self) -> i16 {
        self.argument_2
    }
    
    pub fn transformation(&self) -> &[i16; 4] {
        &self.transformation
    }
}

pub struct HheaTable {
    version: u32,
    ascender: i16,
    descender: i16,
    line_gap: i16,
    advance_width_max: u16,
    min_left_side_bearing: i16,
    min_right_side_bearing: i16,
    x_max_extent: i16,
    caret_slope_rise: i16,
    caret_slope_run: i16,
    caret_offset: i16,
    _reserved1: i16,
    _reserved2: i16,
    _reserved3: i16,
    _reserved4: i16,
    metric_data_format: i16,
    number_of_h_metrics: u16
}

impl HheaTable {
    pub(super) fn new(
        version: u32,
        ascender: i16,
        descender: i16,
        line_gap: i16,
        advance_width_max: u16,
        min_left_side_bearing: i16,
        min_right_side_bearing: i16,
        x_max_extent: i16,
        caret_slope_rise: i16,
        caret_slope_run: i16,
        caret_offset: i16,
        _reserved1: i16,
        _reserved2: i16,
        _reserved3: i16,
        _reserved4: i16,
        metric_data_format: i16,
        number_of_h_metrics: u16
    ) -> Self {
        Self {
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
        }
    }
    
    pub fn version(&self) -> u32 {
        self.version
    }
    
    pub fn ascender(&self) -> i16 {
        self.ascender
    }
    
    pub fn descender(&self) -> i16 {
        self.descender
    }
    
    pub fn line_gap(&self) -> i16 {
        self.line_gap
    }
    
    pub fn advance_width_max(&self) -> u16 {
        self.advance_width_max
    }
    
    pub fn min_left_side_bearing(&self) -> i16 {
        self.min_left_side_bearing
    }
    
    pub fn min_right_side_bearing(&self) -> i16 {
        self.min_right_side_bearing
    }
    
    pub fn x_max_extent(&self) -> i16 {
        self.x_max_extent
    }
    
    pub fn caret_slope_rise(&self) -> i16 {
        self.caret_slope_rise
    }
    
    pub fn caret_slope_run(&self) -> i16 {
        self.caret_slope_run
    }
    
    pub fn caret_offset(&self) -> i16 {
        self.caret_offset
    }
    
    pub fn metric_data_format(&self) -> i16 {
        self.metric_data_format
    }
    
    pub fn number_of_h_metrics(&self) -> u16 {
        self.number_of_h_metrics
    }
}

pub struct HmtxTable {
    entries: Vec<HmtxEntry>,
    shared_advance_width: u16
}

impl HmtxTable {
    pub(super) fn new(
        entries: Vec<HmtxEntry>,
        shared_advance_width: u16
    ) -> Self {
        Self {
            entries,
            shared_advance_width
        }
    }
    
    pub fn entries(&self) -> &[HmtxEntry] {
        &self.entries
    }
    
    pub fn shared_advance_width(&self) -> u16 {
        self.shared_advance_width
    }
}

pub enum HmtxEntry {
    FullMetric {
        advance_width: u16,
        lsb: i16
    },
    LeftoverBearing(i16)
}

pub struct NameTable {
    version: u16,
    count: u16,
    storage_offset: u16,
    records: Vec<NameRecord>,
    lang_tag_count: Option<u16>,
    lang_tag_records: Option<Vec<LangTagRecord>>
}

impl NameTable {
    pub(super) fn new(
        version: u16,
        count: u16,
        storage_offset: u16,
        records: Vec<NameRecord>,
        lang_tag_count: Option<u16>,
        lang_tag_records: Option<Vec<LangTagRecord>>
    ) -> Self {
        Self {
            version,
            count,
            storage_offset,
            records,
            lang_tag_count,
            lang_tag_records
        }
    }
    
    pub fn version(&self) -> u16 {
        self.version
    }
    
    pub fn count(&self) -> u16 {
        self.count
    }
    
    pub fn storage_offset(&self) -> u16 {
        self.storage_offset
    }
    
    pub fn records(&self) -> &[NameRecord] {
        &self.records
    }
    
    pub fn lang_tag_count(&self) -> Option<u16> {
        self.lang_tag_count
    }
    
    pub fn lang_tag_records(&self) -> Option<&[LangTagRecord]> {
        self.lang_tag_records.as_deref()
    }
}

pub struct NameRecord {
    platform_id: u16,
    encoding_id: u16,
    language_id: u16,
    name_id: u16,
    length: u16,
    string_offset: u16,
    string: String
}

impl NameRecord {
    pub(super) fn new(
        platform_id: u16,
        encoding_id: u16,
        language_id: u16,
        name_id: u16,
        length: u16,
        string_offset: u16,
        string: String
    ) -> Self {
        Self {
            platform_id,
            encoding_id,
            language_id,
            name_id,
            length,
            string_offset,
            string
        }
    }
    
    pub fn platform_id(&self) -> u16 {
        self.platform_id
    }
    
    pub fn encoding_id(&self) -> u16 {
        self.encoding_id
    }
    
    pub fn language_id(&self) -> u16 {
        self.language_id
    }
    
    pub fn name_id(&self) -> u16 {
        self.name_id
    }
    
    pub fn length(&self) -> u16 {
        self.length
    }
    
    pub fn string_offset(&self) -> u16 {
        self.string_offset
    }
    
    pub fn string(&self) -> &str {
        &self.string
    }
}

pub struct LangTagRecord {
    length: u16,
    lang_tag_offset: u16,
    string: String
}

impl LangTagRecord {
    pub(super) fn new(
        length: u16,
        lang_tag_offset: u16,
        string: String
    ) -> Self {
        Self {
            length,
            lang_tag_offset,
            string
        }
    }
    
    pub fn length(&self) -> u16 {
        self.length
    }
    
    pub fn lang_tag_offset(&self) -> u16 {
        self.lang_tag_offset
    }
    
    pub fn string(&self) -> &str {
        &self.string
    }
}

pub struct OS2Table {
    version: u16,
    x_avg_char_width: i16,
    us_weight_class: u16,
    us_width_class: u16,
    fs_type: u16,
    y_subscript_x_size: i16,
    y_subscript_y_size: i16,
    y_subscript_x_offset: i16,
    y_subscript_y_offset: i16,
    y_superscript_x_size: i16,
    y_superscript_y_size: i16,
    y_superscript_x_offset: i16,
    y_superscript_y_offset: i16,
    y_strikeout_size: i16,
    y_strikeout_position: i16,
    s_family_class: i16,
    panose: [u8; 10],
    ul_unicode_range_1: u32,
    ul_unicode_range_2: u32,
    ul_unicode_range_3: u32,
    ul_unicode_range_4: u32,
    ach_vend_id: [u8; 4],
    fs_selection: u16,
    us_first_char_index: u16,
    us_last_char_index: u16,
    s_typo_ascender: i16,
    s_typo_descender: i16,
    s_typo_line_gap: i16,
    us_win_ascent: u16,
    us_win_descent: u16,
    // Version 1 Additions
    ul_code_page_range_1: Option<u32>,
    ul_code_page_range_2: Option<u32>,
    // Version 2 Additions
    sx_height: Option<i16>,
    s_cap_height: Option<i16>,
    us_default_char: Option<u16>,
    us_break_char: Option<u16>,
    us_max_context: Option<u16>,
    // Version 5 Additions
    us_lower_optical_point_size: Option<u16>,
    us_upper_optical_point_size: Option<u16>
}

impl OS2Table {
    pub(super) fn new(
        version: u16,
        x_avg_char_width: i16,
        us_weight_class: u16,
        us_width_class: u16,
        fs_type: u16,
        y_subscript_x_size: i16,
        y_subscript_y_size: i16,
        y_subscript_x_offset: i16,
        y_subscript_y_offset: i16,
        y_superscript_x_size: i16,
        y_superscript_y_size: i16,
        y_superscript_x_offset: i16,
        y_superscript_y_offset: i16,
        y_strikeout_size: i16,
        y_strikeout_position: i16,
        s_family_class: i16,
        panose: [u8; 10],
        ul_unicode_range_1: u32,
        ul_unicode_range_2: u32,
        ul_unicode_range_3: u32,
        ul_unicode_range_4: u32,
        ach_vend_id: [u8; 4],
        fs_selection: u16,
        us_first_char_index: u16,
        us_last_char_index: u16,
        s_typo_ascender: i16,
        s_typo_descender: i16,
        s_typo_line_gap: i16,
        us_win_ascent: u16,
        us_win_descent: u16,
        ul_code_page_range_1: Option<u32>,
        ul_code_page_range_2: Option<u32>,
        sx_height: Option<i16>,
        s_cap_height: Option<i16>,
        us_default_char: Option<u16>,
        us_break_char: Option<u16>,
        us_max_context: Option<u16>,
        us_lower_optical_point_size: Option<u16>,
        us_upper_optical_point_size: Option<u16>
    ) -> Self {
        Self {
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
        }
    }
    
    pub fn version(&self) -> u16 {
        self.version
    }
    
    pub fn x_avg_char_width(&self) -> i16 {
        self.x_avg_char_width
    }
    
    pub fn us_weight_class(&self) -> u16 {
        self.us_weight_class
    }
    
    pub fn us_width_class(&self) -> u16 {
        self.us_width_class
    }
    
    pub fn fs_type(&self) -> u16 {
        self.fs_type
    }
    
    pub fn y_subscript_x_size(&self) -> i16 {
        self.y_subscript_x_size
    }
    
    pub fn y_subscript_y_size(&self) -> i16 {
        self.y_subscript_y_size
    }
    
    pub fn y_subscript_x_offset(&self) -> i16 {
        self.y_subscript_x_offset
    }
    
    pub fn y_subscript_y_offset(&self) -> i16 {
        self.y_subscript_y_offset
    }
    
    pub fn y_superscript_x_size(&self) -> i16 {
        self.y_superscript_x_size
    }
    
    pub fn y_superscript_y_size(&self) -> i16 {
        self.y_superscript_y_size
    }
    
    pub fn y_superscript_x_offset(&self) -> i16 {
        self.y_superscript_x_offset
    }
    
    pub fn y_superscript_y_offset(&self) -> i16 {
        self.y_superscript_y_offset
    }
    
    pub fn y_strikeout_size(&self) -> i16 {
        self.y_strikeout_size
    }
    
    pub fn y_strikeout_position(&self) -> i16 {
        self.y_strikeout_position
    }
    
    pub fn s_family_class(&self) -> i16 {
        self.s_family_class
    }
    
    pub fn panose(&self) -> &[u8; 10] {
        &self.panose
    }
    
    pub fn ul_unicode_range_1(&self) -> u32 {
        self.ul_unicode_range_1
    }
    
    pub fn ul_unicode_range_2(&self) -> u32 {
        self.ul_unicode_range_2
    }
    
    pub fn ul_unicode_range_3(&self) -> u32 {
        self.ul_unicode_range_3
    }
    
    pub fn ul_unicode_range_4(&self) -> u32 {
        self.ul_unicode_range_4
    }
    
    pub fn ach_vend_id(&self) -> &[u8; 4] {
        &self.ach_vend_id
    }
    
    pub fn fs_selection(&self) -> u16 {
        self.fs_selection
    }
    
    pub fn us_first_char_index(&self) -> u16 {
        self.us_first_char_index
    }
    
    pub fn us_last_char_index(&self) -> u16 {
        self.us_last_char_index
    }
    
    pub fn s_typo_ascender(&self) -> i16 {
        self.s_typo_ascender
    }
    
    pub fn s_typo_descender(&self) -> i16 {
        self.s_typo_descender
    }
    
    pub fn s_typo_line_gap(&self) -> i16 {
        self.s_typo_line_gap
    }
    
    pub fn us_win_ascent(&self) -> u16 {
        self.us_win_ascent
    }
    
    pub fn us_win_descent(&self) -> u16 {
        self.us_win_descent
    }
    
    pub fn ul_code_page_range_1(&self) -> Option<u32> {
        self.ul_code_page_range_1
    }
    
    pub fn ul_code_page_range_2(&self) -> Option<u32> {
        self.ul_code_page_range_2
    }
    
    pub fn sx_height(&self) -> Option<i16> {
        self.sx_height
    }
    
    pub fn s_cap_height(&self) -> Option<i16> {
        self.s_cap_height
    }
    
    pub fn us_default_char(&self) -> Option<u16> {
        self.us_default_char
    }
    
    pub fn us_break_char(&self) -> Option<u16>  {
        self.us_break_char
    }
    
    pub fn us_max_context(&self) -> Option<u16> {
        self.us_max_context
    }
    
    pub fn us_lower_optical_point_size(&self) -> Option<u16> {
        self.us_lower_optical_point_size
    }
    
    pub fn us_upper_optical_point_size(&self) -> Option<u16> {
        self.us_upper_optical_point_size
    }
}

pub struct PostTable {
    version: u32,
    italic_angle: i32,
    underline_position: i16,
    underline_thickness: i16,
    is_fixed_pitch: u32,
    min_mem_type_42: u32,
    max_mem_type_42: u32,
    min_mem_type_1: u32,
    max_mem_type_1: u32,
    num_glyphs: Option<u16>,
    glyph_name_index: Option<Vec<u16>>,
    names: Option<Vec<String>>
}

impl PostTable {
    pub(super) fn new(
        version: u32,
        italic_angle: i32,
        underline_position: i16,
        underline_thickness: i16,
        is_fixed_pitch: u32,
        min_mem_type_42: u32,
        max_mem_type_42: u32,
        min_mem_type_1: u32,
        max_mem_type_1: u32,
        num_glyphs: Option<u16>,
        glyph_name_index: Option<Vec<u16>>,
        names: Option<Vec<String>>
    ) -> Self {
        Self {
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
        }
    }
    
    pub fn () {
        
    }
}

pub struct VheaTable {
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
    pub _reserved1: i16,
    pub _reserved2: i16,
    pub _reserved3: i16,
    pub _reserved4: i16,
    pub metric_data_format: i16,
    pub num_of_long_ver_metrics: u16
}

pub struct VmtxTable {
    pub entries: Vec<VmtxEntry>,
    pub shared_advance_height: u16
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

pub struct WindowsSubtable {
    pub version: u16,
    pub length: u16,
    pub coverage: u16,
    pub subtable: KernSubtable
}

pub struct MacSubtable {
    pub length: u32,
    pub coverage: u16,
    pub tuple_index: u16,
    pub subtable: KernSubtable
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

pub struct KernPair {
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

pub struct Range {
    pub start_glyph: u16,
    pub end_glyph: u16,
    pub class: u16
}

pub struct GaspTable {
    pub version: u16,
    pub num_ranges: u16,
    pub range_records: Vec<GaspRangeRecord>
}

pub struct GaspRangeRecord {
    pub range_max_ppem: u16,
    pub range_gasp_behavior: u16
}

pub struct GposTable<GposSubtable> {
    pub header: TableHeader,
    pub script_list: ScriptList,
    pub feature_list: FeatureList,
    pub lookup_list: LookupList<GposSubtable>,
    pub feature_variations: Option<FeatureVariations>
}

pub struct GsubTable<GsubSubtable> {
    pub header: TableHeader,
    pub script_list: ScriptList,
    pub feature_list: FeatureList,
    pub lookup_list: LookupList<GsubSubtable>,
    pub feature_variations: Option<FeatureVariations>
}

pub struct TableHeader {
    pub major_version: u16,
    pub minor_version: u16,
    pub script_list_offset: u16,
    pub feature_list_offset: u16,
    pub lookup_list_offset: u16,
    pub feature_variations_offset: Option<u32>
}

pub struct ScriptList {
    pub script_count: u16,
    pub script_records: Vec<ScriptRecord>,
    pub scripts: Vec<Script>
}

pub struct ScriptRecord {
    pub script_tag: [u8; 4],
    pub script_offset: u16
}

pub struct Script {
    pub default_lang_sys_offset: Option<u16>,
    pub default_lang_sys: Option<LangSys>,
    pub lang_sys_count: u16,
    pub lang_sys_records: Vec<LangSysRecord>,
    pub lang_syses: Vec<LangSys>
}

pub struct LangSysRecord {
    pub lang_sys_tag: [u8; 4],
    pub lang_sys_offset: u16,
}

pub struct LangSys {
    pub _lookup_order_offset: u16,
    pub required_feature_index: u16,
    pub feature_index_count: u16,
    pub feature_indices: Vec<u16>
}

#[derive(Clone)]
pub struct FeatureList {
    pub feature_count: u16,
    pub feature_records: Vec<FeatureRecord>,
    pub features: Vec<Feature>
}

#[derive(Clone)]
pub struct FeatureRecord {
    pub feature_tag: [u8; 4],
    pub feature_offset: u16
}

#[derive(Clone)]
pub struct Feature {
    pub feature_params_offset: Option<u16>,
    pub feature_params: Option<FeatureParams>,
    pub lookup_index_count: u16,
    pub lookup_list_indices: Vec<u16>
}

#[derive(Clone)]
pub enum FeatureParams {
    Size {
        design_size: u16,
        subfamily_id: u16,
        subfamily_name_id: u16,
        range_start: u16,
        range_end: u16
    },
    StylisticSet {
        version: u16,
        ui_name_id: u16
    },
    CharacterVariant {
        format: u16,
        feat_ui_label_name_id: u16,
        feat_tooltip_text_name_id: u16,
        sample_text_name_id: u16,
        num_named_parameters: u16,
        first_param_ui_label_name_id: u16,
        char_count: u16,
        character: [u8; 3]
    }
}

pub struct LookupList<T> {
    pub lookup_count: u16,
    pub lookup_offsets: Vec<u16>,
    pub lookups: Vec<Lookup<T>>
}

pub struct Lookup<T> {
    pub lookup_type: u16,
    pub lookup_flag: u16,
    pub subtable_count: u16,
    pub subtable_offsets: Vec<u16>,
    pub subtables: Vec<T>,
    pub mark_filtering_set: Option<u16>
}

pub struct FeatureVariations {
    pub major_version: u16,
    pub minor_version: u16,
    pub feature_variation_record_count: u32,
    pub feature_variation_records: Vec<FeatureVariationRecord>
}

pub struct FeatureVariationRecord {
    pub condition_set_offset: u32,
    pub condition_set: ConditionSet,
    pub feature_table_substitution_offset: u32,
    pub feature_table_substitution: FeatureTableSubstitution
}

pub struct ConditionSet {
    pub condition_count: u16,
    pub condition_offsets: Vec<u32>,
    pub conditions: Vec<Condition>
}

pub enum Condition {
    Format1 {
        axis_index: u16,
        filter_range_min_value: i16,
        filter_range_max_value: i16
    }
}

pub struct FeatureTableSubstitution {
    pub major_version: u16,
    pub minor_version: u16,
    pub substitution_count: u16,
    pub substitution_records: Vec<FeatureTableSubstitutionRecord>
}

pub struct FeatureTableSubstitutionRecord {
    pub feature_index: u16,
    pub alternate_feature_table_offset: u32,
    pub alternate_feature_table: Feature
}

pub enum Coverage {
    Format1 {
        glyph_count: u16,
        glyph_array: Vec<u16>
    },
    Format2 {
        range_count: u16,
        range_records: Vec<CoverageRangeRecord>
    }
}

pub struct CoverageRangeRecord {
    pub start_glyph_id: u16,
    pub end_glyph_id: u16,
    pub start_coverage_index: u16
}

pub enum ClassDef {
    Format1 {
        start_glyph_id: u16,
        glyph_count: u16,
        class_value_array: Vec<u16>
    },
    Format2 {
        class_range_count: u16,
        class_range_records: Vec<ClassRangeRecord>
    }
}

pub struct ClassRangeRecord {
    pub start_glyph_id: u16,
    pub end_glyph_id: u16,
    pub class: u16
}

pub struct Device {
    pub start_size: u16,
    pub end_size: u16,
    pub delta_format: u16,
    pub delta_values: Vec<u16>
}

pub struct VariationIndexTable {
    pub delta_set_outer_index: u16,
    pub delta_set_inner_index: u16,
    pub delta_format: u16
}

pub struct ValueRecord {
    pub x_placement: Option<i16>,
    pub y_placement: Option<i16>,
    pub x_advance: Option<i16>,
    pub y_advance: Option<i16>,
    pub x_pla_device_offset: Option<u16>,
    pub x_pla_device: Option<Device>,
    pub y_pla_device_offset: Option<u16>,
    pub y_pla_device: Option<Device>,
    pub x_adv_device_offset: Option<u16>,
    pub x_adv_device: Option<Device>,
    pub y_adv_device_offset: Option<u16>,
    pub y_adv_device: Option<Device>
}

pub enum Anchor {
    Format1 {
        x_coordinate: i16,
        y_coordinate: i16
    },
    Format2 {
        x_coordinate: i16,
        y_coordinate: i16,
        anchor_point: u16
    },
    Format3 {
        x_coordinate: i16,
        y_coordinate: i16,
        x_device_offset: u16,
        x_device: Device,
        y_device_offset: u16,
        y_device: Device
    }
}

pub struct MarkArray {
    pub mark_count: u16,
    pub mark_records: Vec<MarkRecord>
}

pub struct MarkRecord {
    pub mark_class: u16,
    pub mark_anchor_offset: u16,
    pub mark_anchor: Anchor
}

pub enum GposSubtable {
    Type1(GposType1Format),
    Type2(GposType2Format),
    Type3(GposType3Format),
    Type4(GposType4Format),
    Type5(GposType5Format),
    Type6(GposType6Format),
    Type7(GposType7Format),
    Type8(GposType8Format),
    Type9(GposType9Format)
}

pub enum GposType1Format {
    Format1 {
        coverage_offset: u16,
        coverage: Coverage,
        value_format: u16,
        value_record: ValueRecord
    },
    Format2 {
        coverage_offset: u16,
        coverage: Coverage,
        value_format: u16,
        value_count: u16,
        value_records: Vec<ValueRecord>
    }
}

pub enum GposType2Format {
    Format1 {
        coverage_offset: u16,
        coverage: Coverage,
        value_format1: u16,
        value_format2: u16,
        pair_set_count: u16,
        pair_set_offsets: Vec<u16>,
        pair_sets: Vec<PairSet>
    },
    Format2 {
        coverage_offset: u16,
        coverage: Coverage,
        value_format1: u16,
        value_format2: u16,
        class_def1_offset: u16,
        class_def1: ClassDef,
        class_def2_offset: u16,
        class_def2: ClassDef,
        class1_count: u16,
        class2_count: u16,
        class1_records: Vec<Class1Record>
    }
}

pub struct PairSet {
    pub pair_value_count: u16,
    pub pair_value_records: Vec<PairValueRecord>
}

pub struct PairValueRecord {
    pub second_glyph: u16,
    pub value_record1: ValueRecord,
    pub value_record2: ValueRecord
}

pub struct Class1Record {
    pub class2_records: Vec<Class2Record>
}

pub struct Class2Record {
    pub value_record1: ValueRecord,
    pub value_record2: ValueRecord
}

pub enum GposType3Format {
    Format1 {
        coverage_offset: u16,
        coverage: Coverage,
        entry_exit_count: u16,
        entry_exit_records: Vec<EntryExitRecord>
    }
}

pub struct EntryExitRecord {
    pub entry_anchor_offset: Option<u16>,
    pub entry_anchor: Option<Anchor>,
    pub exit_anchor_offset: Option<u16>,
    pub exit_anchor: Option<Anchor>
}

pub enum GposType4Format {
    Format1 {
        mark_coverage_offset: u16,
        mark_coverage: Coverage,
        base_coverage_offset: u16,
        base_coverage: Coverage,
        mark_class_count: u16,
        mark_array_offset: u16,
        mark_array: MarkArray,
        base_array_offset: u16,
        base_array: BaseArray
    }
}

pub struct BaseArray {
    pub base_count: u16,
    pub base_records: Vec<BaseRecord>
}

pub struct BaseRecord {
    pub base_anchor_offsets: Vec<u16>,
    pub base_anchors: Vec<Anchor>
}

pub enum GposType5Format {
    Format1 {
        mark_coverage_offset: u16,
        mark_coverage: Coverage,
        ligature_coverage_offset: u16,
        ligature_coverage: Coverage,
        mark_class_count: u16,
        mark_array_offset: u16,
        mark_array: MarkArray,
        ligature_array_offset: u16,
        ligature_array: LigatureArray
    }
}

pub struct LigatureArray {
    pub ligature_count: u16,
    pub ligature_attach_offsets: Vec<u16>,
    pub ligature_attaches: Vec<LigatureAttach>
}

pub struct LigatureAttach {
    pub component_count: u16,
    pub component_records: Vec<ComponentRecord>
}

pub struct ComponentRecord {
    pub ligature_anchor_offsets: Vec<u16>,
    pub ligature_anchors: Vec<Anchor>
}

pub enum GposType6Format {
    Format1 {
        mark1_coverage_offset: u16,
        mark1_coverage: Coverage,
        mark2_coverage_offset: u16,
        mark2_coverage: Coverage,
        mark_class_count: u16,
        mark1_array_offset: u16,
        mark1_array: MarkArray,
        mark2_array_offset: u16,
        mark2_array: Mark2Array
    }
}

pub struct Mark2Array {
    pub mark2_count: u16,
    pub mark2_records: Vec<Mark2Record>
}

pub struct Mark2Record {
    pub mark2_anchor_offsets: Vec<u16>,
    pub mark2_anchors: Vec<Anchor>
}

pub enum GposType7Format {
    Format1 {
        coverage_offset: u16,
        coverage: Coverage,
        sub_rule_set_count: u16,
        sub_rule_set_offsets: Vec<u16>,
        sub_rule_sets: Vec<GposSubRuleSet>
    },
    Format2 {
        coverage_offset: u16,
        coverage: Coverage,
        class_def_offset: u16,
        class_def: ClassDef,
        sub_class_set_count: u16,
        sub_class_set_offsets: Vec<u16>,
        sub_class_sets: Vec<GposSubClassSet>
    },
    Format3 {
        glyph_count: u16,
        sub_count: u16,
        coverage_offsets: Vec<u16>,
        coverages: Vec<Coverage>,
        pos_lookup_records: Vec<PosLookupRecord>
    }
}

pub struct GposSubRuleSet {
    pub sub_rule_count: u16,
    pub sub_rule_offsets: Vec<u16>,
    pub sub_rules: Vec<GposSubRule>
}

pub struct GposSubRule {
    pub glyph_count: u16,
    pub sub_count: u16,
    pub input_glyph_ids: Vec<u16>,
    pub pos_lookup_records: Vec<PosLookupRecord>
}

pub struct PosLookupRecord {
    pub glyph_sequence_index: u16,
    pub lookup_list_index: u16
}

pub struct GposSubClassSet {
    pub sub_class_rule_count: u16,
    pub sub_class_rule_offsets: Vec<u16>,
    pub sub_class_rules: Vec<GposSubClassRule>
}

pub struct GposSubClassRule {
    pub glyph_count: u16,
    pub sub_count: u16,
    pub class_ids: Vec<u16>,
    pub pos_lookup_records: Vec<PosLookupRecord>
}

pub enum GposType8Format {
    Format1 {
        coverage_offset: u16,
        coverage: Coverage,
        chain_sub_rule_set_count: u16,
        chain_sub_rule_set_offsets: Vec<u16>,
        chain_sub_rule_sets: Vec<GposChainSubRuleSet>
    },
    Format2 {
        coverage_offset: u16,
        coverage: Coverage,
        backtrack_class_def_offset: u16,
        backtrack_class_def: ClassDef,
        input_class_def_offset: u16,
        input_class_def: ClassDef,
        lookahead_class_def_offset: u16,
        lookahead_class_def: ClassDef,
        chain_sub_class_set_count: u16,
        chain_sub_class_set_offsets: Vec<u16>,
        chain_sub_class_sets: Vec<GposChainSubClassSet>
    },
    Format3 {
        backtrack_glyph_count: u16,
        backtrack_coverage_offsets: Vec<u16>,
        backtrack_coverages: Vec<Coverage>,
        input_glyph_count: u16,
        input_coverage_offsets: Vec<u16>,
        input_coverages: Vec<Coverage>,
        lookahead_glyph_count: u16,
        lookahead_coverage_offsets: Vec<u16>,
        lookahead_coverages: Vec<Coverage>,
        sub_count: u16,
        pos_lookup_records: Vec<PosLookupRecord>
    }
}

pub struct GposChainSubRuleSet {
    pub chain_sub_rule_count: u16,
    pub chain_sub_rule_offsets: Vec<u16>,
    pub chain_sub_rules: Vec<GposChainSubRule>
}

pub struct GposChainSubRule {
    pub backtrack_glyph_count: u16,
    pub backtrack_glyph_ids: Vec<u16>,
    pub input_glyph_count: u16,
    pub input_glyph_ids: Vec<u16>,
    pub lookahead_glyph_count: u16,
    pub lookahead_glyph_ids: Vec<u16>,
    pub sub_count: u16,
    pub pos_lookup_records: Vec<PosLookupRecord>
}

pub struct GposChainSubClassSet {
    pub chain_sub_class_rule_count: u16,
    pub chain_sub_class_rule_offsets: Vec<u16>,
    pub chain_sub_class_rules: Vec<GposChainSubClassRule>
}

pub struct GposChainSubClassRule {
    pub backtrack_glyph_count: u16,
    pub backtrack_class_ids: Vec<u16>,
    pub input_glyph_count: u16,
    pub input_class_ids: Vec<u16>,
    pub lookahead_glyph_count: u16,
    pub lookahead_class_ids: Vec<u16>,
    pub sub_count: u16,
    pub pos_lookup_records: Vec<PosLookupRecord>
}

pub enum GposType9Format {
    Format1 {
        extension_lookup_type: u16,
        extension_offset: u32,
        extension: Box<GposSubtable>
    }
}

pub enum GsubSubtable {
    Type1(GsubType1Format),
    Type2(GsubType2Format),
    Type3(GsubType3Format),
    Type4(GsubType4Format),
    Type5(GsubType5Format),
    Type6(GsubType6Format),
    Type7(GsubType7Format),
    Type8(GsubType8Format)
}

pub enum GsubType1Format {
    Format1 {
        coverage_offset: u16,
        coverage: Coverage,
        delta_glyph_id: i16
    },
    Format2 {
        coverage_offset: u16,
        coverage: Coverage,
        glyph_count: u16,
        substitute_glyph_ids: Vec<u16>
    }
}

pub enum GsubType2Format {
    Format1 {
        coverage_offset: u16,
        coverage: Coverage,
        sequence_count: u16,
        sequence_offsets: Vec<u16>,
        sequences: Vec<Sequence>
    }
}

pub struct Sequence {
    pub glyph_count: u16,
    pub substitute_glyph_ids: Vec<u16>
}

pub enum GsubType3Format {
    Format1 {
        coverage_offset: u16,
        coverage: Coverage,
        alternate_set_count: u16,
        alternate_set_offsets: Vec<u16>,
        alternate_sets: Vec<AlternateSet>
    }
}

pub struct AlternateSet {
    pub glyph_count: u16,
    pub alternate_glyph_ids: Vec<u16>
}

pub enum GsubType4Format {
    Format1 {
        coverage_offset: u16,
        coverage: Coverage,
        ligature_set_count: u16,
        ligature_set_offsets: Vec<u16>,
        ligature_sets: Vec<LigatureSet>
    }
}

pub struct LigatureSet {
    pub ligature_count: u16,
    pub ligature_offsets: Vec<u16>
}

pub struct Ligature {
    pub ligature_glyph: u16,
    pub component_count: u16,
    pub component_glyph_ids: Vec<u16>
}

pub enum GsubType5Format {
    Format1 {
        coverage_offset: u16,
        coverage: Coverage,
        sub_rule_set_count: u16,
        sub_rule_set_offsets: Vec<u16>,
        sub_rule_sets: Vec<GsubSubRuleSet>
    },
    Format2 {
        coverage_offset: u16,
        coverage: Coverage,
        class_def_offset: u16,
        class_def: ClassDef,
        sub_class_set_count: u16,
        sub_class_set_offsets: Vec<u16>,
        sub_class_sets: Vec<GsubSubClassSet>
    },
    Format3 {
        glyph_count: u16,
        sub_count: u16,
        coverage_offsets: Vec<u16>,
        coverages: Vec<Coverage>,
        subst_lookup_records: Vec<SubstLookupRecord>
    }
}

pub struct GsubSubRuleSet {
    pub sub_rule_count: u16,
    pub sub_rule_offsets: Vec<u16>,
    pub sub_rules: Vec<GsubSubRule>
}

pub struct GsubSubRule {
    pub glyph_count: u16,
    pub sub_count: u16,
    pub input_glyph_ids: Vec<u16>,
    pub subst_lookup_records: Vec<SubstLookupRecord>
}

pub struct SubstLookupRecord {
    pub glyph_sequence_index: u16,
    pub lookup_list_index: u16
}

pub struct GsubSubClassSet {
    pub sub_class_rule_count: u16,
    pub sub_class_rule_offsets: Vec<u16>,
    pub sub_class_rules: Vec<GsubSubClassRule>
}

pub struct GsubSubClassRule {
    pub glyph_count: u16,
    pub sub_count: u16,
    pub class_ids: Vec<u16>,
    pub subst_lookup_records: Vec<SubstLookupRecord>
}

pub enum GsubType6Format {
    Format1 {
        coverage_offset: u16,
        coverage: Coverage,
        chain_sub_rule_set_count: u16,
        chain_sub_rule_set_offsets: Vec<u16>,
        chain_sub_rule_sets: Vec<GsubChainSubRuleSet>
    },
    Format2 {
        coverage_offset: u16,
        coverage: Coverage,
        backtrack_class_def_offset: u16,
        backtrack_class_def: ClassDef,
        input_class_def_offset: u16,
        input_class_def: ClassDef,
        lookahead_class_def_offset: u16,
        lookahead_class_def: ClassDef,
        chain_sub_class_set_count: u16,
        chain_sub_class_set_offsets: Vec<u16>,
        chain_sub_class_sets: Vec<GsubChainSubClassSet>
    },
    Format3 {
        backtrack_glyph_count: u16,
        backtrack_coverage_offsets: Vec<u16>,
        backtrack_coverages: Vec<Coverage>,
        input_glyph_count: u16,
        input_coverage_offsets: Vec<u16>,
        input_coverages: Vec<Coverage>,
        lookahead_glyph_count: u16,
        lookahead_coverage_offsets: Vec<u16>,
        lookahead_coverages: Vec<Coverage>,
        sub_count: u16,
        subst_lookup_records: Vec<SubstLookupRecord>
    }
}

pub struct GsubChainSubRuleSet {
    pub chain_sub_rule_count: u16,
    pub chain_sub_rule_offsets: Vec<u16>,
    pub chain_sub_rules: Vec<GsubChainSubRule>
}

pub struct GsubChainSubRule {
    pub backtrack_glyph_count: u16,
    pub backtrack_glyph_ids: Vec<u16>,
    pub input_glyph_count: u16,
    pub input_glyph_ids: Vec<u16>,
    pub lookahead_glyph_count: u16,
    pub lookahead_glyph_ids: Vec<u16>,
    pub sub_count: u16,
    pub subst_lookup_records: Vec<SubstLookupRecord>
}

pub struct GsubChainSubClassSet {
    pub chain_sub_class_rule_count: u16,
    pub chain_sub_class_rule_offsets: Vec<u16>,
    pub chain_sub_class_rules: Vec<GsubChainSubClassRule>
}

pub struct GsubChainSubClassRule {
    pub backtrack_glyph_count: u16,
    pub backtrack_class_ids: Vec<u16>,
    pub input_glyph_count: u16,
    pub input_class_ids: Vec<u16>,
    pub lookahead_glyph_count: u16,
    pub lookahead_class_ids: Vec<u16>,
    pub sub_count: u16,
    pub subst_lookup_records: Vec<SubstLookupRecord>
}

pub enum GsubType7Format {
    Format1 {
        extension_lookup_type: u16,
        extension_offset: u32,
        extension: Box<GsubSubtable>
    }
}

pub enum GsubType8Format {
    Format1 {
        coverage_offset: u16,
        coverage: Coverage,
        backtrack_glyph_count: u16,
        backtrack_coverage_offsets: Vec<u16>,
        backtrack_coverages: Vec<Coverage>,
        lookahead_glyph_count: u16,
        lookahead_coverage_offsets: Vec<u16>,
        lookahead_coverages: Vec<Coverage>,
        glyph_count: u16,
        substitute_glyph_ids: Vec<u16>
    }
}