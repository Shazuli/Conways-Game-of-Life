use super::Chunk;
use core::fmt;

pub enum Direction {
    Clockwise90,
    AntiClockwise90,
    Clockwise180
}

// Algorithms taken from the chess programming Wiki: https://www.chessprogramming.org/Flipping_Mirroring_and_Rotating

const fn flip_vertical(x: u64) -> u64
{
    ( (x << 56)                      ) |
    ( (x << 40) & 0x00ff000000000000 ) |
    ( (x << 24) & 0x0000ff0000000000 ) |
    ( (x <<  8) & 0x000000ff00000000 ) |
    ( (x >>  8) & 0x00000000ff000000 ) |
    ( (x >> 24) & 0x0000000000ff0000 ) |
    ( (x >> 40) & 0x000000000000ff00 ) |
    ( (x >> 56) )
}

const fn flip_diag_a1h8(x: u64) -> u64
{
    const K1: u64 = 0x5500550055005500;
    const K2: u64 = 0x3333000033330000;
    const K4: u64 = 0x0f0f0f0f00000000;

    let mut t: u64;
    let mut x: u64 = x;

    t  = K4 & (x ^ (x << 28));
    x ^=       t ^ (t >> 28) ;
    t  = K2 & (x ^ (x << 14));
    x ^=       t ^ (t >> 14) ;
    t  = K1 & (x ^ (x <<  7));
    x ^        t ^ (t >>  7)
}

const fn mirror_horizontal(x: u64) -> u64
{
    const K1: u64 = 0x5555555555555555;
    const K2: u64 = 0x3333333333333333;
    const K4: u64 = 0x0f0f0f0f0f0f0f0f;

    let mut x: u64 = x;

    x = ((x >> 1) & K1) | ((x & K1) << 1);
    x = ((x >> 2) & K2) | ((x & K2) << 2);
        ((x >> 4) & K4) | ((x & K4) << 4)
}


impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        unsafe {
            write!(f,"[{},{}]\n{:08b}\n{:08b}\n{:08b}\n{:08b}\n{:08b}\n{:08b}\n{:08b}\n{:08b}",
                self.x,self.y,
                self.data.u8x8[0],self.data.u8x8[1],self.data.u8x8[2],self.data.u8x8[3],
                self.data.u8x8[4],self.data.u8x8[5],self.data.u8x8[6],self.data.u8x8[7]
            )
        }
    }
}


impl Chunk {

    /// Mirrors the chunk's cell states vertically.
    pub fn mirror_vertical(&mut self) -> &mut Self
    {
        if !self.all_are_dead() {
            unsafe { self.data.u64 = flip_vertical(self.data.u64); }
        }
        self
    }

    /// Mirrors the chunk's cell states horizontally.
    pub fn mirror_horizontal(&mut self) -> &mut Self
    {
        if !self.all_are_dead() {
            unsafe { self.data.u64 = mirror_horizontal(self.data.u64); }
        }
        self
    }

    /// Rotates the chunk's cell states in that direction.
    pub fn rotate(&mut self, direction: Direction) -> &mut Self
    {
        if !self.all_are_dead() {
            self.data.u64 = match direction {
                Direction::Clockwise90 => {
                    unsafe { flip_vertical(flip_diag_a1h8(self.data.u64)) }
                },
                Direction::AntiClockwise90 => {
                    unsafe { flip_diag_a1h8(flip_vertical(self.data.u64)) }
                },
                Direction::Clockwise180 => {
                    unsafe { mirror_horizontal(flip_vertical(self.data.u64)) }
                }
            };
        }
        self
    }

}