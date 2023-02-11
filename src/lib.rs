//! # Conway's Game of Life
//! 
//! Core functionality for Conway's Game of Life.
//! Includes data type and implementation for manipulating it.
//! Also includes methods for serialising data to file.

// How many bytes are minimally required for each line.
fn total_blocks(columns: u16) -> u16
{
    columns / 8 + ((columns % 8 > 0) as u16)
}

// Decrease the input value until it's smaller than the largest allowed value.
fn loop_around(mut val: u16, biggest: u16) -> u16
{
    while val > biggest {
        val -= biggest;
    }
    val
}

macro_rules! set_alive {
    ($block:expr, $bit:expr) => {
        $block |= 1<<$bit
    };
}

macro_rules! set_dead {
    ($block:expr, $bit:expr) => {
        $block &= !(1<<$bit)
    };
}

pub use self::game_of_life::Field;
pub use self::game_of_life::core;
pub use self::game_of_life::step_multit;
pub use self::game_of_life::serialize;

pub mod game_of_life
{
    #[derive(Clone)]
    pub struct Field {
        pub rows: u16,
        pub columns: u16,
        pub blocks: u16,
        current: Vec<Vec<u8>>,
        next: Vec<Vec<u8>>
    }

    #[doc = "Core functions for handling Field struct."]
    pub mod core
    {
        use std::{cmp::min};
        use crate::{game_of_life::Field, total_blocks, loop_around};
        impl Field {

            #[doc = "Creates a new Field struct and returns it."]
            pub fn new(rows: u16, columns: u16) -> Field
            {
                if rows == 0 || columns == 0 {
                    panic!("Size must be larger than 0");
                }

                let blocks = total_blocks(columns);

                if blocks == 0 {
                    panic!("Block size became 0");
                }

                let line = vec![vec![0; blocks.into()]; rows.into()];

                let f = Field {
                    rows,
                    columns,
                    blocks,
                    current: line.clone(),
                    next: line.clone()
                };
                f
            }

            /// Get mut byte at position.
            /// 
            /// # Example
            /// ```
            /// use Conways_game_of_life_rust::Field;
            ///
            /// let f: Field = Field::new(8,8);
            /// *field.get_at(2,0) = (1<<3) | (1<<4) | (1<<5);
            /// *field.get_at(0,0) = 2;
            /// ```
            pub fn get_at(&mut self, row: u16, block: u16) -> &mut u8
            {
                let (r, b) = (loop_around(row,self.rows), loop_around(block,self.blocks));
                &mut self.current[r as usize][b as usize]
            }

            #[doc = "Returns whenever cell at location is alive."]
            pub fn is_alive(&self, row: u16, column: u16) -> bool
            {
                let (r, c) = (loop_around(row,self.rows), loop_around(column,self.columns));
                self.current[r as usize][(c / 8) as usize] & 1<<(c % 8) >= 1
            }

            #[doc = "Sets cell at location to alive."]
            pub fn set_alive(&mut self, row: u16, column: u16)
            {
                let (r, c) = (loop_around(row,self.rows), loop_around(column,self.columns));
                set_alive!(self.current[r as usize][(c / 8) as usize], c % 8);
            }

            #[doc = "Sets cell at location to dead."]
            pub fn set_dead(&mut self, row: u16, column: u16)
            {
                let (r, c) = (loop_around(row,self.rows), loop_around(column,self.columns));
                set_dead!(self.current[r as usize][(c / 8) as usize], c % 8);
            }

            #[doc = "Sets all cells to dead."]
            pub fn set_all_dead(&mut self)
            {
                for r in 0..self.rows {
                    for b in 0..self.blocks {
                        //self.next[r as usize][b as usize] ^= self.next[r as usize][b as usize];
                        self.current[r as usize][b as usize] ^= self.current[r as usize][b as usize];
                    }
                }
            }

