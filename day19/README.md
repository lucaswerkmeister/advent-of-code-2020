# Day 19

The [day 19 puzzle][day19] asks you to interpret regular expressions written in an unusual notation;
for part 2, we move one step up in the [Chomsky hierarchy][].

I decided to implement this one in AWK again,
since it has fairly good support for working with regular expressions,
including building them up from string fragments.
I toyed with the idea of using `tsort` (compare [day 7](../day07/README.md))
to bring the individual rules of the input into a useful order,
but ultimately didn’t pursue that;
instead, the rules are resolved (once they’ve all been read),
with a recursive function.
Then, at the end, we print all lines that match rule 0:
```awk
if ($0 ~ ("^" patterns[0] "$")) {
  print $0;
}
```

Part two is trickier. Two rules are replaced and become recursive:
```
8: 42 | 42 8
11: 42 31 | 42 11 31
```
Rule 8 now effectively becomes a “plus” operator:
if rule 42 is the pattern `X`, then `8: 42 | 42 8` means `X+`.
Rule 11 is a kind of balanced expression:
if rule 42 was `(`, and rule 31 was `)`, then `11: 42 31 | 42 11 31` would match `()`, `(())`, `((()))`, and so on.
This is a classic example of a language that _cannot_ be matched with regular expressions
(another classic example being [parsing HTML with regex][]).
The task description emphatically reminds us that <q>you only need to handle the rules you have</q>,
and after wasting some time thinking about too-general solutions,
I noticed two important properties in the input:

1. The special rules 8 and 11 are only referenced by the root rule (rule 0),
   never buried deeper within other rules:
   
   ```
   $ grep -w -e 8 -e 11 input
   0: 8 11
   11: 42 31
   8: 42
    ```

2. Rule 8 and 11 reference the same other rule: rule 42.
   This means we aren’t really searching for balanced expressions in the general case;
   any “extra” matches for rule 42 at the beginning of rule 11 can be “shifted” to belong to rule 8 instead,
   so we’re actually looking for strings with _n_ > 1 matches of rule 42 and then `m` < `n` - 1 matches of rule 31.
   (There has to be at least one more match of rule 42 than rule 31 because rule 8 requires at least one occurrence of rule 42.)

To find matches for this special kind of non-regular pattern,
my script first searches for the highest number of repetations of rule 42 that can match at the beginning
(by matching against `^(42){1}`, then `^(42){2}`, etc.),
then for each count _n_ up to that limit checks the input against the pattern `^(42){n}(31){1,n-1}$`,
i.e. rule 42 matches exactly _n_ times and rule 31 between 1 and _n_ - 1 times.

## Usage

```sh
./solve -vpart=1 input 2>/dev/null | wc -l
./solve -vpart=2 input 2>/dev/null | wc -l
```

The `solve` script prints each matching line to standard output,
and (for debugging) the patterns to standard error;
the `part` variable controls whether part 1 or 2 is being solved.

[day19]: https://adventofcode.com/2020/day/19
[Chomsky hierarchy]: https://www.wikidata.org/wiki/Special:GoToLinkedPage/enwiki/Q190913
[parsing HTML with regex]: https://stackoverflow.com/a/1732454
