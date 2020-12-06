# Day 1

The [day 1 puzzle][day1] asks you to find two, and later three,
entries of a list which sum up to 2020.

Upon reading the first part (find *two* entries),
my idea was that for any number in the list you can calculate the “partner” so they sum up to 2020,
by subtracting that number from 2020;
the task is then to check if that other number is in the list.
At this point I remembered the [`look`][look(1)] command,
which can be used to efficiently look up a word in a sorted file.
My solution is therefore a shell script which sorts the input file
and then iterates over it, calculating and looking for the second summand for each input row.
(It gets a bit more tricky because `look` processes input that’s sorted *lexically*,
not numerically, so we have to zero-pad the input file to four digits,
and then ensure that Bash doesn’t interpret the leading zeroes as octal numbers.)

For the second part, finding three matching entries,
I didn’t have any smart ideas for an efficient solution;
I just made the shell script loop over the input file one more time,
summing up the first two numbers and then checking if the remainder exists as a third number,
and that still seems to be fast enough in practice.

## Usage

```sh
./solve
```

The file name `input` is hard-coded,
and the script will generate `input.padded` and `input.sorted` files
without cleaning them up.
In hindsight, this could’ve been done a bit cleaner,
or if I wanted to keep the files around (useful for debugging),
maybe a Makefile would’ve made sense.

[day1]: https://adventofcode.com/2020/day/1
[look(1)]: https://man7.org/linux/man-pages/man1/look.1.html
