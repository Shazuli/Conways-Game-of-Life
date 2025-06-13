//! Contains macros and methods to manipulate a struct Chunk.

use super::Chunk;

/// Specify rotation direction and how much to rotate.
pub enum Rotation {
    ClockwiseDegrees(i16),
    AntiClockwiseDegrees(i16)
}

/// Sets cells in struct Chunk at local position to alive.
/// ```no_run
/// use conways_game_of_life_dyn_lib::{set_cells_alive, ChunkCellData8x8, Chunk};
/// 
/// let mut c = Chunk::new(0, 0, ChunkCellData8x8::from(0));
/// set_cells_alive!(c :=
///     0,0; 1,1; 2,2;
/// );
/// 
/// assert_eq!(unsafe { c.data.u64 }, 0x40201);
/// ```
#[macro_export]
macro_rules! set_cells_alive {
    ($c:ident := $($x:expr, $y:expr);+ $(;)?) => {
        use $crate::IS_ALIVE_BIT as S;
        $(
            $c.set_cell_state(($x + $y * 8) | S);
        )+
    }
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
    let mut x = x;

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

    let mut x = x;

    x = ((x >> 1) & K1) | ((x & K1) << 1);
    x = ((x >> 2) & K2) | ((x & K2) << 2);
        ((x >> 4) & K4) | ((x & K4) << 4)
}


impl Chunk {

    /// Sets all cells to dead for the chunk and returns itself.
    pub const fn set_all_dead(&mut self) -> &mut Self
    {
        unsafe { self.data.u64 ^= self.data.u64; }
        self
    }

    /// Mirrors the chunk's cell states vertically and returns itself.
    pub const fn mirror_vertical(&mut self) -> &mut Self
    {
        let data = unsafe { self.data.u64 };
        if data != 0 {
            self.data.u64 = flip_vertical(data);
        }
        self
    }

    /// Mirrors the chunk's cell states horizontally and returns itself.
    pub const fn mirror_horizontal(&mut self) -> &mut Self
    {
        let data = unsafe { self.data.u64 };
        if data != 0 {
            self.data.u64 = mirror_horizontal(data);
        }
        self
    }

    /// Rotates the chunk's cell states in that direction and returns itself. Supports only 90-degrees turns.
    pub const fn rotate(&mut self, direction: Rotation) -> &mut Self
    {
        let data = unsafe { self.data.u64 };

        if data != 0 {

            use Rotation as R;

            self.data.u64 = match direction {
                R::ClockwiseDegrees(90) | R::AntiClockwiseDegrees(270) | R::AntiClockwiseDegrees(-90) => {
                    flip_vertical(flip_diag_a1h8(data))
                },
                R::AntiClockwiseDegrees(90) | R::ClockwiseDegrees(270) | R::ClockwiseDegrees(-90) => {
                    flip_diag_a1h8(flip_vertical(data))
                },
                R::ClockwiseDegrees(180) | R::AntiClockwiseDegrees(180) | R::ClockwiseDegrees(-180) | R::AntiClockwiseDegrees(-180) => {
                    mirror_horizontal(flip_vertical(data))
                },
                _ => {
                    data
                }
            };
        }
        self
    }

}