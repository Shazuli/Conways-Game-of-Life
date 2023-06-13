//! # Conway's Game of Life
//! 
//! Core functionality for Conway's Game of Life.
//! Includes data type and implementation for manipulating it.
//! Also includes methods for serialising data to file.

macro_rules! set_bit {
    ($val:expr, $bit:expr) => {
        $val |= 1<<$bit
    };
}

macro_rules! clear_bit {
    ($val:expr, $bit:expr) => {
        $val &= !(1<<$bit)
    };
}

#[doc(hidden)]
pub use self::game_of_life::Field;
#[doc(hidden)]
pub use self::game_of_life::core;
#[doc(hidden)]
pub use self::game_of_life::step_multit;
#[doc(hidden)]
pub use self::game_of_life::serialize;

pub mod game_of_life
{
    #[derive(Clone)]
    pub struct Field {
        rows: u16,
        columns: u16,
        blocks: u16,
        current: Vec<Vec<u8>>,
        next: Vec<Vec<u8>>
    }

    ///Core functions for handling Field struct.
    pub mod core
    {
        use crate::game_of_life::Field;
        use std::cmp::min;
        impl Field {

            ///Creates a new Field struct and returns it.
            pub fn new(rows: u16, columns: u16) -> Field
            {
                if rows == 0 || columns == 0 {
                    panic!("Size must be larger than 0");
                }

                // How many bytes are minimally required for each line.
                let blocks = columns / 8 + ((columns % 8 > 0) as u16);

                if blocks == 0 {
                    panic!("Block size became 0");
                }

                let line = vec![vec![0; blocks.into()]; rows.into()];

                let f = Field {
                    rows,
                    columns,
                    blocks,
                    current: line.clone(),
                    next: line.to_owned()
                };

                f
            }

            ///Get rows of Field struct.
            pub fn get_rows(&self) -> u16 {
                self.rows
            }

            ///Get columns of Field struct.
            pub fn get_columns(&self) -> u16 {
                self.columns
            }

            /// Get blocks of Field struct.
            pub fn get_blocks(&self) -> u16 {
                self.blocks
            }

            /// Get mut byte at position.
            /// 
            /// # Example
            /// ```
            /// use Conways_game_of_life_rust::Field;
            ///
            /// let f: Field = Field::new(8, 8);
            /// *field.get_at(2,0) = (1<<3) | (1<<4) | (1<<5);
            /// *field.get_at(0,0) = 2;
            /// ```
            /// ```
            /// use Conways_game_of_life_rust::Field;
            ///
            /// let f: Field = Field::new(26, 26);
            ///
            /// let (r, c) = (13, 19);
            /// let byte: u8 = *field.get_at(r, c / 8);
            /// ```
            pub fn get_at(&mut self, row: u16, block: u16) -> &mut u8
            {
                &mut self.current[row as usize][block as usize]
            }

            /// Set multiple bytes from position.
            pub fn set_at(&mut self, row: u16, block: u16, new_blocks: usize)
            {
                let mut new_block = new_blocks;
                let mut i: u16 = 0;
                while new_block != 0 {
                    *self.get_at(row, i + block) |= new_block as u8;
                    i += 1;
                    new_block >>= i * 8;
                }

            }

            /// Returns whenever cell at location is alive.
            pub fn is_alive(&self, row: u16, column: u16) -> bool
            {
                self.current[row as usize][(column / 8) as usize] & 1<<(column % 8) >= 1
            }

