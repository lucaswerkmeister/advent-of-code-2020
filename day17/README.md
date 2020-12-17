# Day 17

The [day 17 puzzle][day17] asks you to simulate [Conway’s Game of Life][],
but in three and then four dimensions.

For the [day 11 puzzle](../day11/README.md), I had implemented a cellular automaton on a “rigid” grid,
represented as a fixed-sized `Vec` of cells.
For this puzzle, the grid was to be infinite, so I decided to implement it as a set of coordinates instead:
a cell would be active if its coordinates were in the set, or inactive otherwise.
(It helped that this time, unlike in the last puzzle, cells had two possible states, not three.)
This representation was also more sparse than a full vector of cells,
which I assume was even more beneficial in the 4D case (part 2),
though I haven’t benchmarked it against an alternative implementation.

To execute one cycle, I first collect all the potentially active cells for the next cycle
(all the neighbors of all current cells),
then check for each if it’s actually active
(has the right number of active neighbors depending on it being active or inactive so far) or not.
The state struct also has fields for the range of potentially occupied coordinates (useful for printing),
which have to be updated accordingly.

## Usage

```sh
./solve
```

This just runs `cargo run` (the `input` filename is hard-coded),
which solves part 1 and 2 and prints both solutions to standard output.
It runs the program in release mode –
it’s still reasonably fast in debug mode,
but release mode is noticeably faster.

[day17]: https://adventofcode.com/2020/day/17
[Conway’s Game of Life]: https://www.wikidata.org/wiki/Special:GoToLinkedPage/enwiki/Q244615
