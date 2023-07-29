# About
[Advent of Code](https://adventofcode.com/2022/about) is an Advent calendar of small programming puzzles for a variety of skill sets and skill levels that can be solved in any programming language you like. People use them as interview prep, company training, university coursework, practice problems, a speed contest, or to challenge each other.

# Solution Notes
## 2022
### Day 1
Looking at the code, it looks like I'm abusing the standard library. However, the functions used like `sum` or `sort` are trivial to code yourself and I feel focusing on deisgning the solution is far more enriching than designing the algorithms individually.

### Day 2
I overcomplicated the solution by adding adding 3 user defined types, however it helped me in part 2 as I needed to write less code for them. I also learnt how to implement traits like `FromStr` by myself.

### Day 3
I wrote a generic function that would probably be useful in the future and learnt how to write things the rust way - using `fold` and `iter`. Throwing a HashMap at things usually tends to solve them.

### Day 4
Modelling your program using Traits, Enums and Structs in Rust tends to help scalability, I was able to implement the second part of the problem with minimal additions.

### Day 5
The standard library is far more powerful than I thought. `FromStr` is a super useful trait and the stl methods for vecs and strings are extremely useful. I finished the second part with minimal additions, which proves my program design was good.
