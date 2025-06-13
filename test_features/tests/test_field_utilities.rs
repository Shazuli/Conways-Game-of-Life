#[cfg(test)]
mod tests {
    use conways_game_of_life_dyn_lib::*;

    // Checkerboard pattern of cell states.
    const CHECKERBOARD_PATTERN: u64 = 0x55aa55aa55aa55aa;

    macro_rules! create_chunk_vec {
        ($($x:expr,$y:expr),+) => {{
            let mut cs: Vec<Chunk> = Vec::new();
            $(
                cs.push(Chunk::new($x,$y,ChunkCellData8x8::from(0x12345)));
            )?
            cs
        }};
    }

    macro_rules! assert_chunk_data {
        ($f:ident, $x:expr, $y:expr, $data_expected:expr) => {
            assert_eq!(unsafe { $f.find_chunk($x,$y).data.u64 }, $data_expected, "chunk pos: [{},{}]", $x, $y);
        }
    }

    #[test]
    fn test_dedup_merge_chunks()
    {
        let mut chunks_ref = create_chunk_vec!(1,2, 6,4, 8,4, 1,3);
        chunks_ref.sort_unstable();

        let mut chunks = create_chunk_vec!(1,2, 8,4, 6,4, 8,4, 1,2, 1,3, 1,2);
        chunks.sort_unstable();
        field_utilities::dedup_merge_chunks(&mut chunks);
        chunks.sort_unstable();

        assert_eq!(chunks, chunks_ref);
    }

    #[test]
    fn test_set_cell_state()
    {
        let mut f = Field::new();

        assert!(f.set_cell_state(0, 0, true));
        assert_chunk_data!(f, 0, 0, 0x1);

        assert!(f.set_cell_state(7, 7, true));
        assert_chunk_data!(f, 0, 0, 0x8000000000000001);
        assert!(!f.set_cell_state(7, 7, true));
        assert_chunk_data!(f, 0, 0, 0x8000000000000001);

        assert!(f.set_cell_state(0, 0, false));
        assert_chunk_data!(f, 0, 0, 0x8000000000000000);
        assert!(!f.set_cell_state(0, 0, false));
        assert_chunk_data!(f, 0, 0, 0x8000000000000000);

        assert!(f.set_cell_state(8, 8, true));
        assert_chunk_data!(f, 1, 1, 0x1);

        assert!(f.set_cell_state(-1, -1, true));
        assert_chunk_data!(f, -1, -1, 0x8000000000000000);

        assert!(f.set_cell_state(-8, -8, true));
        assert_chunk_data!(f, -1, -1, 0x8000000000000001);

        assert!(f.set_cell_state(-16, -8, true));
        assert_chunk_data!(f, -2, -1, 0x1);

        // Reset...
        f.clear(1);

        let (
            x_min_chunk, x_max_chunk,
            y_min_chunk, y_max_chunk
        ) = (-2, 2, -2, 2);

        // Create checkerboard.
        for x in x_min_chunk * 8..(x_max_chunk+1) * 8 {
            for y in y_min_chunk * 8..(y_max_chunk+1) * 8 {
                if (x ^ y) & 1 != 0 {
                    // Set to alive. Chunk data should change.
                    assert!(f.set_cell_state(x, y, true), "pos: [{x},{y}]");
                } else {
                    // Set to dead. Chunk data shouldn't change.
                    assert!(!f.set_cell_state(x, y, false), "pos: [{x},{y}]");
                }
            }
        }

        // Check if all the chunks has the expected chunk data.
        for x in x_min_chunk..=x_max_chunk {
            for y in y_min_chunk..=y_max_chunk {
                assert_chunk_data!(f, x, y, CHECKERBOARD_PATTERN);
            }
        }
    }

    #[test]
    fn test_get_cell_state()
    {
        let mut f = Field::new();

        assert!(!f.get_cell_state(-3, 5));
        assert!(!f.get_cell_state(-67, -1));
        assert!(!f.get_cell_state(5, 5));
        assert!(!f.get_cell_state(-33, -8));

        let (
            x_min_chunk, x_max_chunk,
            y_min_chunk, y_max_chunk
        ) = (-2, 2, -2, 2);

        // Create checkerboard of cell states.
        {
            let pat = ChunkCellData8x8::from(CHECKERBOARD_PATTERN);

            for x in x_min_chunk..=x_max_chunk {
                for y in y_min_chunk..=y_max_chunk {
                    unsafe { f.unchecked_add_new_chunk(x, y, pat); }
                }
            }
        }

        // Check that all the cell states are what's expected.
        for x in x_min_chunk * 8..(x_max_chunk+1) * 8 {
            for y in y_min_chunk * 8..(y_max_chunk+1) * 8 {
                let is_alive = (x ^ y) & 1 != 0;
                assert_eq!(f.get_cell_state(x, y), is_alive, "pos: [{x},{y}]");
            }
        }
    }
}