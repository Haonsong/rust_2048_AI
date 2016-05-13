* Haonan Song

Artifact:
* git repository: <https://github.com/Haonsong/rust_2048_AI>

> All project files are uploaded to github

## Final milestone

> Final milestone given in Intermediate report is get 2048 most time.

> What I have got is get 2^14 = 16384 most time and gain score over 20K ~ 30K in 5 minutes with multiple thread and searching depth of 6. (Cheating AI)
> Get the 2048 for most time as a non-cheating AI,__**VERY**__ slow, may need 30 minutes.

## Accomplishments

> The genericity -- Vec < Vec < Option< Cell>>>
* Vec of Vec of Option of Cell type
* Vec of Option of Cell type
* Option of Cell type
>
> Feel free to point to the Programming Language Concepts you used, if any:
> genericity, first-class functions, (tail-)recursive functions, parsing,
> AST, interesting data structures or algorithms...

## Work in Progress

> All features is finished for both cheating and non-cheating.

## Technical Challenges

> Since the Windows command line is not working at first, so I decided to move to linux (arch linux).

> Get the linux installed on real machine(Really hard).

> Install & run the rustc & cargo on Linux machine(Pretty easy).

## Challenges

> The ownership of the grid and the vec<vec<option<Cell>> are very hard to be tracked. For example in `next_movement` function, which is the helper for minmax function, I got two types errors: moved value and the ownership which is not long enough.
> / a data structure to solve a particular problem...
>
> The content of this section may be already covered in the
> [Accomplishment Section](#accomplishments). If you don't have anything more
> to say in this section feel free to remove it.

## Group work breakdown

> Single person Project.

## Building / Running the artifact

> The Project is in rust-lang
