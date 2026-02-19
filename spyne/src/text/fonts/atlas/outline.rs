use std::collections;

use crate::text::fonts::parse::{constants::ON_CURVE_POINT, structures::Glyph};

pub enum Segment {
    Line(Point, Point),
    Quad {
        start: Point,
        control: Point,
        end: Point
    }
}

#[derive(Clone)]
struct Point {
    flags: u8,
    x: isize,
    y: isize
}

pub fn create_outline(glyph: Glyph) -> Vec<Segment> {
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
            for (idx, _) in points.into_iter().enumerate() {
                
            }
            for (idx, p) in points.iter().enumerate() {
                match points.get(idx + 1) {
                    Some(p) => {
                        
                    }
                    None => {
                        
                    }
                }
            }
            
            vec![]
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