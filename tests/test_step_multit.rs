#[cfg(test)]
mod tests
{
    use rand::{Rng, SeedableRng};
    use rand_pcg::Pcg32;
    use conways_game_of_life_lib_rust::*;

    const ROWS: u16 = 24;
    const COLUMNS: u16 = 24;
    const SEED: u64 = 65535;

    const STEPS: u32 = 16;

    #[test]
    fn test_step_multit()
    {
        let mut f_ref = Field::new(ROWS, COLUMNS);
        let mut f = Field::new(ROWS, COLUMNS);

        set_random_field(&mut f_ref, SEED);
        set_random_field(&mut f, SEED);

        for r in 0..ROWS {
            for b in 0..COLUMNS/8 {
                assert_eq!(*f.get_at(r, b), *f_ref.get_at(r, b), "Did not match at the beginning");
            }
        }

        for i in 0..STEPS {
            f_ref.step_singlet();
            f_ref.move_next_to_current();
            f.step_multit();
            f.move_next_to_current();

            for r in 0..ROWS {
                for b in 0..COLUMNS/8 {
                    assert_eq!(*f.get_at(r, b), *f_ref.get_at(r, b), "Did not match at r={} b={} in i={}",r,b,i);
                }
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