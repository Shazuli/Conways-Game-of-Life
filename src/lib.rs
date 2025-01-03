macro_rules! set_bit {
    ($val:expr, $bit:expr) => {
        $val |= 1<<$bit
    };
}

/*macro_rules! clear_bit {
    ($val:expr, $bit:expr) => {
        $val &= !(1<<$bit)
    };
}*/

#[doc(hidden)]
pub use self::game_of_life::{Field, Chunk, ChunkCellData, game_of_life_core};

pub mod game_of_life
{
    // 8x8 bitboard.
    #[derive(Clone, Copy)]
    pub union ChunkCellData {
        pub long: u64,
        pub bytes: [u8; 8]
    }

    #[derive(PartialEq, Clone, Copy, Debug)]
    pub struct Chunk {
        pub x: i32,
        pub y: i32,
        pub data: ChunkCellData
    }

    #[derive(Clone, Debug)]
    pub struct Field {
        generation: u32,
        chunks: [Vec<Chunk>; 2]
    }

    /*
    <------+
           |
           |
          \/
    */


    //            $(long => $x:expr, $y:expr, $data:expr);*) => {}
    //           ($field:expr; $(long => $x:expr, $y:expr, $data:expr);* => {});

    /*#[macro_export]
    macro_rules! set_field_chunks {
        ($field:expr; pattern:pat if ) => {
            
        };

        ($field:expr; $($x:expr, $y:expr, long: $data:expr);+) => {

        };

        ($field:expr; $($x:expr, $y:expr, bytes: $data:expr);+) => {

        }
    }*/

    #[macro_export]
    macro_rules! set_field_chunks {
        ($field:expr;
            $($x:expr, $y:expr, long: $data:expr);+
        ) => {
            (
                $(
                    $field.add_chunk($x, $y, ChunkCellData { long: $data }).unwrap(),
                )+
            )
        };
        ($field:expr;
            $($x:expr, $y:expr, bytes: $data:expr);+
        ) => {
            (
                $(
                    $field.add_chunk($x, $y, ChunkCellData { bytes: $data }).unwrap(),
                )+
            )
        }
    }

    /*#[macro_export]
    macro_rules! set_field_chunk_bytes {
        ($field:expr;
            $x:expr, $y:expr, $($byte:expr),+ $(;)? ) => {
                println!();
                $(
                    println!();
                )+
        }
    }*/
    /*macro_rules! set_field_chunk_bytes {
        ($field:expr;
            $x:expr, $y:expr, $($byte:expr),+ $(,)?) => {
    
                $(
                    
                )+
        }
    }*/
    /*macro_rules! set_field_chunk_bytes {
        ($field:expr;
            $($x:expr, $y:expr, $($byte:expr);+ $(,)?)+ $(,)?) => {

                $(
                    
                )+
        }
    }*/

