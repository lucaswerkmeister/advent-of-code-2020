# Day 6

The [day 6 puzzle][day6] asks you to process yes/no answers by travel groups,
where the questions/answers are denoted by letters of the alphabet
(present if yes, absent if no),
each group member’s answers are listed on one line,
and groups are separated by blank lines.

This puzzle felt similar to the [day 4](../day04/README.md) passport puzzle,
which also had groups of input separated by blank lines.
However, this puzzle would require counting,
so I knew that `sed` was not the right fit;
instead, I turned to [AWK][] again –
specifically, GNU AWK:
I think my previous AWK solution, for [day 2](../day02/README.md),
is probably standard AWK, but this time I definitely used some GNUisms.
The first of those is to set the field separator to the empty string,
to iterate over the individual characters of each line easily.
I then used AWK’s wonderful property of auto-initializing variables:
writing `answers[$i]++` (where `i` is the numeric field number)
means to take (or initialize) `answers` as an associative array
and increment (or set to 1) the entry corresponding to the *i*th field,
so that the array would, for each question letter,
contain the number of how many people answered that question with “yes”.

Then, upon encountering a blank line,
the script “summarizes” those answers,
prints that “summary” to standard error,
and then resets itself.
The same thing happens at the end of the input,
and additionally the overall sum is printed.
(This is done in `ENDFILE` – another GNUism – rather than `END`,
so that the script can be used to process multiple files easily:
`./solve sample-input input 2>/dev/null` prints the solution for both inputs.)

Only the “summarizing” is different for part 1 and 2:
for part 1, the summand for each group is
the number of questions to which _anyone_ answered “yes”,
i.e. the length of the array (regardless of the values);
for part 2, it’s the number of questions to which _everyone_ answered “yes”,
i.e. the number of array elements which are equal to the total number of people in the group.
(I first thought about doing this “and” condition
by initializing the array with values for every letter,
and then deleting array entries whenever they weren’t found in a line,
but that didn’t work out at all.)

## Usage

```sh
./solve input 2>/dev/null
```

If standard error is not discarded, it shows the individual summands for each group.
The script can also process multiple files in one go –
the *n*th number printed to standard output corresponds to the *n*th input file.
(You’ll definitely want to redirect standard error away in that case,
otherwise the real solutions on standard output will be indistinguishable from the debug output.)

[day6]: https://adventofcode.com/2020/day/6
[AWK]: https://www.wikidata.org/wiki/Special:GoToLinkedPage/enwiki/Q213970
