#[cfg(test)]
mod tests {
    use conways_game_of_life_dyn_lib::*;
    use conways_game_of_life_dyn_lib::game_of_life_core::chunk_utilities::Direction;

    #[test]
    fn test_mirror_vertical()
    {
        let f = set_field_chunks!(
            0,0,0x4448507048444478;// "R"
        );
        assert_eq!(f.find_chunk(0,0).mirror_vertical().get_data_u64(),0x7844444870504844);
    }

    #[test]
    fn test_mirror_horizontal()
    {
        let f = set_field_chunks!(
            0,0,0x4448507048444478;// "R"
        );
        assert_eq!(f.find_chunk(0,0).mirror_horizontal().get_data_u64(),0x22120a0e1222221e);
    }

    #[test]
    fn test_rotate_clockwise90()
    {
        let f = set_field_chunks!(
            0,0,0x4448507048444478;// "R"
        );
        assert_eq!(f.find_chunk(0,0).rotate(Direction::Clockwise90).get_data_u64(),0x86493111ff00);
    }

    #[test]
    fn test_rotate_anticlockwise90()
    {
        let f = set_field_chunks!(
            0,0,0x4448507048444478;// "R"
        );
        assert_eq!(f.find_chunk(0,0).rotate(Direction::AntiClockwise90).get_data_u64(),0xff888c92610000);
    }

    #[test]
    fn test_rotate_clockwise180()
    {
        let f = set_field_chunks!(
            0,0,0x4448507048444478;// "R"
        );
        assert_eq!(f.find_chunk(0,0).rotate(Direction::Clockwise180).get_data_u64(),0x1e2222120e0a1222);
    }

}