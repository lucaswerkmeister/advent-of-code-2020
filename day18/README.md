# Day 18

The [day 18 puzzle][day18] asks you to evaluate simple arithmetic expressions
(addition, multiplication, parentheses, only positive integers)
with unusual precedence rules –
first `+` and `*` have equal precedence, then `+` binds more tightly.

I implemented this in Rust, using a hand-written parser.
(I briefly looked into Rust parser generators,
but neither of the options looked immediately attractive.)
I’m not sure if the parser strictly follows any particular paradigm,
but it can surely be called “recursive” of some kind.
Each parsing function returns `Result<(Expr, Option<&str>), ParseExprError>`:
a tuple of a parsed expression and some remaining portion of the string, if any,
or a parse error.
Evaluating the `Expr` type is then pretty straightforward.

For part 2, which changes the operator precedence,
I decided to just edit the code to parse using the new precedence;
having copies of it for part 1 and 2 didn’t seem attractive.

## Usage

```sh
./solve
```

This just runs `cargo run` (the `input` filename is hard-coded),
which prints the solution for part 2 to standard output.
For part 1, you’ll need to resort to the Git history.

[day18]: https://adventofcode.com/2020/day/18
