// Maybe switch to getters instead of public fields for extra API safety

pub struct FontFile {
    pub file_type: FontFileType,
    pub bytes: Vec<u8>,
    pub table_records: Vec<TableRecord>
}

pub enum FontFileType {
    TrueType,
    OpenType
}

#[derive(Clone, Copy)]
pub struct TableRecord {
    pub tag: [u8; 4],
    pub checksum: u32,
    pub offset: u32,
    pub length: u32
}

pub(super) struct HeadTable {
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

pub(super) struct MaxpTable {
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

pub(super) struct CmapTable {
    pub version: u16,
    pub num_tables: u16,
    pub encoding_records: Vec<EncodingRecord>,
    pub subtables: Vec<CmapSubtable>
}

pub(super) struct EncodingRecord {
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

pub struct SubHeader {
    pub first_code: u16,
    pub entry_count: u16,
    pub id_delta: i16,
    pub id_range_offset: u16
}

pub struct Group {
    pub start_char_code: u32,
    pub end_char_code: u32,
    pub start_glyph_id: u32
}

pub struct VariationSelectorRecord {
    pub var_selector: [u8; 3],
    pub default_uvs_offset: u32,
    pub non_default_uvs_offset: u32
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

#[derive(Clone, Copy)]
pub struct GlyphHeader {
    pub number_of_contours: i16,
    pub x_min: i16,
    pub y_min: i16,
    pub x_max: i16,
    pub y_max: i16
}

#[derive(Clone, Copy)]
pub struct Component {
    pub flags: u16,
    pub glyph_index: u16,
    pub argument_1: i16,
    pub argument_2: i16,
    pub transformation: [i16; 4]
}

pub(super) struct HheaTable {
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
    pub(super) _reserved1: i16,
    pub(super) _reserved2: i16,
    pub(super) _reserved3: i16,
    pub(super) _reserved4: i16,
    pub metric_data_format: i16,
    pub number_of_h_metrics: u16
}

pub(super) struct HmtxTable {
    pub entries: Vec<HmtxEntry>,
    pub shared_advance_width: u16
}

pub enum HmtxEntry {
    FullMetric {
        advance_width: u16,
        lsb: i16
    },
    LeftoverBearing(i16)
}

pub(super) struct NameTable {
    pub version: u16,
    pub count: u16,
    pub storage_offset: u16,
    pub records: Vec<NameRecord>,
    pub lang_tag_count: Option<u16>,
    pub lang_tag_records: Option<Vec<LangTagRecord>>
}

pub(super) struct NameRecord {
    pub platform_id: u16,
    pub encoding_id: u16,
    pub language_id: u16,
    pub name_id: u16,
    pub length: u16,
    pub string_offset: u16,
    pub string: String
}

pub(super) struct LangTagRecord {
    pub length: u16,
    pub lang_tag_offset: u16,
    pub string: String
}

pub(super) struct OS2Table {
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

pub(super) struct PostTable {
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

pub(super) struct VheaTable {
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
    pub(super) _reserved1: i16,
    pub(super) _reserved2: i16,
    pub(super) _reserved3: i16,
    pub(super) _reserved4: i16,
    pub metric_data_format: i16,
    pub num_of_long_ver_metrics: u16
}

pub(super) struct VmtxTable {
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

pub(super) struct GaspTable {
    pub version: u16,
    pub num_ranges: u16,
    pub range_records: Vec<GaspRangeRecord>
}

pub(super) struct GaspRangeRecord {
    pub range_max_ppem: u16,
    pub range_gasp_behavior: u16
}

pub(super) struct GposTable<GposSubtable> {
    pub header: TableHeader,
    pub script_list: ScriptList,
    pub feature_list: FeatureList,
    pub lookup_list: LookupList<GposSubtable>,
    pub feature_variations: Option<FeatureVariations>
}

pub(super) struct GsubTable<GsubSubtable> {
    pub header: TableHeader,
    pub script_list: ScriptList,
    pub feature_list: FeatureList,
    pub lookup_list: LookupList<GsubSubtable>,
    pub feature_variations: Option<FeatureVariations>
}

pub(super) struct TableHeader {
    pub major_version: u16,
    pub minor_version: u16,
    pub script_list_offset: u16,
    pub feature_list_offset: u16,
    pub lookup_list_offset: u16,
    pub feature_variations_offset: Option<u32>
}

pub(super) struct ScriptList {
    pub script_count: u16,
    pub script_records: Vec<ScriptRecord>,
    pub scripts: Vec<Script>
}

pub(super) struct ScriptRecord {
    pub script_tag: [u8; 4],
    pub script_offset: u16
}

pub(super) struct Script {
    pub default_lang_sys_offset: Option<u16>,
    pub default_lang_sys: Option<LangSys>,
    pub lang_sys_count: u16,
    pub lang_sys_records: Vec<LangSysRecord>,
    pub lang_syses: Vec<LangSys>
}

pub(super) struct LangSysRecord {
    pub lang_sys_tag: [u8; 4],
    pub lang_sys_offset: u16,
}

pub(super) struct LangSys {
    pub(super) _lookup_order_offset: u16,
    pub required_feature_index: u16,
    pub feature_index_count: u16,
    pub feature_indices: Vec<u16>
}

#[derive(Clone)]
pub(super) struct FeatureList {
    pub feature_count: u16,
    pub feature_records: Vec<FeatureRecord>,
    pub features: Vec<Feature>
}

#[derive(Clone)]
pub(super) struct FeatureRecord {
    pub feature_tag: [u8; 4],
    pub feature_offset: u16
}

#[derive(Clone)]
pub(super) struct Feature {
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

pub(super) struct LookupList<T> {
    pub lookup_count: u16,
    pub lookup_offsets: Vec<u16>,
    pub lookups: Vec<Lookup<T>>
}

pub(super) struct Lookup<T> {
    pub lookup_type: u16,
    pub lookup_flag: u16,
    pub subtable_count: u16,
    pub subtable_offsets: Vec<u16>,
    pub subtables: Vec<T>,
    pub mark_filtering_set: Option<u16>
}

pub(super) struct FeatureVariations {
    pub major_version: u16,
    pub minor_version: u16,
    pub feature_variation_record_count: u32,
    pub feature_variation_records: Vec<FeatureVariationRecord>
}

pub(super) struct FeatureVariationRecord {
    pub condition_set_offset: u32,
    pub condition_set: ConditionSet,
    pub feature_table_substitution_offset: u32,
    pub feature_table_substitution: FeatureTableSubstitution
}

pub(super) struct ConditionSet {
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

pub(super) struct FeatureTableSubstitution {
    pub major_version: u16,
    pub minor_version: u16,
    pub substitution_count: u16,
    pub substitution_records: Vec<FeatureTableSubstitutionRecord>
}

pub(super) struct FeatureTableSubstitutionRecord {
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

pub(super) struct VariationIndexTable {
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

pub(super) struct Ligature {
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