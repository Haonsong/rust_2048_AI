# Intermediate Report
## 2048 AI
---
### Attribution
I am doing the 2048 AI as single project. And I did research on the algorithm, look through the code provided and got a simple AI working(based on free cell, depth one, and without considering the randomness.)
I am still going to use the **expectminimax** to - handle the randomness.

 For reading a board:
  - ~~convert the 4x4 board to a 4x4 array of usize~~
  - Use the Struct provided, Grid directly;
    - able to get the value of cell by this way
    ````Rust
    match * vec_grid[0][1]{
      None=>"None",
      Some(ref cell) => cell.unwrap(),
    }
    ````
  - ~~shift and merge: find merge-able square in a reversed direction.~~
    - The Grid has function to take action with out random cell generated.
    - ~~e.g. moving from down to up which cause go through each column from up to down:
      - ~~if this cell have same number as next cell~~
        1. ~~this cell * 2~~
        2. ~~shift up the rest of column~~
        3. ~~from next cell position, find next merge-able square~~
      - shifting
        1.  ~~from the up of column to down~~
        2. ~~if next cell is empty => shift the rest of cell~~
        3. ~~if next cell is not empty => go to next cell~~
        1. NEW: using up,down, left and right function of Grid.

---
### Rules for scoring node:

- the next steps in tree will be the only one without a random case
- scoring a node:
  - sum of number on square that merged (one of two square merged)
  - number of empty squares * largest number
- scoring a node with randomness:
  - sum of children
  - children score = Probability of reaching the node * score of board
- Choose the highest branch as next step
---
### Milestone on April 21:
- [x] read the grid
  - [ ] unwrap the cell and use it to do complex algorithm
- [x] applied the dumb AI: Got score around 1000
    - [x] applied the simple AI: Got score from 700 ~ 7000
        - the simple AI use the __empty_cell__ as evaluation score. Not stable caused by the randomness.
- [x] give back the action of decision
- [ ] expectminimax algorithm without optimization (abandon branches)
- [ ] adjustable searching deepth
    - 4/17 using depth of 1.
