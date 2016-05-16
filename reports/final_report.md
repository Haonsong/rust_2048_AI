* Haonan Song

Artifact:
> * git repository: <https://github.com/Haonsong/rust_2048_AI>

> * All project files are uploaded to github

## Final milestone

> Final milestone given in Intermediate report is get 2048 most time.

> - Using Multi-thread to accelerate the searching speed.
> - What I have got is get 2^14 = 16384 most time and gain score over 20K ~ 30K in 5 minutes with searching depth of 6. (Cheating AI)
> - Get the 2048 for most time as a non-cheating AI,__**VERY**__ slow, may need 30 minutes.

## Accomplishments

> Genericity
* The gird in Vec < Vec < Option< Cell > > >
  * Vec for Vec
  * Vec for Option
  * Option for Cell

> First-class functions
* The `randloop` function is passing the `ai_move` function as a parameter.

> Algorithms
  * minimax for cheating AI
  ```
  function minimax(node, depth)
      if node is a terminal node or depth = 0
          return the value of node
      else
          // Return value of maximum-valued child node
          let α := -∞
          for each child of node
              α := max(α, minimax(child, depth-1))
      return α
  ```
  * Expectminimax for non-cheating AI
  ```
  function expectiminimax(node, depth)
      if node is a terminal node or depth = 0
          return the value of node
      else
        if not random turn
          // Return value of maximum-valued child node
          let α := -∞
          for each child of node
              α := max(α, expectiminimax(child, depth-1))
        else
              // Return weighted average of all child nodes' values
              let α := 0
              for each child of node
                  α := α + (Probability[child] * expectiminimax(child, depth-1))
      return α
  ```

> Recursive functions
  * minimax function will Recursively call itself to match the depth as given.(as shown above)


> Multi-thread:
  * The AI is using Multi-thread to speed up
  * There are four threads for four direction separately

> Optimization of searching:
  * Test the result of the grid move (`Evolution`) before exploring the branch
  * Give up the branch if the result is `Evolution::Nothing`

## Work in Progress

> All features is finished for both cheating and non-cheating.

## Technical Challenges

> Since the Windows command line is not working at first, so I decided to move to linux (Arch Linux).

> - Get the linux installed on real machine(Really hard).
  - Install drivers
  - Install DM and WM


> - Install & run the rustc & cargo on Linux machine(Pretty easy).
  - install the developer tools, like gcc
  - make the installable package from rust source code
  - In my case, I use the `pacman` to install the package

## Challenges

> The ownership of the grid, vec< vec < option < Cell > > >, are very hard to be tracked. For example in `next_movement` function, which is the helper for minimax function, I got two types errors: moved value and the ownership which is not long enough. Solving these two problems by pass the cloned object to other function rather than the original reference to the object.

## Group work breakdown

> Single person Project.

## Building / Running the artifact

> The Project is in Rust Language
