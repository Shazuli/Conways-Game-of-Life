use std::io::stdin;
use inline_colorization::*;

//use std::io::stdin
//use conways_game_of_life_dyn_lib::{Field, ChunkCellData, set_field_chunks, game_of_life::calculate_rules_classic};
use conways_game_of_life_dyn_lib::*;
//use std::cmp::min;


macro_rules! set_bit {
    ($val:expr, $bit:expr) => {
        $val |= 1<<$bit
    };
}




fn main()
{
    let mut field = set_field_chunks!(
        //0,-1, 0xff7e3c18; // Triangle up
       -1, 0, 0x80c0e0f0f0e0c08; // Triangle right
        0, 0, 0x70402; // Glider left
       -1,-1, 0x1e111009; // Spaceship left
    );

    /*let mut field = set_field_chunks!(
        0,0,0xff818181818181ff;
        -1,0,0xff818181818181ff;
        -1,-1,0xff818181818181ff;
        0,-1,0xff818181818181ff;
    );*/

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

    //field.find_mut_chunk(-1,0).unwrap().mirror_y();
    //field.find_mut_chunk(0,-1).unwrap().mirror_x();

    let mut input: String;
    //let (mut x_view, mut y_view): (Range<i32>, Range<i32>) = (-20..20, -20..20);// Visible area
    let (mut x_low, mut x_high, mut y_low, mut y_high): (i32, i32, i32, i32) = (-20, 20, -20, 20);
    const STEP_SIZE: i32 = 3;

    loop {
        input = String::new();

        //println!("Generation: {gen}\n", gen = field.get_generation());

        draw_field(&field, x_low, x_high, y_low, y_high, true);
        /*unsafe {
            print!("{:#x}", field.find_chunk(0,0).unwrap().data.long);
        }*/
        //_draw(&field, -10, -10, 10, 10);

        //print!("{:?}", field.find_chunk(1,3).unwrap().data);
        

        stdin().read_line(&mut input).expect("Could not understand that");


        match input.trim() {
            "q" | "Q" => {
                break
            }

            "u" | "U" => {
                y_low -= STEP_SIZE;
                y_high -= STEP_SIZE;
            }

            "d" | "D" => {
                y_low += STEP_SIZE;
                y_high += STEP_SIZE;
            }

            "r" | "R" => {
                x_low -= STEP_SIZE;
                x_high -= STEP_SIZE;
            }

            "l" | "L" => {
                x_low += STEP_SIZE;
                x_high += STEP_SIZE;
            }

            "0" => {
                x_low = -20;
                x_high = 20;
                y_low = -20;
                y_high = 20;
            }
            

            _ => {
                field.step_singlet(calculate_rules_classic, 3);
            }
        }

    }

    /*draw(&f, 0, 0);
    f.step_singlet();*/

    /*print!("{f:?}");

    f.step_singlet();

    print!("{f:?}");*/

    
}

fn draw_field(f: &Field, x_low: i32, x_high: i32, y_low: i32, y_high: i32, chunk_colors: bool)
{
    let x_range: i32 = if x_low.is_negative() {
        if x_high.is_negative() {
            x_low.abs() - x_high.abs()
        } else {
            x_low.abs() + x_high
        }
    } else {
        (x_low - x_high.abs()).abs()
    };

    let y_range: usize = if y_low.is_negative() {
        if y_high.is_negative() {
            (y_low.abs() - y_high.abs()) as usize
        } else {
            (y_low.abs() + y_high) as usize
        }
    } else {
        (y_low - y_high.abs()).abs() as usize
    };

    //assert_eq!(x_range, 40);
    //assert_eq!(y_range, 40);

    let blocks: u16 = ((x_range + 1) / 8) as u16 + (((x_range + 1) % 8 > 0) as u16);

    let mut current: Vec<Vec<u8>> = vec![vec![0; blocks.into()]; y_range + 1];

    // Buffer up all the cell states.
    for chunk in f.get_current() {
        if !chunk.all_are_dead() {
            for i in 0i8..64 {
                // Get global cell coordinates for that index.
                let global_x = (chunk.get_x() * 8) as i32 + (i % 8) as i32;
                let global_y = (chunk.get_y() * 8) as i32 + (i / 8) as i32;

                // Check if chunk is visible in that range/window size.
                if (global_x >= x_low && global_x <= x_high) && (global_y >= y_low && global_y <= y_high) {

                    if chunk.is_alive(i).unwrap() {
                        // Convert global coordinates to local.
                        // Feels like there are a ton of edge-cases here, so might have to look at this later.
                        let local_x = if x_low.is_negative() { global_x - x_low } else { global_x - x_low.abs() };
                        let local_y = if y_low.is_negative() { global_y - y_low } else { global_y - y_low.abs() };

                        //println!("{local_x} {local_y}");

                        set_bit!(current[local_y as usize][(local_x / 8) as usize], local_x % 8);
                    }
                }
            }
        }
    }

    // Draw it.
    if f.get_generation() != 0 {
        println!("{esc}[{r}A", esc = 27 as char, r = y_range + 4);
    }
    println!("Generation: {gen}\n", gen = f.get_generation());

    let mut state: &str;

    let x_offset: i32 = x_low + (1<<30);
    let y_offset: i32 = y_low + (1<<30);
    for y in 0..y_range {
        for x in (0..x_range).rev() {// Terminal writes left to right, need to go the other way
            state = if current[y as usize][(x / 8) as usize] & 1<<(x % 8) >= 1 { " X" } else { " ." };

            if chunk_colors && (((x + x_offset) / 8) ^ ((y as i32 + y_offset) / 8) as i32) & 1 >= 1 {// Create checkerboard
                print!("{color_red}{style_bold}{state}");
            } else {
                print!("{color_white}{style_bold}{state}");
            }
        }
        println!("{style_reset}");
    }
}