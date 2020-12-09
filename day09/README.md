# Day 9

The [day 9 puzzle][day9] is about partial sums in a list of numbers.

For part 1, we have to find the first number that is not a sum of two out of the last *n* numbers.
At first I wasn’t sure which part to optimize for here:
we could keep a set of all possible sums,
so we could quickly check whether the number was a sum,
but would have to add and remove *n-1* sums for each new number;
or we could keep a set of all tuples, somehow efficiently adding and removing them,
but having to calculate *(n-1)×(n-2)* sums for each new number.
(While working on the [previous puzzle](../day08/README.md),
I had looked at the [collections documentation][collections],
which suggested that `BTreeMap`/`BTreeSet` could work with ranges of entries efficiently,
so I thought that maybe it would be efficient to remove all tuples with the number
which had just “fallen out” of the last *n* numbers.)
Eventually I concluded that keeping a set of sums would be better.
(Soon afterwards, I realized it needs to be a *map*,
from sum to number of occurrences of that sum,
since a sum can occur more than once.)

The Rust code for part 1 is then fairly straightforward, I think –
keep a ring buffer of the last *n* numbers and a set/map of all their sums;
initialize the buffer and sums from the “preamble” of the numbers;
then iterate over the remaining numbers, return early if a number is not in the sums,
otherwise shift one number out of the ring buffer, remove its sums with all remaining numbers,
then add the new number and add its sums with all other numbers to the sums.

For part 2, we then have to take this first number that isn’t a sum of two others,
and find a consecutive range of at least two numbers in the input which sums to that number.
(As I write this, I realize it must really be a range of at least *three* numbers,
since we just determined it’s not the sum of any two numbers, consecutive or not.)
I didn’t do anything clever for this – just iterate over all possible subranges and try to sum them up.
(Writing this, I now realize I could at least stop “growing” a range once its sum exceeds the number,
since I’m already assuming all numbers to be unsigned.)

One thing I struggled a bit with was the return types of my functions,
or the classification of errors and successes.
For a while (including in the “Day 9, part 1” commit),
my `part1` function returned an _error_ when it found a number that wasn’t a sum –
that number is the solution to the puzzle, but the puzzle suggests that’s in some way a mistake in the data.
(The regular return from the function, never reached, was `()` in that case.)
However, when it came to part 2, I needed that number as an intermediate result,
and it seemed wrong to treat it as an error in that case,
so I rearranged the types a bit.

## Usage

```sh
./solve
```

This just runs `cargo run` (the `input` filename is hard-coded).
The `main` function runs both part 1 and 2.

[day9]: https://adventofcode.com/2020/day/9
[collections]: https://doc.rust-lang.org/std/collections/index.html
