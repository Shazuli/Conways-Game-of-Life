use std::{io::stdin};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use conways_game_of_life_lib_rust::Field;

fn main()
{
    let mut field = Field::new(25,25);

    set_random_field(&mut field, 12345678);


    loop {
        let mut input = String::new();

        draw(&field);
        field.step_singlet();
        field.move_next_to_current();

        stdin().read_line(&mut input).expect("Could not understand that");

        match input.trim() {

            "s" | "S" | "save" => {
                field.serialize(String::from("save")).unwrap();
            }

            "q" | "Q" | "quit" => {
                break;
            }

            _ => {
                
            }
        }
        println!("{esc}[{r}A",esc = 27 as char,r = field.columns+2);
    }
}

fn draw(f: &Field)
{
    for c in 0..f.columns {
        for r in 0..f.rows {
            if f.is_alive(r,c) {
                print!("{} ", 'X')
            } else {
                print!("{} ", '.')
            }
        }
        println!();
    }
}

fn set_random_field(f: &mut Field, seed: u64)
{
    let mut rng = Pcg32::seed_from_u64(seed);
    for r in 0..f.rows {
        for b in 0..f.blocks {
            *f.get_at(r, b) = rng.gen_range(0..=255);
        }
    }
}