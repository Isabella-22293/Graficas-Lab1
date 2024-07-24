pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            background_color: 0x000000,
            current_color: 0xFFFFFF
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = self.current_color;
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }
   
    pub fn get_current_color(&self) -> u32 {
        self.current_color
    }
    
    pub fn get_background_color(&self) -> u32 {
        self.background_color
    }

    pub fn flip_horizontal(&mut self) {
        let row_size = self.width;
        
        for y in 0..(self.height / 2) {
            let top_row_start = y * row_size;
            let bottom_row_start = (self.height - y - 1) * row_size;
            
            let top_row = self.buffer[top_row_start..top_row_start + row_size].to_vec();
            let bottom_row = self.buffer[bottom_row_start..bottom_row_start + row_size].to_vec();
            
            self.buffer[top_row_start..top_row_start + row_size].copy_from_slice(&bottom_row);
            self.buffer[bottom_row_start..bottom_row_start + row_size].copy_from_slice(&top_row);
        }
    } 
}

    