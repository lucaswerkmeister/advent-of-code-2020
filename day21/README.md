# Day 21

The [day 21 puzzle][day21] asks you to determine which ingredients contain which allergens,
given a list of ingredient lists with allergens for each
(but only allergen information per list, not for individual ingredients).

My solution for this is fairly straighforward:
parse the input file, and for each allergen keep track of all the ingredient lists in which it appears;
then, intersect the ingredient lists for each allergen,
and if the intersection for any allergen contains just one ingredient,
that’s the ingredient that must contain that allergen
(and, per problem statement, cannot contain any other allergen),
so remove that ingredient from all ingredient lists
(as well as removing the allergen from the map from allergen to ingredient lists).
Repeat that second part until no more allergens remain unassigned.
For a while, I only intersected two lists at a time,
which was good enough to solve the sample input but not the real one,
but fortunately I eventually realized that I should really intersect all lists at once.

For part 2, we’re supposed to sort the ingredients with allergens by the allergen name,
which is trivial given what I already did (find the complete ingredient⇒allergen mapping);
maybe there’s a simpler way to solve part 1 which would’ve left more work for part 2,
similar to how in [day 20](../day20/README.md) I solved part 1 without assembling the image that was then required for part 2?
Not sure.

The Rust code is probably my ugliest in this Advent of Code (yet, I suppose) –
there are no data model structs beyond “newtypes” for `Ingredient` and `Allergen`,
instead functions directly take and return collections of collections
(returning tuples if multiple returns are needed).
Error handling is also done with `.unwrap()` and `.expect()` rather than proper error propagation.
And there’s also a lot of `clone()`ing going on, though I suspect most of that is necessary.
But at least everything has tests again.
I also wrote myself a tiny `set![]` macro for use in the tests.

One slightly annoying part is that Rust collections won’t let you modify them while you iterate them,
which necessitates the `last_resolved_ingredient` helper variant:
instead of immediately removing a resolved ingredient from all ingredient list in the loop,
store it in that variable, jump out of the loop, and then do the removal.
(This also requires some more `clone()`s so that the lists aren’t still borrowed due to the iteration,
which wasn’t easy to figure out from the compiler’s error messages.)

## Usage

```
./solve
```

The `solve` script runs `cargo run` (the `input` filename is hard-coded),
which prints the solution for part 1 and 2.

[day21]: https://adventofcode.com/2020/day/21
