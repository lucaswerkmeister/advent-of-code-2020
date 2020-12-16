# Day 16

The [day 16 puzzle][day16] asks you to decipher tickets with numeric fields in certain ranges.

Part 1 was fairly straightforward to implement, only the input parsing was tedious.
It would’ve been possible, maybe even easier, in shell,
but I had a feeling that part 2 would be more computationally intensive,
and so I went for Rust again.
I wrote data model structs `Field`, `Ticket`, and `Input`,
implemented `FromStr` for each (with tests),
and then a few functions for the actual functionality of the puzzle.

For part 2, we have to find out which field corresponds to which position in the list of values of a ticket.
I first tried finding out the possible fields for each position
(e.g., position 0 can be field X, can’t be field Y due to ticket A, can’t be field Z due to ticket B, etc.),
but that wasn’t enough to unambiguously resolve each position/field.
However, some debug statements showed that it was enough to unambiguously resolve _one_ field,
and there was one more position where only two fields were an option,
so I tried adding a loop which removes each unambiguous field from the possible fields for each other position,
and fortunately that turned out to be enough.

Rust-wise, I think I’m still getting a better grasp of how to use references and borrowing,
and I also learned about the [`Vec::retain()`][] method, which is useful as an in-place filter.

## Usage

```sh
./solve
```

This just runs `cargo run` (the `input` filename is hard-coded),
which solves part 1 and 2 and prints both solutions to standard output.

[day16]: https://adventofcode.com/2020/day/16
[`Vec::retain()`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.retain
