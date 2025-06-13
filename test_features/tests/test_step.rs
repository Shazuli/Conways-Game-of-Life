#[cfg(test)]
mod tests
{
    use conways_game_of_life_dyn_lib::*;
    use conways_game_of_life_dyn_lib::rule_fns::rule_conways_game_of_life as rule;
    use conways_game_of_life_dyn_lib::rule_fns::rule_conways_game_of_life_branchless as rule_branchless;

    macro_rules! into_u64 {
        ($data: expr) => {{
            let tmp: u64 = $data.into(); tmp
        }};
    }

    //Field::step(&mut self, rule, threshold);
    //Field::step_multit(&mut self, rule, threshold, max_threads, prefered_chunks_per_thread);

    macro_rules! test_glider {
        ($rule:ident, $step_fn:ident) => {
            let mut f = Field::new();
            unsafe { f.unchecked_add_new_chunk(0,0,ChunkCellData8x8::from(0x70402)); }

            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x70402);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x2060500);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x6050400);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x60c0200);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0xe080400);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x40c0a0000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0xc0a080000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0xc18040000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x1c10080000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x81814000000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x181410000000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x183008000000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x382010000000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x10302800000000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x30282000000000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x30601000000000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x70402000000000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x2060500000000000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x6050400000000000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x60c0200000000000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0xe080400000000000);
            assert_eq!(into_u64!(f.find_chunk(0,1).data),0x0);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0xc0a0000000000000);
            assert_eq!(into_u64!(f.find_chunk(0,1).data),0x40);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0xa080000000000000);
            assert_eq!(into_u64!(f.find_chunk(0,1).data),0xc0);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x0);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x8040000000000000);
            assert_eq!(into_u64!(f.find_chunk(0,1).data),0xc0);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x100000000000000);
            assert_eq!(into_u64!(f.find_chunk(1,1).data),0x0);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x80000000000000);
            assert_eq!(into_u64!(f.find_chunk(0,1).data),0xc0);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x100000000000000);
            assert_eq!(into_u64!(f.find_chunk(1,1).data),0x1);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,1).data),0x8080);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x100000000000000);
            assert_eq!(into_u64!(f.find_chunk(1,1).data),0x1);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,1).data),0x8040);
            assert_eq!(into_u64!(f.find_chunk(1,1).data),0x101);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x0);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,1).data),0x8000);
            assert_eq!(into_u64!(f.find_chunk(1,1).data),0x103);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x0);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,1).data),0x8000);
            assert_eq!(into_u64!(f.find_chunk(1,1).data),0x302);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,1).data),0x80);
            assert_eq!(into_u64!(f.find_chunk(1,1).data),0x10302);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,1).data),0x30202);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,1).data),0x30601);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,1).data),0x70402);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,1).data),0x2060500);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,1).data),0x6050400);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,1).data),0x60c0200);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,1).data),0xe080400);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,1).data),0x40c0a0000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,1).data),0xc0a080000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,1).data),0xc18040000);
        }
    }

    macro_rules! test_spaceship {
        ($rule:ident, $step_fn:ident) => {
            let mut f = Field::new();
            unsafe { f.unchecked_add_new_chunk(0,0,ChunkCellData8x8::from(0x1e111009)); }
    
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x1e111009);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0xc1e361800);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x1220223c00);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x306c3c18);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x78444024);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x3078d86000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x488088f000);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x0);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0xc0b0f060);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x10000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0xe0100090);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x1010100);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0xc0e0608000);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x1030100);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x200020c000);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x102020300);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0xc0c080);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x3060301);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x80400040);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x7040402);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(0,0).data),0x80800000);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x3070d0600);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x408080f00);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0xc1b0f06);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x1e111009);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0xc1e361800);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x1220223c00);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x306c3c18);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x78444024);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x3078d86000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x488088f000);
            assert_eq!(into_u64!(f.find_chunk(2,0).data),0x0);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0xc0b0f060);
            assert_eq!(into_u64!(f.find_chunk(2,0).data),0x10000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0xe0100090);
            assert_eq!(into_u64!(f.find_chunk(2,0).data),0x1010100);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0xc0e0608000);
            assert_eq!(into_u64!(f.find_chunk(2,0).data),0x1030100);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x200020c000);
            assert_eq!(into_u64!(f.find_chunk(2,0).data),0x102020300);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0xc0c080);
            assert_eq!(into_u64!(f.find_chunk(2,0).data),0x3060301);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x80400040);
            assert_eq!(into_u64!(f.find_chunk(2,0).data),0x7040402);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(1,0).data),0x80800000);
            assert_eq!(into_u64!(f.find_chunk(2,0).data),0x3070d0600);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(2,0).data),0x408080f00);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(2,0).data),0xc1b0f06);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(2,0).data),0x1e111009);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(2,0).data),0xc1e361800);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(2,0).data),0x1220223c00);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(2,0).data),0x306c3c18);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(2,0).data),0x78444024);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(2,0).data),0x3078d86000);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(2,0).data),0x488088f000);
            assert_eq!(into_u64!(f.find_chunk(3,0).data),0x0);
            f.$step_fn($rule,3);
            assert_eq!(into_u64!(f.find_chunk(2,0).data),0xc0b0f060);
            assert_eq!(into_u64!(f.find_chunk(3,0).data),0x10000);
        }
    }

    #[test]
    fn test_step_glider()
    {
        test_glider!(rule, step);
    }

    #[test]
    fn test_step_glider_branchless()
    {
        test_glider!(rule_branchless, step);
    }

    #[test]
    fn test_step_spaceship()
    {
        test_spaceship!(rule, step);
    }

    #[test]
    fn test_step_spaceship_branchless()
    {
        test_spaceship!(rule_branchless, step);
    }

    #[ignore = "todo"]
    #[test]
    fn test_step_multit_glider()
    {
    }

    #[ignore = "todo"]
    #[test]
    fn test_step_multit_glider_branchless()
    {
    }

    #[ignore = "todo"]
    #[test]
    fn test_step_multit_spaceship()
    {
    }

    #[ignore = "todo"]
    #[test]
    fn test_step_multit_spaceship_branchless()
    {
    }

}