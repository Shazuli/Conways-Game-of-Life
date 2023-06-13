#[cfg(test)]
mod tests
{
    use conways_game_of_life_lib_rust::*;
    use rand::{Rng, SeedableRng};
    use rand_pcg::Pcg32;


    const ROWS: u16 = 500;
    const COLUMNS: u16 = 500;
    const SEED: u64 = 65535;
    const PATH: &str = "test_serialize";


    #[test]
    fn test_serialize()
    {
        let mut f1 = Field::new(ROWS, COLUMNS);

        set_random_field(&mut f1, SEED);

        f1.serialize(String::from(PATH)).unwrap();

        let mut f2 = Field::deserialize(String::from(PATH)).unwrap();

        assert_eq!(f1.get_rows(), f2.get_rows(), "Rows didn't match");
        assert_eq!(f1.get_columns(), f2.get_columns(), "Columns didn't match");
        for r in 0..ROWS {
            for b in 0..COLUMNS/8 {
                assert_eq!(*f1.get_at(r, b), *f2.get_at(r, b), "Did not match at r={} b={}", r, b);
            }
        }


    }

    fn set_random_field(f: &mut Field, seed: u64)
    {
        let mut rng = Pcg32::seed_from_u64(seed);
        for r in 0..f.get_rows() {
            for b in 0..f.get_blocks() {
                *f.get_at(r, b) = rng.gen_range(0..=255);
            }
        }
    }
}