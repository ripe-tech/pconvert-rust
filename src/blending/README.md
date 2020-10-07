# Blending module

Source files:

- [`mod.rs`](./mod.rs)
    - enum of available blending algorithms
    - abstract blending algorithm function
    - (de)multiply functions
    - other utility functions

- [`algorithms.rs`](./algorithms.rs)
    - concrete implementation of several blending algorithms (all inlined when possible)

- [`params.rs`](./params.rs)
    - definition of a generic structure for blending algorithm extra parameters
