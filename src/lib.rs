#![cfg_attr(not(feature = "std"), no_std)]

#[doc(hidden)]
pub use self::game_of_life_core::{ChunkCellData, Chunk, Field, game_of_life};

//#[cfg(feature = "alloc")]
extern crate alloc;

/// Core module containing helper functions, structs, macros and methods for Conway's Game of Life modules.
pub mod game_of_life_core
{
    use alloc::vec::Vec;
    
    macro_rules! set_bit {
        ($val:expr, $bit:expr) => {
            $val |= 1<<$bit
        };
    }

    /// Chunk data for cell states in a 8x8 bitboard (LSB top-right, MSB bottom-left).
    #[derive(Clone, Copy)]
    pub union ChunkCellData {
        pub u64: u64,
        pub u32x2: [u32; 2],
        pub u8x8: [u8; 8]
    }

    /// Chunk holding chunk data and chunk coordinates.
    #[derive(Clone, Copy)]
    pub struct Chunk {
        x: i32,
        y: i32,
        data: ChunkCellData
    }

    /// The field holding current generation number and current- and next generation of chunks.
    #[derive(Clone)]
    pub struct Field {
        generation: u32,
        chunks: [Vec<Chunk>; 2]
    }

    /*
    + <----0
           |
           |
          \/
          +
    */

    impl Default for ChunkCellData {
        fn default() -> Self
        {
            ChunkCellData { u64: 0 }
        }
    }

    impl Default for Chunk {
        fn default() -> Self
        {
            Chunk { x: 0, y: 0, data: ChunkCellData::default() }
        }
    }

    impl Chunk {

        pub fn get_x(&self) -> i32
        {
            self.x
        }

        pub fn get_y(&self) -> i32
        {
            self.y
        }

        /// Get cell states as u64.
        pub fn get_data_u64(&self) -> u64
        {
            unsafe { self.data.u64 }
        }

        /// Get cell states as mutable u64.
        pub fn get_mut_data_u64(&mut self) -> &mut u64
        {
            unsafe { &mut self.data.u64 }
        }

        /// Get if cell at index is alive.
        pub fn is_alive(&self, index: i8) -> Result<bool, (&str, i8)>
        {
            if index < 0 || index > 63 {
                return Err(("Out of bounds", index));
            }
            Ok(unsafe { self.data.u64 } & 1<<index >= 1)
        }

        /// Gets if all cells in chunk are dead.
        pub fn all_are_dead(&self) -> bool
        {
            unsafe { self.data.u64 == 0 }
        }

        /// Sets all cells to dead for the chunk.
        pub fn set_all_dead(&mut self)
        {
            unsafe { self.data.u64 ^= self.data.u64; }
        }

    }

    impl Field {
        /// Create a new empty Field struct.
        pub fn new() -> Self
        {
            Field {
                generation: 0,
                chunks: [Vec::new(), Vec::new()]
            }
        }

        /// Get current generation.
        pub fn get_generation(&self) -> u32
        {
            self.generation
        }

        /// Get current generation Vec of Chunks.
        pub fn get_current(&self) -> &Vec<Chunk>
        {
            &self.chunks[(self.generation as u8 & 1) as usize]
        }

        /// Get current generation of mut Vec of Chunks.
        fn get_mut_current(&mut self) -> &mut Vec<Chunk>
        {
            &mut self.chunks[(self.generation as u8 & 1) as usize]
        }

        /// Get next generation of mut Vec of Chunks.
        fn get_mut_next(&mut self) -> &mut Vec<Chunk>
        {
            &mut self.chunks[(!(self.generation as u8) & 1) as usize]
        }
    }

    // Cell index for cells with all neighbours within the same chunk and cell index for neighbours within- and outside its chunk.
    const CELL_INDEX_INNER: &[i8; 36] = &[
        09,10,11,12,13,14,
        17,18,19,20,21,22,
        25,26,27,28,29,30,
        33,34,35,36,37,38,
        41,42,43,44,45,46,
        49,50,51,52,53,54
    ];
    const CELL_INDEX_OUTER: &[i8; 28] = &[
        00,01,02,03,04,05,06,07,
        08,                  15,
        16,                  23,
        24,                  31,
        32,                  39,
        40,                  47,
        48,                  55,
        56,57,58,59,60,61,62,63
    ];
    // Bit-masks to check if index rotated around in x- or y direction.
    const X_MASK: u64 = 0x7e7e7e7e7e7e7e7e;
    const Y_MASK: u64 = 0x00ffffffffffff00;
    // 3x3 bit-mask at [1,1] (111\n101\n111).
    const BIT_MASK_1_1: u32 = 0x70507;


