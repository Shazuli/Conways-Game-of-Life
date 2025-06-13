//! Contains macros and methods to manipulate a struct Field.

use super::{
    find_chunk_binary_search, find_chunk_linear_search, find_mut_chunk_binary_search, find_mut_chunk_linear_search,
    Chunk, ChunkCellData8x8, Field,
    IS_ALIVE_BIT
};
use alloc::vec::Vec;


/// Creates a new struct Field with chunks to it or adds chunks to an existing one.
/// Duplicate coordinates gets merged into one.
/// ```no_run
/// use conways_game_of_life_dyn_lib::{set_field, Chunk, ChunkCellData8x8, Field};
/// 
/// let mut f = set_field!(
///     0, 0, 0x18_30_08_00_00_00;
///     0,-1, 0x18_30_08_00_00_00;
/// );
/// ```
/// ```no_run
/// use conways_game_of_life_dyn_lib::{set_field, Chunk, ChunkCellData8x8, Field};
/// 
/// let mut f = Field::new();
/// set_field!(f =>
///     0, 0, 0x18_30_08_00_00_00;
///     0,-1, 0x18_30_08_00_00_00;
/// );
/// ```
#[macro_export]
macro_rules! set_field {
    ($($x:expr, $y:expr, $data:expr);+ $(;)?) => {{
        use $crate::{ChunkCellData8x8 as CD, Chunk as C, Field as F, field_utilities::dedup_merge_chunks};

        const INVOKED_ELEM_COUNT: usize = 0 $( + { let _ = stringify!($x); 1 })+;

        let mut chunks: Vec<C> = Vec::with_capacity(INVOKED_ELEM_COUNT);
        $(
            chunks.push(C::new($x,$y,CD::from($data)));
        )+
        chunks.sort_unstable();
        dedup_merge_chunks(&mut chunks);

        let mut f = F::with_capacity(chunks.len());

        chunks.iter().for_each(|c|unsafe {
            f.unchecked_add_new_chunk(c.get_x(),c.get_y(),CD::from(c.data));
        });
        f
    }};
    ($f:ident => $($x:expr, $y:expr, $data:expr);+ $(;)?) => {{
        use $crate::{ChunkCellData8x8 as CD, Chunk as C, field_utilities::dedup_merge_chunks};

        const INVOKED_ELEM_COUNT: usize = 0 $( + { let _ = stringify!($x); 1 })+;
        
        let mut chunks: Vec<C> = Vec::with_capacity(INVOKED_ELEM_COUNT);
        $(
            chunks.push(C::new($x,$y,CD::from($data)));
        )+
        chunks.sort_unstable();
        dedup_merge_chunks(&mut chunks);

        $f.reserve(chunks.len() - ($f.capacity().0 - $f.len().0), 1);

        chunks.iter().for_each(|c|{
            $f.add_new_chunk(c.get_x(),c.get_y(),CD::from(c.data))
                .expect("Chunk already defined in `Field` in `set_field` macro");
        });
    }}
}

/// Merge chunks with same position. Merges all chunks if sorted.
pub fn dedup_merge_chunks(chunks: &mut Vec<Chunk>)
{
    // Reverse direction and use `swap_remove`?
    let mut end = chunks.len() - 1;
    let mut i = 0;

    let (mut c, mut c_peek): (&Chunk, &Chunk);

    while i<end {

        c = &chunks[i];
        c_peek = &chunks[i+1];

        if c.x == c_peek.x && c.y == c_peek.y {

            unsafe {
                chunks[i].data.u64 |= c_peek.data.u64;
            }
            chunks.remove(i+1);
            end -= 1;
        } else {
            i += 1;
        }
    }
}


/// Get chunk's inner coordinate from a global coordinate.
#[inline]
const fn to_inner_chunks_coord(global: i32) -> i8
{
    if global.is_negative() {
        (7 - ((global.abs() - 1) % 8)) as i8
    } else {
        (global % 8) as i8
    }
}

/// Get chunk coordinate from a global coordinate.
#[inline]
const fn to_chunk_coord(global: i32) -> i32
{
    if global.is_negative() {
        (global - 7) / 8
    } else {
        global / 8
    }
}


impl Field {

    /// Get cell state at global position and `false` if not found.
    pub fn get_cell_state(&self, x: i32, y: i32) -> bool
    {
        // Get the chunk coordinates.
        let chunk_coords = (
            to_chunk_coord(x),
            to_chunk_coord(y)
        );

        if let Some(c) = find_chunk_linear_search(self.get_current(), chunk_coords) {
            // Found it.
            if c.all_are_dead() { return false; }

            let index = to_inner_chunks_coord(x) + to_inner_chunks_coord(y) * 8;

            c.get_cell_state(index)
                .expect("Index out of bounds [Field::get_cell_state]")
        } else {
            // Didn't find it so must be dead.
            false
        }
        
    }

