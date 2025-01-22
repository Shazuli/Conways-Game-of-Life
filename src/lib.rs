#![no_std]

#[doc(hidden)]
pub use self::game_of_life_core::{Field, Chunk, ChunkCellData, game_of_life, game_of_life::calculate_rules_classic};

extern crate alloc;

/// Main core module containing helper functions, structs and methods for running Conway's Game of Life.
pub mod game_of_life_core
{
    macro_rules! set_bit {
        ($val:expr, $bit:expr) => {
            $val |= 1<<$bit
        };
    }

    use alloc::vec::Vec;

    // 8x8 bitboard.
    #[derive(Clone, Copy)]
    pub union ChunkCellData {
        pub long: u64,
        pub bytes: [u8; 8]
    }

    // Chunk holding chunk data and coordinates.
    #[derive(Clone, Copy, Debug)]
    pub struct Chunk {
        x: i32,
        y: i32,
        data: ChunkCellData
    }

    // The field holding current generation and current- and next generation of chunks.
    #[derive(Clone, Debug)]
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


    impl Chunk {

        pub fn get_x(&self) -> i32
        {
            self.x
        }

        pub fn get_y(&self) -> i32
        {
            self.y
        }

        pub fn get_data_long(&self) -> u64
        {
            unsafe { self.data.long }
        }

        /// Get if cell at index is alive.
        pub fn is_alive(&self, index: i8) -> Result<bool, (&str, i8)>
        {
            if index < 0 || index > 63 {
                return Err(("Out of bounds", index));
            }
            unsafe { Ok(self.data.long & 1<<index >= 1) }
        }

        /// Gets if all cells in chunk are dead.
        pub fn all_are_dead(&self) -> bool
        {
            unsafe { self.data.long == 0 }
        }

        /// Sets all cells to dead for the chunk.
        pub fn set_all_dead(&mut self)
        {
            unsafe { self.data.long ^= self.data.long; }
        }

    }

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
    const DUMMY_CHUNK: Chunk = Chunk {x: 0, y: 0, data: ChunkCellData {long: 0}};
    // Bit-masks to check if index rotated around.
    const X_MASK: u64 = 0x7e7e7e7e7e7e7e7e;
    const Y_MASK: u64 = 0x00ffffffffffff00;
    // 3x3 bit-mask at [1,1] (111\n101\n111).
    const BIT_MASK_1_1: u64 = 0x70507;


