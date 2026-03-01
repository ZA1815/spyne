use spyne_text::fonts::{atlas::{generator::{Atlas, AtlasAlgorithm}, outline::create_outline, rasterizer::rasterize}, parse::structures::{FontFile, Glyph}};

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