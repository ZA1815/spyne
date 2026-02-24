use crate::text::fonts::{atlas::outline::Segment, parse::structures::GlyphHeader};

// UNOPTIMIZED, BENCHMARK LATER AND SEE IF IT NEEDS TO BE OPTIMIZED

pub fn rasterize(outline: Vec<Vec<Segment>>, glyph_header: GlyphHeader) -> Vec<Vec<u8>> {
    let x_diff = (glyph_header.x_max - glyph_header.x_min) as usize;
    let y_diff = (glyph_header.y_max - glyph_header.y_min) as usize;
    let mut bitmap = vec![vec![0; x_diff]; y_diff];
    bitmap.iter_mut().enumerate().for_each(|(row, row_vec)| {
        row_vec.iter_mut().enumerate().for_each(|(col, pixel)| {
            let x_coord = (col as i16 + glyph_header.x_min) as isize;
            let y_coord = (row as i16 + glyph_header.y_min) as isize;
            let mut num_intersections: usize = 0;
            outline.iter().flatten().for_each(|seg| {
                match seg {
                    Segment::Line(p1, p2) => {
                        let (min, max) = if p1.x < p2.x { (p1.x, p2.x) } else { (p2.x, p1.x) };
                        if (min..max).contains(&x_coord) && p2.x - p1.x != 0 { 
                            let t = (x_coord - p1.x) as f32 / (p2.x - p1.x) as f32;
                            if (0.0..1.0).contains(&t) {
                                let y_int = (1.0 - t) * p1.y as f32 + t * p2.y as f32;
                                if y_int > y_coord as f32 {
                                    num_intersections += 1;
                                }
                            }
                        }
                    }
                    Segment::Quad {
                        start,
                        control,
                        end
                    } => {
                        let a = start.x - 2 * control.x + end.x;
                        let b = 2 * (control.x - start.x);
                        let c = start.x - x_coord;
                        let discriminant = (b.pow(2) - 4 * a * c) as f32;
                        if discriminant > 0.0 {
                            let t1 = (-b as f32 + discriminant.sqrt()) / (2 * a) as f32;
                            let t2 = (-b as f32 - discriminant.sqrt()) / (2 * a) as f32;
                            let mut quad_bezier = |t: f32| {
                                if (0.0..1.0).contains(&t) {
                                    let y_int = (start.y as f32 * (1.0 - t) + control.y as f32 * t) * (1.0 - t) + (control.y as f32 * (1.0 - t) + end.y as f32 * t) * t;
                                    if y_int > y_coord as f32 {
                                        num_intersections += 1;
                                    }
                                }
                            };
                            quad_bezier(t1);
                            quad_bezier(t2);
                        }
                    }
                }
            });
            
            if num_intersections % 2 != 0 {
                *pixel = 255;
            }
        });
    });
    
    bitmap
}

#[cfg(test)]
mod test {
    use crate::text::fonts::{atlas::{outline::{Point, Segment}, rasterizer::rasterize}, parse::{constants::ON_CURVE_POINT, structures::GlyphHeader}};

    // Currently only tests line segments (a square), add test for beziers later
    #[test]
    fn test_rasterizer() {
        let outline: Vec<Vec<Segment>> = vec![
            vec![
                Segment::Line(
                    Point { flags: ON_CURVE_POINT, x: 0, y: 0 },
                    Point { flags: ON_CURVE_POINT, x: 10, y: 0 }
                ),
                Segment::Line(
                    Point { flags: ON_CURVE_POINT, x: 10, y: 0 },
                    Point { flags: ON_CURVE_POINT, x: 10, y: -10 }
                ),
                Segment::Line(
                    Point { flags: ON_CURVE_POINT, x: 10, y: -10 }, 
                    Point { flags: ON_CURVE_POINT, x: 0, y: -10 }
                ),
                Segment::Line(
                    Point { flags: ON_CURVE_POINT, x: 0, y: -10 }, 
                    Point { flags: ON_CURVE_POINT, x: 0, y: 0 }
                )
            ]
        ];
        let glyph_header = GlyphHeader {
            number_of_contours: 1,
            x_min: 0,
            y_min: -20,
            x_max: 10,
            y_max: 20
        };
        let bitmap = rasterize(outline, glyph_header);
        assert_eq!(bitmap[17][4], 255);
        assert_eq!(bitmap[12][7], 255);
        assert_eq!(bitmap[5][8], 0);
    }
}