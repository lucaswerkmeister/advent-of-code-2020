# Day 14

The [day 14 puzzle][day14] asks you to interpret memory assignments influenced by a bitmask.

I was going to solve this in Rust, but the laptop I’m working on now doesn’t have Rust installed,
and the internet connection on the train wasn’t good enough to download it,
so I had to use what was already installed, and decided to go back to Bash.
Its parsing facilities aren’t half bad,
and especially the ability to match a regular expression with `[[ lhs =~ pattern ]]`
and then access the matches in the `BASH_REMATCH` array was useful.
And of course, Bash has arithmetic on 64-bit integers
(wide enough for the 36-bit puzzle numbers),
including bitwise and/or and shift operators,
which I used to interpret the instructions.

For part 2, a single assignment line can assign to many memory locations,
depending on how many “floating bits” (`X` characters) the mask has;
that sounded scary, so I checked first what the maximum number of Xs in my input was:

```sh
$ sed 's/[^X]//g' input | wc -L
9
```

Nine bits (512 addresses) seemed low enough that it should be possible to “materialize” all addresses,
without having to somehow track them implicitly to save memory or processing time.
I wrote a recursive function to expand these addresses,
and then assigned to the `mem` array for each address.
The resulting program takes a nontrivial amount of time (36 seconds),
but still low enough that I’m willing to ascribe it to general Bash slowness
([bash(1), BUGS][]: “it’s too big and too slow”),
and don’t see the need to optimize it further.

## Usage

```sh
./solve input
```

The script solves parts 1 and 2 and prints both solutions.
You can also run it with `sample-input-part1` or `sample-input-part2`,
and ignore the second or first line, respectively.

[day14]: https://adventofcode.com/2020/day/14
[bash(1), BUGS]: https://man7.org/linux/man-pages/man1/bash.1.html#BUGS
