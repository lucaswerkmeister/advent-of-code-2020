# Day 13

The [day 13 puzzle][day13] asks you to work with departure times of buses
which each depart every _n_ minutes.

Part one seemed fairly straightforward –
take the remainder of your earliest departure time modulo the departure time of each bus,
and check where it’s largest.
(There’s also a factor of minus one somewhere in there,
represented by the `departure - …` in my `waiting_time` function,
that I haven’t yet completely thought through.)
I implemented this in Bash, since Bash has 64-bit integer arithmetic
and therefore should be good enough for this.

Part two is harder:
we need to find the earliest timestamp where the zeroth listed bus leaves on that timestamp,
the first listed bus leaves on that timestamp plus one,
and so forth.
My “brute force” solution to this was to take the largest bus interval,
multiply that by a counter and add however many minutes necessary for that bus,
then check if the other buses matched.
In other words, I calculated a timestamp that would definitely match the largest bus
(so that we could skip the largest number of timestamps that definitely wouldn’t work),
and then checked all other buses against it.
This worked well enough for the sample inputs, but took way too long for the real input.
I reimplemented it in Rust, but that didn’t help very much either.
(I eventually killed my Rust program after over two and a half hours of CPU time.)

I then decided to write down the puzzle as a set of formulas on a piece of paper.
After getting it wrong –

```
THIS IS NOT CORRECT
 7 % t = 0     7 ≡ 0  mod t
13 % t = 1    13 ≡ 1  mod t
59 % t = 4    59 ≡ 4  mod t
...
```

– _twice_ –

```
THIS IS NOT CORRECT EITHER
 7 ≡  0       mod t
13 ≡ -1 ≡ 12  mod t
59 ≡ -4 ≡ 55  mod t
31 ≡ -6 ≡ 25  mod t
```

– I finally got the correct form:

```
t ≡  0       mod 7
t ≡ -1 ≡ 12  mod 13
t ≡ -4 ≡ 55  mod 59
t ≡ -6 ≡ 25  mod 31
t ≡ -7 ≡ 12  mod 19
```

And this, thankfully, looks just like the input to the [Chinese remainder theorem][CRT].
(I had actually written the first of the wrong versions above before even starting the brute-force solution,
and concluded that it didn’t look like the Chinese remainder theorem would help here.
I’m glad I eventually tried again.)
I once heard someone say (possibly [Julia Evans][b0rk]?)
that one major use of a formal computer science education is to teach you a lot of terms to google,
and that was certainly true here:
I absolutely couldn’t have told you what the Chinese remainder theorem is or how it works,
but I remembered the name and that it had something to do with solving sets of modulo equations,
and that it was useful in cryptography (the context where I’d heard about it).
So I implemented the [extended Euclidean algorithm][EEA] and the Chinese remainder theorem in Bash,
following their descriptions on German Wikipedia ([EEA permalink][], [CRT permalink][]),
and that worked like a charm.

I should maybe point out that this solution only works if the departure times of the buses are all pairwise coprime;
however, I had already confirmed earlier that at least my input was even totally prime:
```sh
$ sed -n '2s/,/\n/gp' input | grep -v x | factor
19: 19
37: 37
523: 523
13: 13
23: 23
29: 29
547: 547
41: 41
17: 17
```

## Usage

```sh
./solve input
```

The script takes one file name as input and prints the solutions for part 1 and 2.
(The brute-force solutions of part 2, both in Bash and Rust,
are present in the script but commented out at the bottom.)

[day13]: https://adventofcode.com/2020/day/13
[CRT]: https://www.wikidata.org/wiki/Special:GoToLinkedPage/enwiki/Q193878
[b0rk]: https://twitter.com/b0rk/
[EEA]: https://www.wikidata.org/wiki/Special:GoToLinkedPage/enwiki/Q1362750
[EEA permalink]: https://de.wikipedia.org/wiki/Special:PermanentLink/206096743#Rekursive_Variante_2
[CRT permalink]: https://de.wikipedia.org/wiki/Special:PermanentLink/200389318#Finden_einer_Lösung
