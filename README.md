# Conway's Game of Life

(Conway's) Game of Life implemented in Rust as a library. (W.I.P)

This is the successor to the previous library that was more limited in functionality and size.

The field is divided into 8x8 chunks with its own coordinates and data and de-allocates when there are no living cells in it.
Can specify what rule for determin how cells change from generation to generation, with a default implementation for Conway's Game of Life's rules pre-defined.

This version have a dynamic field size limited by the available memory of the host.