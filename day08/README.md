# Day 8

The [day 8 puzzle][day8] asks you to emulate a simple factional assembly language.
There is one accumulator as storage and three operation codes:
<abbr title=accumulator>ACC</abbr>,
<abbr title=jump>JMP</abbr> and
<abbr title="no operation">NOP</abbr>.

I implemented this in Rust: parse the program and then implement an interpreter,
which steps through the instructions one at a time,
keeping track of which instructions were already seen so that we can abort in case of an infinite loop.
This took a while to get right, but I don’t see a lot to talk about here.

For part 2, the puzzle assumes that exactly one instruction in the program was corrupted,
from NOP to JMP or vice versa.
(This meant that I had to adjust my parser, which had previously been discarding the argument of NOPs.)
I didn’t do anything clever here, just brute-force it –
iterate over all indices, toggle the corresponding instruction,
run the interpreter and see if it aborted with infinite loop or regular termination.
(This had also not been distinguished earlier.)

## Usage

```sh
./solve
```

This just runs `cargo run` (the `input` filename is hard-coded).
The `main` function runs both part 1 and 2.

[day8]: https://adventofcode.com/2020/day/8
