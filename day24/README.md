# Day 24

The [day 24 puzzle][day24] asks you to simulate a cellular automaton on a hexagonal grid,
where the initial state is given as a series of
<abbr title=east>E</abbr>/<abbr title=southeast>SE</abbr>/<abbr title=southwest>SW</abbr>/<abbr title=west>W</abbr>/<abbr title=northwest>NW</abbr>/<abbr title=northeast>NE</abbr>
directions indicating tiles to be flipped.

My solution is fairly similar to the [day 17 solution](../day17/README.md),
with another unlimited grid represented as a set of coordinates of active tiles.
Not much to say about it, except maybe that I tried using an array for the `neighbor_coordinates` function,
but then I couldn’t use `into_iter()` on it because [array_value_iter] is still experimental,
so I switched to a `Vec` instead.

## Usage

```sh
./solve
```

Prints the day 1 and 2 solutions, from a hard-coded `input` filename.

[day24]: https://adventofcode.com/2020/day/24
[array_value_iter]: https://github.com/rust-lang/rust/pull/65819
