use std::{io::stdin};
use conways_game_of_life_lib_rust::Field;

fn main()
{
    let mut field = Field::new(24,24);

    
    *field.get_at(0,0) = 2;
    *field.get_at(1,0) = 4;
    *field.get_at(2,0) = 7;

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