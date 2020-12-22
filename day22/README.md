# Day 22

The [day 22 puzzle][day22] asks you to simulate a simple card game between two players;
in part 2, the card game becomes recursive.

I went back to the shell for this one – I just felt like taking a break from Rust for a bit.
Bash has one-dimensional arrays and operators to slice them around,
so the puzzle was fairly doable,
including part 2, once I declared all required variables to be `local`
(so that the recursion wouldn’t break).
The `part2` function is pretty slow for my real input,
taking some four and a half minutes to complete,
but I think that’s acceptable and just due to Bash being slow in general.

To debug part 2, I made the script print a log that’s almost identical to the one in the puzzle description
(there’s a handful of extra blank lines, and I use ’ U+2019 instead of ' U+0027 to sidestep quoting issues).
Removing that logging (most of the `printf >&2` statements, and also the statements to track the current game/round number)
speeds the script up so that it only takes about a minute and a half to complete for my input.
(I don’t feel inclined to commit this version, though.)

Not much else to say about this solution, I think.

## Usage

```sh
./solve input 2>/dev/null
```

This solves part 1 and 2 and prints both solutions to standard error.
(If you want to see the part 2 log, remove the `2>/dev/null`.)

[day22]: https://adventofcode.com/2020/day/22
