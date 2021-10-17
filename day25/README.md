# Day 25

The [day 25 puzzle][day25] asks you to break a cryptosystem that I’m pretty sure is just standard [Diffie-Hellman][],
but over a small enough group that brute-forcing the private keys is feasible.

I will freely admit that I didn’t recognize the Diffie-Hellman at first,
but I remembered that the specific problem being described was called the [discrete logarithm][],
and fortunately the German Wikipedia article for that listed Diffie-Hellman as one area where it’s used.
(The English Wikipedia article only mentions “several important algorithms in public-key cryptography”.)

I implemented a simple `mod_pow` function and a struct representing a partial Diffie-Hellman “system”,
where the private keys and shared key may or may not be known (`Option` type),
along with several constructors depending on what information you have available.
I then added a function that cracks the shared key by trying all possible private keys in ascending order.
There are some better algorithms to solve the discrete logarithm
(of the ones Wikipedia mentions, “baby-step giant-step” and “Pollard’s rho algorithm for logarithms” ring a bell from my university courses),
but they turned out not to be necessary;
the only optimization I implemented was to not recalculate the `mod_pow` for each iteration,
but instead keep the current result and multiply it once per iteration.

I expected that part 2 would require us to actually encrypt something,
but it turns out there is no part 2: the last star is free.
So now I have a struct that holds and cracks keys but has no actual encryption/decryption functionality.
Oh well :)

## Usage

```sh
./solve
```

This reads the input from the hard-coded `input` filename and prints the part 1 solution.
(There is no part 2.)

[day25]: https://adventofcode.com/2020/day/25
[Diffie-Hellman]: https://www.wikidata.org/wiki/Special:GoToLinkedPage/enwiki/Q623447
[discrete logarithm]: https://www.wikidata.org/wiki/Special:GoToLinkedPage/enwiki/Q864003
