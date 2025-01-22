#[cfg(test)]
mod tests {
    use conways_game_of_life_dyn_lib::{Field, set_field_chunks, game_of_life::calculate_rules_classic};

    #[test]
    fn test_step_glider()
    {
        let mut f = set_field_chunks!(
            0,0,0x70402;// Glider to left down
        );

        assert_eq!(f.find_chunk(0,0).get_data_long(), 0x70402);
        f.step_singlet(calculate_rules_classic, 3);
    }
    #[test]
    fn test_step_spaceship()
    {
        let mut f = set_field_chunks!(
            0,0,0x1e111009;// Spaceship to left
        );

        assert_eq!(f.find_chunk(0,0).get_data_long(), 0x1e111009);
        f.step_singlet(calculate_rules_classic, 3);
    }
}