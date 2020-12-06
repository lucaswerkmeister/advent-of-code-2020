# Day 3

The [day 3 puzzle][day3] asks you to find the number of trees
that you encounter on a diagonal / straight-line path through a forest
(specified as ASCII art input),
first for a single slope, then for several others.

This can probably be done in a shell script,
but I felt like going for a more “solid” solution this time,
and writing a “proper” program in Rust.
I defined a `Square` data model enum, with the values `Space` and `Tree`,
and some code to parse it (initially from a string, later from a char instead),
and then a `Map` data model struct with a list of `Square`s and a `width` and `height`,
and some more code to parse that from a string.
The `Map` also has a method to access the square at a given (x,y) position
(taking into account the topology of the map, repeating to the right ad infinitum),
and then a `count_slope` method counts the trees encountered along a certain slope.
(I included unit tests for some of this, but not for `count_slope` for some reason.)

I heard of the [Cargo Advent of Code helper][cargo-aoc], but didn’t want to use it here;
it seems to expect all the solutions to be in a single `src/` directory,
whereas I wanted to continue the pattern of one directory per day,
allowing different programming languages in each directory.
Maybe I’ll use the helper next year.

## Usage

```sh
./solve
```

This just runs `cargo run`.

[day3]: https://adventofcode.com/2020/day/3
[cargo-aoc]: https://github.com/gobanos/cargo-aoc
