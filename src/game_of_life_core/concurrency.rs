use super::{calc_chunk_inner, calc_chunk_outer, Chunk, ChunkCellData};
use core::slice;
use std::{cmp:: min, num::NonZero, sync::{Arc, Mutex}, thread};


/*struct ChunkSlice {
    count: u16
}

impl Iterator for ChunkSlice {
    
    type Item = Vec<Chunk>;

    fn next(&mut self) -> Option<Self::Item>
    {

    }
}*/

/*impl Iterator for super::Field {

    type Item = Vec<Chunk>;

    fn next(&mut self) -> Option<Self::Item>
    {

        todo!()
    }
}*/

impl super::Field {

    /// Step the simulation once in multiple threads and increment generation count.
    /// Requires a "rules" fn to apply when calculating the next generation.
    /// Threshold number of living neighbours when to abort and assume the cell will die.
    /// Max threads to run and prefered number of chunks each thread should do.
    pub fn step_multit(&mut self, rules: fn(&mut ChunkCellData, i8, i8), threshold: i8, max_threads: NonZero<u8>, prefered_chunks_per_thread: NonZero<u16>)
    {
        let current: &Vec<Chunk> = self.get_current();
        let mut next: Arc<Mutex<Vec<Chunk>>> = Arc::new(Mutex::new(Vec::new()));

        // List of dummy chunk coordinates that may become real, pre-allocate at least current.capacity * 8 for it having 8 total neighbours.
        let mut dummy_chunks_pos: Arc<Mutex<Vec<(i32, i32)>>> = Arc::new(Mutex::new(Vec::with_capacity(current.capacity() * 8)));

        {
            // TODO Calculate number of threads and chunks divided over them.
            let num_threads: u8 = 4;
            let slice_size: u16 = 16;

            /*
            If #chunks is twice as big as prefered_chunks_per_thread then it should be divided between two threads.
             */

            //let iter = current.chunks(prefered_chunks_per_thread.get() as usize);


            thread::scope(|s| {

                //let mut slice: Vec<Chunk>;
                //for slice_start in (0..(num_threads as u16 * slice_size)).step_by(slice_size.into()) {
                    //slice = Vec::new();
                    //slice.clone_from_slice(&current[slice_start..(slice_start + slice_size)]);

                for i in 0..num_threads {

                    // Create iter with start and end, copy it to the thread.
                    let a = &current[4];

                    s.spawn(move || {

                        //let slice = slice.clone();



                    });
                }
            });
        }
    }
}

