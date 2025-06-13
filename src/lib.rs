#![cfg_attr(not(feature = "use_std"), no_std)]

//! Library containing structs, methods, functions and macros to run Game of Life on a large play field.
//! It's divided into multiple modules that can be enabled or disabled, including running it with `no_std`.

pub use self::game_of_life_core::{Chunk, ChunkCellData8x8, Field, CellContext, CellStateContainer};
//pub use self::game_of_life_core::CalcNewChunkState;

pub use self::game_of_life_core::{IS_ALIVE_BIT, IS_ALIVE_BIT_MASK};

#[cfg(feature = "rule_fns")]
pub use self::game_of_life_core::rule_fns;

#[cfg(feature = "step_single_thread")]
pub use self::game_of_life_core::step_single_thread;

#[cfg(feature = "chunk_utilities")]
pub use self::game_of_life_core::chunk_utilities;

#[cfg(feature = "field_utilities")]
pub use self::game_of_life_core::field_utilities;

#[cfg(feature = "debug")]
pub use self::game_of_life_core::debug;

#[cfg(feature = "serde")]
pub use self::game_of_life_core::serde;

//#[cfg(all(feature = "concurrency", target_has_atomic = "ptr"))]
#[cfg(feature = "concurrency")]
pub use self::game_of_life_core::concurrency;

extern crate alloc;


