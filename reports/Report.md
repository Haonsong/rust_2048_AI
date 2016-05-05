# Project Proposal
## 2048 AI
---
### Basic Idea
For this project, I am going to use the **expectminimax** algorithm to handle the random case. Starting with search depth of 4. With Max score and average score of chance case to decide the next stop is up, down, right or lift.


<div class="page-break"></div>

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
    /*
    else if random event at node
        // Return weighted average of all child nodes' values
        let α := 0
        for each child of node
            α := α + (Probability[child] * expectiminimax(child, depth-1))
    */
    return α
```

[reference](https://en.wikipedia.org/wiki/Expectiminimax_tree)
<div class="page-break"></div>
---
### Functions:
- AI move => move the frame
- Minimax helper => get the score of different directions
- Minimax => recursively go through all possible grid
- Scoring => grade the grid


---
### Performance
 - Highest score 150k and largest number 2^13
 - Average 110k and 2^13
 - Depth : 4
 - Slightly optimized

---
### Rules for scoring node:

- difference between cell
- the amount of same cell
- empty row and col
- large on edge
- large number near the maximum cell

<div class="page-break"></div>
