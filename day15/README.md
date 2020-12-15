# Day 15

The [day 15 puzzle][day15] asks you to follow a counting game,
where you refer back to the last turn where a number was said.

I started out implementing this in Bash, for no particular reason,
though it was kind of nice for prototyping (`declare -p` is convenient).
I don’t have any great insights to offer – it was just a lot of trial and error,
and eventually I had fixed all the off-by-one mistakes and got the right solution.

Part 2 is then effectively the same as part 1, but with a much higher limit –
high enough to rule out Bash.
So I rewrote my solution in JS, line for line,
and while that still took 9½ minutes of CPU time,
it was fast enough to find the solution within an acceptable time.
(I assume it can be optimized further but I couldn’t be bothered.)

## Usage

```sh
./solve input
```

The committed `solve` script prints the solutions for part 1 and 2.
(The Bash version, which you can resurrect from the Git history, only prints part 1.)

[day15]: https://adventofcode.com/2020/day/15