            /// Count 3x3 around r c including r c.
            pub fn count_neighbours(&self, r: u16, c: u16) -> i8
            {
                let mut cnt = 0;
                if r == 0 {
                    for ro in r..min(r+2,self.rows) {
                        if c == 0 {
                            for co in c..min(c+2, self.columns) {
                                cnt += self.is_alive(ro, co) as i8;
                            }
                        } else {
                            for co in c-1..min(c+2, self.columns) {
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
                cnt
            }

            ///Sets cell at location to alive.
            pub fn set_alive(&mut self, row: u16, column: u16)
            {
                set_bit!(self.current[row as usize][(column / 8) as usize], column % 8);
            }

            ///Sets cell at location to dead.
            pub fn set_dead(&mut self, row: u16, column: u16)
            {
                clear_bit!(self.current[row as usize][(column / 8) as usize], column % 8);
            }

            ///Sets all cells to dead.
            pub fn set_all_dead(&mut self)
            {
                for r in 0..self.rows {
                    for b in 0..self.blocks {
                        self.current[r as usize][b as usize] ^= self.current[r as usize][b as usize];
                    }
                }
            }

            ///Make next generation current generation.
            pub fn move_next_to_current(&mut self)
            {
                let mask = (1<<(self.columns % 8))-1;
                for r in 0..self.rows {
                    for b in 0..self.blocks {
                        if mask > 0 && b == self.blocks-1 {
                            self.current[r as usize][b as usize] = self.next[r as usize][b as usize] & mask;// Zero unused bits
                        } else {
                            self.current[r as usize][b as usize] = self.next[r as usize][b as usize];
                        }
                    }
                }
            }

            /// Step the simulation once in a single thread and stores the result in memory.
            /// 
            /// # Examples
            /// ```
            /// use conways_game_of_life_rust::{Field, set_field};
            /// 
            /// let mut f = Field::new(8, 8);
            /// set_field!(&mut f;
            ///     0, 0, 2;
            ///     1, 0, 4;
            ///     2, 0, 7;
            /// );
            /// 
            /// f.step_singlet();
            /// f.move_next_generation_to_current();
            /// assert_eq!(0, *f.get(0, 0));
            /// assert_eq!(5, *f.get(1, 0));
            /// assert_eq!(6, *f.get(2, 0));
            /// assert_eq!(2, *f.get(3, 0));
            /// ```
            pub fn step_singlet(&mut self)
            {
                let (mut alive, mut cnt): (bool, i8);

                for r in 0..self.rows {
                    for c in 0..self.columns {
                        alive = self.is_alive(r, c);
                        cnt = self.count_neighbours(r, c) - (alive as i8);

                        // Game logic ..
                        if alive {
                            if cnt == 2 || cnt == 3 {// Living cell has 3 or 4 living neighbours survives
                                set_bit!(self.next[r as usize][(c / 8) as usize], c % 8);
                            } else {
                                clear_bit!(self.next[r as usize][(c / 8) as usize], c % 8);
                            }
                        } else if cnt == 3 {// Dead cell becomes alive if it has exactly 3 living neighbours
                            set_bit!(self.next[r as usize][(c / 8) as usize], c % 8);
                        }
                    }
                }
            }
        }

        /// Clears the Field and sets Field struct cell bytes.
        /// 
        /// # Example
        /// ```
        /// use Conways_game_of_life_rust::{Field, set_field};
        ///
        /// let f: Field = Field::new(24,24);
        /// set_field!(&mut f;
        ///     3,  1, 24;
        ///     4,  1, 24;
        ///     6,  1, 60;
        ///     7,  1, 102;
        ///     8,  1, 129;
        ///     10, 1, 129;
        ///     11, 1, 165;
        ///     12, 1, 24;
        ///     13, 1, 24;
        ///     14, 1, 102;
        /// );
        /// ```
        #[macro_export]
        macro_rules! set_field {
            ($field:expr;
                $($r:expr, $b:expr, $block:expr;)+ $(,)?) => {
                    Field::set_all_dead($field);
                    $(
                        Field::set_at($field, $r, $b, $block);
                    )+
            }
        }
    }

    ///Concurrency module for running the simulation in multiple threads.
    pub mod step_multit
    {
        use crate::game_of_life::Field;
        use std::{thread, sync::{Arc, Mutex}};
        impl Field {
            /// Step the simulation once in multiple threads and stores the result in memory.
            ///
            /// # Examples
            /// ```
            /// use conways_game_of_life_rust::{Field, set_field};
            /// 
            /// let mut f = Field::new(8, 8);
            /// set_field!(&mut f;
            ///     0, 0, 2;
            ///     1, 0, 4;
            ///     2, 0, 7;
            /// );
            /// 
            /// f.step_multit();
            /// f.move_next_generation_to_current();
            /// assert_eq!(0, *f.get(0, 0));
            /// assert_eq!(5, *f.get(1, 0));
            /// assert_eq!(6, *f.get(2, 0));
            /// assert_eq!(2, *f.get(3, 0));
            /// ```
            pub fn step_multit(&mut self)
            {
                let f = &self;
                let bytes = &Arc::new(Mutex::new(self.next.to_owned()));

                thread::scope(|s| {
                    
                    for r in 0..self.rows {
                        for b in 0..self.blocks {

                            s.spawn(move || {
                                let r = r.clone();
                                let b = b.clone();

                                let (mut alive, mut cnt): (bool, i8);

                                let mut new_block: u8 = 0;

                                for bo in 0..8 {
                                    let c = b * 8 + bo;

                                    alive = f.is_alive(r, c);
                                    cnt = f.count_neighbours(r, c) - (alive as i8);

                                    // Game logic ..
                                    if alive {
                                        if cnt == 2 || cnt == 3 {// Living cell has 3 or 4 living neighbours survives
                                            set_bit!(new_block, bo);
                                        }
                                    } else if cnt == 3 {// Dead cell becomes alive if it has exactly 3 living neighbours
                                        set_bit!(new_block, bo);
                                    }
                                }

                                bytes.lock().unwrap()[r as usize][b as usize] = new_block;
                            });
                        }
                    }
                });
                self.next = bytes.lock().unwrap().to_owned();
            }
        }
    }

    ///Read and write from system from- to Field struct.
    pub mod serialize {
        use crate::game_of_life::Field;
        use std::{fs::File, io::{Write, Read}};

        impl Field {

            ///Writes Field struct to a file on the system.
            pub fn serialize(&self, path: String) -> std::io::Result<()>
            {
                let mut file = File::create(path)?;

                file.write_all(&self.rows.to_be_bytes())?;

                let data: u8 = (&self.columns % 8) as u8;// 3 LSB are offset, 5 left for other things
                file.write_all(&[data])?;

                for bl in self.current.iter() {
                    file.write_all(&bl)?;
                }

                Ok(())
            }

            ///Reads a file on the system to load a Field struct.
            pub fn deserialize(path: String) -> std::io::Result<Field>
            {
                let mut file = File::open(path)?;

                let mut buf = [0; 2];
                file.read(&mut buf)?;
                let rows = u16::from_be_bytes(buf);

                let mut buf = [0; 1];
                file.read(&mut buf)?;
                let data = buf[0];
                let offset = data & 7;

                let mut buf: Vec<u8> = Vec::new();
                file.read_to_end(&mut buf)?;

                let columns = ((buf.len() / rows as usize - ((offset > 0) as usize)) * 8 + offset as usize) as u16;

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