    /// Set global cell's position to state and returns `bool` if cell state changed.
    pub fn set_cell_state(&mut self, x: i32, y: i32, state: bool) -> bool
    {
        let current = self.get_mut_current();

        // Get the chunk coordinates.
        let chunk_coords = (
            to_chunk_coord(x),
            to_chunk_coord(y)
        );

        let index = to_inner_chunks_coord(x) + to_inner_chunks_coord(y) * 8;

        if let Some(c) = find_mut_chunk_linear_search(current, chunk_coords) {
            // Found it.
            let new_state = IS_ALIVE_BIT * state as i8;

            c.set_cell_state(index | new_state)
                .expect("Index out of bounds [Field::set_cell_state]")

        } else if state {
            // Didn't find it. Create a new chunk and push it.
            current.push(Chunk { x: chunk_coords.0, y: chunk_coords.1, data: ChunkCellData8x8::from(1<<index) });
            true
        } else {
            // If not found and was going to be set to dead, do nothing and return nothing changed.
            false
        }
    }

    /// Returns the number of chunks in the vector, also referred to as its 'length'.
    /// `0` = current, `1` = next.
    #[inline]
    pub fn len(&self) -> (usize, usize)
    {
        (self.get_current().len(), self.get_next().len())
    }

    /// Returns the number of chunks the vector can hold without reallocating.
    /// `0` = current, `1` = next.
    #[inline]
    pub fn capacity(&self) -> (usize, usize)
    {
        (self.get_current().capacity(), self.get_next().capacity())
    }

    /// Reserves capacity for at least additional more chunks to be inserted into the current and next vector.
    /// Bit-flag for which vector(s) to mutate. `1` = current, `2` = next.
    pub fn reserve(&mut self, additional: usize, vec_select_flags: u8)
    {
        if vec_select_flags & 1 != 0 {
            self.get_mut_current().reserve(additional);
        }
        if vec_select_flags & 2 != 0 {
            self.get_mut_next().reserve(additional);
        }
    }

    /// Merges chunks with same coordinates.
    /// Bit-flag for which vector(s) to mutate. `1` = current, `2` = next.
    pub fn merge_chunks(&mut self, vec_select_flags: u8)
    {
        if vec_select_flags & 1 != 0 {
            let chunks = self.get_mut_current();
            chunks.sort_unstable();
            dedup_merge_chunks(chunks);
        }
        if vec_select_flags & 2 != 0 {
            let chunks = self.get_mut_next();
            chunks.sort_unstable();
            dedup_merge_chunks(chunks);
        }
    }

    /// Sorts chunks **without** preserving the initial order of equal chunks.
    /// Bit-flag for which vector(s) to mutate. `1` = current, `2` = next.
    pub fn sort_unstable_chunks(&mut self, vec_select_flags: u8)
    {
        if vec_select_flags & 1 != 0 {
            self.get_mut_current().sort_unstable();
        }
        if vec_select_flags & 2 != 0 {
            self.get_mut_next().sort_unstable();
        }
    }

    /// Clears the vector, removing all chunks.
    /// Bit-flag for which vector(s) to mutate. `1` = current, `2` = next.
    ///
    /// Note that this method has no effect on the allocated capacity of the vector.
    pub fn clear(&mut self, vec_select_flags: u8)
    {
        if vec_select_flags & 1 != 0 {
            self.get_mut_current().clear();
        }
        if vec_select_flags & 2 != 0 {
            self.get_mut_next().clear();
        }
    }

    /// Searches for Chunk at position and returns a clone of it or if not found returns a chunk with all cells dead.
    /// If not sorted it may not find it.
    pub fn find_chunk_binary_search(&self, x: i32, y: i32) -> Chunk
    {
        find_chunk_binary_search(self.get_current(), (x, y))
            .unwrap_or(&Chunk { x, y, data: ChunkCellData8x8::from(0) }).clone()
    }

    /// Searches for chunk at position and returns a Option with a mutable chunk or None.
    /// If not sorted it may not find it.
    pub fn find_mut_chunk_binary_search(&mut self, x: i32, y: i32) -> Option<&mut Chunk>
    {
        find_mut_chunk_binary_search(self.get_mut_current(), (x, y))
    }
}