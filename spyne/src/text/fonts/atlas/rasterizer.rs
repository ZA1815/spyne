use crate::text::fonts::{atlas::outline::Segment, parse::structures::GlyphHeader};

pub fn rasterize(outline: Vec<Vec<Segment>>, glyph_header: GlyphHeader) -> Vec<Vec<bool>> {
    let x_diff = (glyph_header.x_max - glyph_header.x_min) as usize;
    let y_diff = (glyph_header.y_max - glyph_header.y_min) as usize;
    let mut bitmap = vec![vec![false; x_diff]; y_diff];
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
                *pixel = true
            }
        });
    });
    
    bitmap
}