# Day 7

The [day 7 puzzle][day7] asks you to traverse a graph of relationship between different-colored bags.

My first idea here was that maybe I could abuse Git to solve this puzzle.
Part 1, how many colors can contain a shiny gold bag,
corresponds to reachability of the shiny gold node,
or in Git terms, `git tag --contains`.
I thought that maybe I could create an empty repository and write a bunch of refs,
whose destination might not even exist at the time of writing –
i.e. I’d write `refs/bag/bright-white -> refs/bag/shiny-gold` before writing `refs/bag/shiny-gold` –
and at the end, all the dangling references would have been resolved.
However, I realized that this doesn’t work when a bag contains multiple other bags:
the way to represent this would be a merge commit,
which makes all of its parents reachable,
but you can only create a merge commit if you already have the parent commit hashes.
(As I’m writing this, I realize this _might_ be solvable with `git replace`, but too late.)

The problem here was to “process” the graph in the right order:
“handle” (in whatever way) each bag before all of its “dependencies”.
I happened to remember an obscure coreutils command, [`tsort`][info tsort], which does just that:
given a list of partial orderings on standard input,
it performs a _topological sort_ and prints the input “members” in sorted order.
So the idea was to rewrite the input, which was of the form

> light red bags contain 1 bright white bag, 2 muted yellow bags.  
> dark orange bags contain 3 bright white bags, 4 muted yellow bags.

into the `tsort`-friendly

> ```
> light-red bright-white
> light-red muted-yellow
> dark-orange bright-white
> dark-orange muted-yellow
> ```

and then `tsort` would print a list of colors, and that would tell me… something.

My first attempt, that the solution would simply be the position or line number of `shiny-gold` in the output –
all the other colors which could contain it would be printed before it – didn’t work out.
The `tsort` output typically isn’t fully determined by the input,
and so the utility has some leeway as to how it sorts the output:
there’s nothing stopping it from putting, say, `faded-blue` either above or below `shiny-gold`,
if the two colors aren’t very connected in the graph.
Put differently, `tsort` is under no obligation to put the one color I’m interested in, `shiny-gold`,
as early as possible in the output,
so that the colors before it are really _only_ the colors related to `shiny-gold`.

(You may notice that I’m phrasing this very vaguely and avoid saying whether bags contain or are contained in other bags.
This is simply because I can’t be bothered to think the “dependency order” through again.
I eventually got it right in the solution after some trial and error, that’s good enough.)

At this point, I finally accepted that I wouldn’t be able to solve the puzzle in a fully “streaming” fashion:
I’d have to read the input fully into memory, and keep it there,
in order to be able to access the data of arbitrary colors later.
The solution shell script essentially does that –
read the input into an associative bash array,
write the partial orderings and pipe them into `tsort`,
then iterate over the `tsort` output.
In part 1, if `tsort` prints a bag color which is in a “seen” array –
and that array only starts out with `shiny-gold`, the “starting” color –
then it adds all the related colors also to the “seen” array.
At the end, the number of entries in the “seen” array,
minus one (`shiny-gold`), is the solution.

In part 2, it gets a bit more complicated, but not much;
we now also have to parse and keep track of the number of bags contained in other bags,
and multiply and sum up these numbers, keeping track of them in another array.
(This time, the array starts out empty, and at the end we access the `shiny-gold` member.)
Again, `tsort` helps us by printing out the colors in the right order,
so that, if (as above) “light red bags contain 1 bright white bag, 2 muted yellow bags”,
we’ve already seen `bright-white` and `muted-yellow` and calculated their numbers
by the time we reach `light-red`.

The parsing of the input file itself was mainly done in `sed`;
maybe it could be done in pure Bash, but I didn’t really feel like it.
Some Bash syntax is then used to split up the `sed` output –
split fields with `read` (according to `IFS=' '`, i.e. the Internal Field Separator is a space),
and for part 2, also use `${NAME#PREFIX}` (remove prefix) and `${NAME%SUFFIX}` (remove suffix)
to get just the number or color out of a string like `2:muted-yellow`.
(The prefix/suffix is a glob pattern, so `*:` and `:*` mean anything before and after a colon, respectively.)

## Usage

```sh
./solve input 2>/dev/null
```

The `part1` function (never called in the final committed version) printed some debug output to standard error;
in `part2`, I removed all the debug output I had before committing.
(Not very consistent, I know.)

[day7]: https://adventofcode.com/2020/day/7
[info tsort]: https://www.gnu.org/software/coreutils/manual/html_node/tsort-invocation.html
