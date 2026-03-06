pub struct Atlas {
    lookup_table: Vec<Option<GlyphRegion>>,
    algorithm: AtlasAlgorithm,
    width: usize,
    height: usize,
    current_x: usize,
    current_y: usize
}

impl Atlas {
    pub fn lookup_table(&self) -> &[Option<GlyphRegion>] {
        &self.lookup_table
    }
    
    pub fn algorithm(&self) -> AtlasAlgorithm {
        self.algorithm
    }
    
    pub fn width(&self) -> usize {
        self.width
    }
    
    pub fn height(&self) -> usize {
        self.height
    }
    
    pub fn current_x(&self) -> usize {
        self.current_x
    }
    
    pub fn current_y(&self) -> usize {
        self.current_y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlyphRegion {
    x: usize,
    y: usize,
    width: usize,
    height: usize
}

impl GlyphRegion {
    pub fn x(&self) -> usize {
        self.x
    }
    
    pub fn y(&self) -> usize {
        self.y
    }
    
    pub fn width(&self) -> usize {
        self.width
    }
    
    pub fn height(&self) -> usize {
        self.height
    }
}

impl Atlas {
    pub fn new(algorithm: AtlasAlgorithm) -> Self {
        Self {
            lookup_table: vec![None; 1114112],
            algorithm,
            width: 0,
            height: 0,
            current_x: 0,
            current_y: 0
        }
    }
    
    pub fn append(&mut self, bitmaps: Vec<(char, Vec<Vec<u8>>)>) -> Vec<(GlyphRegion, Vec<u8>)> {
        match self.algorithm {
            AtlasAlgorithm::ShelfPacker => self.shelf_packer(bitmaps)
        }
    }
    
    // UNOPTIMIZED, BENCHMARK LATER AND SEE IF IT NEEDS TO BE OPTIMIZED
    
    fn shelf_packer(&mut self, mut bitmaps: Vec<(char, Vec<Vec<u8>>)>) -> Vec<(GlyphRegion, Vec<u8>)> {
        bitmaps.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
        let mut area = 0;
        bitmaps.iter()
            .for_each(|(_, bitmap)| {
                area += bitmap.len() * bitmap[0].len();
            });
        area += self.width * self.width;
        let n = area.isqrt();
        let width = if (n & (n - 1)) != 0 { 1 << (usize::BITS - n.leading_zeros()) } else { n };
        if self.height == 0 { self.height = bitmaps[0].1.len(); }
        if self.width < width { self.width = width; }
        bitmaps.into_iter()
            .map(|(char, bitmap)| {
                if self.current_x + bitmap[0].len() > self.width || bitmap.len() > self.height - self.current_y {
                    self.current_x = 0;
                    self.current_y = self.height;
                    self.height += bitmap.len();
                }
                
                let region = GlyphRegion {
                    x: self.current_x,
                    y: self.current_y,
                    width: bitmap[0].len(),
                    height: bitmap.len()
                };
                
                self.lookup_table[char as usize] = Some(region);
                
                self.current_x += bitmap[0].len();
                
                (region, bitmap.into_iter().flatten().collect::<Vec<u8>>())
            }).collect::<Vec<(GlyphRegion, Vec<u8>)>>()
    }
}

#[derive(Clone, Copy)]
pub enum AtlasAlgorithm {
    ShelfPacker
    // MaxRects
    // Skyline
}

#[cfg(test)]
mod test {
    use crate::fonts::atlas::generator::{Atlas, AtlasAlgorithm};

    #[test]
    fn test_shelf_packer() {
        let mut atlas = Atlas::new(AtlasAlgorithm::ShelfPacker);
        let a_bitmap = vec![
            vec![0, 1, 0],
            vec![1, 1, 1],
            vec![1, 0, 1]
        ];
        let d_bitmap = vec![
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![1, 1, 0]
        ];
        let b_bitmap = vec![
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![1, 1, 0]
        ];
        let regions = atlas.append(
            vec![
                ('A', a_bitmap),
                ('D', d_bitmap),
                ('B', b_bitmap)
            ]
        );
        let b_glyph_region = atlas.lookup_table['B' as usize];
        match b_glyph_region {
            Some(gr) => {
                assert_eq!(gr.x, 0);
                assert_eq!(gr.y, 0);
                assert_eq!(gr.width, 3);
                assert_eq!(gr.height, 5);
            }
            None => panic!("Glyph 'B' doesn't exist when it clearly should")
        }
        assert_eq!(regions[0].1.len(), 15);
        assert_eq!(regions[0].1[0 * 3 + 0], 1);
        assert_eq!(regions[0].1[0 * 3 + 1], 1);
        assert_eq!(regions[0].1[0 * 3 + 2], 0);
        assert_eq!(regions[0].1[1 * 3 + 0], 1);
        assert_eq!(regions[0].1[1 * 3 + 1], 0);
        assert_eq!(regions[0].1[1 * 3 + 2], 1);
        assert_eq!(regions[0].1[2 * 3 + 0], 1);
        assert_eq!(regions[0].1[2 * 3 + 1], 1);
        assert_eq!(regions[0].1[2 * 3 + 2], 0);
        assert_eq!(regions[0].1[3 * 3 + 1], 0);
        assert_eq!(regions[0].1[3 * 3 + 2], 1);
        assert_eq!(regions[0].1[4 * 3 + 0], 1);
        assert_eq!(regions[0].1[4 * 3 + 1], 1);
        assert_eq!(regions[0].1[4 * 3 + 2], 0);
        let a_glyph_region = atlas.lookup_table['A' as usize];
        match a_glyph_region {
            Some(gr) => {
                assert_eq!(gr.x, 3);
                assert_eq!(gr.y, 0);
                assert_eq!(gr.width, 3);
                assert_eq!(gr.height, 3);
            }
            None => panic!("Glyph 'A' doesn't exist when it clearly should")
        }
        assert_eq!(regions[1].1[0 * 3 + 0], 0);
        assert_eq!(regions[1].1[0 * 3 + 1], 1);
        assert_eq!(regions[1].1[0 * 3 + 2], 0);
        assert_eq!(regions[1].1[1 * 3 + 0], 1);
        assert_eq!(regions[1].1[1 * 3 + 1], 1);
        assert_eq!(regions[1].1[1 * 3 + 2], 1);
        assert_eq!(regions[1].1[2 * 3 + 0], 1);
        assert_eq!(regions[1].1[2 * 3 + 1], 0);
        assert_eq!(regions[1].1[2 * 3 + 2], 1);
        // Horizontal overflow
        let d_glyph_region = atlas.lookup_table['D' as usize];
        match d_glyph_region {
            Some(gr) => {
                assert_eq!(gr.x, 0);
                assert_eq!(gr.y, 5);
                assert_eq!(gr.width, 3);
                assert_eq!(gr.height, 3);
            },
            None => panic!("Glyph 'D' doesn't exist when it clearly should")
        }
        assert_eq!(regions[2].1[0 * 3 + 0], 1);
        assert_eq!(regions[2].1[0 * 3 + 1], 1);
        assert_eq!(regions[2].1[0 * 3 + 2], 0);
        assert_eq!(regions[2].1[1 * 3 + 0], 1);
        assert_eq!(regions[2].1[1 * 3 + 1], 0);
        assert_eq!(regions[2].1[1 * 3 + 2], 1);
        assert_eq!(regions[2].1[2 * 3 + 0], 1);
        assert_eq!(regions[2].1[2 * 3 + 1], 1);
        assert_eq!(regions[2].1[2 * 3 + 2], 0);
        
        // Vertical Overflow
        let mut atlas = Atlas::new(AtlasAlgorithm::ShelfPacker);
        let a_bitmap = vec![
            vec![0, 1, 0],
            vec![1, 1, 1],
            vec![1, 0, 1]
        ];
        let d_bitmap = vec![
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![1, 1, 0]
        ];
        let b_bitmap = vec![
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![1, 1, 0]
        ];
        let a_region = atlas.append(vec![('A', a_bitmap)]);
        let d_region = atlas.append(vec![('D', d_bitmap)]);
        let b_region = atlas.append(vec![('B', b_bitmap)]);
        let a_glyph_region = atlas.lookup_table['A' as usize];
        match a_glyph_region {
            Some(gr) => {
                assert_eq!(gr.x, 0);
                assert_eq!(gr.y, 0);
                assert_eq!(gr.width, 3);
                assert_eq!(gr.height, 3);
            }
            None => panic!("Glyph 'A' doesn't exist when it clearly should")
        }
        assert_eq!(a_region[0].1[0 * 3 + 0], 0);
        assert_eq!(a_region[0].1[0 * 3 + 1], 1);
        assert_eq!(a_region[0].1[0 * 3 + 2], 0);
        assert_eq!(a_region[0].1[1 * 3 + 0], 1);
        assert_eq!(a_region[0].1[1 * 3 + 1], 1);
        assert_eq!(a_region[0].1[1 * 3 + 2], 1);
        assert_eq!(a_region[0].1[2 * 3 + 0], 1);
        assert_eq!(a_region[0].1[2 * 3 + 1], 0);
        assert_eq!(a_region[0].1[2 * 3 + 2], 1);
        let d_glyph_region = atlas.lookup_table['D' as usize];
        match d_glyph_region {
            Some(gr) => {
                assert_eq!(gr.x, 3);
                assert_eq!(gr.y, 0);
                assert_eq!(gr.width, 3);
                assert_eq!(gr.height, 3);
            },
            None => panic!("Glyph 'D' doesn't exist when it clearly should")
        }
        assert_eq!(d_region[0].1[0 * 3 + 0], 1);
        assert_eq!(d_region[0].1[0 * 3 + 1], 1);
        assert_eq!(d_region[0].1[0 * 3 + 2], 0);
        assert_eq!(d_region[0].1[1 * 3 + 0], 1);
        assert_eq!(d_region[0].1[1 * 3 + 1], 0);
        assert_eq!(d_region[0].1[1 * 3 + 2], 1);
        assert_eq!(d_region[0].1[2 * 3 + 0], 1);
        assert_eq!(d_region[0].1[2 * 3 + 1], 1);
        assert_eq!(d_region[0].1[2 * 3 + 2], 0);
        let b_glyph_region = atlas.lookup_table['B' as usize];
        match b_glyph_region {
            Some(gr) => {
                assert_eq!(gr.x, 0);
                assert_eq!(gr.y, 3);
                assert_eq!(gr.width, 3);
                assert_eq!(gr.height, 5);
            }
            None => panic!("Glyph 'B' doesn't exist when it clearly should")
        }
        assert_eq!(b_region[0].1.len(), 15);
        assert_eq!(b_region[0].1[0 * 3 + 0], 1);
        assert_eq!(b_region[0].1[0 * 3 + 1], 1);
        assert_eq!(b_region[0].1[0 * 3 + 2], 0);
        assert_eq!(b_region[0].1[1 * 3 + 0], 1);
        assert_eq!(b_region[0].1[1 * 3 + 1], 0);
        assert_eq!(b_region[0].1[1 * 3 + 2], 1);
        assert_eq!(b_region[0].1[2 * 3 + 0], 1);
        assert_eq!(b_region[0].1[2 * 3 + 1], 1);
        assert_eq!(b_region[0].1[2 * 3 + 2], 0);
        assert_eq!(b_region[0].1[3 * 3 + 0], 1);
        assert_eq!(b_region[0].1[3 * 3 + 1], 0);
        assert_eq!(b_region[0].1[3 * 3 + 2], 1);
        assert_eq!(b_region[0].1[4 * 3 + 0], 1);
        assert_eq!(b_region[0].1[4 * 3 + 1], 1);
        assert_eq!(b_region[0].1[4 * 3 + 2], 0);
    }
}