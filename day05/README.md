# Day 5

The [day 5 puzzle][day5] is about boarding pass numbers,
interpreted as decimals but specified in a sequence of letters
for the **f**ront, **b**ack, **l**eft or **r**ight part of the plane.

This is effectively just a version of binary numbers,
where F and L both stand for 0 and B and R for 1.
I used [`tr`][tr(1)] to translate the input into binary,
and then [`bc`][info bc] to convert it to decimal,
since I remembered that `bc` lets you control the input and output base
using the `ibase` and `obase` variables, respectively.
For both parts of the puzzle, the input was also sorted numerically.
(The sorting could just as well have taken place before the base conversion,
but doing it afterwards seemed to make more sense to me for some reason.)
The solution for part 1 (highest number) is then the last row (`tail -1`).

For part 2, we had to find our own boarding pass number,
which is the one “gap” in the otherwise consecutive list of numbers.
Upon reading this, I first thought about using [`comm`][comm(1)] and [`seq`][seq(1)],
which should make it possible to find all input numbers missing from the sequence from 1 to, uh, some upper limit;
however, the task then went on to say
that some range of numbers would be missing from the beginning and end of the input.
I couldn’t think of a good way to make this work with `comm` and `seq`,
so I decided to instead loop over the sorted input in the shell script,
and print and exit as soon as a gap between the previous and current line was discovered.
This way, the gap at the beginning didn’t matter
(the script started counting wherever the input started,
whether that was at 1 or above)
and the gap at the end would never be seen,
since the script would exit as soon as it found the first gap.

## Usage 

```sh
./solve input
```

[day5]: https://adventofcode.com/2020/day/5
[tr(1)]: https://man7.org/linux/man-pages/man1/tr.1.html
[info bc]: https://www.gnu.org/software/bc/manual/html_chapter/bc_toc.html
[comm(1)]: https://man7.org/linux/man-pages/man1/comm.1.html
[seq(1)]: https://man7.org/linux/man-pages/man1/seq.1.html
