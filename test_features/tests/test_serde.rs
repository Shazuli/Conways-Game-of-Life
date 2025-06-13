#[cfg(test)]
mod tests
{
    use conways_game_of_life_dyn_lib::*;

    #[test]
    fn test_serde_json()
    {
        let f = set_field!(
            0,0,0x4448507048444478;// "R"
           -2,4,0x1e111009;// Spaceship to left
        );

        let serialized = serde_json::to_string(&f).unwrap();
        
        let mut f_from_json: Field = serde_json::from_str(&serialized).unwrap();
        assert_eq!(f, f_from_json);

        set_field!(f_from_json =>
            1,1,0x70402;
        );
        assert_ne!(f, f_from_json);

        let mut f_from_json: Field = serde_json::from_str(&serialized).unwrap();

        assert_eq!(f, f_from_json);

        f_from_json.find_mut_chunk(0,0).unwrap().set_all_dead();
        assert_ne!(f, f_from_json);
    }

    #[ignore = "todo"]
    #[test]
    fn test_serde_postcard()
    {
    }

}