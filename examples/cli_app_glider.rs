use std::{ops::Range, io::stdin};
use inline_colorization::*;

//use std::io::stdin
use conways_game_of_life_dyn_lib::*;
//use std::cmp::min;


macro_rules! set_bit {
    ($val:expr, $bit:expr) => {
        $val |= 1<<$bit
    };
}

fn main()
{
    let mut field = Field::new();

    /*field.get_mut_current().push(Chunk {x: -1, y: -1, data: ChunkCellData {
        bytes: [
            0,0,
            0b01000,
            0b00100,
            0b11100,
            0,0,0
        ]
    }});*/

    /*field.get_mut_current().push(Chunk {x: 2, y: 2, data: ChunkCellData { long: 0xff818181818181ff }});
    field.get_mut_current().push(Chunk {x: 0, y: 1, data: ChunkCellData { long: 0xff818181818181ff }});
    field.get_mut_current().push(Chunk {x: 1, y: 1, data: ChunkCellData { long: 0xff818181818181ff }});
    field.get_mut_current().push(Chunk {x: -1, y: 1, data: ChunkCellData { long: 0xff818181818181ff }});*/

    /*field.add_chunk(2, 2, ChunkCellData { long: 0xff818181818181ff }).unwrap();
    field.add_chunk(0, 1, ChunkCellData { long: 0xff818181818181ff }).unwrap();
    field.add_chunk(1, 1, ChunkCellData { long: 0xff818181818181ff }).unwrap();
    field.add_chunk(-1, 1, ChunkCellData { long: 0xff818181818181ff }).unwrap();*/

    /*set_field_chunks!(&mut field;
       -1,-1, long: 0xff818181818181ff;
        0,-1, long: 0xff818181818181ff;
       -1, 0, long: 0xff818181818181ff;
        0, 0, long: 0xff818181818181ff
    );*/

    set_field_chunks!(&mut field;
        0, 0, bytes: [0,0,2<<2,4<<2,7<<2,0,0,0]
    );

    /*set_field_chunks!(&mut field;
        2, 2, long: 0xff818181818181ff;
        0, 1, long: 0xff818181818181ff;
        1, 1, long: 0xff818181818181ff;
       -1, 1, long: 0xff818181818181ff
    );

    field.add_chunk(0, 0, ChunkCellData {bytes: [
        0b00000000,
        0b00011000,
        0b00111100,
        0b01111110,
        0b11111111,
        0b00000000,
        0b00000000,
        0b00000000]}).unwrap();*/

    /*set_field_chunk_bytes!(field;
        0, 0,
        0b01000,
        0b00100,
        0b11100;
    );*/

    /*set_field_chunks!(&mut field;
        -1, -1, long: 0x70402 << 2;
        -1,  1, long: 0x70402
        //-1,  2, long: 0xff7e3c1800 // Arrow
    );*/

    /*unsafe {
        let a = ChunkCellData {bytes: [
            0b00000000,
            0b00011000,
            0b00111100,
            0b01111110,
            0b11111111,
            0b00000000,
            0b00000000,
            0b00000000]};
        println!("{:#x}", a.long);
    }*/

    let mut input: String;
    let (mut x_view, mut y_view): (Range<i32>, Range<i32>) = (-20..20, -20..20);// Visible area

    loop {
        input = String::new();

        //println!("Generation: {gen}\n", gen = field.get_generation());

        draw_field(&field, x_view.clone(), y_view.clone());
        //_draw(&field, -10, -10, 10, 10);
        

        stdin().read_line(&mut input).expect("Could not understand that");


        match input.trim() {
            "q" | "Q" => {
                break
            }

            "u" | "U" => {
                y_view.start -= 1;
                y_view.end -= 1;
            }

            "d" | "D" => {
                y_view.start += 1;
                y_view.end += 1;
            }

            "l" | "L" => {
                x_view.start -= 1;
                x_view.end -= 1;
            }

            "r" | "R" => {
                x_view.start += 1;
                x_view.end += 1;
            }

            "0" => {
                x_view.start = -20;
                x_view.end = 20;
                y_view.start = -20;
                y_view.end = 20;
            }
            

            _ => {
                field.step_singlet();
            }
        }

    }

    /*draw(&f, 0, 0);
    f.step_singlet();*/

    /*print!("{f:?}");

    f.step_singlet();

    print!("{f:?}");*/

    
}

fn draw_field<'a>(f: &'a Field, x_range: Range<i32>, y_range: Range<i32>)
{
    let blocks: u16 = ((x_range.len() + 1) / 8) as u16 + (((x_range.len() + 1) % 8 > 0) as u16);

    let mut current: Vec<Vec<u8>> = vec![vec![0; blocks.into()]; y_range.len() + 1];

    // Buffer up all the cells.
    for chunk in f.get_current() {
        if !chunk.all_are_dead() {
            for i in 0i8..64 {
                let global_x = (chunk.x * 8) as i32 + (i % 8) as i32;
                let global_y = (chunk.y * 8) as i32 + (i / 8) as i32;

                if (x_range.contains(&global_x) || x_range.end == global_x) && (y_range.contains(&global_y) || y_range.end == global_y) {

                    if chunk.is_alive(i) {
                        // Convert global coordinates to local.
                        // Feels like there are a ton of edge-cases here, so might have to look at this later.
                        let local_x = if x_range.start.is_negative() { global_x - x_range.start } else { global_x };
                        let local_y = if y_range.start.is_negative() { global_y - y_range.start } else { global_y };

                        //println!("{local_x} {local_y}");

                        set_bit!(current[local_y as usize][(local_x / 8) as usize], local_x % 8);
                    }
                }
            }
        }
    }

    // Draw it.
    println!("{esc}[{r}A", esc = 27 as char, r = y_range.len() + 5);
    println!("Generation: {gen}\n", gen = f.get_generation());
    
    for y in 0..y_range.len() + 1 {
        for x in 0..x_range.len() + 1 {
            let state = if current[y as usize][(x / 8) as usize] & 1<<(x % 8) >= 1 { " X" } else { " ." };

            let is_white: bool = (((x as i32 + x_range.start + (1<<30)) / 8) ^ ((y as i32 + y_range.start + (1<<30)) / 8)) & 1 >= 1;

            if is_white {
                print!("{color_white}{style_bold}{state}");
            } else {
                print!("{color_red}{style_bold}{state}");
            }
        }
        println!("{style_reset}");
    }

    // God, this took fricking months to get right.
    
}

fn _draw_chunk(f: &Field, x: i32, y:i32)
{
    for c in f.get_current().iter() {
        if c.x == x && c.y == y {
            for i in 0..64 {
                if c.is_alive(i) {
                    print!("X ");
                } else {
                    print!(". ");
                }

                if i % 8 == 7 {
                    println!();
                }
            }
        }
    }
}