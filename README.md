# SandSim
A sand simulation written in Rust using macroquad
## Build Instructions
Clone the Repo and then run:
```
cargo build
cargo run
```
## Usage Instructions
Click or drag with your mouse to create new sand!

## Algorithm used
The simulation uses a grid of cells that are either filled (by sand) or empty. The algorithm iterates through every cell and if the cell is filled the program goes through some checks.
Checks:

- If cell 1 is empty the program empties the current cell and fills cell 1
- Else if cell 2 is empty the program empties the current cell and fills cell 2
- Else if cell 3 is empty the program empties the current cell and fills cell 3
- Else the current cell stay's filled
