* Haonan Song

Artifact:
> * git repository: <https://github.com/Haonsong/rust_2048_AI>

> * All project files are uploaded to github

## Final milestone

> Final milestone given in Intermediate report is get 2048 most time.

> - What I have got is get 2^14 = 16384 most time and gain score over 20K ~ 30K in 5 minutes with multiple thread and searching depth of 6. (Cheating AI)
> - Get the 2048 for most time as a non-cheating AI,__**VERY**__ slow, may need 30 minutes.

## Accomplishments

> Genericity
* The gird in Vec < Vec < Option< Cell > > >
  * Vec of Vec of Option of Cell type
  * Vec of Option of Cell type
  * Option of Cell type

> First-class functions
* The `randloop` function is passing the `ai_move` function as a parameter.

> Algorithms
  * minimax for cheating AI
  * Expectminimax for non-cheating AI

> Recursive functions
  * minmax function will Recursively call itself to match the depth as given.

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

> Get the linux installed on real machine(Really hard).

> Install & run the rustc & cargo on Linux machine(Pretty easy).

## Challenges

> The ownership of the grid, vec< vec < option < Cell > > >, are very hard to be tracked. For example in `next_movement` function, which is the helper for minimax function, I got two types errors: moved value and the ownership which is not long enough. Solving these two problems by pass the cloned object to other function rather than the original reference to the object.

## Group work breakdown

> Single person Project.

## Building / Running the artifact

> The Project is in Rust Language
