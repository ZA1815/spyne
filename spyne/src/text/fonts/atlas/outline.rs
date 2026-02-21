use crate::text::fonts::parse::{constants::{ARGS_ARE_XY_VALUES, ON_CURVE_POINT, WE_HAVE_A_SCALE, WE_HAVE_A_TWO_BY_TWO, WE_HAVE_AN_X_AND_Y_SCALE}, structures::Glyph};

pub enum Segment {
    Line(Point, Point),
    Quad {
        start: Point,
        control: Point,
        end: Point
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub flags: u8,
    pub x: isize,
    pub y: isize
}

pub fn create_outline(glyph: &Glyph, lookup: &[Glyph]) -> Vec<Vec<Segment>> {
    match glyph {
        Glyph::Simple {
            end_pts_of_contours,
            flags,
            x_coordinates,
            y_coordinates,
            ..
        } => {
            if flags.len() != x_coordinates.len() || x_coordinates.len() != y_coordinates.len() {
                // Change this to an error later
                panic!("Lengths of flags, x_coordinates, and y_coordinates Vecs are not equal");
            }
            let points_unsliced: Vec<Point> = flags.iter()
                .zip(x_coordinates.iter())
                .zip(y_coordinates.iter())
                .map(|((f, x), y)| {
                    Point {
                        flags: *f,
                        x: *x as isize,
                        y: *y as isize
                    }
                })
                .collect();
            let mut points: Vec<Vec<Point>> = Vec::with_capacity(end_pts_of_contours.len());
            let mut start_idx: usize = 0;
            for idx in end_pts_of_contours.iter() {
                points.push(points_unsliced[start_idx..=*idx as usize].to_vec());
                start_idx = *idx as usize + 1;
            }
            
            points.iter_mut()
                .for_each(|seg| {
                    let mut inserted: usize = 0;
                    let seg_vec = seg.to_vec();
                    let mut points_iter = seg_vec.into_iter().enumerate().peekable();
                    while let Some((_, p1)) = points_iter.next() {
                        if p1.flags & ON_CURVE_POINT == 0 {
                            match points_iter.peek() {
                                Some((idx, p2)) => {
                                    if p2.flags & ON_CURVE_POINT == 0 {
                                        let new_point = Point {
                                            flags: ON_CURVE_POINT,
                                            x: (p1.x + p2.x) / 2,
                                            y: (p1.y + p2.y) / 2
                                        };
                                        seg.insert(*idx + inserted, new_point);
                                        inserted += 1;
                                    }
                                },
                                None => {
                                    let first_point = &seg[0];
                                    if first_point.flags & ON_CURVE_POINT == 0 {
                                        let new_point = Point {
                                            flags: ON_CURVE_POINT,
                                            x: (p1.x + first_point.x) / 2,
                                            y: (p1.y + first_point.y) / 2
                                        };
                                        seg.insert(0, new_point);
                                    }
                                }
                            }
                        }
                    }
                    let (idx, _) = seg.iter().enumerate().find(|(_, p)| p.flags & ON_CURVE_POINT != 0).unwrap();
                    seg.rotate_left(idx);
                });
            
            points.into_iter()
                .map(|seg| {
                    let mut segment: Vec<Segment> = Vec::new();
                    let mut seg_iter = seg.iter().peekable();
                    let mut current_start_point: Point = *seg_iter.next().unwrap();
                    let mut needs_closing: bool = true;
                    while let Some(second_point) = seg_iter.next() {
                        needs_closing = true;
                        if second_point.flags & ON_CURVE_POINT != 0 {
                            segment.push(Segment::Line(current_start_point, *second_point));
                            current_start_point = *second_point;
                        }
                        else {
                            if let Some(third_point) = seg_iter.next() {
                                segment.push(Segment::Quad {
                                    start: current_start_point,
                                    control: *second_point,
                                    end: *third_point
                                });
                                current_start_point = *third_point;
                            }
                            else {
                                needs_closing = false;
                                let third_point = &seg[0];
                                segment.push(Segment::Quad {
                                    start: current_start_point,
                                    control: *second_point,
                                    end: *third_point
                                });
                                current_start_point = *third_point;
                            }
                        }
                    }
                    
                    if needs_closing {
                        segment.push(Segment::Line(current_start_point, seg[0]));
                    }
                    
                    segment
                }).collect::<Vec<Vec<Segment>>>()
        },
        Glyph::Composite {
            components,
            ..
        } => {
            components.iter()
                .flat_map(|comp| {
                    let glyph = &lookup[comp.glyph_index as usize];
                    let mut glyph_base = create_outline(glyph, lookup);
                    let a: f32;
                    let b: f32;
                    let c: f32;
                    let d: f32;
                    if comp.flags & WE_HAVE_A_SCALE != 0 {
                        a = comp.transformation[0] as f32 / 16384.0;
                        b = 0.0;
                        c = 0.0;
                        d = a;
                    }
                    else if comp.flags & WE_HAVE_AN_X_AND_Y_SCALE != 0 {
                        a = comp.transformation[0] as f32 / 16384.0;
                        b = 0.0;
                        c = 0.0;
                        d = comp.transformation[1] as f32 / 16384.0;
                    }
                    else if comp.flags & WE_HAVE_A_TWO_BY_TWO != 0 {
                        a = comp.transformation[0] as f32 / 16384.0;
                        b = comp.transformation[1] as f32 / 16384.0;
                        c = comp.transformation[2] as f32 / 16384.0;
                        d = comp.transformation[3] as f32 / 16384.0;
                    }
                    else {
                        a = 1.0;
                        b = 0.0;
                        c = 0.0;
                        d = 1.0;
                    }
                    let transform = |point: &mut Point| {
                        let orig_x = point.x;
                        let orig_y = point.y;
                        point.x = (orig_x as f32 * a + orig_y as f32 * c).round() as isize;
                        point.y = (orig_x as f32 * b + orig_y as f32 * d).round() as isize;
                        if comp.flags & ARGS_ARE_XY_VALUES != 0 {
                            point.x += comp.argument_1 as isize;
                            point.y += comp.argument_2 as isize;
                        }
                    };
                    glyph_base.iter_mut().flatten().for_each(|seg| {
                        match seg {
                            Segment::Line(p1, p2) => {
                                transform(p1);
                                transform(p2);
                            },
                            Segment::Quad { start, control, end } => {
                                transform(start);
                                transform(control);
                                transform(end);
                            }
                        }
                    });
                    
                    glyph_base
                }).collect()
        }
    }
}