# Day 2

The [day 2 puzzle][day2] asks you to check passwords against password policies,
both specified in the same input file.
In part 1, the password policy specifies a number of occurrences of a character,
whereas in part 2 it names two indices and the character must occur in at least one of them.

My first idea was to interpret a password policy `1-3 a` as the regular expression `a{1,3}`;
there are several command line programs to work with regular expressions,
but to build them dynamically, [AWK][] seemed like the best choice.
I only realized later that this interpretation is incorrect:
that password policy does not require 1-3 *consecutive* occurrences of `a`,
only 1-3 occurrences overall.
That made AWK a bit less useful for the puzzle,
but it’s still a generally decent programming language for simple input processing,
so I was still able to solve the puzzle with some `gsub()` and `length()` usage.
(To parse the password policy, I set the field separator to the regex `[- :]`,
i.e. fields could be split on any of those characters,
and then interpreted fields 1, 2, 3 and 5 as the parts of the policy.)

For part 2, it was convenient that AWK also counts strings from index 1, not 0,
since that was also the convention used in the puzzle.

## Usage

```sh
./solve input 2>/dev/null | wc -l
```

The script prints “matches” lines to standard output
and “does not match” lines to standard error,
so by counting the number of output lines we obtain the solution,
the number of valid passwords.
In hindsight, it would’ve been nicer to print both kinds of lines to standard error,
and also count matches in the script and print those to standard output at the end.

[day2]: https://adventofcode.com/2020/day/2
[AWK]: https://www.wikidata.org/wiki/Special:GoToLinkedPage/enwiki/Q213970
