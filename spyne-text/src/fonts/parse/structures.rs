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
    
    pub fn tag(&self) -> [u8; 4] {
        self.tag
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
    
    pub fn var_selector(&self) -> [u8; 3] {
        self.var_selector
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
    pub(crate) fn new(
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
    
    pub fn transformation(&self) -> [i16; 4] {
        self.transformation
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
    
    pub fn panose(&self) -> [u8; 10] {
        self.panose
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
    
    pub fn ach_vend_id(&self) -> [u8; 4] {
        self.ach_vend_id
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
    
    pub fn version(&self) -> u32 {
        self.version
    }
    
    pub fn italic_angle(&self) -> i32 {
        self.italic_angle
    }
    
    pub fn underline_position(&self) -> i16 {
        self.underline_position
    }
    
    pub fn underline_thickness(&self) -> i16 {
        self.underline_thickness
    }
    
    pub fn is_fixed_pitch(&self) -> u32 {
        self.is_fixed_pitch
    }
    
    pub fn min_mem_type_42(&self) -> u32 {
        self.min_mem_type_42
    }
    
    pub fn max_mem_type_42(&self) -> u32 {
        self.max_mem_type_42
    }
    
    pub fn min_mem_type_1(&self) -> u32 {
        self.min_mem_type_1
    }
    
    pub fn max_mem_type_1(&self) -> u32 {
        self.max_mem_type_1
    }
    
    pub fn num_glyphs(&self) -> Option<u16> {
        self.num_glyphs
    }
    
    pub fn glyph_name_index(&self) -> Option<&[u16]> {
        self.glyph_name_index.as_deref()
    }
    
    pub fn names(&self) -> Option<&[String]> {
        self.names.as_deref()
    }
}

pub struct VheaTable {
    version: u32,
    vert_typo_ascender: i16,
    vert_typo_descender: i16,
    vert_typo_line_gap: i16,
    advance_height_max: u16,
    min_top_side_bearing: i16,
    min_bottom_side_bearing: i16,
    y_max_extent: i16,
    caret_slope_rise: i16,
    caret_slope_run: i16,
    caret_offset: i16,
    _reserved1: i16,
    _reserved2: i16,
    _reserved3: i16,
    _reserved4: i16,
    metric_data_format: i16,
    num_of_long_ver_metrics: u16
}

impl VheaTable {
    pub(super) fn new(
        version: u32,
        vert_typo_ascender: i16,
        vert_typo_descender: i16,
        vert_typo_line_gap: i16,
        advance_height_max: u16,
        min_top_side_bearing: i16,
        min_bottom_side_bearing: i16,
        y_max_extent: i16,
        caret_slope_rise: i16,
        caret_slope_run: i16,
        caret_offset: i16,
        _reserved1: i16,
        _reserved2: i16,
        _reserved3: i16,
        _reserved4: i16,
        metric_data_format: i16,
        num_of_long_ver_metrics: u16
    ) -> Self {
        Self {
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
        }
    }
    
    pub fn version(&self) -> u32 {
        self.version
    }
    
    pub fn vert_typo_ascender(&self) -> i16 {
        self.vert_typo_ascender
    }
    
    pub fn vert_typo_descender(&self) -> i16 {
        self.vert_typo_descender
    }
    
    pub fn vert_typo_line_gap(&self) -> i16 {
        self.vert_typo_line_gap
    }
    
    pub fn advance_height_max(&self) -> u16 {
        self.advance_height_max
    }
    
    pub fn min_top_side_bearing(&self) -> i16 {
        self.min_top_side_bearing
    }
    
    pub fn min_bottom_side_bearing(&self) -> i16 {
        self.min_bottom_side_bearing
    }
    
    pub fn y_max_extent(&self) -> i16 {
        self.y_max_extent
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
    
    pub fn num_of_long_ver_metrics(&self) -> u16 {
        self.num_of_long_ver_metrics
    }
}

pub struct VmtxTable {
    entries: Vec<VmtxEntry>,
    shared_advance_height: u16
}

impl VmtxTable {
    pub(super) fn new(
        entries: Vec<VmtxEntry>,
        shared_advance_height: u16
    ) -> Self {
        Self {
            entries,
            shared_advance_height
        }
    }
    
    pub fn entries(&self) -> &[VmtxEntry] {
        &self.entries
    }
    
    pub fn shared_advance_height(&self) -> u16 {
        self.shared_advance_height
    }
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
    version: u16,
    length: u16,
    coverage: u16,
    subtable: KernSubtable
}

impl WindowsSubtable {
    pub(super) fn new(
        version: u16,
        length: u16,
        coverage: u16,
        subtable: KernSubtable
    ) -> Self {
        Self {
            version,
            length,
            coverage,
            subtable
        }
    }
    
    pub fn version(&self) -> u16 {
        self.version
    }
    
    pub fn length(&self) -> u16 {
        self.length
    }
    
    pub fn coverage(&self) -> u16 {
        self.coverage
    }
    
    pub fn subtable(&self) -> &KernSubtable {
        &self.subtable
    }
}

pub struct MacSubtable {
    length: u32,
    coverage: u16,
    tuple_index: u16,
    subtable: KernSubtable
}

impl MacSubtable {
    pub(super) fn new(
        length: u32,
        coverage: u16,
        tuple_index: u16,
        subtable: KernSubtable
    ) -> Self {
        Self {
            length,
            coverage,
            tuple_index,
            subtable
        }
    }
    
    pub fn length(&self) -> u32 {
        self.length
    }
    
    pub fn coverage(&self) -> u16 {
        self.coverage
    }
    
    pub fn tuple_index(&self) -> u16 {
        self.tuple_index
    }
    
    pub fn subtable(&self) -> &KernSubtable {
        &self.subtable
    }
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
    left: u16,
    right: u16,
    value: i16
}

impl KernPair {
    pub(super) fn new(
        left: u16,
        right: u16,
        value: i16
    ) -> Self {
        Self {
            left,
            right,
            value
        }
    }
    
    pub fn left(&self) -> u16 {
        self.left
    }
    
    pub fn right(&self) -> u16 {
        self.right
    }
    
    pub fn value(&self) -> i16 {
        self.value
    }
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
    start_glyph: u16,
    end_glyph: u16,
    class: u16
}

impl Range {
    pub(super) fn new(
        start_glyph: u16,
        end_glyph: u16,
        class: u16
    ) -> Self {
        Self {
            start_glyph,
            end_glyph,
            class
        }
    }
    
    pub fn start_glyph(&self) -> u16 {
        self.start_glyph
    }
    
    pub fn end_glyph(&self) -> u16 {
        self.end_glyph
    }
    
    pub fn class(&self) -> u16 {
        self.class
    }
}

pub struct GaspTable {
    version: u16,
    num_ranges: u16,
    range_records: Vec<GaspRangeRecord>
}

impl GaspTable {
    pub(super) fn new(
        version: u16,
        num_ranges: u16,
        range_records: Vec<GaspRangeRecord>
    ) -> Self {
        Self {
            version,
            num_ranges,
            range_records
        }
    }
    
    pub fn version(&self) -> u16 {
        self.version
    }
    
    pub fn num_ranges(&self) -> u16 {
        self.num_ranges
    }
    
    pub fn range_records(&self) -> &[GaspRangeRecord] {
        &self.range_records
    }
}

pub struct GaspRangeRecord {
    range_max_ppem: u16,
    range_gasp_behavior: u16
}

impl GaspRangeRecord {
    pub(super) fn new(
        range_max_ppem: u16,
        range_gasp_behavior: u16
    ) -> Self {
        Self {
            range_max_ppem,
            range_gasp_behavior
        }
    }
    
    pub fn range_max_ppem(&self) -> u16 {
        self.range_max_ppem
    }
    
    pub fn range_gasp_behavior(&self) -> u16 {
        self.range_gasp_behavior
    }
}

pub struct GposTable<GposSubtable> {
    header: TableHeader,
    script_list: ScriptList,
    feature_list: FeatureList,
    lookup_list: LookupList<GposSubtable>,
    feature_variations: Option<FeatureVariations>
}

impl GposTable<GposSubtable> {
    pub(super) fn new(
        header: TableHeader,
        script_list: ScriptList,
        feature_list: FeatureList,
        lookup_list: LookupList<GposSubtable>,
        feature_variations: Option<FeatureVariations>
    ) -> Self {
        Self {
            header,
            script_list,
            feature_list,
            lookup_list,
            feature_variations
        }
    }
    
    pub fn header(&self) -> TableHeader {
        self.header
    }
    
    pub fn script_list(&self) -> &ScriptList {
        &self.script_list
    }
    
    pub fn feature_list(&self) -> &FeatureList {
        &self.feature_list
    }
    
    pub fn lookup_list(&self) -> &LookupList<GposSubtable> {
        &self.lookup_list
    }
    
    pub fn feature_variations(&self) -> Option<&FeatureVariations> {
        self.feature_variations.as_ref()
    }
}

pub struct GsubTable<GsubSubtable> {
    header: TableHeader,
    script_list: ScriptList,
    feature_list: FeatureList,
    lookup_list: LookupList<GsubSubtable>,
    feature_variations: Option<FeatureVariations>
}

impl GsubTable<GsubSubtable> {
    pub fn new(
        header: TableHeader,
        script_list: ScriptList,
        feature_list: FeatureList,
        lookup_list: LookupList<GsubSubtable>,
        feature_variations: Option<FeatureVariations>
    ) -> Self {
        Self {
            header,
            script_list,
            feature_list,
            lookup_list,
            feature_variations
        }
    }
    
    pub fn header(&self) -> &TableHeader {
        &self.header
    }
    
    pub fn script_list(&self) -> &ScriptList {
        &self.script_list
    }
    
    pub fn feature_list(&self) -> &FeatureList {
        &self.feature_list
    }
    
    pub fn lookup_list(&self) -> &LookupList<GsubSubtable> {
        &self.lookup_list
    }
    
    pub fn feature_variations(&self) -> Option<&FeatureVariations> {
        self.feature_variations.as_ref()
    }
}

#[derive(Clone, Copy)]
pub struct TableHeader {
    major_version: u16,
    minor_version: u16,
    script_list_offset: u16,
    feature_list_offset: u16,
    lookup_list_offset: u16,
    feature_variations_offset: Option<u32>
}

impl TableHeader {
    pub fn new(
        major_version: u16,
        minor_version: u16,
        script_list_offset: u16,
        feature_list_offset: u16,
        lookup_list_offset: u16,
        feature_variations_offset: Option<u32>
    ) -> Self {
        TableHeader {
            major_version,
            minor_version,
            script_list_offset,
            feature_list_offset,
            lookup_list_offset,
            feature_variations_offset,
        }
    }
    
    pub fn major_version(&self) -> u16 {
        self.major_version
    }
    
    pub fn minor_version(&self) -> u16 {
        self.minor_version
    }
    
    pub fn script_list_offset(&self) -> u16 {
        self.script_list_offset
    }
    
    pub fn feature_list_offset(&self) -> u16 {
        self.feature_list_offset
    }
    
    pub fn lookup_list_offset(&self) -> u16 {
        self.lookup_list_offset
    }
    
    pub fn feature_variations_offset(&self) -> Option<u32> {
        self.feature_variations_offset
    }
}

pub struct ScriptList {
    script_count: u16,
    script_records: Vec<ScriptRecord>,
    scripts: Vec<Script>
}

impl ScriptList {
    pub fn new(
        script_count: u16,
        script_records: Vec<ScriptRecord>,
        scripts: Vec<Script>
    ) -> Self {
        Self {
            script_count,
            script_records,
            scripts,
        }
    }
    
    pub fn script_count(&self) -> u16 {
        self.script_count
    }
    
    pub fn script_records(&self) -> &[ScriptRecord] {
        &self.script_records
    }
    
    pub fn scripts(&self) -> &[Script] {
        &self.scripts
    }
}

pub struct ScriptRecord {
    script_tag: [u8; 4],
    script_offset: u16
}

impl ScriptRecord {
    pub fn new(
        script_tag: [u8; 4],
        script_offset: u16
    ) -> Self {
        Self {
            script_tag,
            script_offset,
        }
    }
    
    pub fn script_tag(&self) -> [u8; 4] {
        self.script_tag
    }
    
    pub fn script_offset(&self) -> u16 {
        self.script_offset
    }
}

pub struct Script {
    default_lang_sys_offset: Option<u16>,
    default_lang_sys: Option<LangSys>,
    lang_sys_count: u16,
    lang_sys_records: Vec<LangSysRecord>,
    lang_syses: Vec<LangSys>
}

impl Script {
    pub fn new(
        default_lang_sys_offset: Option<u16>,
        default_lang_sys: Option<LangSys>,
        lang_sys_count: u16,
        lang_sys_records: Vec<LangSysRecord>,
        lang_syses: Vec<LangSys>
    ) -> Self {
        Self {
            default_lang_sys_offset,
            default_lang_sys,
            lang_sys_count,
            lang_sys_records,
            lang_syses,
        }
    }
    
    pub fn default_lang_sys_offset(&self) -> Option<u16> {
        self.default_lang_sys_offset
    }
    
    pub fn default_lang_sys(&self) -> Option<&LangSys> {
        self.default_lang_sys.as_ref()
    }
    
    pub fn lang_sys_count(&self) -> u16 {
        self.lang_sys_count
    }
    
    pub fn lang_sys_records(&self) -> &[LangSysRecord] {
        &self.lang_sys_records
    }
    
    pub fn lang_syses(&self) -> &[LangSys] {
        &self.lang_syses
    }
}

pub struct LangSysRecord {
    lang_sys_tag: [u8; 4],
    lang_sys_offset: u16,
}

impl LangSysRecord {
    pub fn new(
        lang_sys_tag: [u8; 4],
        lang_sys_offset: u16
    ) -> Self {
        Self {
            lang_sys_tag,
            lang_sys_offset,
        }
    }
    
    pub fn lang_sys_tag(&self) -> [u8; 4] {
        self.lang_sys_tag
    }
    
    pub fn lang_sys_offset(&self) -> u16 {
        self.lang_sys_offset
    }
}

pub struct LangSys {
    _lookup_order_offset: u16,
    required_feature_index: u16,
    feature_index_count: u16,
    feature_indices: Vec<u16>
}

impl LangSys {
    pub fn new(
        _lookup_order_offset: u16,
        required_feature_index: u16,
        feature_index_count: u16,
        feature_indices: Vec<u16>
    ) -> Self {
        Self {
            _lookup_order_offset,
            required_feature_index,
            feature_index_count,
            feature_indices,
        }
    }
    
    pub fn required_feature_index(&self) -> u16 {
        self.required_feature_index
    }
    
    pub fn feature_index_count(&self) -> u16 {
        self.feature_index_count
    }
    
    pub fn feature_indices(&self) -> &[u16] {
        &self.feature_indices
    }
}

#[derive(Clone)]
pub struct FeatureList {
    feature_count: u16,
    feature_records: Vec<FeatureRecord>,
    features: Vec<Feature>
}

impl FeatureList {
    pub fn new(
        feature_count: u16,
        feature_records: Vec<FeatureRecord>,
        features: Vec<Feature>
    ) -> Self {
        Self {
            feature_count,
            feature_records,
            features,
        }
    }
    
    pub fn feature_count(&self) -> u16 {
        self.feature_count
    }
    
    pub fn feature_records(&self) -> &[FeatureRecord] {
        &self.feature_records
    }
    
    pub fn features(&self) -> &[Feature] {
        &self.features
    }
}

#[derive(Clone)]
pub struct FeatureRecord {
    feature_tag: [u8; 4],
    feature_offset: u16
}

impl FeatureRecord {
    pub fn new(
        feature_tag: [u8; 4],
        feature_offset: u16
    ) -> Self {
        Self {
            feature_tag,
            feature_offset,
        }
    }
    
    pub fn feature_tag(&self) -> [u8; 4] {
        self.feature_tag
    }
    
    pub fn feature_offset(&self) -> u16 {
        self.feature_offset
    }
}

#[derive(Clone)]
pub struct Feature {
    feature_params_offset: Option<u16>,
    feature_params: Option<FeatureParams>,
    lookup_index_count: u16,
    lookup_list_indices: Vec<u16>
}

impl Feature {
    pub fn new(
        feature_params_offset: Option<u16>,
        feature_params: Option<FeatureParams>,
        lookup_index_count: u16,
        lookup_list_indices: Vec<u16>
    ) -> Self {
        Self {
            feature_params_offset,
            feature_params,
            lookup_index_count,
            lookup_list_indices,
        }
    }
    
    pub fn feature_params_offset(&self) -> Option<u16> {
        self.feature_params_offset
    }
    
    pub fn feature_params(&self) -> Option<FeatureParams> {
        self.feature_params
    }
    
    pub fn lookup_index_count(&self) -> u16 {
        self.lookup_index_count
    }
    
    pub fn lookup_list_indices(&self) -> &[u16] {
        &self.lookup_list_indices
    }
}

#[derive(Clone, Copy)]
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
    lookup_count: u16,
    lookup_offsets: Vec<u16>,
    lookups: Vec<Lookup<T>>
}

impl<T> LookupList<T> {
    pub fn new(
        lookup_count: u16,
        lookup_offsets: Vec<u16>,
        lookups: Vec<Lookup<T>>
    ) -> Self {
        Self {
            lookup_count,
            lookup_offsets,
            lookups,
        }
    }
    
    pub fn lookup_count(&self) -> u16 {
        self.lookup_count
    }
    
    pub fn lookup_offsets(&self) -> &[u16] {
        &self.lookup_offsets
    }
    
    pub fn lookups(&self) -> &[Lookup<T>] {
        &self.lookups
    }
}

pub struct Lookup<T> {
    lookup_type: u16,
    lookup_flag: u16,
    subtable_count: u16,
    subtable_offsets: Vec<u16>,
    subtables: Vec<T>,
    mark_filtering_set: Option<u16>
}

impl<T> Lookup<T> {
    pub fn new(
        lookup_type: u16,
        lookup_flag: u16,
        subtable_count: u16,
        subtable_offsets: Vec<u16>,
        subtables: Vec<T>,
        mark_filtering_set: Option<u16>
    ) -> Self {
        Self {
            lookup_type,
            lookup_flag,
            subtable_count,
            subtable_offsets,
            subtables,
            mark_filtering_set
        }
    }
    
    pub fn lookup_type(&self) -> u16 {
        self.lookup_type
    }
    
    pub fn lookup_flag(&self) -> u16 {
        self.lookup_flag
    }
    
    pub fn subtable_count(&self) -> u16 {
        self.subtable_count
    }
    
    pub fn subtable_offsets(&self) -> &[u16] {
        &self.subtable_offsets
    }
    
    pub fn subtables(&self) -> &[T] {
        &self.subtables
    }
    
    pub fn mark_filtering_set(&self) -> Option<u16> {
        self.mark_filtering_set
    }
}

pub struct FeatureVariations {
    major_version: u16,
    minor_version: u16,
    feature_variation_record_count: u32,
    feature_variation_records: Vec<FeatureVariationRecord>
}

impl FeatureVariations {
    pub fn new(
        major_version: u16,
        minor_version: u16,
        feature_variation_record_count: u32,
        feature_variation_records: Vec<FeatureVariationRecord>
    ) -> Self {
        Self {
            major_version,
            minor_version,
            feature_variation_record_count,
            feature_variation_records
        }
    }
    
    pub fn major_version(&self) -> u16 {
        self.major_version
    }
    
    pub fn minor_version(&self) -> u16 {
        self.minor_version
    }
    
    pub fn feature_variation_record_count(&self) -> u32 {
        self.feature_variation_record_count
    }
    
    pub fn feature_variation_records(&self) -> &[FeatureVariationRecord] {
        &self.feature_variation_records
    }
}

pub struct FeatureVariationRecord {
    condition_set_offset: u32,
    condition_set: ConditionSet,
    feature_table_substitution_offset: u32,
    feature_table_substitution: FeatureTableSubstitution
}

impl FeatureVariationRecord {
    pub(super) fn new(
        condition_set_offset: u32,
        condition_set: ConditionSet,
        feature_table_substitution_offset: u32,
        feature_table_substitution: FeatureTableSubstitution
    ) -> Self {
        Self {
            condition_set_offset,
            condition_set,
            feature_table_substitution_offset,
            feature_table_substitution
        }
    }
    
    pub fn condition_set_offset(&self) -> u32 {
        self.condition_set_offset
    }
    
    pub fn condition_set(&self) -> &ConditionSet {
        &self.condition_set
    }
    
    pub fn feature_table_substitution_offset(&self) -> u32 {
        self.feature_table_substitution_offset
    }
    
    pub fn feature_table_substitution(&self) -> &FeatureTableSubstitution {
        &self.feature_table_substitution
    }
}

pub struct ConditionSet {
    condition_count: u16,
    condition_offsets: Vec<u32>,
    conditions: Vec<Condition>
}

impl ConditionSet {
    pub(super) fn new(
        condition_count: u16,
        condition_offsets: Vec<u32>,
        conditions: Vec<Condition>
    ) -> Self {
        Self {
            condition_count,
            condition_offsets,
            conditions
        }
    }
    
    pub fn condition_count(&self) -> u16 {
        self.condition_count
    }
    
    pub fn condition_offsets(&self) -> &[u32] {
        &self.condition_offsets
    }
    
    pub fn conditions(&self) -> &[Condition] {
        &self.conditions
    }
}

pub enum Condition {
    Format1 {
        axis_index: u16,
        filter_range_min_value: i16,
        filter_range_max_value: i16
    }
}

pub struct FeatureTableSubstitution {
    major_version: u16,
    minor_version: u16,
    substitution_count: u16,
    substitution_records: Vec<FeatureTableSubstitutionRecord>
}

impl FeatureTableSubstitution {
    pub(super) fn new(
        major_version: u16,
        minor_version: u16,
        substitution_count: u16,
        substitution_records: Vec<FeatureTableSubstitutionRecord>
    ) -> Self {
        Self {
            major_version,
            minor_version,
            substitution_count,
            substitution_records
        }
    }
    
    pub fn major_version(&self) -> u16 {
        self.major_version
    }
    
    pub fn minor_version(&self) -> u16 {
        self.minor_version
    }
    
    pub fn substitution_count(&self) -> u16 {
        self.substitution_count
    }
    
    pub fn substitution_records(&self) -> &[FeatureTableSubstitutionRecord] {
        &self.substitution_records
    }
}

pub struct FeatureTableSubstitutionRecord {
    feature_index: u16,
    alternate_feature_table_offset: u32,
    alternate_feature_table: Feature
}

impl FeatureTableSubstitutionRecord {
    pub(super) fn new(
        feature_index: u16,
        alternate_feature_table_offset: u32,
        alternate_feature_table: Feature
    ) -> Self {
        Self {
            feature_index,
            alternate_feature_table_offset,
            alternate_feature_table
        }
    }
    
    pub fn feature_index(&self) -> u16 {
        self.feature_index
    }
    
    pub fn alternate_feature_table_offset(&self) -> u32 {
        self.alternate_feature_table_offset
    }
    
    pub fn alternate_feature_table(&self) -> &Feature {
        &self.alternate_feature_table
    }
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
    start_glyph_id: u16,
    end_glyph_id: u16,
    start_coverage_index: u16
}

impl CoverageRangeRecord {
    pub(super) fn new(
        start_glyph_id: u16,
        end_glyph_id: u16,
        start_coverage_index: u16
    ) -> Self {
        Self {
            start_glyph_id,
            end_glyph_id,
            start_coverage_index
        }
    }
    
    pub fn start_glyph_id(&self) -> u16 {
        self.start_glyph_id
    }
    
    pub fn end_glyph_id(&self) -> u16 {
        self.end_glyph_id
    }
    
    pub fn start_coverage_index(&self) -> u16 {
        self.start_coverage_index
    }
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
    start_glyph_id: u16,
    end_glyph_id: u16,
    class: u16
}

impl ClassRangeRecord {
    pub(super) fn new(
        start_glyph_id: u16,
        end_glyph_id: u16,
        class: u16
    ) -> Self {
        Self {
            start_glyph_id,
            end_glyph_id,
            class
        }
    }
    
    pub fn start_glyph_id(&self) -> u16 {
        self.start_glyph_id
    }
    
    pub fn end_glyph_id(&self) -> u16 {
        self.end_glyph_id
    }
    
    pub fn class(&self) -> u16 {
        self.class
    }
}

pub enum DeviceOrVariationIndex {
    Device(Device),
    VariationIndex(VariationIndex)
}

pub struct Device {
    start_size: u16,
    end_size: u16,
    delta_format: u16,
    delta_values: Vec<u16>
}

impl Device {
    pub(super) fn new(
        start_size: u16,
        end_size: u16,
        delta_format: u16,
        delta_values: Vec<u16>
    ) -> Self {
        Self {
            start_size,
            end_size,
            delta_format,
            delta_values
        }
    }
    
    pub fn start_size(&self) -> u16 {
        self.start_size
    }
    
    pub fn end_size(&self) -> u16 {
        self.end_size
    }
    
    pub fn delta_format(&self) -> u16 {
        self.delta_format
    }
    
    pub fn delta_values(&self) -> &[u16] {
        &self.delta_values
    }
}

pub struct VariationIndex {
    delta_set_outer_index: u16,
    delta_set_inner_index: u16,
    delta_format: u16
}

impl VariationIndex {
    pub(super) fn new(
        delta_set_outer_index: u16,
        delta_set_inner_index: u16,
        delta_format: u16
    ) -> Self {
        Self {
            delta_set_outer_index,
            delta_set_inner_index,
            delta_format
        }
    }
    
    pub fn delta_set_outer_index(&self) -> u16 {
        self.delta_set_outer_index
    }
    
    pub fn delta_set_inner_index(&self) -> u16 {
        self.delta_set_inner_index
    }
    
    pub fn delta_format(&self) -> u16 {
        self.delta_format
    } 
}

pub struct ValueRecord {
    x_placement: Option<i16>,
    y_placement: Option<i16>,
    x_advance: Option<i16>,
    y_advance: Option<i16>,
    x_pla_device_offset: Option<u16>,
    x_pla_device: Option<DeviceOrVariationIndex>,
    y_pla_device_offset: Option<u16>,
    y_pla_device: Option<DeviceOrVariationIndex>,
    x_adv_device_offset: Option<u16>,
    x_adv_device: Option<DeviceOrVariationIndex>,
    y_adv_device_offset: Option<u16>,
    y_adv_device: Option<DeviceOrVariationIndex>
}

impl ValueRecord {
    pub(super) fn new(
        x_placement: Option<i16>,
        y_placement: Option<i16>,
        x_advance: Option<i16>,
        y_advance: Option<i16>,
        x_pla_device_offset: Option<u16>,
        x_pla_device: Option<DeviceOrVariationIndex>,
        y_pla_device_offset: Option<u16>,
        y_pla_device: Option<DeviceOrVariationIndex>,
        x_adv_device_offset: Option<u16>,
        x_adv_device: Option<DeviceOrVariationIndex>,
        y_adv_device_offset: Option<u16>,
        y_adv_device: Option<DeviceOrVariationIndex>
    ) -> Self {
        Self {
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
        }
    }
    
    pub fn x_placement(&self) -> Option<i16> {
        self.x_placement
    }
    
    pub fn y_placement(&self) -> Option<i16> {
        self.y_placement
    }
    
    pub fn x_advance(&self) -> Option<i16> {
        self.x_advance
    }
    
    pub fn y_advance(&self) -> Option<i16> {
        self.y_advance
    }
    
    pub fn x_pla_device_offset(&self) -> Option<u16> {
        self.x_pla_device_offset
    }
    
    pub fn x_pla_device(&self) -> Option<&DeviceOrVariationIndex> {
        self.x_pla_device.as_ref()
    }
    
    pub fn y_pla_device_offset(&self) -> Option<u16> {
        self.y_pla_device_offset
    }
    
    pub fn y_pla_device(&self) -> Option<&DeviceOrVariationIndex> {
        self.y_pla_device.as_ref()
    }
    
    pub fn x_adv_device_offset(&self) -> Option<u16> {
        self.x_adv_device_offset
    }
    
    pub fn x_adv_device(&self) -> Option<&DeviceOrVariationIndex> {
        self.x_adv_device.as_ref()
    }
    
    pub fn y_adv_device_offset(&self) -> Option<u16> {
        self.y_adv_device_offset
    }
    
    pub fn y_adv_device(&self) -> Option<&DeviceOrVariationIndex> {
        self.y_adv_device.as_ref()
    } 
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
        x_device: DeviceOrVariationIndex,
        y_device_offset: u16,
        y_device: DeviceOrVariationIndex
    }
}

pub struct MarkArray {
    mark_count: u16,
    mark_records: Vec<MarkRecord>
}

impl MarkArray {
    pub(super) fn new(
        mark_count: u16,
        mark_records: Vec<MarkRecord>
    ) -> Self {
        Self {
            mark_count,
            mark_records
        }
    }
    
    pub fn mark_count(&self) -> u16 {
        self.mark_count
    }
    
    pub fn mark_records(&self) -> &[MarkRecord] {
        &self.mark_records
    } 
}

pub struct MarkRecord {
    mark_class: u16,
    mark_anchor_offset: u16,
    mark_anchor: Anchor
}

impl MarkRecord {
    pub(super) fn new(
        mark_class: u16,
        mark_anchor_offset: u16,
        mark_anchor: Anchor
    ) -> Self {
        Self {
            mark_class,
            mark_anchor_offset,
            mark_anchor
        }
    }
    
    pub fn mark_class(&self) -> u16 {
        self.mark_class
    }
    
    pub fn mark_anchor_offset(&self) -> u16 {
        self.mark_anchor_offset
    }
    
    pub fn mark_anchor(&self) -> &Anchor {
        &self.mark_anchor
    }
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
    pair_value_count: u16,
    pair_value_records: Vec<PairValueRecord>
}

impl PairSet {
    pub(super) fn new(
        pair_value_count: u16,
        pair_value_records: Vec<PairValueRecord>
    ) -> Self {
        Self {
            pair_value_count,
            pair_value_records
        }
    }
    
    pub fn pair_value_count(&self) -> u16 {
        self.pair_value_count
    }
    
    pub fn pair_value_records(&self) -> &[PairValueRecord] {
        &self.pair_value_records
    } 
}

pub struct PairValueRecord {
    second_glyph: u16,
    value_record1: ValueRecord,
    value_record2: ValueRecord
}

impl PairValueRecord {
    pub(super) fn new(
        second_glyph: u16,
        value_record1: ValueRecord,
        value_record2: ValueRecord
    ) -> Self {
        Self {
            second_glyph,
            value_record1,
            value_record2
        }
    }
    
    pub fn second_glyph(&self) -> u16 {
        self.second_glyph
    }
    
    pub fn value_record1(&self) -> &ValueRecord {
        &self.value_record1
    }
    
    pub fn value_record2(&self) -> &ValueRecord {
        &self.value_record2
    } 
}

pub struct Class1Record {
    class2_records: Vec<Class2Record>
}

impl Class1Record {
    pub(super) fn new(
        class2_records: Vec<Class2Record>
    ) -> Self {
        Self {
            class2_records
        }
    }
    
    pub fn class2_records(&self) -> &[Class2Record] {
        &self.class2_records
    } 
}

pub struct Class2Record {
    value_record1: ValueRecord,
    value_record2: ValueRecord
}

impl Class2Record {
    pub(super) fn new(
        value_record1: ValueRecord,
        value_record2: ValueRecord
    ) -> Self {
        Self {
            value_record1,
            value_record2
        }
    }
    
    pub fn value_record1(&self) -> &ValueRecord {
        &self.value_record1
    }
    
    pub fn value_record2(&self) -> &ValueRecord {
        &self.value_record2
    } 
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
    entry_anchor_offset: Option<u16>,
    entry_anchor: Option<Anchor>,
    exit_anchor_offset: Option<u16>,
    exit_anchor: Option<Anchor>
}

impl EntryExitRecord {
    pub(super) fn new(
        entry_anchor_offset: Option<u16>,
        entry_anchor: Option<Anchor>,
        exit_anchor_offset: Option<u16>,
        exit_anchor: Option<Anchor>
    ) -> Self {
        Self {
            entry_anchor_offset,
            entry_anchor,
            exit_anchor_offset,
            exit_anchor
        }
    }
    
    pub fn entry_anchor_offset(&self) -> Option<u16> {
        self.entry_anchor_offset
    }
    
    pub fn entry_anchor(&self) -> Option<&Anchor> {
        self.entry_anchor.as_ref()
    }
    
    pub fn exit_anchor_offset(&self) -> Option<u16> {
        self.exit_anchor_offset
    }
    
    pub fn exit_anchor(&self) -> Option<&Anchor> {
        self.exit_anchor.as_ref()
    } 
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
    base_count: u16,
    base_records: Vec<BaseRecord>
}

impl BaseArray {
    pub(super) fn new(
        base_count: u16,
        base_records: Vec<BaseRecord>
    ) -> Self {
        Self {
            base_count,
            base_records
        }
    }
    
    pub fn base_count(&self) -> u16 {
        self.base_count
    }
    
    pub fn base_records(&self) -> &[BaseRecord] {
        &self.base_records
    } 
}

pub struct BaseRecord {
    base_anchor_offsets: Vec<u16>,
    base_anchors: Vec<Anchor>
}

impl BaseRecord {
    pub(super) fn new(
        base_anchor_offsets: Vec<u16>,
        base_anchors: Vec<Anchor>
    ) -> Self {
        Self {
            base_anchor_offsets,
            base_anchors
        }
    }
    
    pub fn base_anchor_offsets(&self) -> &[u16] {
        &self.base_anchor_offsets
    }
    
    pub fn base_anchors(&self) -> &[Anchor] {
        &self.base_anchors
    } 
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
    ligature_count: u16,
    ligature_attach_offsets: Vec<u16>,
    ligature_attaches: Vec<LigatureAttach>
}

impl LigatureArray {
    pub(super) fn new(
        ligature_count: u16,
        ligature_attach_offsets: Vec<u16>,
        ligature_attaches: Vec<LigatureAttach>
    ) -> Self {
        Self {
            ligature_count,
            ligature_attach_offsets,
            ligature_attaches
        }
    }
    
    pub fn ligature_count(&self) -> u16 {
        self.ligature_count
    }
    
    pub fn ligature_attach_offsets(&self) -> &[u16] {
        &self.ligature_attach_offsets
    }
    
    pub fn ligature_attaches(&self) -> &[LigatureAttach] {
        &self.ligature_attaches
    } 
}

pub struct LigatureAttach {
    component_count: u16,
    component_records: Vec<ComponentRecord>
}

impl LigatureAttach {
    pub(super) fn new(
        component_count: u16,
        component_records: Vec<ComponentRecord>
    ) -> Self {
        Self {
            component_count,
            component_records
        }
    }
    
    pub fn component_count(&self) -> u16 {
        self.component_count
    }
    
    pub fn component_records(&self) -> &[ComponentRecord] {
        &self.component_records
    } 
}

pub struct ComponentRecord {
    ligature_anchor_offsets: Vec<u16>,
    ligature_anchors: Vec<Anchor>
}

impl ComponentRecord {
    pub(super) fn new(
        ligature_anchor_offsets: Vec<u16>,
        ligature_anchors: Vec<Anchor>
    ) -> Self {
        Self {
            ligature_anchor_offsets,
            ligature_anchors
        }
    }
    
    pub fn ligature_anchor_offsets(&self) -> &[u16] {
        &self.ligature_anchor_offsets
    }
    
    pub fn ligature_anchors(&self) -> &[Anchor] {
        &self.ligature_anchors
    } 
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
    mark2_count: u16,
    mark2_records: Vec<Mark2Record>
}

impl Mark2Array {
    pub(super) fn new(
        mark2_count: u16,
        mark2_records: Vec<Mark2Record>
    ) -> Self {
        Self {
            mark2_count,
            mark2_records
        }
    }
    
    pub fn mark2_count(&self) -> u16 {
        self.mark2_count
    }
    
    pub fn mark2_records(&self) -> &[Mark2Record] {
        &self.mark2_records
    } 
}

pub struct Mark2Record {
    mark2_anchor_offsets: Vec<u16>,
    mark2_anchors: Vec<Anchor>
}

impl Mark2Record {
    pub(super) fn new(
        mark2_anchor_offsets: Vec<u16>,
        mark2_anchors: Vec<Anchor>
    ) -> Self {
        Self {
            mark2_anchor_offsets,
            mark2_anchors
        }
    }
    
    pub fn mark2_anchor_offsets(&self) -> &[u16] {
        &self.mark2_anchor_offsets
    }
    
    pub fn mark2_anchors(&self) -> &[Anchor] {
        &self.mark2_anchors
    } 
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
    sub_rule_count: u16,
    sub_rule_offsets: Vec<u16>,
    sub_rules: Vec<GposSubRule>
}

impl GposSubRuleSet {
    pub(super) fn new(
        sub_rule_count: u16,
        sub_rule_offsets: Vec<u16>,
        sub_rules: Vec<GposSubRule>
    ) -> Self {
        Self {
            sub_rule_count,
            sub_rule_offsets,
            sub_rules
        }
    }
    
    pub fn sub_rule_count(&self) -> u16 {
        self.sub_rule_count
    }
    
    pub fn sub_rule_offsets(&self) -> &[u16] {
        &self.sub_rule_offsets
    }
    
    pub fn sub_rules(&self) -> &[GposSubRule] {
        &self.sub_rules
    } 
}

pub struct GposSubRule {
    glyph_count: u16,
    sub_count: u16,
    input_glyph_ids: Vec<u16>,
    pos_lookup_records: Vec<PosLookupRecord>
}

impl GposSubRule {
    pub(super) fn new(
        glyph_count: u16,
        sub_count: u16,
        input_glyph_ids: Vec<u16>,
        pos_lookup_records: Vec<PosLookupRecord>
    ) -> Self {
        Self {
            glyph_count,
            sub_count,
            input_glyph_ids,
            pos_lookup_records
        }
    }
    
    pub fn glyph_count(&self) -> u16 {
        self.glyph_count
    }
    
    pub fn sub_count(&self) -> u16 {
        self.sub_count
    }
    
    pub fn input_glyph_ids(&self) -> &[u16] {
        &self.input_glyph_ids
    }
    
    pub fn pos_lookup_records(&self) -> &[PosLookupRecord] {
        &self.pos_lookup_records
    } 
}

pub struct PosLookupRecord {
    glyph_sequence_index: u16,
    lookup_list_index: u16
}

impl PosLookupRecord {
    pub(super) fn new(
        glyph_sequence_index: u16,
        lookup_list_index: u16
    ) -> Self {
        Self {
            glyph_sequence_index,
            lookup_list_index
        }
    }
    
    pub fn glyph_sequence_index(&self) -> u16 {
        self.glyph_sequence_index
    }
    
    pub fn lookup_list_index(&self) -> u16 {
        self.lookup_list_index
    } 
}

pub struct GposSubClassSet {
    sub_class_rule_count: u16,
    sub_class_rule_offsets: Vec<u16>,
    sub_class_rules: Vec<GposSubClassRule>
}

impl GposSubClassSet {
    pub(super) fn new(
        sub_class_rule_count: u16,
        sub_class_rule_offsets: Vec<u16>,
        sub_class_rules: Vec<GposSubClassRule>
    ) -> Self {
        Self {
            sub_class_rule_count,
            sub_class_rule_offsets,
            sub_class_rules
        }
    }
    
    pub fn sub_class_rule_count(&self) -> u16 {
        self.sub_class_rule_count
    }
    
    pub fn sub_class_rule_offsets(&self) -> &[u16] {
        &self.sub_class_rule_offsets
    }
    
    pub fn sub_class_rules(&self) -> &[GposSubClassRule] {
        &self.sub_class_rules
    } 
}

pub struct GposSubClassRule {
    glyph_count: u16,
    sub_count: u16,
    class_ids: Vec<u16>,
    pos_lookup_records: Vec<PosLookupRecord>
}

impl GposSubClassRule {
    pub(super) fn new(
        glyph_count: u16,
        sub_count: u16,
        class_ids: Vec<u16>,
        pos_lookup_records: Vec<PosLookupRecord>
    ) -> Self {
        Self {
            glyph_count,
            sub_count,
            class_ids,
            pos_lookup_records
        }
    }
    
    pub fn glyph_count(&self) -> u16 {
        self.glyph_count
    }
    
    pub fn sub_count(&self) -> u16 {
        self.sub_count
    }
    
    pub fn class_ids(&self) -> &[u16] {
        &self.class_ids
    }
    
    pub fn pos_lookup_records(&self) -> &[PosLookupRecord] {
        &self.pos_lookup_records
    } 
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
    chain_sub_rule_count: u16,
    chain_sub_rule_offsets: Vec<u16>,
    chain_sub_rules: Vec<GposChainSubRule>
}

impl GposChainSubRuleSet {
    pub(super) fn new(
        chain_sub_rule_count: u16,
        chain_sub_rule_offsets: Vec<u16>,
        chain_sub_rules: Vec<GposChainSubRule>
    ) -> Self {
        Self {
            chain_sub_rule_count,
            chain_sub_rule_offsets,
            chain_sub_rules
        }
    }
    
    pub fn chain_sub_rule_count(&self) -> u16 {
        self.chain_sub_rule_count
    }
    
    pub fn chain_sub_rule_offsets(&self) -> &[u16] {
        &self.chain_sub_rule_offsets
    }
    
    pub fn chain_sub_rules(&self) -> &[GposChainSubRule] {
        &self.chain_sub_rules
    } 
}

pub struct GposChainSubRule {
    backtrack_glyph_count: u16,
    backtrack_glyph_ids: Vec<u16>,
    input_glyph_count: u16,
    input_glyph_ids: Vec<u16>,
    lookahead_glyph_count: u16,
    lookahead_glyph_ids: Vec<u16>,
    sub_count: u16,
    pos_lookup_records: Vec<PosLookupRecord>
}

impl GposChainSubRule {
    pub(super) fn new(
        backtrack_glyph_count: u16,
        backtrack_glyph_ids: Vec<u16>,
        input_glyph_count: u16,
        input_glyph_ids: Vec<u16>,
        lookahead_glyph_count: u16,
        lookahead_glyph_ids: Vec<u16>,
        sub_count: u16,
        pos_lookup_records: Vec<PosLookupRecord>
    ) -> Self {
        Self {
            backtrack_glyph_count,
            backtrack_glyph_ids,
            input_glyph_count,
            input_glyph_ids,
            lookahead_glyph_count,
            lookahead_glyph_ids,
            sub_count,
            pos_lookup_records
        }
    }
    
    pub fn backtrack_glyph_count(&self) -> u16 {
        self.backtrack_glyph_count
    }
    
    pub fn backtrack_glyph_ids(&self) -> &[u16] {
        &self.backtrack_glyph_ids
    }
    
    pub fn input_glyph_count(&self) -> u16 {
        self.input_glyph_count
    }
    
    pub fn input_glyph_ids(&self) -> &[u16] {
        &self.input_glyph_ids
    }
    
    pub fn lookahead_glyph_count(&self) -> u16 {
        self.lookahead_glyph_count
    }
    
    pub fn lookahead_glyph_ids(&self) -> &[u16] {
        &self.lookahead_glyph_ids
    }
    
    pub fn sub_count(&self) -> u16 {
        self.sub_count
    }
    
    pub fn pos_lookup_records(&self) -> &[PosLookupRecord] {
        &self.pos_lookup_records
    } 
}

pub struct GposChainSubClassSet {
    chain_sub_class_rule_count: u16,
    chain_sub_class_rule_offsets: Vec<u16>,
    chain_sub_class_rules: Vec<GposChainSubClassRule>
}

impl GposChainSubClassSet {
    pub(super) fn new(
        chain_sub_class_rule_count: u16,
        chain_sub_class_rule_offsets: Vec<u16>,
        chain_sub_class_rules: Vec<GposChainSubClassRule>
    ) -> Self {
        Self {
            chain_sub_class_rule_count,
            chain_sub_class_rule_offsets,
            chain_sub_class_rules
        }
    }
    
    pub fn chain_sub_class_rule_count(&self) -> u16 {
        self.chain_sub_class_rule_count
    }
    
    pub fn chain_sub_class_rule_offsets(&self) -> &[u16] {
        &self.chain_sub_class_rule_offsets
    }
    
    pub fn chain_sub_class_rules(&self) -> &[GposChainSubClassRule] {
        &self.chain_sub_class_rules
    } 
}

pub struct GposChainSubClassRule {
    backtrack_glyph_count: u16,
    backtrack_class_ids: Vec<u16>,
    input_glyph_count: u16,
    input_class_ids: Vec<u16>,
    lookahead_glyph_count: u16,
    lookahead_class_ids: Vec<u16>,
    sub_count: u16,
    pos_lookup_records: Vec<PosLookupRecord>
}

impl GposChainSubClassRule {
    pub(super) fn new(
        backtrack_glyph_count: u16,
        backtrack_class_ids: Vec<u16>,
        input_glyph_count: u16,
        input_class_ids: Vec<u16>,
        lookahead_glyph_count: u16,
        lookahead_class_ids: Vec<u16>,
        sub_count: u16,
        pos_lookup_records: Vec<PosLookupRecord>
    ) -> Self {
        Self {
            backtrack_glyph_count,
            backtrack_class_ids,
            input_glyph_count,
            input_class_ids,
            lookahead_glyph_count,
            lookahead_class_ids,
            sub_count,
            pos_lookup_records
        }
    }
    
    pub fn backtrack_glyph_count(&self) -> u16 {
        self.backtrack_glyph_count
    }
    
    pub fn backtrack_class_ids(&self) -> &[u16] {
        &self.backtrack_class_ids
    }
    
    pub fn input_glyph_count(&self) -> u16 {
        self.input_glyph_count
    }
    
    pub fn input_class_ids(&self) -> &[u16] {
        &self.input_class_ids
    }
    
    pub fn lookahead_glyph_count(&self) -> u16 {
        self.lookahead_glyph_count
    }
    
    pub fn lookahead_class_ids(&self) -> &[u16] {
        &self.lookahead_class_ids
    }
    
    pub fn sub_count(&self) -> u16 {
        self.sub_count
    }
    
    pub fn pos_lookup_records(&self) -> &[PosLookupRecord] {
        &self.pos_lookup_records
    } 
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
    glyph_count: u16,
    substitute_glyph_ids: Vec<u16>
}

impl Sequence {
    pub(super) fn new(
        glyph_count: u16,
        substitute_glyph_ids: Vec<u16>
    ) -> Self {
        Self {
            glyph_count,
            substitute_glyph_ids
        }
    }
    
    pub fn glyph_count(&self) -> u16 {
        self.glyph_count
    }
    
    pub fn substitute_glyph_ids(&self) -> &[u16] {
        &self.substitute_glyph_ids
    } 
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
    glyph_count: u16,
    alternate_glyph_ids: Vec<u16>
}

impl AlternateSet {
    pub(super) fn new(
        glyph_count: u16,
        alternate_glyph_ids: Vec<u16>
    ) -> Self {
        Self {
            glyph_count,
            alternate_glyph_ids
        }
    }
    
    pub fn glyph_count(&self) -> u16 {
        self.glyph_count
    }
    
    pub fn alternate_glyph_ids(&self) -> &[u16] {
        &self.alternate_glyph_ids
    } 
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
    ligature_count: u16,
    ligature_offsets: Vec<u16>,
    ligatures: Vec<Ligature>
}

impl LigatureSet {
    pub(super) fn new(
        ligature_count: u16,
        ligature_offsets: Vec<u16>,
        ligatures: Vec<Ligature>
    ) -> Self {
        Self {
            ligature_count,
            ligature_offsets,
            ligatures
        }
    }
    
    pub fn ligature_count(&self) -> u16 {
        self.ligature_count
    }
    
    pub fn ligature_offsets(&self) -> &[u16] {
        &self.ligature_offsets
    }
    
    pub fn ligatures(&self) -> &[Ligature] {
        &self.ligatures
    }
}

pub struct Ligature {
    ligature_glyph: u16,
    component_count: u16,
    component_glyph_ids: Vec<u16>
}

impl Ligature {
    pub(super) fn new(
        ligature_glyph: u16,
        component_count: u16,
        component_glyph_ids: Vec<u16>
    ) -> Self {
        Self {
            ligature_glyph,
            component_count,
            component_glyph_ids
        }
    }
    
    pub fn ligature_glyph(&self) -> u16 {
        self.ligature_glyph
    }
    
    pub fn component_count(&self) -> u16 {
        self.component_count
    }
    
    pub fn component_glyph_ids(&self) -> &[u16] {
        &self.component_glyph_ids
    } 
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
    sub_rule_count: u16,
    sub_rule_offsets: Vec<u16>,
    sub_rules: Vec<GsubSubRule>
}

impl GsubSubRuleSet {
    pub(super) fn new(
        sub_rule_count: u16,
        sub_rule_offsets: Vec<u16>,
        sub_rules: Vec<GsubSubRule>
    ) -> Self {
        Self {
            sub_rule_count,
            sub_rule_offsets,
            sub_rules
        }
    }
    
    pub fn sub_rule_count(&self) -> u16 {
        self.sub_rule_count
    }
    
    pub fn sub_rule_offsets(&self) -> &[u16] {
        &self.sub_rule_offsets
    }
    
    pub fn sub_rules(&self) -> &[GsubSubRule] {
        &self.sub_rules
    } 
}

pub struct GsubSubRule {
    glyph_count: u16,
    sub_count: u16,
    input_glyph_ids: Vec<u16>,
    subst_lookup_records: Vec<SubstLookupRecord>
}

impl GsubSubRule {
    pub(super) fn new(
        glyph_count: u16,
        sub_count: u16,
        input_glyph_ids: Vec<u16>,
        subst_lookup_records: Vec<SubstLookupRecord>
    ) -> Self {
        Self {
            glyph_count,
            sub_count,
            input_glyph_ids,
            subst_lookup_records
        }
    }
    
    pub fn glyph_count(&self) -> u16 {
        self.glyph_count
    }
    
    pub fn sub_count(&self) -> u16 {
        self.sub_count
    }
    
    pub fn input_glyph_ids(&self) -> &[u16] {
        &self.input_glyph_ids
    }
    
    pub fn subst_lookup_records(&self) -> &[SubstLookupRecord] {
        &self.subst_lookup_records
    } 
}

pub struct SubstLookupRecord {
    glyph_sequence_index: u16,
    lookup_list_index: u16
}

impl SubstLookupRecord {
    pub(super) fn new(
        glyph_sequence_index: u16,
        lookup_list_index: u16
    ) -> Self {
        Self {
            glyph_sequence_index,
            lookup_list_index
        }
    }
    
    pub fn glyph_sequence_index(&self) -> u16 {
        self.glyph_sequence_index
    }
    
    pub fn lookup_list_index(&self) -> u16 {
        self.lookup_list_index
    } 
}

pub struct GsubSubClassSet {
    sub_class_rule_count: u16,
    sub_class_rule_offsets: Vec<u16>,
    sub_class_rules: Vec<GsubSubClassRule>
}

impl GsubSubClassSet {
    pub(super) fn new(
        sub_class_rule_count: u16,
        sub_class_rule_offsets: Vec<u16>,
        sub_class_rules: Vec<GsubSubClassRule>
    ) -> Self {
        Self {
            sub_class_rule_count,
            sub_class_rule_offsets,
            sub_class_rules
        }
    }
    
    pub fn sub_class_rule_count(&self) -> u16 {
        self.sub_class_rule_count
    }
    
    pub fn sub_class_rule_offsets(&self) -> &[u16] {
        &self.sub_class_rule_offsets
    }
    
    pub fn sub_class_rules(&self) -> &[GsubSubClassRule] {
        &self.sub_class_rules
    } 
}

pub struct GsubSubClassRule {
    glyph_count: u16,
    sub_count: u16,
    class_ids: Vec<u16>,
    subst_lookup_records: Vec<SubstLookupRecord>
}

impl GsubSubClassRule {
    pub(super) fn new(
        glyph_count: u16,
        sub_count: u16,
        class_ids: Vec<u16>,
        subst_lookup_records: Vec<SubstLookupRecord>
    ) -> Self {
        Self {
            glyph_count,
            sub_count,
            class_ids,
            subst_lookup_records
        }
    }
    
    pub fn glyph_count(&self) -> u16 {
        self.glyph_count
    }
    
    pub fn sub_count(&self) -> u16 {
        self.sub_count
    }
    
    pub fn class_ids(&self) -> &[u16] {
        &self.class_ids
    }
    
    pub fn subst_lookup_records(&self) -> &[SubstLookupRecord] {
        &self.subst_lookup_records
    } 
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
    chain_sub_rule_count: u16,
    chain_sub_rule_offsets: Vec<u16>,
    chain_sub_rules: Vec<GsubChainSubRule>
}

impl GsubChainSubRuleSet {
    pub(super) fn new(
        chain_sub_rule_count: u16,
        chain_sub_rule_offsets: Vec<u16>,
        chain_sub_rules: Vec<GsubChainSubRule>
    ) -> Self {
        Self {
            chain_sub_rule_count,
            chain_sub_rule_offsets,
            chain_sub_rules
        }
    }
    
    pub fn chain_sub_rule_count(&self) -> u16 {
        self.chain_sub_rule_count
    }
    
    pub fn chain_sub_rule_offsets(&self) -> &[u16] {
        &self.chain_sub_rule_offsets
    }
    
    pub fn chain_sub_rules(&self) -> &[GsubChainSubRule] {
        &self.chain_sub_rules
    } 
}

pub struct GsubChainSubRule {
    backtrack_glyph_count: u16,
    backtrack_glyph_ids: Vec<u16>,
    input_glyph_count: u16,
    input_glyph_ids: Vec<u16>,
    lookahead_glyph_count: u16,
    lookahead_glyph_ids: Vec<u16>,
    sub_count: u16,
    subst_lookup_records: Vec<SubstLookupRecord>
}

impl GsubChainSubRule {
    pub(super) fn new(
        backtrack_glyph_count: u16,
        backtrack_glyph_ids: Vec<u16>,
        input_glyph_count: u16,
        input_glyph_ids: Vec<u16>,
        lookahead_glyph_count: u16,
        lookahead_glyph_ids: Vec<u16>,
        sub_count: u16,
        subst_lookup_records: Vec<SubstLookupRecord>
    ) -> Self {
        Self {
            backtrack_glyph_count,
            backtrack_glyph_ids,
            input_glyph_count,
            input_glyph_ids,
            lookahead_glyph_count,
            lookahead_glyph_ids,
            sub_count,
            subst_lookup_records
        }
    }
    
    pub fn backtrack_glyph_count(&self) -> u16 {
        self.backtrack_glyph_count
    }
    
    pub fn backtrack_glyph_ids(&self) -> &[u16] {
        &self.backtrack_glyph_ids
    }
    
    pub fn input_glyph_count(&self) -> u16 {
        self.input_glyph_count
    }
    
    pub fn input_glyph_ids(&self) -> &[u16] {
        &self.input_glyph_ids
    }
    
    pub fn lookahead_glyph_count(&self) -> u16 {
        self.lookahead_glyph_count
    }
    
    pub fn lookahead_glyph_ids(&self) -> &[u16] {
        &self.lookahead_glyph_ids
    }
    
    pub fn sub_count(&self) -> u16 {
        self.sub_count
    }
    
    pub fn subst_lookup_records(&self) -> &[SubstLookupRecord] {
        &self.subst_lookup_records
    } 
}

pub struct GsubChainSubClassSet {
    chain_sub_class_rule_count: u16,
    chain_sub_class_rule_offsets: Vec<u16>,
    chain_sub_class_rules: Vec<GsubChainSubClassRule>
}

impl GsubChainSubClassSet {
    pub(super) fn new(
        chain_sub_class_rule_count: u16,
        chain_sub_class_rule_offsets: Vec<u16>,
        chain_sub_class_rules: Vec<GsubChainSubClassRule>
    ) -> Self {
        Self {
            chain_sub_class_rule_count,
            chain_sub_class_rule_offsets,
            chain_sub_class_rules
        }
    }
    
    pub fn chain_sub_class_rule_count(&self) -> u16 {
        self.chain_sub_class_rule_count
    }
    
    pub fn chain_sub_class_rule_offsets(&self) -> &[u16] {
        &self.chain_sub_class_rule_offsets
    }
    
    pub fn chain_sub_class_rules(&self) -> &[GsubChainSubClassRule] {
        &self.chain_sub_class_rules
    } 
}

pub struct GsubChainSubClassRule {
    backtrack_glyph_count: u16,
    backtrack_class_ids: Vec<u16>,
    input_glyph_count: u16,
    input_class_ids: Vec<u16>,
    lookahead_glyph_count: u16,
    lookahead_class_ids: Vec<u16>,
    sub_count: u16,
    subst_lookup_records: Vec<SubstLookupRecord>
}

impl GsubChainSubClassRule {
    pub(super) fn new(
        backtrack_glyph_count: u16,
        backtrack_class_ids: Vec<u16>,
        input_glyph_count: u16,
        input_class_ids: Vec<u16>,
        lookahead_glyph_count: u16,
        lookahead_class_ids: Vec<u16>,
        sub_count: u16,
        subst_lookup_records: Vec<SubstLookupRecord>
    ) -> Self {
        Self {
            backtrack_glyph_count,
            backtrack_class_ids,
            input_glyph_count,
            input_class_ids,
            lookahead_glyph_count,
            lookahead_class_ids,
            sub_count,
            subst_lookup_records
        }
    }
    
    pub fn backtrack_glyph_count(&self) -> u16 {
        self.backtrack_glyph_count
    }
    
    pub fn backtrack_class_ids(&self) -> &[u16] {
        &self.backtrack_class_ids
    }
    
    pub fn input_glyph_count(&self) -> u16 {
        self.input_glyph_count
    }
    
    pub fn input_class_ids(&self) -> &[u16] {
        &self.input_class_ids
    }
    
    pub fn lookahead_glyph_count(&self) -> u16 {
        self.lookahead_glyph_count
    }
    
    pub fn lookahead_class_ids(&self) -> &[u16] {
        &self.lookahead_class_ids
    }
    
    pub fn sub_count(&self) -> u16 {
        self.sub_count
    }
    
    pub fn subst_lookup_records(&self) -> &[SubstLookupRecord] {
        &self.subst_lookup_records
    } 
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