//! Contains function(s) for calculating the next generation and examples for implementing custom rule functions.

use super::{ChunkCellData8x8, CellContext, IS_ALIVE_BIT_MASK};


/// Rule function for calculating cells for the original implementation of CGoL.
/// Calculate cell state at cell_index from result of cell_state.
/// If sign bit is set for cell_state then cell at index is alive, remaining bits are living neighbours.
/// Check source for examples.
pub const fn rule_conways_game_of_life(data: &mut ChunkCellData8x8, cell_index: i8, cell_state: CellContext)
{
    /*
    1. Any live cell with fewer than two live neighbors dies, as if by underpopulation.
    2. Any live cell with two or three live neighbors lives on to the next generation.
    3. Any live cell with more than three live neighbors dies, as if by overpopulation.
    4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
    */

    if cell_state.is_negative() {// Negative == cell at cell_index is alive

        // Mask to get living neighbour count.
        if let 2 | 3 = cell_state & IS_ALIVE_BIT_MASK {// Living cell with 2 or 3 living neighbours survives

            set_state!(data, cell_index);
        }
    } else if cell_state == 3 {// Dead cell becomes alive if it has exactly 3 living neighbours

        set_state!(data, cell_index);
    }
}

/// Rule function for calculating cells for the original implementation of CGoL but branchless.
/// Is slightly faster than the regular one.
pub const fn rule_conways_game_of_life_branchless(data: &mut ChunkCellData8x8, cell_index: i8, cell_state: CellContext)
{
    let is_alive = cell_state.is_negative() as i8;
    let masked = IS_ALIVE_BIT_MASK & cell_state;
    
    set_state!(data, cell_index, (masked == 3) | (masked * is_alive == 2));
}

/// Rule function for calculating cells for Grounded Life.
pub const fn rule_grounded_game_of_life(data: &mut ChunkCellData8x8, cell_index: i8, cell_state: CellContext)
{
    /*
    1. Any live cell with fewer than two live neighbors dies, as if by underpopulation.
    2. Any live cell with two or three live neighbors lives on to the next generation.
    3. Any live cell with more than three live neighbors dies, as if by overpopulation.
    4. Any dead cell with exactly three or five live neighbors becomes a live cell, as if by reproduction.
    */

    if cell_state.is_negative() {

        if let 2 | 3 = cell_state & IS_ALIVE_BIT_MASK {// Living cell with 2 or 3 living neighbours survives

            set_state!(data, cell_index);
        }
    } else if let 3 | 5 = cell_state {// Dead cell becomes alive if it has exactly 3 or 5 living neighbours

        set_state!(data, cell_index);
    }
}