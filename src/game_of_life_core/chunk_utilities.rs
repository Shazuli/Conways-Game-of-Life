impl super::Chunk {

    /// Mirrors the chunk's cell states in the X-axis.
    pub fn mirror_x(&mut self)
    {
        for i in 0..4 {
            let j = 7 - i;
            unsafe {
                if self.data.bytes[i] != self.data.bytes[j] {// Skip if they are the same
                    // Switch place of two bytes without a third tmp variable.
                    self.data.bytes[i] = self.data.bytes[i] ^ self.data.bytes[j];
                    self.data.bytes[j] = self.data.bytes[i] ^ self.data.bytes[j];
                    self.data.bytes[i] = self.data.bytes[i] ^ self.data.bytes[j];
                }
            }
        }
    }

    /// Mirrors the chunk's cell states in the Y-axis.
    pub fn mirror_y(&mut self)
    {
        for i in 0..8 {
            let mut byte = unsafe { self.data.bytes[i] };
            if byte != 0 || byte != 0xff {// Skip if reversing does nothing for simple patterns
                // Fancy bit-hack.
                byte = (byte & 0xf0) >> 4 | (byte & 0x0f) << 4;
                byte = (byte & 0xcc) >> 2 | (byte & 0x33) << 2;
                byte = (byte & 0xaa) >> 1 | (byte & 0x55) << 1;
                unsafe { self.data.bytes[i] = byte; }
            }
        }
    }

    /*pub unsafe fn transpose(&mut self, m: i32, n: i32)
    {
    }*/

}