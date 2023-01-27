#[cfg(test)]
mod tests
{
    use conways_game_of_life_rust::*;
    #[test]
    fn test_glider_top_left()
    {
        let mut f = Field::new(8, 8);
        *f.get_at(0,0) = 2;
        *f.get_at(1,0) = 4;
        *f.get_at(2,0) = 7;

        f.step_singlet();
        f.move_next_to_current();
        assert_eq!(0, *f.get_at(0,0));
        assert_eq!(5, *f.get_at(1,0));
        assert_eq!(6, *f.get_at(2,0));
        assert_eq!(2, *f.get_at(3,0));

        f.step_singlet();
        f.move_next_to_current();
        assert_eq!(4, *f.get_at(1,0));
        assert_eq!(5, *f.get_at(2,0));
        assert_eq!(6, *f.get_at(3,0));
    }
}