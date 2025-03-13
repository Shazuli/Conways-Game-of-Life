use super::{ChunkCellData, Chunk, Field, calc_chunk_inner, calc_chunk_outer};
use alloc::{borrow::ToOwned, vec::Vec};

/// Creates a new Field struct with chunks to it or adds chunks to an existing one.
/// # Example
/// ```
/// let mut f = set_field_chunks!(
///     0, 0, 0x18_30_08_00_00_00;
///     0,-1, 0x18_30_08_00_00_00;
/// );
/// ```
/// ```
/// let mut f = Field::new();
/// set_field_chunks!(f;
///     0, 0, 0x18_30_08_00_00_00;
///     0,-1, 0x18_30_08_00_00_00;
/// );
/// ```
#[macro_export]
macro_rules! set_field_chunks {
    ($($x:expr, $y:expr, $data:expr);+ $(;)?) => {{
        use conways_game_of_life_dyn_lib::game_of_life_core::ChunkCellData;
        let mut f = Field::new();
        // TODO: check if all the [x,y] coordinates are unique before adding them.
        unsafe {
            $(
                f.force_add_chunk($x,$y,ChunkCellData{u64:$data});
            )+
        }
        f
    }};
    ($f:ident; $($x:expr, $y:expr, $data:expr);+ $(;)?) => {{
        use conways_game_of_life_dyn_lib::game_of_life_core::ChunkCellData;
        $(
            $f.add_chunk($x,$y,ChunkCellData{u64:$data}).unwrap();
        )+
    }}
}

/// Rule function for calculating cells for the original implementation of GOL.
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
                set_bit!(new_data.u64, cell_index)
            }
        }
    } else if cell_state == 3 {// Dead cell becomes alive if it has exactly 3 living neighbours
        unsafe {
            set_bit!(new_data.u64, cell_index)
        }
    }
}

impl Field {

    /// Adds a new chunk in the Field at position and returns a Result with a copy of the chunk. Will fail if already defined. This function is O(n).
    /// 
    /// # Example
    /// ```
    /// let mut f = Field::new();
    /// 
    /// f.add_chunk(-1, 0, ChunkCellData { u64: 0x18_30_08_00_00_00}).unwrap();// Glider
    /// ```
    pub fn add_chunk(&mut self, x: i32, y: i32, data: ChunkCellData) -> Result<Chunk, &str>
    {
        let current = self.get_mut_current();

        for c in current.iter() {
            if c.x == x && c.y == y {
                return Err("Already defined");
            }
        }
        let new_chunk = Chunk { x, y, data };
        current.push(new_chunk);
        Ok(new_chunk)
    }

    /// Adds a new chunk in the Field at position and returns a copy of the Chunk without checking if already defined.
    /// 
    /// # Example
    /// ```
    /// let mut f = Field::new();
    /// 
    /// unsafe { f.force_add_chunk(-1, 0, ChunkCellData { u64: 0x18_30_08_00_00_00}); }// Glider
    /// ```
    pub unsafe fn force_add_chunk(&mut self, x: i32, y: i32, data: ChunkCellData) -> Chunk
    {
        let new_chunk = Chunk { x, y, data };
        self.get_mut_current().push(new_chunk);
        new_chunk
    }

    /// Searches for Chunk at position and returns a clone of it or if not found returns a Chunk with all cells dead.
    pub fn find_chunk(&self, x: i32, y: i32) -> Chunk
    {
        for c in self.get_current().iter() {
            if c.x == x && c.y == y {
                return c.clone();
            }
        }
        Chunk { x, y, data: ChunkCellData { u64: 0 } }
    }

    /// Searches for Chunk at position and returns a Option with a mutable Chunk or None.
    pub fn find_mut_chunk(&mut self, x: i32, y: i32) -> Option<&mut Chunk>
    {
        for c in self.get_mut_current().iter_mut() {
            if c.x == x && c.y == y {
                return Some(c);
            }
        }
        None
    }

