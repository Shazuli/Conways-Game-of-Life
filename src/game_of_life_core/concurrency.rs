use super::{ChunkCellData, {calc_chunk_inner, calc_chunk_outer}};
use alloc::{vec::Vec, sync::Arc};
use core::num::NonZero;

//#![cfg_attr(not(feature = "std"), no_std)]
//#[cfg(feature = "std")]

impl super::Field {

    /// Step the simulation once in multiple threads and increment generation count.
    /// Requires a "rules" fn to apply when calculating the next generation.
    /// Threshold number of living neighbours when to abort assume the cell will die.
    /// Max threads to run and bias for how many chunks each threads should do.
    
    pub fn step_multit(&mut self, rules: fn(&mut ChunkCellData, i8, i8), threshold: i8, max_threads: NonZero<u8>, bias: u16)
    {
        todo!()
    }
}