/// Core module containing helper functions, structs, trait impls and methods for Game of Life modules.
mod game_of_life_core
{
    use core::{
        cmp::Ordering::{self, Equal, Greater, Less},
        ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign}
    };
    use alloc::vec::Vec;

    /// Set cell at index to alive or to state.
    /// Doesn't check for overflow.
    #[cfg_attr(any(feature = "rule_fns", feature = "chunk_utilities"), macro_export)]
    macro_rules! set_state {
        ($val:ident, $index:expr) => {
            unsafe { $val.u64 |= 1<<$index }
        };
        ($val:ident, $index:expr, $state:expr) => {
            unsafe { $val.u64 |= ($state as u64)<<$index }
        }
    }

    /// Set cell at index to dead.
    /// Doesn't check for overflow.
    #[cfg_attr(any(feature = "rule_fns", feature = "chunk_utilities"), macro_export)]
    macro_rules! set_dead {
        ($val:ident, $index:expr) => {
            unsafe { $val.u64 &= !(1<<$index) }
        }
    }

    #[cfg(feature = "rule_fns")]
    pub mod rule_fns;
    #[cfg(feature = "step_single_thread")]
    pub mod step_single_thread;
    #[cfg(feature = "debug")]
    pub mod debug;
    #[cfg(feature = "chunk_utilities")]
    pub mod chunk_utilities;
    #[cfg(feature = "field_utilities")]
    pub mod field_utilities;
    #[cfg(feature = "serde")]
    pub mod serde;
    //#[cfg(all(feature = "concurrency", target_has_atomic = "ptr"))]
    #[cfg(feature = "concurrency")]
    pub mod concurrency;


    /// Container for holding cell states.
    pub trait CellStateContainer: Sized + Eq + PartialEq + Ord + PartialOrd + Clone + Copy + Default {

        /// Holds information for a cell's current state and how many living neighbours it has/what index it has.
        type CellContext;

        /// Length of one side.
        const SIDE_LENGTH: u16;
        /// Index range.
        const INDEX_MAX_RANGE: Self::CellContext;

        /// Sign bit to mark variable that cell is alive/will be set to alive.
        const IS_ALIVE_BIT: Self::CellContext;
        /// Bit-mask to get the number of living neighbours/index from variable.
        const IS_ALIVE_BIT_MASK: Self::CellContext;

        /// Get data as byte slice.
        fn as_byte_slice(&self) -> &[u8];

        fn all_are_dead(&self) -> bool;

        unsafe fn unchecked_get_cell_state(&self, index: Self::CellContext) -> bool;

        fn get_cell_state(&self, index: Self::CellContext) -> Option<bool>;

        fn set_cell_state(&mut self, state: Self::CellContext) -> Option<bool>;
    }

    /// Chunk data for cell states in a 8x8 bitboard (LSB = top-right, MSB = bottom-left).
    #[derive(Clone, Copy, Eq)]
    pub union ChunkCellData8x8 {
        pub u64: u64,
        pub u32x2: [u32; 2],
        pub u8x8: [u8; 8]
    }

    /// Holds information for a cell's current state and how many living neighbours it has/what index it has.
    pub type CellContext = i8;

    /// Chunk holding chunk data and chunk coordinates.
    #[derive(Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chunk {
        x: i32,
        y: i32,
        pub data: ChunkCellData8x8
    }

    /// The field holding current generation number and current- and next generation of chunks.
    #[must_use]
    #[derive(Clone)]
    pub struct Field {
        generation: u64,
        chunks: [Vec<Chunk>; 2]
    }

    /*     -
    + <----0 -
           |
           |
          \/
          +
    */


    impl CellStateContainer for ChunkCellData8x8 {

        type CellContext = i8;

        const SIDE_LENGTH: u16 = 8;
        const INDEX_MAX_RANGE: Self::CellContext = 64;

        const IS_ALIVE_BIT: Self::CellContext = -0x80;
        const IS_ALIVE_BIT_MASK: Self::CellContext = !Self::IS_ALIVE_BIT;

        fn as_byte_slice(&self) -> &[u8]
        {
            unsafe { self.u8x8.as_slice() }
        }

        fn all_are_dead(&self) -> bool
        {
            unsafe { self.u64 == 0 }
        }

        unsafe fn unchecked_get_cell_state(&self, index: Self::CellContext) -> bool
        {
            (unsafe { self.u64 } & 1<<index != 0)
        }

        fn get_cell_state(&self, index: Self::CellContext) -> Option<bool>
        {
            if let 0..Self::INDEX_MAX_RANGE = index {
                Some(unsafe { self.unchecked_get_cell_state(index) })
            } else {
                None
            }
        }

        fn set_cell_state(&mut self, state: Self::CellContext) -> Option<bool>
        {
            let index = state & Self::IS_ALIVE_BIT_MASK;

            if let 0..Self::INDEX_MAX_RANGE = index {

                let old = unsafe { self.u64 };

                if state.is_negative() {
                    set_state!(self, index);
                } else {
                    set_dead!(self, index);
                }

                Some(unsafe { self.u64 != old })
            } else {
                None
            }
        }

    }

    impl From<u64> for ChunkCellData8x8 {
        fn from(value: u64) -> Self
        {
            ChunkCellData8x8 { u64: value }
        }
    }

    impl From<[u8; 8]> for ChunkCellData8x8 {
        fn from(value: [u8; 8]) -> Self
        {
            ChunkCellData8x8 { u8x8: value }
        }
    }

    impl Into<u64> for ChunkCellData8x8 {
        fn into(self) -> u64
        {
            unsafe { self.u64 }
        }
    }

    impl Into<[u8; 8]> for ChunkCellData8x8 {
        fn into(self) -> [u8; 8]
        {
            unsafe { self.u8x8 }
        }
    }

    impl PartialEq for ChunkCellData8x8 {
        fn eq(&self, other: &Self) -> bool
        {
            unsafe { self.u64 == other.u64 }
        }
    }

    impl Ord for ChunkCellData8x8 {
        fn cmp(&self, other: &Self) -> core::cmp::Ordering
        {
            unsafe { self.u64.cmp(&other.u64) }
        }
    }

    impl PartialOrd for ChunkCellData8x8 {
        fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>
        {
            Some(self.cmp(other))
        }
    }

    impl BitAnd for ChunkCellData8x8 {
        type Output = Self;

        fn bitand(self, rhs: Self) -> Self::Output
        {
            Self { u64: unsafe { self.u64 & rhs.u64 } }
        }
    }

    impl BitAndAssign for ChunkCellData8x8 {
        fn bitand_assign(&mut self, rhs: Self)
        {
            unsafe { self.u64 &= rhs.u64; }
        }
    }

    impl BitOr for ChunkCellData8x8 {
        type Output = Self;

        fn bitor(self, rhs: Self) -> Self::Output
        {
            Self { u64: unsafe { self.u64 | rhs.u64 } }
        }
    }

    impl BitOrAssign for ChunkCellData8x8 {
        fn bitor_assign(&mut self, rhs: Self)
        {
            unsafe { self.u64 |= rhs.u64; }
        }
    }

    impl BitXor for ChunkCellData8x8 {
        type Output = Self;

        fn bitxor(self, rhs: Self) -> Self::Output
        {
            Self { u64: unsafe { self.u64 ^ rhs.u64 } }
        }
    }

    impl BitXorAssign for ChunkCellData8x8 {
        fn bitxor_assign(&mut self, rhs: Self)
        {
            unsafe { self.u64 ^= rhs.u64; }
        }
    }

    impl Default for ChunkCellData8x8 {
        fn default() -> Self
        {
            ChunkCellData8x8 { u64: 0 }
        }
    }


    //impl<T: CellStateContainer> Chunk<T> {
    impl Chunk {

        /// Constructs a new [`Chunk`].
        /// 
        /// Top right is [0,0], bottom left is [7,7].
        /// ```no_run
        /// use conways_game_of_life_dyn_lib::{ChunkCellData8x8, Chunk};
        /// 
        /// let c = Chunk::new(1, -2, ChunkCellData8x8::from(0x70402));
        /// ```
        #[inline]
        #[must_use]
        pub const fn new(x: i32, y: i32, data: ChunkCellData8x8) -> Self
        {
            Chunk { x, y, data }
        }

        /// Get x coordinate for chunk.
        #[inline]
        pub const fn get_x(&self) -> i32
        {
            self.x
        }

        /// Get y coordinate for chunk.
        #[inline]
        pub const fn get_y(&self) -> i32
        {
            self.y
        }

        /// Get coordinates as a tuple.
        #[inline]
        pub const fn get_coordinates(&self) -> (i32, i32)
        {
            (self.x, self.y)
        }

        /// Get if all cells in chunk are dead.
        #[inline]
        pub fn all_are_dead(&self) -> bool
        {
            //unsafe { self.data.u64 == 0 }
            self.data.all_are_dead()
        }

        #[inline(always)]
        /// Get cell state at index. Doesn't check if out of bounds.
        pub unsafe fn unchecked_get_cell_state(&self, index: i8) -> bool
        {
            //(unsafe { self.data.u64 } & 1<<index != 0)
            unsafe { self.data.unchecked_get_cell_state(index) }
        }

        /// Get cell state at index. Returns `None` if index is out of bounds and `bool` otherwise.
        /// ```no_run
        /// use conways_game_of_life_dyn_lib::{ChunkCellData8x8, Chunk};
        /// 
        /// let c = Chunk::new(1, -2, ChunkCellData8x8::from(0x4448507048444478));
        /// 
        /// let (x, y): (i8, i8) = (2, 1);
        /// assert_eq!(c.get_cell_state(x + y * 8), Some(true));
        /// 
        /// let (x, y): (i8, i8) = (5, 1);
        /// assert_eq!(c.get_cell_state(x + y * 8), Some(false));
        /// 
        /// let (x, y): (i8, i8) = (5, 9);
        /// assert_eq!(c.get_cell_state(x + y * 8), None);
        /// ```
        #[inline(always)]
        pub fn get_cell_state(&self, index: i8) -> Option<bool>
        {
            self.data.get_cell_state(index)
            /*if let 0..64 = index {
                Some(unsafe { self.data.unchecked_get_cell_state(index) })
            } else {
                None
            }*/
        }

        /// Set cell state at index. Returns None if out of bounds and `bool` if cell state changed.
        /// Sign bit determines the cell's new state.
        /// ```no_run
        /// use conways_game_of_life_dyn_lib::{ChunkCellData8x8, Chunk, IS_ALIVE_BIT};
        /// 
        /// let mut c = Chunk::new(1, -2, ChunkCellData8x8::from(0));
        /// assert!(c.all_are_dead());
        /// 
        /// let (x, y): (i8, i8) = (4, 3);
        /// assert_eq!(c.set_cell_state((x + y * 8) | IS_ALIVE_BIT), Some(true));
        /// assert!(!c.all_are_dead());
        /// assert_eq!(c.set_cell_state((x + y * 8) | IS_ALIVE_BIT), Some(false));
        /// assert!(!c.all_are_dead());
        /// 
        /// assert_eq!(unsafe { c.data.u64 }, 0x10000000);
        /// 
        /// assert_eq!(c.set_cell_state(x + y * 8), Some(true));
        /// assert!(c.all_are_dead());
        /// assert_eq!(c.set_cell_state(x + y * 8), Some(false));
        /// ```
        #[inline(always)]
        pub fn set_cell_state(&mut self, state: CellContext) -> Option<bool>
        {
            self.data.set_cell_state(state)
            /*let index: i8 = state & IS_ALIVE_BIT_MASK;

            if let 0..64 = index {

                let mut data = self.data;
                let old_data = data;

                if state.is_negative() {
                    set_state!(data, index);
                } else {
                    set_dead!(data, index);
                }

                let changed = unsafe { data.u64 != old_data.u64 };
                if changed { self.data = data; }

                Some(changed)
            } else {
                None
            }*/
        }
    }


    impl Field {
        /// Constructs a new, empty [`Field`].
        #[inline]
        #[must_use]
        pub const fn new() -> Self
        {
            Field {
                generation: 0,
                chunks: [Vec::new(), Vec::new()]
            }
        }

        /// Constructs a new, empty [`Field`] with at least the specified capacity for chunks * 2.
        #[inline]
        #[must_use]
        pub fn with_capacity(capacity: usize) -> Self
        {
            Field {
                generation: 0,
                chunks: [Vec::with_capacity(capacity), Vec::with_capacity(capacity)]
            }
        }

        /// Get current generation count.
        #[inline]
        pub const fn get_generation(&self) -> u64
        {
            self.generation
        }

        /// Get current generation as a slice.
        #[inline]
        pub fn get_current_slice(&self) -> &[Chunk]
        {
            &self.chunks[get_current_gen_index(self.generation)].as_slice()
        }

        /// Get current generation vector of chunks.
        #[inline]
        pub const fn get_current(&self) -> &Vec<Chunk>
        {
            &self.chunks[get_current_gen_index(self.generation)]
        }

        /// Get current generation of mut vector of chunks.
        #[inline]
        const fn get_mut_current(&mut self) -> &mut Vec<Chunk>
        {
            &mut self.chunks[get_current_gen_index(self.generation)]
        }

        /// Get next generation as a slice.
        #[inline]
        pub fn get_next_slice(&self) -> &[Chunk]
        {
            &self.chunks[get_next_gen_index(self.generation)].as_slice()
        }

        /// Get next generation vector of chunks.
        #[inline]
        pub const fn get_next(&self) -> &Vec<Chunk>
        {
            &self.chunks[get_next_gen_index(self.generation)]
        }

        /// Get next generation of mut vector of chunks.
        #[inline]
        const fn get_mut_next(&mut self) -> &mut Vec<Chunk>
        {
            &mut self.chunks[get_next_gen_index(self.generation)]
        }

        /// Adds a new Chunk to the Field at position and returns a copy of the chunk without checking if already defined.
        pub unsafe fn unchecked_add_new_chunk(&mut self, x: i32, y: i32, data: ChunkCellData8x8) -> Chunk
        {
            let new_chunk = Chunk { x, y, data };
            self.get_mut_current().push(new_chunk);
            new_chunk
        }

        /// Adds a Chunk to the Field and returns a copy of the chunk without checking if already defined.
        pub unsafe fn unchecked_add_chunk(&mut self, chunk: Chunk) -> Chunk
        {
            self.get_mut_current().push(chunk);
            chunk
        }

        /// Adds a new chunk to the Field at position and returns a Option with a copy of the chunk and None if already defined.
        pub fn add_new_chunk(&mut self, x: i32, y: i32, data: ChunkCellData8x8) -> Option<Chunk>
        {
            let current = self.get_mut_current();

            if let None = find_chunk_linear_search(&current, (x, y)) {
                let new_chunk = Chunk { x, y, data };
                current.push(new_chunk);
                Some(new_chunk)
            } else {
                None
            }
        }

        /// Adds a chunk to the Field and returns a Option with a copy of the chunk and None if already defined.
        pub fn add_chunk(&mut self, chunk: Chunk) -> Option<Chunk>
        {
            let current = self.get_mut_current();

            if let None = find_chunk_linear_search(&current, (chunk.x, chunk.y)) {
                current.push(chunk);
                Some(chunk)
            } else {
                None
            }
        }

        /// Deletes Chunk at position and returns a clone of it and None if not found.
        pub fn delete_chunk(&mut self, x: i32, y: i32) -> Option<Chunk>
        {
            let current = self.get_mut_current();

            find_chunk_index_linear_search(&current, &(x, y))
                .and_then(|i| Some(current.swap_remove(i)))
        }

        /// Searches for Chunk at position and returns a clone of it or if not found returns a Chunk with all cell states dead.
        pub fn find_chunk(&self, x: i32, y: i32) -> Chunk
        {
            find_chunk_linear_search(self.get_current(), (x, y))
                .unwrap_or(&Chunk { x, y, data: ChunkCellData8x8::from(0) }).clone()
        }

        /// Searches for chunk at position and returns a Option with a mutable Chunk or None.
        pub fn find_mut_chunk(&mut self, x: i32, y: i32) -> Option<&mut Chunk>
        {
            find_mut_chunk_linear_search(self.get_mut_current(), (x, y))
        }
    }


    // Dummy chunk for reference.
    const DUMMY_CHUNK: Chunk = Chunk { x: 0, y: 0, data: ChunkCellData8x8 { u64: 0 } };
    // Cell index for cells with all its neighbours within the same chunk and cell index for neighbours within- and outside its chunk.
    const CELL_INDEX_INNER: [i8; 36] = [
        09,10,11,12,13,14,
        17,18,19,20,21,22,
        25,26,27,28,29,30,
        33,34,35,36,37,38,
        41,42,43,44,45,46,
        49,50,51,52,53,54
    ];
    const CELL_INDEX_OUTER: [i8; 28] = [
        00,01,02,03,04,05,06,07,
        08,                  15,
        16,                  23,
        24,                  31,
        32,                  39,
        40,                  47,
        48,                  55,
        56,57,58,59,60,61,62,63
    ];
    // The threshold for how many chunks can be held in the stack before using the heap.
    const CHUNKS_IN_STACK: usize = 64;
    // Array of neighbour chunk index, except for the middle.
    const NEIGHBOUR_CHUNK_INDEX: [usize; 8] = [0,1,2,3,5,6,7,8];
    /*const SHIFT_MASKS: (u64, u64) = (
        0x7e7e7e7e7e7e7e7e,
        0x00ffffffffffff00
    );*/
    // Bit-masks helper to check if index rotated around in x- or y direction.
    const X_MASK: u64 = 0x7e7e7e7e7e7e7e7e;
    const Y_MASK: u64 = 0x00ffffffffffff00;
    // Bit-mask to only get neighbour cell states at index 9 (coordinate [1,1]).
    const BIT_MASK_1_1: u32 = 0x70507;
    /// Sign bit to mark variable that cell is alive/will be set to alive.
    pub const IS_ALIVE_BIT: CellContext = -0x80;
    /// Bit-mask to get the number of living neighbours/index from variable.
    pub const IS_ALIVE_BIT_MASK: CellContext = !IS_ALIVE_BIT;


    /// Get index for current generation, can only be `0` or `1`.
    #[inline(always)]
    const fn get_current_gen_index(generation: u64) -> usize
    {
        (generation as u8 & 1) as usize
    }

    /// Get index for next generation, can only be `0` or `1`.
    #[inline(always)]
    const fn get_next_gen_index(generation: u64) -> usize
    {
        (!(generation as u8) & 1) as usize
    }

    /// Get neighbour chunks coordinates relative to origin in correct order for an array:
    /// 2 1 0
    /// 5 4 3
    /// 8 7 6
    #[inline]
    const fn get_neighbour_chunk_xy(ori: &(i32, i32)) -> [(i32, i32); 9]
    {
        [
            (ori.0-1, ori.1-1),(ori.0, ori.1-1),(ori.0+1, ori.1-1),
            (ori.0-1, ori.1  ),(ori.0, ori.1  ),(ori.0+1, ori.1  ),
            (ori.0-1, ori.1+1),(ori.0, ori.1+1),(ori.0+1, ori.1+1)
        ]
    }

    /// Compares two coordinates lexicographically.
    #[inline]
    const fn cmp_coords(a: &(i32, i32), b: &(i32, i32)) -> Ordering
    {
        // Get the difference, upgrade to i64 to avoid over/underflow.
        let (diff0, diff1) = (
            a.0 as i64 - b.0 as i64,
            a.1 as i64 - b.1 as i64
        );

        if diff0 | diff1 == 0 {
            Equal
        } else if diff0 > 0 || (diff0 == 0 && diff1 > 0) {
            Greater
        } else {
            Less
        }
    }

    /// Find chunk index using linear search.
    /// `None` if not found.
    #[inline]
    const fn find_chunk_index_linear_search(slice: &[Chunk], coords: &(i32, i32)) -> Option<usize>
    {
        //slice.iter().position(|c| cmp_coords(&c.get_coordinates(), &coords).is_eq())
        let (mut i, len) = (0, slice.len());

        while i<len {
            if cmp_coords(&slice[i].get_coordinates(), coords).is_eq() {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    /// Find chunk index using binary search, slice must be sorted.
    /// `None` if not found.
    #[inline]
    const fn find_chunk_index_binary_search(slice: &[Chunk], coords: &(i32, i32)) -> Option<usize>
    {
        // Algorithm taken from @Computerphile's videos on YouTube.

        let (mut r, mut l) = (0, (slice.len() - 1) as i64);

        while r <= l {

            let m = r + (l - r) / 2;
            let mi = m as usize;

            match cmp_coords(coords, &slice[mi].get_coordinates()) {
                Less    => l = m - 1,
                Greater => r = m + 1,
                Equal   => return Some(mi)// Found it.
            }
        }
        // Didn't find it.
        None
    }

    /// Find mut chunk using linear search.
    /// `None` if not found.
    #[inline(always)]
    const fn find_mut_chunk_linear_search(slice: &mut [Chunk], coords: (i32, i32)) -> Option<&mut Chunk>
    {
        if let Some(i) = find_chunk_index_linear_search(slice, &coords) {
            Some(&mut slice[i])
        } else {
            None
        }   
    }

    /// Find chunk using linear search.
    /// `None` if not found.
    #[inline(always)]
    const fn find_chunk_linear_search(slice: &[Chunk], coords: (i32, i32)) -> Option<&Chunk>
    {
        if let Some(i) = find_chunk_index_linear_search(slice, &coords) {
            Some(&slice[i])
        } else {
            None
        } 
    }

    /// Find mut chunk using binary search, slice must be sorted.
    /// `None` if not found.
    #[inline(always)]
    const fn find_mut_chunk_binary_search(slice: &mut [Chunk], coords: (i32, i32)) -> Option<&mut Chunk>
    {
        if let Some(i) = find_chunk_index_binary_search(slice, &coords) {
            Some(&mut slice[i])
        } else {
            None
        }   
    }

    /// Find chunk using binary search, slice must be sorted.
    /// `None` if not found.
    #[inline(always)]
    const fn find_chunk_binary_search(slice: &[Chunk], coords: (i32, i32)) -> Option<&Chunk>
    {
        if let Some(i) = find_chunk_index_binary_search(slice, &coords) {
            Some(&slice[i])
        } else {
            None
        }   
    }

    /// Calculate the inner cells area of a chunk.
    fn calc_chunk_inner(chunk: &Chunk, rule: fn(&mut ChunkCellData8x8, i8, CellContext)) -> ChunkCellData8x8
    {
        let mut new_data = ChunkCellData8x8::from(0);// Assume every cell died
        let mut data_masked = ChunkCellData8x8::default();// Must be init. to work, but don't care what it is
        let mut cell_state: CellContext;

        for cell_index in CELL_INDEX_INNER {

            // If cell is alive, set sign bit.
            cell_state = if unsafe { chunk.unchecked_get_cell_state(cell_index) } { IS_ALIVE_BIT } else { 0 };

            // Calculate 3x3 and stop if exceeding the threshold.
            // Move- and mask current index pos. to [1,1].
            unsafe { data_masked.u32x2[0] = (chunk.data.u64 >> (cell_index - 9)) as u32 & BIT_MASK_1_1; }

            // Count remaining bits.
            cell_state += unsafe { data_masked.u32x2[0] }.count_ones() as i8;
            
            
            /*while unsafe { data_masked.u32x2[0] } != 0 && cell_state & IS_ALIVE_BIT_MASK <= threshold {

                unsafe { data_masked.u32x2[0] &= data_masked.u32x2[0] - 1; }// POPCNT
                cell_state += 1;

                // Count LSB for each byte.
                /*cell_state += unsafe {
                    (data_masked.u8x8[0] & 1) +
                    (data_masked.u8x8[1] & 1) +
                    (data_masked.u8x8[2] & 1)
                } as i8;

                // Exit if exceeding the threshold.
                if cell_state & IS_ALIVE_BIT_MASK > threshold {
                    continue 'index;
                }

                // Shift out the bits and get the next ones to the left.
                unsafe {
                    data_masked.u8x8[0] >>= 1;
                    data_masked.u8x8[1] >>= 1;
                    data_masked.u8x8[2] >>= 1;
                }*/
            }*/

            rule(&mut new_data, cell_index, cell_state);
        }
        new_data
    }

    /// Calculate the outer cells index of chunks->4 and skip if exceeding the threshold.
    /// Remaining chunks in array is neighbouring chunks in order: top right to bottom left.
    fn calc_chunk_outer(chunks: &[&Chunk; 9], threshold: i8, rule: fn(&mut ChunkCellData8x8, i8, CellContext)) -> ChunkCellData8x8
    {
        let mut new_data = ChunkCellData8x8::from(0);// Assume every cell died
        let mut cell_state: CellContext;
        
        'index: for cell_index in CELL_INDEX_OUTER {

            // If cell is alive, set sign bit.
            cell_state = if unsafe { chunks[4].unchecked_get_cell_state(cell_index) } { IS_ALIVE_BIT } else { 0 };

            // Calculate 3x3 and stop if exceeding the threshold.
            for x_offset in -1i8..=1 {
                for y_offset in -1i8..=1 {

                    if x_offset | y_offset != 0 {// Skip if offset == [0,0]

                        let mut chunk_index_offset: usize = 4;// Start in the middle chunk
                        let mut cell_index_offset: i8 = cell_index;// Start in middle of neighbours

                        // Closure for reducing repeats.
                        let mut move_chunk_cell_index = |chunk_scalar: usize, cell_scalar: i8, offset: i8, mask: &u64|
                        {
                            let mask_dir = mask | if offset.is_negative() { mask >> cell_scalar } else { mask << cell_scalar };
                            let to_shift = cell_index + offset * cell_scalar;

                            // Check if it went out of bounds.
                            if (to_shift >= 0 && to_shift < 64) && mask_dir & (1<<to_shift) != 0 {
                                // In same chunk.
                                cell_index_offset += offset * cell_scalar;
                            } else if offset == -1 {
                                // Looped around to the other side
                                cell_index_offset += 7 * cell_scalar;
                                chunk_index_offset -= chunk_scalar;
                            } else {
                                // Looped around to the other, other side.
                                cell_index_offset -= 7 * cell_scalar;
                                chunk_index_offset += chunk_scalar;
                            }
                        };

                        if x_offset != 0 { move_chunk_cell_index(1, 1, x_offset, &X_MASK); }
                        if y_offset != 0 { move_chunk_cell_index(3, 8, y_offset, &Y_MASK); }

                        cell_state += unsafe { chunks[chunk_index_offset].unchecked_get_cell_state(cell_index_offset) } as i8;
                    }
                }

                // Exit if exceeding the threshold.
                if cell_state & IS_ALIVE_BIT_MASK > threshold {
                    continue 'index;
                }
            }

            rule(&mut new_data, cell_index, cell_state);
        }
        new_data
    }


    #[cfg(test)]
    mod tests {
        use super::*;
        use rand::{rngs::SmallRng, RngCore, SeedableRng};
        use alloc::format;
        
        #[test]
        fn test_cmp_coords()
        {
            let mut rng = SmallRng::seed_from_u64(0x123456789abc);

            for _ in 0u8..=255 {
                let a = (rng.next_u32() as i32, rng.next_u32() as i32);
                let b = (rng.next_u32() as i32, rng.next_u32() as i32);

                let a_fmt = format!("({} {})", a.0, a.1);
                let b_fmt = format!("({} {})", b.0, b.1);

                assert_eq!(a.cmp(&b), cmp_coords(&a, &b), "Not correct to `core::tuple::cmp`; a={}, b={}", a_fmt, b_fmt);
            }
        }
    }
}