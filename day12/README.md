# Day 12

The [day 12 puzzle][day12] asks you to interpret navigation instructions for a ship,
cruising around on a 2D plane.

My first thought for implementing this was to use matrices for the rotation instructions.
(Well, admittedly, my zeroth thought was “can’t I just sum the distances and ignore all turns and directions?”,
but a quick glance at the description confirmed that, no,
we’re looking for the distance from the starting point, not the distance traveled.)
Using matrices should make it fairly straightforward to handle non-right angle turns
(a quick `grep ^R input | sort | uniq -c` showed no such turns were in the input, but whatever),
so I thought about which programming languages would make working with matrices easy,
and while it’s surely possible in Rust, **R** sounded like a tempting answer.
I hadn’t worked with R before
(beyond some very simple commands to test the [AUR FastR packages][fastr-jdk11-bin], which I maintain),
so I thought this would be a good opportunity to gain a bit of experience with it.

After wasting some time trying to install [ESS][emacs-ess] –
the PGP key was not on most keyservers, importing it from [keys.openpgp.org][] didn’t make `makepkg` happy,
and eventually I gave up and used `makepkg --skippgpcheck` –
the actual coding in R seemed straightforward enough.
A mixture of the [R Programming][] Wikibook and random other internet pages showed
that function calls look normal, `c()` makes a vector, `matrix(c(), nrow=2, ncol=2)` makes a matrix,
and assignments are written with `<-` (apparently `=` also works but I guess I’ll use the more conventional `<-`).
`readLines()` provides the lines of a file,
and `substr` and `as.numeric` can be used to split an instruction into action and value.

I was then somewhat disappointed to discover that [R has no switch statement][R-switch];
rather, there is a `switch` function,
which looks like `switch(x, 'a'=1, 'b'=2)` and returns a value.
I started wondering if it was possible to interpret each instruction as one matrix,
which the `switch` could return and would then be multiplied onto the current “state”;
this led me down a rabbit hole of [affine transformations][],
but eventually I concluded it probably can’t be done:
moving north/east/south/west and rotating left/right should all work,
but I don’t think it’s possible to move forward in the current direction.
So I resorted to putting brace-enclosed code blocks in the `switch()` function,
as in [this StackOverflow answer][R-switch-answer],
and that worked out just fine:
keep the current position and direction as separate variables,
and update one of them in each switch block using the right vector/matrix operations.
R’s vectorization was also fairly useful:
for example, `c(0, 1) * value` multiplies a scalar with a vector,
and `abs(pos)` takes the absolute value of each coordinate of the position.

Part 2 changes the interpretation of the directions
(to something that’s probably rather more realistic),
replacing the direction state with a waypoint.
Moving the waypoint is also fairly straightforward –
for the rotation actions, we have to do a transformation to make the waypoint’s position relative to the ship,
but that’s just a simple subtraction and then addition again at the end.

## Usage

```
./solve input
```

`input` can also be another file name (but only one).
The script solves both part 1 and 2 and prints both answers to standard output.

[day12]: https://adventofcode.com/2020/day/12
[fastr-jdk11-bin]: https://aur.archlinux.org/packages/fastr-jdk11-bin/
[emacs-ess]: https://aur.archlinux.org/packages/emacs-ess/
[keys.openpgp.org]: https://keys.openpgp.org/search?q=1248E0A068E0DB0F
[R Programming]: https://en.wikibooks.org/wiki/R_Programming
[R-switch]: https://stackoverflow.com/q/10393508/1420237
[affine transformations]: https://www.wikidata.org/wiki/Special:GoToLinkedPage/enwiki/Q382497
[R-switch-answer]: https://stackoverflow.com/a/10393550/1420237
