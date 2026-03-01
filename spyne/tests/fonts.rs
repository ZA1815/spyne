use spyne::text::fonts::{atlas::{generator::{Atlas, AtlasAlgorithm}, outline::{create_outline}, rasterizer::rasterize}, parse::{constants::ON_CURVE_POINT, structures::{CmapSubtable, FontFile, Glyph}}};

#[test]
fn test_font_parser() {
    let font_path = format!("{}/tests/fixtures/FiraCode-Medium.ttf", env!("CARGO_MANIFEST_DIR"));
    let font_file = match FontFile::parse_font_file(font_path) {
        Ok(ff) => ff,
        Err(e) => panic!("FONT FILE PARSE FAIL: {}", e)
    };
    
    let head = match font_file.parse_head() {
        Ok(ht) => ht,
        Err(e) => panic!("HEAD TABLE PARSE FAIL: {}", e)
    };
    assert_eq!(head.units_per_em(), 1950);
    assert_eq!(head.x_min(), -3573);
    assert_eq!(head.y_min(), -1000);
    assert_eq!(head.x_max(), 2384);
    assert_eq!(head.y_max(), 2400);
    assert_eq!(head.mac_style(), 0);
    assert_eq!(head.lowest_rec_ppem(), 6);
    assert_eq!(head.font_direction_hint(), 2);
    assert_eq!(head.index_to_loc_format(), 1);
    
    let maxp = match font_file.parse_maxp() {
        Ok(mt) => mt,
        Err(e) => panic!("MAXP TABLE PARSE FAIL: {}", e)
    };
    assert_eq!(maxp.version(), 0x10000);
    assert_eq!(maxp.num_glyphs(), 2030);
    assert_eq!(maxp.max_points(), Some(518));
    assert_eq!(maxp.max_contours(), Some(96));
    assert_eq!(maxp.max_composite_points(), Some(112));
    assert_eq!(maxp.max_composite_contours(), Some(10));
    assert_eq!(maxp.max_zones(), Some(2));
    assert_eq!(maxp.max_twilight_points(), Some(576));
    
    let cmap = match font_file.parse_cmap() {
        Ok(ct) => ct,
        Err(e) => panic!("CMAP TABLE PARSE FAIL: {}", e)
    };
    let subtable_idx = cmap.encoding_records().iter().position(|er| er.platform_id() == 3 && er.encoding_id() == 10).unwrap();
    let subtable = cmap.subtables()[subtable_idx].to_owned();
    match subtable {
        CmapSubtable::Format12 { groups, .. } => {
            let group = groups.iter().find(|g| g.start_char_code() <= 73 && g.end_char_code() >= 73).unwrap();
            assert_eq!(group.start_glyph_id() + (73 - group.start_char_code()), 43);
        }
        _ => ()
    }
    
    let loca = match font_file.parse_loca(maxp.num_glyphs(), head.index_to_loc_format()) {
        Ok(lt) => lt,
        Err(e) => panic!("LOCA TABLE PARSE FAIL: {}", e)
    };
    let glyf = match font_file.parse_glyf(loca) {
        Ok(gt) => gt,
        Err(e) => panic!("GLYF TABLE PARSE FAIL: {}", e)
    };
    match glyf[43].to_owned() {
        Some(glyph) => {
            match glyph {
                Glyph::Simple {
                    header,
                    end_pts_of_contours,
                    flags,
                    x_coordinates,
                    y_coordinates,
                    ..
                } => {
                    assert_eq!(header.x_min(), 185);
                    assert_eq!(header.y_min(), 0);
                    assert_eq!(header.x_max(), 1015);
                    assert_eq!(header.y_max(), 1380);
                    assert_eq!(end_pts_of_contours.len(), 1);
                    assert_eq!(end_pts_of_contours[0], 11);
                    assert!(flags.iter().all(|f| f & ON_CURVE_POINT != 0));
                    assert_eq!(x_coordinates, vec![1015, 1015, 717, 717, 1015, 1015, 185, 185, 483, 483, 185, 185]);
                    assert_eq!(y_coordinates, vec![1380, 1203, 1203, 178, 178, 0, 0, 178, 178, 1203, 1203, 1380]);
                }
                Glyph::Composite { .. } => panic!("GLYPH POSITION IS WRONG BECAUSE I IS NOT A COMPOSITE GLYPH")
            }
        }
        None => panic!("COULDN'T FIND I IN GLYF VEC")
    }
    
    // Add more tests as we need to use more parts of the parser
}

#[test]
fn test_atlas() {
    let font_path = format!("{}/tests/fixtures/FiraCode-Medium.ttf", env!("CARGO_MANIFEST_DIR"));
    let font_file = match FontFile::parse_font_file(font_path) {
        Ok(ff) => ff,
        Err(e) => panic!("FONT FILE PARSE FAIL: {}", e)
    };
    
    let head = match font_file.parse_head() {
        Ok(ht) => ht,
        Err(e) => panic!("HEAD TABLE PARSE FAIL: {}", e)
    };
    
    let maxp = match font_file.parse_maxp() {
        Ok(mt) => mt,
        Err(e) => panic!("MAXP TABLE PARSE FAIL: {}", e)
    };
    
    let loca = match font_file.parse_loca(maxp.num_glyphs(), head.index_to_loc_format()) {
        Ok(lt) => lt,
        Err(e) => panic!("LOCA TABLE PARSE FAIL: {}", e)
    };
    
    let glyf = match font_file.parse_glyf(loca) {
        Ok(gt) => gt,
        Err(e) => panic!("GLYF TABLE PARSE FAIL: {}", e)
    };
    
    let i_glyph = glyf[43].clone().unwrap().to_owned();
    let outline = create_outline(&i_glyph, &glyf);
    let mut atlas = Atlas::new(AtlasAlgorithm::ShelfPacker);
    match i_glyph {
        Glyph::Simple { header, .. } => {
            let bitmap = rasterize(outline, header);
            atlas.append(vec![('I', bitmap)]);
        }
        _ => unreachable!()
    };
    // Fields are private now, make them public if need to test again
    // assert_eq!(atlas.lookup_table()[73], Some(GlyphRegion { x: 0, y: 0, width: 1015 - 185, height: 1380 - 0 }));
}