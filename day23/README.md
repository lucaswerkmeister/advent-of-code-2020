# Day 23

The [day 23 puzzle][day23] asks you to simulate a cup-shuffling game.

This one seemed fairly similar to [day 22](../day22/README.md),
and I could’ve implemented it in Bash as well,
but I went for Rust because I had a shrewd feeling that part 2 was going to require more performance,
something like “simulate the game for a million moves”,
which turned out to be fairly accurate
(in fact part 2 requires ten million moves and also one million cups).

I implemented this using a `Vec` to hold the cups,
and arranged such that the current cup is always at the front of the `Vec`
(rather than having a separate index for the current cup).
This worked well enough for ten cups, but for part 2, it was rather inefficient;
it eventually completed, but only an hour and forty minutes.
I only realized afterwards that linked lists would probably be a much better data structure for this,
but I haven’t implemented a linked list solution yet,
and I’m not sure if I’ll have the time for it.

## Usage

```sh
./solve
```

This runs `cargo run --release`, which prints the part 1 and 2 solutions to standard output.
(The `input` filename is hard-coded.)

[day23]: https://adventofcode.com/2020/day/23
