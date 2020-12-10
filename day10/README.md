# Day 10

The [day 10 puzzle][day10] asks you to analyse a set of adapters,
which are to be plugged into one another
(note: I’m pretty sure you should never do this, *and especially not on an airplane*),
where each adapter can raise a “joltage” level by increments of one, two or three, starting from zero.

I went back to the shell for this one, since part 1 seemed fairly simple:
`sort` the input, then iterate over it in `awk` and track the difference to the previous level.
I’m again using the auto-initializing variables:
in `diffs[$1-prev]++`, `prev` can be uninitialized (zero, conveniently also the base “joltage”),
and `diffs[X]` can be a missing array element (zero, also the initial counter).
At the end we need to increment the number of “three” differences by one,
because our input is missing the “built-in adapter”,
which the puzzle says is always three higher than the highest adapter.
(This threw me off for a bit at first.)

Part 2, determining how many valid combinations there are to chain the adapters
(where adapters can be at most three above the previous adapter,
so if adapters are consecutive, you can leave some out, but not too many),
sounds scary at first, but turned out to be fairly simple.
The key is to track how many combinations there are to reach “joltage” _x_,
and then, when the next adapter has level _y > x_,
then there are as many ways to reach level _y_ as there are to reach level _y - 1_, _y - 2_, or _y - 3_ together.

My only worry was that part 2 might involve numbers that would not fit into 64-bit integers.
I could see in the [POSIX `awk` manpage][awk(1p)], Lexical Conventions 8,
that behavior for numbers “too large or too small to be representable”
(with a reference to the C standard) is undefined.
However, a bit of googling revealed that GNU AWK has a `-M` option
to enable arbitrary precision arithmetic on numbers,
and was able to test this locally with
```sh
$ gawk 'BEGIN{print 2^65}'
36893488147419103232
$ gawk 'BEGIN{print 2^65 - 1}'
36893488147419103232
$ gawk -M 'BEGIN{print 2^65 - 1}'
36893488147419103231
```
to verify that my copy of the program was actually built with the necessary libraries.
But then I proceeded to write my solution, test it, and submit it,
before realizing I’d forgotten to specify the `-M` option!
It turns out that the solution for my input is “only” about 49 bits wide –
enough for a 64-bit integer, and also for a 64-bit floating-point number (53-bit mantissa),
which is what GNU AWK apparently uses by default.
This is fortunate, because when I add `-M` to my solution, it prints 0 for some reason.
🤷

## Usage

```sh
./solve input
```

The `solve` script accepts multiple files –
for instance, you can `./solve sample-input-{1,2}` –
and prints the part 1 and 2 solutions one after the other.

[day10]: https://adventofcode.com/2020/day/10
[awk(1p)]: https://man7.org/linux/man-pages/man1/awk.1p.html
