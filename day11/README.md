# Day 11

The [day 11 puzzle][day11] asks you to simulate a cellular automaton,
similar to the famous [Game of Life][].

Rust seemed like the obvious tool for this, so I used it again.
I also used [Immutable Data Structures for Rust][im],
which I had heard about in a conference talk,
to implement the board:
this ought to reuse the parts of the board that don’t change,
without using a lot of memory to store copies of them.
(I haven’t benchmarked this against an alternative implementation, though).
Beyond that, I don’t think I have a lot to say about this solution;
I’m just slowly getting more familiar with Rust
(e.g. trying to figure out where to use references).

## Usage

```sh
./solve
```

This just runs `cargo run` (the `input` filename is hard-coded),
though in `--release` mode –
in the default `--debug` mode, the program takes several seconds.
The `main` function runs both part 1 and 2.

[day11]: https://adventofcode.com/2020/day/11
[Game of Life]: https://www.wikidata.org/wiki/Special:GoToLinkedPage/enwiki/Q244615
[im]: https://docs.rs/im/15.0.0/im/