    /// Get if cell at global position is alive and always false if not found. [UNTESTED]
    pub fn is_alive(&self, x: i32, y: i32) -> bool
    {
        // Get the chunk coordinates.
        let (chunk_x, chunk_y) = (x / 8, y / 8);

        for c in self.get_current().iter() {
            if c.x == chunk_x && c.y == chunk_y {
                if c.all_are_dead() {
                    break
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
        let current: &Vec<Chunk> = self.get_current();
        let mut next: Vec<Chunk> = Vec::new();

        // List of dummy chunk coordinates that may become real, pre-allocate at least current.capacity * 8 for it having 8 total neighbours.
        let mut dummy_chunks_pos: Vec<(i32, i32)> = Vec::with_capacity(current.capacity() * 8);

        let dummy_chunk_ref = Chunk::default();

        // Array of neighbour chunk index except for the middle.
        let neightbour_chunk_cache_array: [usize; 8] = [0,1,2,3,5,6,7,8];

        // Calculate a chunk's cell states.
        for c in current.iter() {

            if c.all_are_dead() {
                continue
            }

            // Cache surrounding chunks.
            let mut neightbour_chunks_cache: [&Chunk; 9] = [&dummy_chunk_ref; 9];
            {
                let neighbour_chunk_xy: [(i32, i32); 9] = [
                    (c.x-1, c.y-1),(c.x, c.y-1),(c.x+1, c.y-1),
                    (c.x-1, c.y  ),   (0,0),    (c.x+1, c.y  ),
                    (c.x-1, c.y+1),(c.x, c.y+1),(c.x+1, c.y+1)
                ];

                neightbour_chunks_cache[4] = &c;
                'neighbour_chunk_caching: for i in neightbour_chunk_cache_array {
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
                calc_chunk_inner(c, threshold, rules).u64 |
                calc_chunk_outer(neightbour_chunks_cache, threshold, rules).u64
            };

            if new_chunk_data != 0 {// Only push to the list if it has living cells
                next.push(Chunk { x: c.x, y: c.y, data: ChunkCellData { u64: new_chunk_data } });
            }
        }

        
        // Calculate the outer cell range of all the dummy chunks.
        for chunk_pos in dummy_chunks_pos.iter() {
            // Cache surrounding chunks.
            let mut neightbour_chunks_cache: [&Chunk; 9] = [&dummy_chunk_ref; 9];
            {
                let neighbour_chunk_xy: [(i32, i32); 9] = [
                    (chunk_pos.0-1, chunk_pos.1-1),(chunk_pos.0, chunk_pos.1-1),(chunk_pos.0+1, chunk_pos.1-1),
                    (chunk_pos.0-1, chunk_pos.1  ),          (0,0),             (chunk_pos.0+1, chunk_pos.1  ),
                    (chunk_pos.0-1, chunk_pos.1+1),(chunk_pos.0, chunk_pos.1+1),(chunk_pos.0+1, chunk_pos.1+1)
                ];

                'neighbour_chunk_caching: for i in neightbour_chunk_cache_array {
                    for c in current.iter() {
                        if c.x == neighbour_chunk_xy[i].0 && c.y == neighbour_chunk_xy[i].1 {
                            neightbour_chunks_cache[i] = &c;
                            continue 'neighbour_chunk_caching;
                        }
                    }
                }
            }

            // Only need to calculate the outer cells.
            let new_chunk_data = unsafe { calc_chunk_outer(neightbour_chunks_cache, threshold, rules).u64 };

            if new_chunk_data != 0 {// Only push to the list if it has living cells
                next.push(Chunk { x: chunk_pos.0, y: chunk_pos.1, data: ChunkCellData { u64: new_chunk_data } });
            }
        }

        *self.get_mut_next() = next.to_owned();
        self.generation += 1;
    }
}