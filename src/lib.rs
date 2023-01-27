//! # Conway's Game of Life
//! 
//! Core functionality for Conway's Game of Life.
//! Includes data type and implementation for manipulating it.
//! Also includes methods for serialising data.

macro_rules! loop_around {
    ($val:expr,$biggest:expr) => {
        {
            if $val >= $biggest {
                $val-$biggest
            } else {
                $val
            }
        }
    }
}

pub use self::game_of_life::Field;
pub use self::game_of_life::core;
pub use self::game_of_life::serialize;

pub mod game_of_life
{
    #[derive(Debug)]
    pub struct Field {
        pub rows: u16,
        pub columns: u16,
        blocks: u16,
        current: Vec<Vec<u8>>,
        next: Vec<Vec<u8>>
    }

    #[doc = "Core functions for handling Field struct."]
    pub mod core
    {
        use std::thread;
        use crate::game_of_life::Field;
        impl Field {

            /// Creates a new Field struct and returns it.
            /// 
            /// # Examples
            /// ```
            /// use Conways_game_of_life_rust::Field;
            /// 
            /// let mut f = Field::new(8,8);
            /// *field.get(0,0) = 2;
            /// *field.get(1,0) = 4;
            /// *field.get(2,0) = 7;
            /// ```
            pub fn new(rows: u16, columns: u16) -> Field
            {
                if rows == 0 || columns == 0 {
                    panic!("Size must be larger than 0");
                }

                let blocks: u16 = rows / 8 + ((columns % 8 > 0) as u16);// Counts how many bytes are required for each line

                let f = Field {
                    rows,
                    columns,
                    blocks,
                    current: vec![vec![0; blocks.into()]; rows.into()],
                    next: vec![vec![0; blocks.into()]; rows.into()]
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
                let (r, b) = (loop_around!(row,self.rows), loop_around!(block,self.blocks));
                &mut self.current[r as usize][b as usize]
            }

            #[doc = "Returns whenever cell at locatin is alive."]
            pub fn is_alive(&self, row: u16, column: u16) -> bool
            {
                let (r, c) = (loop_around!(row,self.rows), loop_around!(column,self.columns));
                self.current[r as usize][(c / 8) as usize] & 1<<(c % 8) >= 1
            }

            #[doc = "Sets cell at coordinate to alive."]
            fn set_alive(&mut self, row: u16, column: u16)
            {
                let (r, c) = (loop_around!(row,self.rows), loop_around!(column,self.columns));
                self.next[r as usize][(c / 8) as usize] |= 1<<(c % 8);
            }

            #[doc = "Sets cell at coordinate to dead."]
            fn set_dead(&mut self, row: u16, column: u16)
            {
                let (r, c) = (loop_around!(row,self.rows), loop_around!(column,self.columns));
                self.next[r as usize][(c / 8) as usize] &= !(1<<(c % 8));
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
                        cnt = -(alive as i8);// We don't want to count the middle cell

                        // Count 9x9. Gotta be a better way to do this.
                        if r == 0 {
                            for ro in r..r+2 {
                                if c == 0 {
                                    for co in c..c+2 {
                                        cnt += self.is_alive(ro, co) as i8;
                                    }
                                } else {
                                    for co in c-1..c+2 {
                                        cnt += self.is_alive(ro, co) as i8;
                                    }
                                }
                            }
                        } else {
                            for ro in r-1..r+2 {
                                if c == 0 {
                                    for co in c..c+2 {
                                        cnt += self.is_alive(ro, co) as i8;
                                    }
                                } else {
                                    for co in c-1..c+2 {
                                        cnt += self.is_alive(ro, co) as i8;
                                    }
                                }
                            }
                        }

                        // Game logic ..
                        if alive {
                            if cnt == 2 || cnt == 3 {// Living cell has 3 or 4 living neighbours survives
                                self.set_alive(r, c);
                            } else {
                                self.set_dead(r, c);
                            }
                        } else {
                            if cnt == 3 {// Dead cell becomes alive if it has exactly 3 living neighbours
                                self.set_alive(r, c);
                            } else {
                                self.set_dead(r, c);
                            }
                        }
                    }
                }
            }

            /// Step the simulation once in multiple threads and stores the result in memory.
            /// 
            pub fn step_multit(&mut self)
            {
                //let pools = ThreadPool::new(2);
                for r in 0..self.rows {
                    for b in 0..self.blocks {
                        thread::spawn(|| {

                        });

                    }
                }
            }
        }
    }

    #[doc = "Read and write from system from- to Field struct."]
    pub mod serialize {
        use crate::game_of_life::Field;
        use std::{fs::File, io, io::{Write, Read}};

        impl Field {

            #[doc = "Writes Field struct to a file on the system."]
            pub fn serialize(&self, path: &str) -> std::io::Result<()>
            {

                let mut file = File::create(path)?;



                Ok(())
                //let mut f = File::open(path)?;

                //let mut buf = String::new();
                //f.read_to_string(&mut buf)?;



                /*let f = match f {
                    Ok(file) => file,
                    Err(error) => match error.kind() {
                        ErrorKind::NotFound => match File::create(path) {
                            Ok(fc) => fc,
                            Err(e) => panic!("Problem creating file {:?}", e),
                        },
                        other_error => {
                            panic!("Problem opening file {:?}", other_error)
                        }
                    }
                };*/

            }

            #[doc = "Reads a file on the system to a Field struct."]
            pub fn deserialize(&mut self, path: String)
            {

            }
        }
    }
}