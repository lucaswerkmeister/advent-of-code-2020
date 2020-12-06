# Day 4

The [day 4 puzzle][day4] asks you to validate passpords,
which are given as lists of key:value pairs across one or more lines,
separated by blank lines.

The spreading of fields across multiple lines felt to me
like it was designed to make this puzzle harder to solve by shell scripts
(which generally like to process one line at a time),
but I was not going to be defeated by that!
A few years ago, I had read the [GNU sed manual][info sed] front-to-back for some reason
(it’s not that long and fairly well-written),
and I still remembered that sed has two buffers of text:
one for the current line, which is what most sed scripts use exclusively,
but there’s also another “auxiliary” one which survives across lines
and can be used for more complex scripts.
(They’re called the “pattern space” and “hold space”,
though I certainly didn’t remember the exact names.)

My solution is a sed script which appends input lines to the hold space
until it encounters an empty line,
at which point it can now process the entire passpord record at once.
Processing the record means checking the pattern for each field,
discarding the record if it doesn’t match,
and printing it at the end;
the solution is then the number of records printed.

For the first part, the “pattern” for each field is simple,
since we only need to check that the field is present at all.
The second part complicated this a bit, and several fields are required to be within a certain numerical range.
This is not very nice with regular expressions, but possible,
and fortunately sed lets you check the input for several expressions at once,
by nesting them in blocks: the line
```sed
/ byr:19[2-9][0-9]\b/ ! { / byr:200[0-2]\b/ ! d }
```
means to **d**iscard any record that does not match a birth year from 1920-1999
and also does not match a birth year from 2000-2002.

## Usage

```sh
./solve input | wc -l
```

The script prints valid records, one per line,
so for the solution we have to count them outside the script.

[day4]: https://adventofcode.com/2020/day/4
[info sed]: https://www.gnu.org/software/sed/manual/html_node/index.html
