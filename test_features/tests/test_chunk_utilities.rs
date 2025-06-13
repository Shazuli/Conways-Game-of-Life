#[cfg(test)]
mod tests {
    use conways_game_of_life_dyn_lib::*;
    use conways_game_of_life_dyn_lib::chunk_utilities::Rotation as R;

    macro_rules! into_u64 {
        ($data: expr) => {{
            let tmp:u64=$data.into();tmp
        }};
    }


    #[test]
    fn test_mirror_vertical()
    {
        let f = set_field!(
            0,0,0x4448507048444478;// "R"
            //0,0,0x0008507048444478;// "R"
            //0,0,0x4440000000000000;// "R"
            //2,2,0x56565;
        );
        assert_eq!(into_u64!(f.find_chunk(0,0).mirror_vertical().data),0x7844444870504844);
    }

    #[test]
    fn test_mirror_horizontal()
    {
        let f = set_field!(
            0,0,0x4448507048444478;// "R"
        );
        assert_eq!(into_u64!(f.find_chunk(0,0).mirror_horizontal().data),0x22120a0e1222221e);
    }

    #[test]
    fn test_rotate_clockwise90()
    {
        let f = set_field!(
            0,0,0x4448507048444478;// "R"
        );
        assert_eq!(into_u64!(f.find_chunk(0,0).rotate(R::ClockwiseDegrees(90)).data),0x86493111ff00);
    }

    #[test]
    fn test_rotate_anticlockwise90()
    {
        let f = set_field!(
            0,0,0x4448507048444478;// "R"
        );
        assert_eq!(into_u64!(f.find_chunk(0,0).rotate(R::AntiClockwiseDegrees(90)).data),0xff888c92610000);
    }

    #[test]
    fn test_rotate_clockwise180()
    {
        let f = set_field!(
            0,0,0x4448507048444478;// "R"
        );
        assert_eq!(into_u64!(f.find_chunk(0,0).rotate(R::ClockwiseDegrees(180)).data),0x1e2222120e0a1222);
    }

}