            #[doc = "Make next generation current generation."]
            pub fn move_next_to_current(&mut self)
            {
                for r in 0..self.rows {
                    for b in 0..self.blocks {
                        self.current[r as usize][b as usize] = self.next[r as usize][b as usize];
                    }
                }
            }

            /// Step the simulation once in a single thread and stores the result in memory.
            /// 
            /// # Examples
            /// ```
            /// use conways_game_of_life_rust::Field;
            /// 
            /// let mut f = Field::new(8,8);
            /// *field.get(0,0) = 2;
            /// *field.get(1,0) = 4;
            /// *field.get(2,0) = 7;
            /// 
            /// f.step_singlet();
            /// f.move_next_generation_to_current();
            /// assert_eq!(0, *f.get(0,0));
            /// assert_eq!(5, *f.get(1,0));
            /// assert_eq!(6, *f.get(2,0));
            /// assert_eq!(2, *f.get(3,0));
            /// ```
            pub fn step_singlet(&mut self)
            {
                let (mut alive, mut cnt): (bool, i8);

                for r in 0..self.rows {
                    for c in 0..self.columns {
                        alive = self.is_alive(r, c);
                        cnt = -(alive as i8);// Don't count the middle cell

                        // Count 9x9. Gotta be a better way to do this.
                        if r == 0 {
                            for ro in r..min(r+2,self.rows) {
                                if c == 0 {
                                    for co in c..min(c+2,self.columns) {
                                        cnt += self.is_alive(ro, co) as i8;
                                    }
                                } else {
                                    for co in c-1..min(c+2,self.columns) {
                                        cnt += self.is_alive(ro, co) as i8;
                                    }
                                }
                            }
                        } else {
                            for ro in r-1..min(r+2,self.rows) {
                                if c == 0 {
                                    for co in c..min(c+2,self.columns) {
                                        cnt += self.is_alive(ro, co) as i8;
                                    }
                                } else {
                                    for co in c-1..min(c+2,self.columns) {
                                        cnt += self.is_alive(ro, co) as i8;
                                    }
                                }
                            }
                        }

                        // Game logic ..
                        if alive {
                            if cnt == 2 || cnt == 3 {// Living cell has 3 or 4 living neighbours survives
                                set_alive!(self.next[r as usize][(c / 8) as usize], c % 8);// Alive
                            } else {
                                set_dead!(self.next[r as usize][(c / 8) as usize], c % 8);// Dead
                            }
                        } else {
                            if cnt == 3 {// Dead cell becomes alive if it has exactly 3 living neighbours
                                set_alive!(self.next[r as usize][(c / 8) as usize], c % 8);
                            } else {
                                set_dead!(self.next[r as usize][(c / 8) as usize], c % 8);// Dead
                            }
                        }
                    }
                }
            }
        }
    }

