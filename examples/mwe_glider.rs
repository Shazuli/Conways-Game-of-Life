use conways_game_of_life_dyn_lib::{ChunkCellData8x8, Field, rule_fns::rule_conways_game_of_life as rule};
use std::io::stdin;


fn main()
{
    let mut f = Field::new();
    f.add_new_chunk(0, 0, ChunkCellData8x8::from(0x70402));// 'Glider' pattern

    let mut input: String;

    loop {
        
        print_field(&f, -20, 20, -20, 20);

        input = String::new();
        stdin().read_line(&mut input)
            .expect("Something went wrong");

        match input.trim() {
            "q" | "Q" => {
                break;
            }

            _ => {
                f.step(rule, 3);
            }
        }
    }

}


/// Prints `Field` into the terminal.
fn print_field(f: &Field, x_low: i32, x_high: i32, y_low: i32, y_high: i32)
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

    let blocks: u16 = ((x_range + 1) / 8) as u16 + (((x_range + 1) % 8 > 0) as u16);

    let mut current: Vec<Vec<u8>> = vec![vec![0; blocks.into()]; y_range + 1];

    // Buffer up all the cell states.
    for chunk in f.get_current_slice() {
        if !chunk.all_are_dead() {
            for i in 0i8..64 {
                // Get global cell coordinates for that index.
                let global_x = (chunk.get_x() * 8) as i32 + (i % 8) as i32;
                let global_y = (chunk.get_y() * 8) as i32 + (i / 8) as i32;

                // Check if chunk is visible in that range/window size.
                if (global_x >= x_low && global_x <= x_high) && (global_y >= y_low && global_y <= y_high) {

                    if chunk.get_cell_state(i).unwrap() {
                        // Convert global coordinates to local.
                        let local_x = if x_low.is_negative() { global_x - x_low } else { global_x - x_low.abs() };
                        let local_y = if y_low.is_negative() { global_y - y_low } else { global_y - y_low.abs() };

                        // Set that bit.
                        current[local_y as usize][(local_x / 8) as usize] |= 1<<(local_x % 8);
                    }
                }
            }
        }
    }

    println!("Generation: {gen}\n", gen = f.get_generation());

    let mut state: &str;

    for y in 0..y_range {
        for x in (0..x_range).rev() {// Terminal writes left to right, need to go the other way
            state = if current[y as usize][(x / 8) as usize] & 1<<(x % 8) >= 1 { " X" } else { " ." };

            print!("{state}");
        }
        println!();
    }
}