    const BIT_MASK_1_1: &'static ChunkCellData = &ChunkCellData { bytes: [// 3x3 bit-mask at [1,1]
        7,5,7,
        0,0,0,0,0
    ]};
    const CELL_RANGE_INNER: &'static [i8; 36] = &[
        9, 10,11,12,13,14,
        17,18,19,20,21,22,
        25,26,27,28,29,30,
        33,34,35,36,37,38,
        41,42,43,44,45,46,
        49,50,51,52,53,54
    ];
    const CELL_RANGE_OUTER: &'static [i8; 28] = &[
        0, 1, 2, 3, 4, 5, 6, 7,
        8,                   15,
        16,                  23,
        24,                  31,
        32,                  39,
        40,                  47,
        48,                  55,
        56,57,58,59,60,61,62,63
    ];

    const DUMMY_CHUNK: Chunk = Chunk {x: 0, y: 0, data: ChunkCellData {long: 0}};


    pub mod game_of_life_core {
        use core::fmt;
        use std::{cell, cmp::min, ops::{BitAnd, BitOr}};
        use itertools::Itertools;

        use crate::{Field, Chunk, ChunkCellData, game_of_life::{BIT_MASK_1_1, CELL_RANGE_INNER, CELL_RANGE_OUTER}};

        impl fmt::Debug for ChunkCellData {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                unsafe {
                    write!(f, "\n{:08b}\n{:08b}\n{:08b}\n{:08b}\n{:08b}\n{:08b}\n{:08b}\n{:08b}",
                        self.bytes[0],
                        self.bytes[1],
                        self.bytes[2],
                        self.bytes[3],
                        self.bytes[4],
                        self.bytes[5],
                        self.bytes[6],
                        self.bytes[7]
                    )
                }
            }
        }

        impl BitAnd for ChunkCellData {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self::Output {
                Self {
                    long: unsafe { self.long & rhs.long }
                }
            }
        }

        impl BitOr for ChunkCellData {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self::Output {
                Self {
                    long: unsafe { self.long | rhs.long }
                }
            }
        }

        impl PartialEq for ChunkCellData {
            fn eq(&self, other: &Self) -> bool {
                unsafe {
                    self.long == other.long
                }
            }
        }


        /// Macro to perform the rules for the classic game.
        /// $new_data for storing the result, $cell_index for which cell is to be changed and
        /// $cell_state containing if the cell is alive and how many living neighbours it has.
        /*macro_rules! perform_rules_classic {
            ($new_data:ident, $cell_index:ident, $cell_state:ident) => {
                /*
                1. Any live cell with fewer than two live neighbors dies, as if by underpopulation.
                2. Any live cell with two or three live neighbors lives on to the next generation.
                3. Any live cell with more than three live neighbors dies, as if by overpopulation.
                4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
                */
                if $cell_state.is_negative() {// Is negative = cell is alive
                    
                    // Mask with 0x7f to get the correct neighbour count.
                    if $cell_state & 0x7f == 2 || $cell_state & 0x7f == 3 {// Living cell with 2 or 3 living neighbours survives
                        unsafe {
                            set_bit!($new_data.long, $cell_index)
                        }
                    }
                } else if $cell_state == 3 {// Dead cell becomes alive if it has exactly 3 living neighbours
                    unsafe {
                        set_bit!($new_data.long, $cell_index)
                    }
                }
            };
        }*/


        fn calculate_rules_classic(new_data: &mut ChunkCellData, cell_index: &i8, cell_state: i8)// -> ChunkCellData
        {
            /*
            1. Any live cell with fewer than two live neighbors dies, as if by underpopulation.
            2. Any live cell with two or three live neighbors lives on to the next generation.
            3. Any live cell with more than three live neighbors dies, as if by overpopulation.
            4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
            */

            //let mut new_data: ChunkCellData = ChunkCellData {long: 0};

            if cell_state.is_negative() {// Is negative = cell is alive
                
                // Mask with 0x7f to get the correct neighbour count.
                if cell_state & 0x7f == 2 || cell_state & 0x7f == 3 {// Living cell with 2 or 3 living neighbours survives
                    unsafe {
                        set_bit!(new_data.long, cell_index)
                    }
                }
            } else if cell_state == 3 {// Dead cell becomes alive if it has exactly 3 living neighbours
                unsafe {
                    set_bit!(new_data.long, cell_index)
                }
            }

            //new_data
        }

        /// Calculate the inner cells area of a chunk and skip if exceeding the threshold.
        fn calc_chunk_inner(chunk: &Chunk, threshold: i8) -> ChunkCellData
        {
            let mut cell_state: i8;
            
            let mut new_data: ChunkCellData = ChunkCellData { long: 0 };// Assume every cell died

            'cells_inner: for cell_index in CELL_RANGE_INNER.iter() {

                // If cell is alive set sign bit.
                cell_state = if chunk.is_alive(*cell_index) { -0x80 } else { 0 };

                // Calculate 3x3 and stop if exceeding the threshold.
                let mut data_masked = unsafe { chunk.data.long >> (cell_index-9) & BIT_MASK_1_1.long };// Move relevent bits to align to LSB and mask using 3x3 at [1,1]
                
                while data_masked != 0 {
                    cell_state += (data_masked & 1) as i8;

                    if cell_state & 0x7f > threshold {
                        continue 'cells_inner
                    }

                    data_masked >>= 1;
            }

                //perform_rules_classic!(new_data, cell_index, cell_state)
                calculate_rules_classic(&mut new_data, cell_index, cell_state);
            }
            new_data
        }

        /// Calculate the outer cells area of chunks->4 and skip if exceeding the threshold.
        fn calc_chunk_outer(chunks: [&Chunk; 9], threshold: i8) -> ChunkCellData
        {
            let mut cell_state: i8;
            
            let mut new_data: ChunkCellData = ChunkCellData { long: 0 };// Assume every cell died

            let (mut chunk_index, mut cell_index_offset): (usize, i8);

            'cells_outer: for cell_index in CELL_RANGE_OUTER.iter() {

                cell_state = if chunks[4].is_alive(*cell_index) { -0x80 } else { 0 };

                /*
                2 1 0
                5 4 3
                8 7 6

                0, 1, 2, 3, 4, 5, 6, 7, 8

                0, 1, 2, 3, 4, 5, 6, 7,
                8,                   15,
                16,                  23,
                24,                  31,
                32,                  39,
                40,                  47,
                48,                  55,
                56,57,58,59,60,61,62,63

                */

                //println!("{cell_index}");

                // Calculate 3x3 and stop if exceeding the threshold.
                for x_offset in -1i8..=1 {
                    for y_offset in -1i8..=1 {

                        if x_offset | y_offset != 0 {

                            chunk_index = 4;// Start in the middle

                            cell_index_offset = *cell_index;

                            /*if !CELL_RANGE_INNER.contains(&(cell_index + x_offset)) {
                                if x_offset == -1 {

                                }
                            }*/

                            {
                                let cell_index_offset_x = cell_index + x_offset;
                                if cell_index_offset_x > 7 && cell_index_offset_x < 56 {
                                    
                                }
                                /*if (!CELL_RANGE_INNER.contains(&cell_index_offset_x) && cell_index_offset_x > 7 && cell_index_offset_x < 55) || cell_index_offset_x < 0 || cell_index_offset_x > 63 {
                                    // We went out of bounds, looped around.

                                    //chunk_index += x_offset as usize;
                                    if x_offset == -1 {
                                        cell_index_offset += 7;
                                        chunk_index -= 1;
                                    } else {
                                        cell_index_offset -= 7;
                                        chunk_index += 1;
                                    }
                                    //cell_index_offset += 7 * x_offset;

                                    
                                } else {
                                    cell_index_offset += x_offset;
                                }*/
                            }

                            /*{
                                let cell_index_offset_y = cell_index + y_offset * 8;
                                if !CELL_RANGE_INNER.contains(&cell_index_offset_y) || cell_index_offset_y < 0 || cell_index_offset_y > 63 {
                                    // We went out of bounds, looped around.

                                    //chunk_index += (y_offset * 3) as usize;
                                    //cell_index_offset += 63 * y_offset;
                                    if y_offset == -1 {
                                        cell_index_offset += 56;
                                        chunk_index -= 3;
                                    } else {
                                        cell_index_offset -= 56;
                                        chunk_index += 3;
                                    }
                                    
                                } else {
                                    cell_index_offset += y_offset;
                                }
                            }*/


                            // Check if out-of-bounds.
                            /*if !(*cell_index < 8) && !(*cell_index > 56) && CELL_RANGE_OUTER.contains(&(cell_index + x_offset)) {


                                if x_offset == -1 {
                                    // Is out of bounds to the left.
                                    chunk_index += 1;
                                    cell_index_offset += 7;
                                } else if x_offset == 1 {
                                    // Is out of bounds to the right.
                                    chunk_index -= 1;
                                    cell_index_offset -= 7;
                                }
                            } else {
                                cell_index_offset += x_offset;
                            }*/
                            /*if x_offset == -1 {
                                if !(*cell_index > 8) && !(*cell_index < 56) && CELL_RANGE_OUTER.contains(&(cell_index + x_offset)) {
                                    // Right.
                                    chunk_index += 1;
                                    cell_index_offset += 7;
                                }
                            } else if x_offset == 1 {
                                if CELL_RANGE_OUTER.contains(&(cell_index + x_offset)) {
                                    // Left.
                                    chunk_index -= 1;
                                    cell_index_offset -= 7;
                                }
                            }*/

                            //cell_offset = cell_index + x_offset + y_offset * 8;// TODO This is incorrect

                            // Offset the y-axis.
                            /*if cell_offset < 0 {
                                // Up.
                                chunk_index -= 3;
                                cell_offset = 56 + cell_index % 8;
                            } else if cell_offset > 63 {
                                // Down.
                                chunk_index += 3;
                                cell_offset = cell_index % 8;
                            }

                            // Offset the x-axis.
                            if cell_index % 8 + x_offset >= 8 {
                                // Left.
                                chunk_index += 1;
                                cell_offset += 7;
                            } else if cell_index % 8 + x_offset < 0 {
                                // Right.
                                chunk_index -= 1;
                                cell_offset -= 7;
                            }*/

                            /*if cell_index - (cell_index + x_offset) < 0 {
                                // Right.
                                chunk_index += 1;
                                cell_offset -= 7;
                                //cell_offset = cell_index + y_offset * 8 + cell_index % 8;
                            } else if cell_index - (cell_index + x_offset) > 0 {
                                // Left.
                                chunk_index -= 1;
                                cell_offset += 7;
                            }*/

                            //println!("[{x_offset}, {y_offset}] = {cell_offset}");
                            

                            cell_state += chunks[chunk_index].is_alive(cell_index_offset) as i8;

                            if cell_state & 0x7f > threshold {
                                continue 'cells_outer
                            }
                        }
                    }
                }
                //perform_rules_classic!(new_data, cell_index, cell_state)
                calculate_rules_classic(&mut new_data, cell_index, cell_state);
            }
            new_data
        }

        impl Chunk {

            /*pub fn get_x(&self) -> i32
            {
                self.x
            }

            pub fn get_y(&self) -> i32
            {
                self.y
            }*/

            pub fn is_alive(&self, index: i8) -> bool
            {
                if index < 0 || index > 63 {
                    panic!("Out of bounds (index={index})")
                }
                unsafe {
                    //self.data.bytes[(index / 8) as usize] & 1<<(index % 8) >= 1
                    self.data.long & 1<<index >= 1
                }
            }

            pub fn all_are_dead(&self) -> bool
            {
                unsafe { self.data.long == 0 }
            }

            pub fn set_all_dead(&mut self)
            {
                unsafe { self.data.long ^= self.data.long; }
            }

            pub fn mirror_x(&mut self)
            {
                for i in 0..4 {
                    let j = 7 - i;
                    unsafe {
                        if self.data.bytes[i] != self.data.bytes[j] {// Skip if they are the same
                            self.data.bytes[i] = self.data.bytes[i] ^ self.data.bytes[j];
                            self.data.bytes[j] = self.data.bytes[i] ^ self.data.bytes[j];
                            self.data.bytes[i] = self.data.bytes[i] ^ self.data.bytes[j];
                        }
                    }
                }
            }

            pub fn mirror_y(&mut self)
            {
                for i in 0..8 {
                    let mut byte = unsafe { self.data.bytes[i] };
                    if byte != 0 || byte != 0xff {// Skip if reversing does nothing for simple patterns
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

        impl Field {

            /// Create a new empty Field struct.
            pub fn new() -> Field
            {
                let chunks: Vec<Chunk> = Vec::new();
                //chunks.push_front(DUMMY_CHUNK.clone());

                Field {
                    generation: 0,
                    chunks: [chunks.clone(), chunks.to_owned()]
                }
            }

            pub fn add_chunk(&mut self, x: i32, y:i32, data: ChunkCellData) -> Result<Chunk, &str>
            {
                let new_chunk = Chunk {x, y, data};

                let current = self.get_mut_current();

                for c in current.iter() {
                    if c.x == x && c.y == y {
                        return Err("Already defined.");
                    }
                }

                current.push(new_chunk);
                Ok(new_chunk)
            }

            pub fn get_generation(&self) -> u32
            {
                self.generation
            }

            pub fn get_current(&self) -> &Vec<Chunk>
            {
                &self.chunks[(self.generation as u8 & 1) as usize]
            }

            fn get_mut_current(&mut self) -> &mut Vec<Chunk>
            {
                &mut self.chunks[(self.generation as u8 & 1) as usize]
            }

            fn get_mut_next(&mut self) -> &mut Vec<Chunk>
            {
                &mut self.chunks[(!(self.generation as u8) & 1) as usize]
            }

            pub fn is_alive(&self, x: i32, y: i32) -> bool
            {
                //let (chunk_x, chunk_y): (i32, i32) = (x / 8, y / 8);
                //let (local_x, local_y): (i8, i8);

                

                /*if x.is_positive() {
                    //local_x = (8 + x % 8) as i8;
                    chunk_x = x / 8;
                    local_x = (x % 8) as i8;
                } else {
                    chunk_x = x / 8 - 1;
                    local_x = (x % 8) as i8;
                    //local_x = (x % 8) as i8;
                }
                
                if y.is_positive() {
                    chunk_y = y / 8;
                    local_y = (y % 8) as i8;
                    //local_y = (8 + y % 8) as i8;
                } else {
                    chunk_y = y / 8 - 1;
                    local_y = (y % 8) as i8;
                    //local_y = (y % 8) as i8;
                }*/

                //let min = min(x, y).abs();
                //let (local_x, local_y) = ((x + min) % 8, (y + min) % 8);
                //let (local_x, local_y);

                /*if x == -1 {
                    local_x = 0;
                } else {
                    local_x = x.abs() % 8;
                }

                if y == -1 {
                    local_y = 0;
                } else {
                    local_y = y.abs() % 8;
                }*/

                /*let (local_x, local_y) = (
                    if x.is_negative() {(x.abs() -1) % 8} else {x.abs() % 8},
                    if y.is_negative() {(y.abs() -1) % 8} else {y.abs() % 8},
                );*/

                //let index = local_x + local_y * 8;

                /*let mut index = if local_x.is_positive() { local_x } else { 7 + local_x };

                if local_y.is_positive() {
                    index += local_y * 8;
                } else {
                    index += (7 + local_y) * 8;
                }

                let index = index;*/

                /*let mut index = (local_x + local_y * 8) as i8;

                if index.is_negative() {
                    index = index.abs()+1;
                }*/
                /*let mut index: i8;

                if local_x.is_negative() {
                    index = (8 + local_x) as i8;
                } else {
                    index = local_x as i8;
                }

                if local_y.is_negative() {
                    index += ((8 + local_y) * 8) as i8;
                } else {
                    index += (local_y * 8) as i8;
                }*/

                //println!("[{x}, {y}] -> [{local_x}, {local_y}] = {index}");

                
                //let (chunk_x, chunk_y): (i32, i32) = (x / 8, y / 8);
                let (chunk_x, chunk_y): (i32, i32) = (
                    if x.is_negative() {x / 8 - 1} else {x / 8},
                    if y.is_negative() {y / 8 - 1} else {y / 8});

                for c in self.get_current().iter() {
                    if c.x == chunk_x && c.y == chunk_y {
                        //let (x_offset, y_offset) = (x.abs(), y.abs());
                        let (x_offset, y_offset) = (
                            x.abs(),
                            y.abs());

                        let (x_norm, y_norm) = (x_offset, y_offset);

                        let (local_x, local_y): (i8, i8) = (
                            (x_norm % 8) as i8,
                            (y_norm % 8) as i8);

                        let index = local_x + (local_y * 8);

                        return c.is_alive(index);
                    }
                }
                false
            }

            /*pub fn count_neighbours(&self, x: i32, y: i32) -> i8
            {
                todo!()
            }*/

            /// Step the simulation once in a single thread and increment generation count.
            pub fn step_singlet(&mut self)
            {
                let current = self.get_mut_current();
                let mut next = Vec::new();


                let mut dummy_chunks_pos: Vec<(i32, i32)> = Vec::with_capacity(current.capacity() * 8);// Possibly as a HashMap?


                'chunks: for chunk in current.iter() {// Calculate a chunk

                    if chunk.all_are_dead() {// Skip if a chunk's cells are all dead
                        continue 'chunks
                    }

                    /*
                    2 1 0
                    5 4 3
                    8 7 6
                    */
                    // Cache surrounding chunks.
                    let mut neightbour_chunks_cache: [&Chunk; 9] = [&crate::game_of_life::DUMMY_CHUNK; 9];
                    {
                        let neighbour_chunk_xy: [(i32, i32); 9] = [
                            (chunk.x-1, chunk.y-1),(chunk.x, chunk.y-1),(chunk.x+1, chunk.y-1),
                            (chunk.x-1, chunk.y  ),         (0,0),      (chunk.x+1, chunk.y  ),
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
                            // If it doesn't exist keep the dummy chunk pos. for further calculations.
                            dummy_chunks_pos.push(neighbour_chunk_xy[i]);
                        }
                    }

                    // Calculate next generation for the chunk.
                    let new_data =
                        calc_chunk_inner(chunk, 3) |
                        calc_chunk_outer(neightbour_chunks_cache, 3);

                    unsafe {
                        if new_data.long != 0 {// Only push to the list if it has living cells
                            next.push(Chunk { x: chunk.x, y: chunk.y, data: new_data});
                        }
                    }
                }

                

                // Calculate the outer cell range of all the dummy chunks. Skip dublicate.
                /*for i_pos in dummy_chunks_pos.iter().unique() {

                    let chunk = Chunk { x: i_pos.0, y: i_pos.1, data: ChunkCellData { long: 0 }};

                    let mut neightbour_chunks_cache: [&Chunk; 9] = [&crate::game_of_life::DUMMY_CHUNK; 9];
                    {
                        let neighbour_chunk_xy: [(i32, i32); 9] = [
                            (chunk.x-1, chunk.y-1),(chunk.x, chunk.y-1),(chunk.x+1, chunk.y-1),
                            (chunk.x-1, chunk.y),         (0,0),        (chunk.x+1, chunk.y),
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
                        }
                    }
                    let new_data = calc_chunk_outer(neightbour_chunks_cache, 3);
                    unsafe {
                        if new_data.long != 0 {// Only push to the list if it has living cells
                            next.push(Chunk { x: chunk.x, y: chunk.y, data: new_data});
                        }
                    }
                }*/

                *self.get_mut_next() = next.to_owned();
                self.generation += 1;
            }
        }
    }
}


/*pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/