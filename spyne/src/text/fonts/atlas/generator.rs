struct Atlas {
    pub texture: Vec<u8>,
    pub lookup_table: Vec<Option<GlyphRegion>>,
    pub algorithm: AtlasAlgorithm,
    pub width: usize,
    pub height: usize,
    pub current_x: usize,
    pub current_y: usize
}

#[derive(Clone, Copy)]
struct GlyphRegion {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize
}

impl Atlas {
    pub fn new(algorithm: AtlasAlgorithm) -> Self {
        Self {
            texture: Vec::new(),
            lookup_table: vec![None; 1114112],
            algorithm,
            width: 0,
            height: 0,
            current_x: 0,
            current_y: 0
        }
    }
    
    pub fn append(&mut self, bitmaps: Vec<(char, Vec<Vec<u8>>)>) {
        match self.algorithm {
            AtlasAlgorithm::ShelfPacker => self.shelf_packer(bitmaps)
        }
    }
    
    // UNOPTIMIZED, BENCHMARK LATER AND SEE IF IT NEEDS TO BE OPTIMIZED
    
    fn shelf_packer(&mut self, mut bitmaps: Vec<(char, Vec<Vec<u8>>)>) {
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
        if self.width < width {
            let mut new_tex = vec![0; width * width];
            if self.width != 0 {
                self.texture.chunks(self.width).into_iter().enumerate().for_each(|(idx, row)| {
                    new_tex[idx * width..idx * width + self.width].copy_from_slice(row);
                });
            }
            self.texture = new_tex;
            self.width = width;
        }
        bitmaps.iter()
            .for_each(|(char, bitmap)| {
                if self.current_x + bitmap[0].len() > self.width || bitmap.len() > self.height - self.current_y {
                    self.current_x = 0;
                    self.current_y = self.height;
                    self.height += bitmap.len();
                }
                
                self.lookup_table[*char as usize] = Some(GlyphRegion {
                    x: self.current_x,
                    y: self.current_y,
                    width: bitmap[0].len(),
                    height: bitmap.len()
                });
                
                bitmap.iter()
                    .enumerate()
                    .for_each(|(row_idx, row)| {
                        row.iter()
                            .enumerate()
                            .for_each(|(bit_idx, bit)| {
                                self.texture[(self.current_y + row_idx) * self.width + (self.current_x + bit_idx)] = *bit;
                            });
                    });
                
                self.current_x += bitmap[0].len();
            });
    }
}

pub enum AtlasAlgorithm {
    ShelfPacker
    // MaxRects
    // Skyline
}

