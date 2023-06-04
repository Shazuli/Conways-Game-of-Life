use std::io::stdin;
use conways_game_of_life_lib_rust::{Field, set_field};

fn main()
{
    let mut field = Field::new(24,24);
    set_field!(&mut field;
        0, 0, 2;  10, 0, 2;
        1, 0, 4;  11, 0, 4;
        2, 0, 7;  12, 0, 7;

        0, 1, 2<<5;
        1, 1, 4<<5;
        2, 1, 7<<5;
    );

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
        println!("{esc}[{r}A",esc = 27 as char,r = field.get_columns()+2);
    }
}

fn draw(f: &Field)
{
    for c in 0..f.get_columns() {
        for r in 0..f.get_rows() {
            if f.is_alive(r,c) {
                print!("{} ", 'X');
            } else {
                print!("{} ", '.');
            }
        }
        println!();
    }
}