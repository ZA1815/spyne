struct Atlas {
    texture: Vec<u8>,
    lookup_table: Vec<Option<GlyphRegion>>,
    algorithm: AtlasAlgorithm,
    width: usize,
    height: usize,
    current_x: usize,
    current_y: usize
}

#[derive(Clone, Copy)]
struct GlyphRegion {
    x: usize,
    y: usize,
    width: usize,
    height: usize
}

impl Atlas {
    pub fn new(algorithm: AtlasAlgorithm) -> Self {
        Self {
            texture: Vec::new(),
            lookup_table: Vec::new(),
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
        if self.width == 0 {
            let mut area = 0;
            bitmaps.iter()
                .for_each(|(_, bitmap)| {
                    area += bitmap.len() * bitmap[0].len();
                });
            let n = area.isqrt();
            if (n & (n - 1)) != 0 { self.width = 1 << (usize::BITS - n.leading_zeros()); } else { self.width = n };
            self.texture = vec![0; self.width * self.width];
            self.lookup_table = vec![None; 1114112];
        }
        self.height = bitmaps[0].1.len();
        bitmaps.iter()
            .for_each(|(char, bitmap)| {
                if self.current_x + bitmap[0].len() > self.width {
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