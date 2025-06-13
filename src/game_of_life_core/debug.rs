//! Contains methods to debug struct Chunk and struct Field.

use super::{Chunk, Field};
use core::fmt::{Debug, Display, Formatter, Result};


impl Debug for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result
    {
        unsafe {
            write!(f,"Chunk {{ x: {}, y: {}, data: {} }}",
                self.x,self.y,self.data.u64
            )
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result
    {
        unsafe {
            write!(f,"[{},{}]\n{:08b}\n{:08b}\n{:08b}\n{:08b}\n{:08b}\n{:08b}\n{:08b}\n{:08b}",
                self.x,self.y,
                self.data.u8x8[0],self.data.u8x8[1],self.data.u8x8[2],self.data.u8x8[3],
                self.data.u8x8[4],self.data.u8x8[5],self.data.u8x8[6],self.data.u8x8[7]
            )
        }
    }
}

impl Debug for Field {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result
    {
        write!(f,"Field {{ generation: {}, current: {:?} }}",self.generation,self.get_current())
    }
}

impl Display for Field {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result
    {
        write!(f,"gen:{}\n{:#?}",self.generation,self.get_current())
    }
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool
    {
        let left_current = self.get_current();
        let right_current = other.get_current();

        if self.generation != other.generation || left_current.len() != right_current.len() {
            return false;
        }

        // This may not be the best way to do this...
        let (mut left, mut right) = (left_current.clone(), right_current.clone());

        left.sort_unstable();
        right.sort_unstable();

        let matching = left.iter().zip(&right).filter(|&(l, r)| l == r).count();
        matching == left.len() && matching == right.len()
    }
}

impl Eq for Field {}