    /// Calculate the inner cells area of a chunk and skip if exceeding the threshold.
    fn calc_chunk_inner(chunk: &Chunk, threshold: i8, rules: fn(&mut ChunkCellData, i8, i8)) -> ChunkCellData
    {
        let mut cell_state: i8;
        let mut new_data = ChunkCellData { u64: 0 };// Assume every cell died
        let mut data_masked = ChunkCellData::default();

        'cells_inner: for cell_index in CELL_INDEX_INNER.iter() {

            // If cell is alive set sign bit.
            cell_state = if chunk.is_alive(*cell_index).unwrap() { -0x80 } else { 0 };

            // Calculate 3x3 and stop if exceeding the threshold.
            {                
                unsafe { data_masked.u32x2[0] = (chunk.data.u64 >> (cell_index-9)) as u32 & BIT_MASK_1_1; }
                
                while unsafe { data_masked.u32x2[0] } != 0 {
                    // Count LSB for each byte.
                    cell_state += unsafe {
                        (data_masked.u8x8[0] & 1) as i8 +
                        (data_masked.u8x8[1] & 1) as i8 +
                        (data_masked.u8x8[2] & 1) as i8
                    };

                    // Exit if exceeding the threshold.
                    if cell_state & 0x7f > threshold {
                        continue 'cells_inner
                    }

                    // Shift out the bits and get the next ones.
                    unsafe {
                        data_masked.u8x8[0] >>= 1;
                        data_masked.u8x8[1] >>= 1;
                        data_masked.u8x8[2] >>= 1;
                    }
                }
            }
            rules(&mut new_data, *cell_index, cell_state);
        }
        new_data
    }

    /// Calculate the outer cells index of chunks->4 and skip if exceeding the threshold.
    /// Remaining chunks in array is neighbouring chunks in order: top right to bottom left.
    /// 2 1 0
    /// 5 4 3
    /// 8 7 6
    fn calc_chunk_outer(chunks: [&Chunk; 9], threshold: i8, rules: fn(&mut ChunkCellData, i8, i8)) -> ChunkCellData
    {
        let mut new_data = ChunkCellData { u64: 0 };// Assume every cell died
        let mut cell_state: i8;
        let (mut chunk_index_offset, mut cell_index_offset): (usize, i8);

        'cells_outer: for cell_index in CELL_INDEX_OUTER.iter() {

            // If cell is alive set sign bit.
            cell_state = if chunks[4].is_alive(*cell_index).unwrap() { -0x80 } else { 0 };

            // Calculate 3x3 and stop if exceeding the threshold.
            for x_offset in -1i8..=1 {
                for y_offset in -1i8..=1 {

                    if x_offset | y_offset != 0 {// Skip if offset == [0,0]

                        chunk_index_offset = 4;// Start in the middle chunk
                        cell_index_offset = *cell_index;// Start in middle of neighbours

                        // TODO: this may need some refactoring.

                        if x_offset == -1 {
                            let x_to_shift = *cell_index + x_offset;
                            // Check if it went out-of-bounds to the right.
                            if (x_to_shift >= 0 && x_to_shift < 64) && (X_MASK | (X_MASK >> 1)) & (1<<x_to_shift) >= 1 {
                                // In same chunk.
                                cell_index_offset += x_offset;
                            } else {
                                // Looped around.
                                cell_index_offset += 7;
                                chunk_index_offset -= 1;
                            }
                        } else if x_offset == 1 {
                            let x_to_shift = *cell_index + x_offset;
                            // Check if it went out-of-bounds to the left.
                            if (x_to_shift >= 0 && x_to_shift < 64) && (X_MASK | (X_MASK << 1)) & (1<<x_to_shift) >= 1 {
                                // In same chunk.
                                cell_index_offset += x_offset;
                            } else {
                                // Looped around.
                                cell_index_offset -= 7;
                                chunk_index_offset += 1;
                            }
                        }

                        if y_offset == -1 {
                            let y_to_shift = *cell_index + y_offset * 8;
                            // Check if it went out-of-bounds up.
                            if (y_to_shift >= 0 && y_to_shift < 64) && (Y_MASK | (Y_MASK >> 8)) & (1<<y_to_shift) >= 1 {
                                // In same chunk.
                                cell_index_offset += y_offset * 8;
                            } else {
                                // Looped around.
                                cell_index_offset += 56;
                                chunk_index_offset -= 3;
                            }
                        } else if y_offset == 1 {
                            let y_to_shift = *cell_index + y_offset * 8;
                            // Check if it went out-of-bounds down.
                            if (y_to_shift >= 0 && y_to_shift < 64) && (Y_MASK | (Y_MASK << 8)) & (1<<y_to_shift) >= 1 {
                                // In same chunk.
                                cell_index_offset += y_offset * 8;
                            } else {
                                // Looped around.
                                cell_index_offset -= 56;
                                chunk_index_offset += 3;
                            }
                        }

                        cell_state += chunks[chunk_index_offset].is_alive(cell_index_offset).unwrap() as i8;

                        if cell_state & 0x7f > threshold {
                            continue 'cells_outer
                        }
                    }
                }
            }
            rules(&mut new_data, *cell_index, cell_state);
        }
        new_data
    }

    /// Contains methods, functions and macros to run Conway's Game of Life.
    pub mod game_of_life;
    /// Contains more methods to manipulate a Chunk and for debugging.
    #[cfg(feature = "chunk_utilities")]
    pub mod chunk_utilities;
    /// Contains methods to serialize and deserialize a Field struct and Chunk struct using Serde.
    #[cfg(feature = "serialization")]
    pub mod serialization;
    /// Contains methods for concurrency.
    #[cfg(all(feature = "concurrency", target_has_atomic = "ptr"))]
    pub mod concurrency;
}