#[cfg(test)]
mod test {
    use crate::text::fonts::atlas::generator::{Atlas, AtlasAlgorithm};

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
        atlas.append(
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
        assert_eq!(atlas.texture.len(), 64);
        assert_eq!(atlas.texture[0 * 8 + 0], 1);
        assert_eq!(atlas.texture[0 * 8 + 1], 1);
        assert_eq!(atlas.texture[0 * 8 + 2], 0);
        assert_eq!(atlas.texture[1 * 8 + 0], 1);
        assert_eq!(atlas.texture[1 * 8 + 1], 0);
        assert_eq!(atlas.texture[1 * 8 + 2], 1);
        assert_eq!(atlas.texture[2 * 8 + 0], 1);
        assert_eq!(atlas.texture[2 * 8 + 1], 1);
        assert_eq!(atlas.texture[2 * 8 + 2], 0);
        assert_eq!(atlas.texture[3 * 8 + 0], 1);
        assert_eq!(atlas.texture[3 * 8 + 1], 0);
        assert_eq!(atlas.texture[3 * 8 + 2], 1);
        assert_eq!(atlas.texture[4 * 8 + 0], 1);
        assert_eq!(atlas.texture[4 * 8 + 1], 1);
        assert_eq!(atlas.texture[4 * 8 + 2], 0);
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
        assert_eq!(atlas.texture[0 * 8 + 3], 0);
        assert_eq!(atlas.texture[0 * 8 + 4], 1);
        assert_eq!(atlas.texture[0 * 8 + 5], 0);
        assert_eq!(atlas.texture[1 * 8 + 3], 1);
        assert_eq!(atlas.texture[1 * 8 + 4], 1);
        assert_eq!(atlas.texture[1 * 8 + 5], 1);
        assert_eq!(atlas.texture[2 * 8 + 3], 1);
        assert_eq!(atlas.texture[2 * 8 + 4], 0);
        assert_eq!(atlas.texture[2 * 8 + 5], 1);
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
        assert_eq!(atlas.texture[5 * 8 + 0], 1);
        assert_eq!(atlas.texture[5 * 8 + 1], 1);
        assert_eq!(atlas.texture[5 * 8 + 2], 0);
        assert_eq!(atlas.texture[6 * 8 + 0], 1);
        assert_eq!(atlas.texture[6 * 8 + 1], 0);
        assert_eq!(atlas.texture[6 * 8 + 2], 1);
        assert_eq!(atlas.texture[7 * 8 + 0], 1);
        assert_eq!(atlas.texture[7 * 8 + 1], 1);
        assert_eq!(atlas.texture[7 * 8 + 2], 0);
        
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
        atlas.append(vec![('A', a_bitmap)]);
        atlas.append(vec![('D', d_bitmap)]);
        atlas.append(vec![('B', b_bitmap)]);
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
        assert_eq!(atlas.texture[0 * 8 + 0], 0);
        assert_eq!(atlas.texture[0 * 8 + 1], 1);
        assert_eq!(atlas.texture[0 * 8 + 2], 0);
        assert_eq!(atlas.texture[1 * 8 + 0], 1);
        assert_eq!(atlas.texture[1 * 8 + 1], 1);
        assert_eq!(atlas.texture[1 * 8 + 2], 1);
        assert_eq!(atlas.texture[2 * 8 + 0], 1);
        assert_eq!(atlas.texture[2 * 8 + 1], 0);
        assert_eq!(atlas.texture[2 * 8 + 2], 1);
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
        assert_eq!(atlas.texture[0 * 8 + 3], 1);
        assert_eq!(atlas.texture[0 * 8 + 4], 1);
        assert_eq!(atlas.texture[0 * 8 + 5], 0);
        assert_eq!(atlas.texture[1 * 8 + 3], 1);
        assert_eq!(atlas.texture[1 * 8 + 4], 0);
        assert_eq!(atlas.texture[1 * 8 + 5], 1);
        assert_eq!(atlas.texture[2 * 8 + 3], 1);
        assert_eq!(atlas.texture[2 * 8 + 4], 1);
        assert_eq!(atlas.texture[2 * 8 + 5], 0);
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
        assert_eq!(atlas.texture.len(), 64);
        assert_eq!(atlas.texture[3 * 8 + 0], 1);
        assert_eq!(atlas.texture[3 * 8 + 1], 1);
        assert_eq!(atlas.texture[3 * 8 + 2], 0);
        assert_eq!(atlas.texture[4 * 8 + 0], 1);
        assert_eq!(atlas.texture[4 * 8 + 1], 0);
        assert_eq!(atlas.texture[4 * 8 + 2], 1);
        assert_eq!(atlas.texture[5 * 8 + 0], 1);
        assert_eq!(atlas.texture[5 * 8 + 1], 1);
        assert_eq!(atlas.texture[5 * 8 + 2], 0);
        assert_eq!(atlas.texture[6 * 8 + 0], 1);
        assert_eq!(atlas.texture[6 * 8 + 1], 0);
        assert_eq!(atlas.texture[6 * 8 + 2], 1);
        assert_eq!(atlas.texture[7 * 8 + 0], 1);
        assert_eq!(atlas.texture[7 * 8 + 1], 1);
        assert_eq!(atlas.texture[7 * 8 + 2], 0);
    }
}