    #[doc = "Concurrency module for running the simulation in multiple threads."]
    pub mod step_multit
    {
        use crate::{game_of_life::Field};
        use std::{thread::{self, ScopedJoinHandle}, cmp::min};
        impl Field {
            /// Step the simulation once in multiple threads and stores the result in memory.
            ///
            /// # Examples
            /// ```
            /// use conways_game_of_life_rust::Field;
            /// 
            /// let mut f = Field::new(8,8);
            /// *field.get(0,0) = 2;
            /// *field.get(1,0) = 4;
            /// *field.get(2,0) = 7;
            /// 
            /// f.step_multit();
            /// f.move_next_generation_to_current();
            /// assert_eq!(0, *f.get(0,0));
            /// assert_eq!(5, *f.get(1,0));
            /// assert_eq!(6, *f.get(2,0));
            /// assert_eq!(2, *f.get(3,0));
            /// ```
            pub fn step_multit<'a>(&'a mut self)
            {
                let mut data_next: Vec<(u16,u16,u8)> = Vec::with_capacity((self.rows * self.blocks) as usize);
                let f = &self;
                thread::scope(|s| {
                    
                    let mut th: Vec<ScopedJoinHandle<(u16,u16,u8)>> = Vec::with_capacity((self.rows * self.blocks) as usize);
                    for r in 0..self.rows {
                        for b in 0..self.blocks {

                            th.push(
                                s.spawn(move || {
                                    let r = r.clone();
                                    let b = b.clone();

                                    let (mut alive, mut cnt): (bool, i8);

                                    let mut new_block: u8 = 0;

                                    for bo in 0..8 {
                                        let c = b*8 + bo;

                                        alive = f.is_alive(r, c);
                                        cnt = -(alive as i8);// Don't count the middle cell

                                        // Count 9x9.
                                        if r == 0 {
                                            for ro in r..min(r+2,f.rows) {
                                                if c == 0 {
                                                    for co in c..min(c+2,f.columns) {
                                                        cnt += f.is_alive(ro, co) as i8;
                                                    }
                                                } else {
                                                    for co in c-1..min(c+2,f.columns) {
                                                        cnt += f.is_alive(ro, co) as i8;
                                                    }
                                                }
                                            }
                                        } else {
                                            for ro in r-1..min(r+2,f.rows) {
                                                if c == 0 {
                                                    for co in c..min(c+2,f.columns) {
                                                        cnt += f.is_alive(ro, co) as i8;
                                                    }
                                                } else {
                                                    for co in c-1..min(c+2,f.columns) {
                                                        cnt += f.is_alive(ro, co) as i8;
                                                    }
                                                }
                                            }
                                        }

                                        // Game logic ..
                                        if alive {
                                            if cnt == 2 || cnt == 3 {// Living cell has 3 or 4 living neighbours survives
                                                set_alive!(new_block, bo);
                                            }
                                        } else {
                                            if cnt == 3 {// Dead cell becomes alive if it has exactly 3 living neighbours
                                                set_alive!(new_block, bo);
                                            }
                                        }
                                    }

                                    (r,b,new_block)
                                })
                            );
                        }
                    }
                    // let mut iter_th = th.iter().cycle();
                    
                    // loop {
                    //     let t = iter_th.next().unwrap();
                    //     if t.is_finished() {
                    //         iter_th
                    //     }
                    // }
                    
                    for t in th {
                        data_next.push(t.join().unwrap());// have to store the result in a variable outside the scope handler
                    }
                });
                
                //self.next[1][1] = 1;
                for bl in data_next.iter() {
                    self.next[bl.0 as usize][bl.1 as usize] = bl.2;
                }
            }
        }
    }

    #[doc = "Read and write from system from- to Field struct."]
    pub mod serialize {
        use crate::{game_of_life::Field};
        use std::{fs::File, io::{Write, Read}};

        impl Field {

            #[doc = "Writes Field struct to a file on the system."]
            pub fn serialize(&self, path: String) -> std::io::Result<()>
            {

                let mut file = File::create(path)?;

                file.write_all(&self.rows.to_be_bytes())?;
                file.write_all(&self.columns.to_be_bytes())?;

                for bl in self.current.iter() {
                    file.write_all(&bl)?;
                }

                Ok(())
            }

            #[doc = "Reads a file on the system to a Field struct."]
            pub fn deserialize(path: String) -> std::io::Result<Field>
            {
                let mut file = File::open(path)?;

                let mut buf = [0; 2];
                file.read(&mut buf)?;
                let rows = (buf[0]<<4 | buf[1]) as u16;
                file.read(&mut buf)?;
                let columns = (buf[0]<<4 | buf[1]) as u16;

                let mut buf: Vec<u8> = Vec::new();
                file.read_to_end(&mut buf)?;

                let mut f = Field::new(rows, columns);

                for r in 0..rows {
                    for b in 0..f.blocks {
                        f.current[r as usize][b as usize] = buf[(r * f.blocks + b) as usize];
                    }
                }

                Ok(f)
            }
        }
    }
}