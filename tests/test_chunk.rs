#[cfg(test)]
mod tests {
    use conways_game_of_life_dyn_lib::*;

    
    #[test]
    fn test_new_chunk()
    {
        let c = Chunk::new(10, -20, ChunkCellData8x8::from(0x12345));

        {
            let num_bytes = std::mem::size_of::<Chunk>();
            assert_eq!(num_bytes, 16, "`Chunk` is {} bytes! Has it changed?", num_bytes);
        }

        assert_eq!(c.get_x(), 10);
        assert_eq!(c.get_y(), -20);
        assert_eq!(unsafe { c.data.u64 }, 0x12345);
    }

    #[test]
    fn test_all_are_dead()
    {
        let c = Chunk::new(0, 0, ChunkCellData8x8::from(0));
        assert!(c.all_are_dead());
    }

    #[test]
    fn test_set_cell_state()
    {
        let mut c = Chunk::new(0, 0, ChunkCellData8x8::from(0));

        assert!(c.all_are_dead());

        let (x1, y1): (i8, i8) = (4, 3);
        assert_eq!(c.set_cell_state((x1 + y1 * 8) | IS_ALIVE_BIT), Some(true));
        assert!(!c.all_are_dead());
        assert_eq!(c.set_cell_state((x1 + y1 * 8) | IS_ALIVE_BIT), Some(false));
 
        assert_eq!(unsafe { c.data.u64 }, 0x10000000);

        let (x2, y2): (i8, i8) = (5, 4);
        assert_eq!(c.set_cell_state((x2 + y2 * 8) | IS_ALIVE_BIT), Some(true));

        assert_eq!(unsafe { c.data.u64 }, 0x2010000000);

        assert_eq!(c.set_cell_state(100), None);

        assert_eq!(c.set_cell_state(x1 + y1 * 8), Some(true));
        assert!(!c.all_are_dead());
        assert_eq!(c.set_cell_state(x2 + y2 * 8), Some(true));
        assert!(c.all_are_dead());
        assert_eq!(c.set_cell_state(x2 + y2 * 8), Some(false));
    }

    #[test]
    fn test_get_cell_state()
    {
        let c = Chunk::new(1, -2, ChunkCellData8x8::from(0x4448507048444478));

        let (x, y): (i8, i8) = (2, 1);
        assert_eq!(c.get_cell_state(x + y * 8), Some(true));

        let (x, y): (i8, i8) = (5, 1);
        assert_eq!(c.get_cell_state(x + y * 8), Some(false));

        let (x, y): (i8, i8) = (5, 9);
        assert_eq!(c.get_cell_state(x + y * 8), None);
    }

}