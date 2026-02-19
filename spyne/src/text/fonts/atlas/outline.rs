use crate::text::fonts::parse::{constants::ON_CURVE_POINT, structures::Glyph};

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
    flags: u8,
    x: isize,
    y: isize
}

pub fn create_outline(glyph: Glyph) -> Vec<Vec<Segment>> {
    match glyph {
        Glyph::Simple {
            header,
            end_pts_of_contours,
            instruction_length,
            instructions,
            flags,
            x_coordinates,
            y_coordinates
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
                    let (idx, _) = seg.iter().enumerate().find(|(idx, p)| p.flags & ON_CURVE_POINT != 0).unwrap();
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
            header,
            components,
            instruction_length,
            instructions
        } => {
            vec![]
        }
    }
}