    /// Calculate the inner cells area of a chunk and skip if exceeding the threshold.
    fn calc_chunk_inner(chunk: &Chunk, threshold: i8, rules: fn(&mut ChunkCellData, i8, i8)) -> ChunkCellData
    {
        let mut cell_state: i8;
        let mut new_data: ChunkCellData = ChunkCellData { long: 0 };// Assume every cell died

        'cells_inner: for cell_index in CELL_INDEX_INNER.iter() {

            // If cell is alive set sign bit.
            cell_state = if chunk.is_alive(*cell_index).unwrap() { -0x80 } else { 0 };
            {
                // Calculate 3x3 and stop if exceeding the threshold.
                let mut data_masked = unsafe { chunk.data.long >> (cell_index-9) & BIT_MASK_1_1 };// Move relevent bits to align to LSB and mask using 3x3 at [1,1]
                
                while data_masked != 0 {
                    cell_state += (data_masked & 1) as i8;

                    if cell_state & 0x7f > threshold {
                        continue 'cells_inner
                    }

                    data_masked >>= 1;
                }
            }
            rules(&mut new_data, *cell_index, cell_state);
        }
        new_data
    }

    /// Calculate the outer cells index of chunks->4 and skip if exceeding the threshold.
    /// Remaining chunks in array is neighbouring chunks in order: top right to bottom left.
    fn calc_chunk_outer(chunks: [&Chunk; 9], threshold: i8, rules: fn(&mut ChunkCellData, i8, i8)) -> ChunkCellData
    {
        let mut cell_state: i8;
        
        let mut new_data = ChunkCellData { long: 0 };// Assume every cell died

        let (mut chunk_index_offset, mut cell_index_offset): (usize, i8);

        'cells_outer: for cell_index in CELL_INDEX_OUTER.iter() {

            // If cell is alive set sign bit.
            cell_state = if chunks[4].is_alive(*cell_index).unwrap() { -0x80 } else { 0 };

            /*
            2 1 0
            5 4 3
            8 7 6
            */

            // Calculate 3x3 and stop if exceeding the threshold.
            for x_offset in -1i8..=1 {
                for y_offset in -1i8..=1 {

                    if x_offset | y_offset != 0 {// Skip if offset == [0,0]

                        chunk_index_offset = 4;// Start in the middle
                        cell_index_offset = *cell_index;// Start in middle of neighbours


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

    /// Contains more methods to manipulate a chunk.
    #[cfg(feature = "chunk_utilities")]
    pub mod chunk_utilities;
    /// Contains methods to serialize and deserialize Field struct.
    #[cfg(feature = "serialization")]
    pub mod serialization;
    /// Contains methods for concurrency.
    #[cfg(all(feature = "concurrency", target_has_atomic = "ptr"))]
    pub mod concurrency;

    /// Main methods and functions to run Conway's Game of Life.
    pub mod game_of_life {
        use alloc::{borrow::ToOwned, vec::Vec};
        use core::fmt;
        use super::{Field, Chunk, ChunkCellData, calc_chunk_inner, calc_chunk_outer};


        /// Adds chunks with data to a existing Field struct or returns a new one.
        /// # Example
        /// ```
        /// let mut f = set_field_chunks!(
        ///     0, 0, 0x183008000000;
        ///     0,-1, 0x183008000000;
        /// );
        /// ```
        /// ```
        /// let mut f = Field::new();
        /// set_field_chunks!(f;
        ///     0, 0, 0x183008000000;
        ///     0,-1, 0x183008000000;
        /// );
        /// ```
        #[macro_export]
        macro_rules! set_field_chunks {
            ($f:ident; $($x:expr, $y:expr, $data:expr);+ $(;)?) => {
                $(
                    $f.add_chunk($x,$y,conways_game_of_life_dyn_lib::game_of_life_core::ChunkCellData{long:$data}).unwrap();
                )+
            };
            ($($x:expr, $y:expr, $data:expr);+ $(;)?) => {{
                let mut f = Field::new();
                $(
                    f.add_chunk($x,$y,conways_game_of_life_dyn_lib::game_of_life_core::ChunkCellData{long:$data}).unwrap();
                )+
                f
            }};
        }

        impl fmt::Debug for ChunkCellData {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                unsafe {
                    write!(f,"{:08b}\n{:08b}\n{:08b}\n{:08b}\n{:08b}\n{:08b}\n{:08b}\n{:08b}",
                        self.bytes[0],self.bytes[1],self.bytes[2],self.bytes[3],self.bytes[4],self.bytes[5],self.bytes[6],self.bytes[7]
                    )
                }
            }
        }

        /// Calculate cell state at cell_index from result of cell_state.
        /// If sign bit is set for cell_state then cell at index is alive, remaining bits are living neighbours.
        pub fn calculate_rules_classic(new_data: &mut ChunkCellData, cell_index: i8, cell_state: i8)
        {
            /*
            1. Any live cell with fewer than two live neighbors dies, as if by underpopulation.
            2. Any live cell with two or three live neighbors lives on to the next generation.
            3. Any live cell with more than three live neighbors dies, as if by overpopulation.
            4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
            */

            if cell_state.is_negative() {// Is negative == cell is alive
                
                let cell_state = cell_state & 0x7f;// Mask with 0x7f to get the correct neighbour count

                if cell_state == 2 || cell_state == 3 {// Living cell with 2 or 3 living neighbours survives
                    unsafe {
                        set_bit!(new_data.long, cell_index)
                    }
                }
            } else if cell_state == 3 {// Dead cell becomes alive if it has exactly 3 living neighbours
                unsafe {
                    set_bit!(new_data.long, cell_index)
                }
            }
        }

        impl Field {

            /// Create a new empty Field struct.
            pub fn new() -> Self
            {
                let chunks: Vec<Chunk> = Vec::new();
                Field {
                    generation: 0,
                    chunks: [chunks.clone(), chunks.to_owned()]
                }
            }

            /// Adds a new chunk in the Field at position. Will fail if already defined.
            /// 
            /// # Example
            /// ```
            /// let mut f = Field::new();
            /// 
            /// f.add_chunk(-1, 0, ChunkCellData { long: 0x18_30_08_00_00_00}).unwrap();// Glider
            /// ```
            pub fn add_chunk(&mut self, x: i32, y: i32, data: ChunkCellData) -> Result<Chunk, &str>
            {
                let current = self.get_mut_current();

                for c in current.iter() {
                    if c.x == x && c.y == y {
                        return Err("Already defined");
                    }
                }
                let new_chunk = Chunk {x, y, data};
                current.push(new_chunk);
                Ok(new_chunk)
            }

            /// Searches for chunk at position and empty chunk if not found.
            pub fn find_chunk(&self, x: i32, y: i32) -> Chunk
            {
                for c in self.get_current().iter() {
                    if c.x == x && c.y == y {
                        return *c;
                    }
                }
                Chunk{x,y,data:ChunkCellData{long:0}}
            }

            /// Searches for chunk at position and returns a Option with mutable chunk or None.
            pub fn find_mut_chunk(&mut self, x: i32, y: i32) -> Option<&mut Chunk>
            {
                for c in self.get_mut_current().iter_mut() {
                    if c.x == x && c.y == y {
                        return Some(c);
                    }
                }
                None
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

            /// Get if cell at global position is alive and always false if not found.
            pub fn is_alive(&self, x: i32, y: i32) -> bool
            {
                // Get the chunk coordinates.
                let (chunk_x, chunk_y) = (x / 8, y / 8);

                for c in self.get_current().iter() {
                    if c.x == chunk_x && c.y == chunk_y {
                        if c.all_are_dead() {
                            break;
                        }
                        // Normalise the coordinates to the chunk coordinates.
                        let x_norm_local: i8 = if x.is_negative() { (7 - ((x.abs() - 1) % 8)) as i8 } else { (x % 8) as i8 };
                        let y_norm_local: i8 = if y.is_negative() { (7 - ((y.abs() - 1) % 8)) as i8 } else { (y % 8) as i8 };

                        return c.is_alive(x_norm_local + y_norm_local * 8).unwrap();
                    }
                }
                false
            }

            /// Step the simulation once in a single thread and increment generation count.
            /// Requires a "rules" fn to apply when calculating the next generation.
            /// Threshold number of living neighbours when to abort and assume the cell will die.
            pub fn step_singlet(&mut self, rules: fn(&mut ChunkCellData, i8, i8), threshold: i8)
            {
                let current: &mut Vec<Chunk> = self.get_mut_current();
                let mut next: Vec<Chunk> = Vec::new();

                // List of dummy chunk coordinates that may become real, pre-allocate at least current.capacity * 8 for it having 8 total neighbours.
                let mut dummy_chunks_pos: Vec<(i32, i32)> = Vec::with_capacity(current.capacity() * 8);


                // Calculate a chunk's cell states.
                'chunks: for chunk in current.iter() {

                    if chunk.all_are_dead() {
                        continue 'chunks
                    }

                    /*
                    2 1 0
                    5 4 3
                    8 7 6
                    */
                    // Cache surrounding chunks.
                    let mut neightbour_chunks_cache: [&Chunk; 9] = [&crate::game_of_life_core::DUMMY_CHUNK; 9];
                    {
                        let neighbour_chunk_xy: [(i32, i32); 9] = [
                            (chunk.x-1, chunk.y-1),(chunk.x, chunk.y-1),(chunk.x+1, chunk.y-1),
                            (chunk.x-1, chunk.y  ),      (0,0),         (chunk.x+1, chunk.y  ),
                            (chunk.x-1, chunk.y+1),(chunk.x, chunk.y+1),(chunk.x+1, chunk.y+1)
                        ];

                        neightbour_chunks_cache[4] = &chunk;
                        'neighbour_chunk_caching: for i in [0,1,2,3,5,6,7,8] {
                            for c in current.iter() {

                                if c.x == neighbour_chunk_xy[i].0 && c.y == neighbour_chunk_xy[i].1 {
                                    neightbour_chunks_cache[i] = &c;
                                    continue 'neighbour_chunk_caching
                                }
                            }
                            // If it doesn't exist keep the dummy chunk pos. for further calculations, also prevent duplicates.
                            if !dummy_chunks_pos.contains(&neighbour_chunk_xy[i]) {
                                dummy_chunks_pos.push(neighbour_chunk_xy[i]);
                            }
                            
                        }
                    }

                    // Calculate next generation for the chunk.
                    let new_chunk_data = unsafe {
                        calc_chunk_inner(chunk, threshold, rules).long |
                        calc_chunk_outer(neightbour_chunks_cache, threshold, rules).long
                    };

                    if new_chunk_data != 0 {// Only push to the list if it has living cells
                        next.push(Chunk { x: chunk.x, y: chunk.y, data: ChunkCellData { long: new_chunk_data }});
                    }
                }

                
                // Calculate the outer cell range of all the dummy chunks.
                for chunk_pos in dummy_chunks_pos.iter() {
                    // Cache surrounding chunks.
                    let mut neightbour_chunks_cache: [&Chunk; 9] = [&crate::game_of_life_core::DUMMY_CHUNK; 9];
                    {
                        let neighbour_chunk_xy: [(i32, i32); 9] = [
                            (chunk_pos.0-1, chunk_pos.1-1),(chunk_pos.0, chunk_pos.1-1),(chunk_pos.0+1, chunk_pos.1-1),
                            (chunk_pos.0-1, chunk_pos.1  ),               (0,0),        (chunk_pos.0+1, chunk_pos.1  ),
                            (chunk_pos.0-1, chunk_pos.1+1),(chunk_pos.0, chunk_pos.1+1),(chunk_pos.0+1, chunk_pos.1+1)
                        ];

                        'neighbour_chunk_caching: for i in [0,1,2,3,5,6,7,8] {
                            for c in current.iter() {
                                if c.x == neighbour_chunk_xy[i].0 && c.y == neighbour_chunk_xy[i].1 {
                                    neightbour_chunks_cache[i] = &c;
                                    continue 'neighbour_chunk_caching;
                                }
                            }
                        }
                    }

                    // Only need to calculate the outer cells.
                    let new_chunk_data = unsafe { calc_chunk_outer(neightbour_chunks_cache, threshold, rules).long };

                    if new_chunk_data != 0 {// Only push to the list if it has living cells
                        next.push(Chunk { x: chunk_pos.0, y: chunk_pos.1, data: ChunkCellData { long: new_chunk_data }});
                    }
                }

                *self.get_mut_next() = next.to_owned();
                self.generation += 1;
            }
        }
    }
}