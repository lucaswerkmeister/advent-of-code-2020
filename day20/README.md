# Day 20

The [day 20 puzzle][day20] asks you to assemble an overall image from smaller tiles
based on overlapping edges of the tiles, which may be rotated or flipped randomly.

I knew this one would require more than some shell commands, so I started like with all my Rust solutions:
define some data model structs, write code to parse them (i.e. implement `FromStr`), and test it.
(I saw that both the sample input and my real input had 10×10 tiles, so I hard-coded the size of a tile into the program:
it’s represented as an array of 100 `bool`s.)
Then I added functions to calculate the edges of the tile, represented as integers,
and tested those too.

I then reasoned that I might be able to save a lot of work:
for part 1, we are only interested in the product of the IDs of the four corner tiles
I thought that maybe I could avoid the whole work to assemble the full image at all,
since I only needed the four corner tiles,
and those should be identifiable readily enough:
those would be the tiles whose edges were not found on any other tiles.
I originally thought this was just a hypothesis worth trying out,
but the task actually seems to confirm we can rely on this property
(i.e., the edge tile’s edges won’t happen to reoccur somewhere inside the image):

> Tiles at the edge of the image also have this border, but **the outermost edges won't line up with any other tiles.**

So I wrote an intended-to-be-temporary function to count how many times a tile appeared,
printed those numbers,
and analyzed them with a shell pipeline:

```
$ cargo run | sort | uniq -c
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/day20`
      4 4
     40 6
    100 8
```

So, four tiles (the corner tiles) have four shared edges
(the two non-corner edges, times two due to potential flipping),
forty tiles (the non-corner edge tiles) have six shared edges,
and a hundred tiles share all eight edges with at least one other tile.
(My input is for a 12×12 grid.)

This means that the corner tiles are uniquely identifiable,
so I turned my “temporary” function into a `part1` function to return the product of their IDs,
and that was part 1 solved without assembling any tiles into a full image.

However, for part 2 we actually have to assemble the image,
in order to search for patterns in it.
Before I started this, I wanted to know if there were any ambiguous edges,
so I tweaked the `part1` function to print the number of tiles that have each edge:

```rust
tiles_by_edge.values().map(|tiles| tiles.len()).for_each(|len| println!("{}", len));
```

```
$ cargo run | sort | uniq -c
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/day20`
     96 1
    528 2
```

96 edges are only seen on one tile (the outer edges: 96 = 12 × 4 × 2),
and 528 edges are seen on exactly two tiles;
no edges are seen on more than two tiles.
This means we can pick one of the corner tiles we already identified as the first tile,
rotate it so the unique edges are on the outside,
and the adjacent tiles will always be clearly defined –
no guessing or backtracking should be needed.
(I checked that this holds true for the sample input as well.)

After implementing functions to flip and turn an individual tile,
I decided to skip writing tests for the rest of the part 2 solution,
because it seemed like the tests would take long to write and have huge test data sets.
In hindsight, this was probably a mistake;
I lost plenty of time debugging silly bugs that would’ve been easier to catch with unit tests.
Not much else to say about part 2.
(`cargo fmt` and `cargo clippy` are nice, though.)

## Usage

```sh
./solve
```

The script runs `cargo run` (the `input` filename is hard-coded),
which prints the solutions for part 1 and 2.

[day20]: https://adventofcode.com/2020/day/20
