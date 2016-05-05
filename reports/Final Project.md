# Project Proposal
## 2048 AI
---
### Basic Idea
For this project, I am going to use the **expectminimax** algorithm to handle the random case. Starting with search depth of 4. With Max score and average score of chance case to decide the next stop is up, down, right or lift.
- For reading a board:
  - convert the 4x4 board to a 4x4 array of usize
  - shift and merge: find merge-able square in a reversed direction.
    - e.g. moving from down to up which cause go through each column from up to down:
      - if this cell have same number as next cell
        1. this cell * 2
        2. shift up the rest of column
        3. from next cell position, find next merge-able square
      - shifting
        1. from the up of column to down
        2. if next cell is empty => shift the rest of cell
        3. if next cell is not empty => go to next cell





---
### Algorithm
```
function expectiminimax(node, depth)
    if node is a terminal node or depth = 0
        return the value of node
    else if we are to play at node
        // Return value of maximum-valued child node
        let α := -∞
        for each child of node
            α := max(α, expectiminimax(child, depth-1))
    else if random event at node
        // Return weighted average of all child nodes' values
        let α := 0
        for each child of node
            α := α + (Probability[child] * expectiminimax(child, depth-1))
    return α
```

[reference](https://en.wikipedia.org/wiki/Expectiminimax_tree)

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
### Milestone on April 17:
- [ ] read the grid
- [x] applied the dumb AI: Got score around 1000
    - [x] applied the simple AI: Got score from 700 ~ 7000
        - the simple AI use the __grid.score__ as evaluation score. Not stable caused by the randomness.
- [x] give back the action of decision
- [ ] expectminimax algorithm without optimization (abandon branches)
- [ ] adjustable searching deepth
    - 4/17 using depth of 1.
