//! Contains a method to step a simulation once in a single thread.

use super::{
    calc_chunk_inner, calc_chunk_outer, cmp_coords, find_chunk_binary_search, get_neighbour_chunk_xy,
    CellContext, Chunk, ChunkCellData8x8, Field,
    DUMMY_CHUNK, NEIGHBOUR_CHUNK_INDEX, CHUNKS_IN_STACK
};
use alloc::vec::Vec;


impl Field {
    /// Step a simulation once in a single thread and increment generation count.
    /// Requires a "rule" function pointer for calculating the next generation.
    /// Threshold number of living neighbours when to abort and assume the cell will die.
    /// ```no_run
    /// use conways_game_of_life_dyn_lib::{ChunkCellData8x8, Field, rule_fns::rule_conways_game_of_life as rule};
    /// 
    /// let mut f = Field::new();
    /// f.add_new_chunk(0, 0, ChunkCellData8x8::from(0x70402));
    /// 
    /// assert_eq!(f.get_generation(), 0);
    /// f.step(rule, 3);
    /// assert_eq!(f.get_generation(), 1);
    /// ```
    pub fn step(&mut self, rule: fn(&mut ChunkCellData8x8, i8, CellContext), threshold: i8)
    {
        let current = {
            //let mut tmp = self.get_current().clone();

            let mut tmp = tinyvec::tiny_vec!([Chunk; CHUNKS_IN_STACK]);
            tmp.extend_from_slice(self.get_current());

            // Sort by coordinates.
            tmp.sort_unstable_by(|a, b| cmp_coords(&a.get_coordinates(), &b.get_coordinates()));
            tmp
        };

        let next = self.get_mut_next();
        next.clear();

        // List of dummy chunk coordinates that may become real, pre-allocate at least current.len() * 8.
        let mut dummy_chunks_pos: Vec<(i32, i32)> = Vec::with_capacity(current.len() * 8);

        //let mut dummy_chunks_pos = tinyvec::tiny_vec!([(i32, i32); CHUNKS_IN_STACK * 2]);

        // Calculate a chunk's cell states.
        for c in current.iter() {

            if c.all_are_dead() { continue; }

            // Cache surrounding chunks.
            let mut neightbour_chunks_cache = [&DUMMY_CHUNK; 9];
            {
                let neighbour_chunk_xy = get_neighbour_chunk_xy(&(c.x, c.y));

                neightbour_chunks_cache[4] = &c;
                for i in &NEIGHBOUR_CHUNK_INDEX {

                    //if let Ok(index) = current.binary_search_by(|c| cmp_coords(&c.get_coordinates(), &neighbour_chunk_xy[*i])) {
                    if let Some(c) = find_chunk_binary_search(&current, neighbour_chunk_xy[*i]) {
                        // Found it.
                        neightbour_chunks_cache[*i] = c;
                        //neightbour_chunks_cache[*i] = &current[index];
                    } else {
                        // If it doesn't exist keep the dummy chunk pos. for further calculations.
                        dummy_chunks_pos.push(neighbour_chunk_xy[*i]);
                    }
                }
            }

            // Calculate next generation for the chunk.
            let new_chunk_data = calc_chunk_inner(c, rule) | calc_chunk_outer(&neightbour_chunks_cache, threshold, rule);

            if unsafe { new_chunk_data.u64 } != 0 {// Only push to the list if it has living cells
                next.push(Chunk { x: c.x, y: c.y, data: new_chunk_data });
            }
        }

        // Sort- and remove duplicates.
        dummy_chunks_pos.sort_unstable();
        dummy_chunks_pos.dedup();

        /*let dummy_chunks_pos = {

            let mut tmp = tinyvec::tiny_vec!([(i32, i32); CHUNKS_IN_STACK * 2]);
            tmp.extend_from_slice(dummy_chunks_pos.as_slice());
            tmp
        };*/

        // Calculate the outer cell range of all the dummy chunks.
        for chunk_pos in dummy_chunks_pos.iter() {
            // Cache surrounding chunks.
            let mut neightbour_chunks_cache: [&Chunk; 9] = [&DUMMY_CHUNK; 9];
            {
                let neighbour_chunk_xy = get_neighbour_chunk_xy(chunk_pos);

                for i in &NEIGHBOUR_CHUNK_INDEX {

                    //if let Ok(index) = current.binary_search_by(|c| cmp_coords(&c.get_coordinates(), &neighbour_chunk_xy[*i])) {
                    if let Some(c) = find_chunk_binary_search(&current, neighbour_chunk_xy[*i]) {
                        // Found it.
                        neightbour_chunks_cache[*i] = c;
                        //neightbour_chunks_cache[*i] = &current[index];
                    }
                }
            }

            let new_chunk_data = calc_chunk_outer(&neightbour_chunks_cache, threshold, rule);

            if unsafe { new_chunk_data.u64 } != 0 {// Only push to the list if it has living cells
                next.push(Chunk { x: chunk_pos.0, y: chunk_pos.1, data: new_chunk_data });
            }
        }

        self.generation += 1;// Flips current/next
    }
}