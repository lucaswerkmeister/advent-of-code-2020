use std::collections::{HashMap, VecDeque};
use std::collections::hash_map::Entry;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseIntError;

#[derive(Clone, Debug, Eq, PartialEq)]
enum PuzzleError {
    IoError, // does not include the io::Error, otherwise we couldn’t derive Clone+Eq+PartialEq
    ParseIntError(ParseIntError),
    InvalidSum(u64),
}

impl fmt::Display for PuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for PuzzleError {}

impl From<ParseIntError> for PuzzleError {
    fn from(e: ParseIntError) -> Self {
        PuzzleError::ParseIntError(e)
    }
}

impl From<io::Error> for PuzzleError {
    fn from(_e: io::Error) -> Self {
        PuzzleError::IoError
    }
}

fn part1<T: BufRead>(input: T, preamble_length: usize) -> Result<(), PuzzleError> {
    let mut buffer = VecDeque::<u64>::with_capacity(preamble_length);
    let mut sums = HashMap::<u64, u64>::with_capacity(preamble_length * preamble_length);
    let mut lines = input.lines();
    for line in lines.by_ref().take(preamble_length) {
        let num = line?.parse()?;
        for &num2 in &buffer {
            *sums.entry(num + num2).or_default() += 1;
        }
        buffer.push_back(num);
    };
    for line in lines {
        let num = line?.parse()?;
        if !sums.contains_key(&num) {
            return Err(PuzzleError::InvalidSum(num));
        }
        let former_num = buffer.pop_front().expect("empty ring buffer");
        for &num2 in &buffer {
            match sums.entry(former_num + num2) {
                Entry::Occupied(mut entry) => {
                    let sum = entry.get_mut();
                    *sum -= 1;
                    if *sum == 0 {
                        entry.remove();
                    }
                },
                _ => panic!("missing sum {}", former_num + num2),
            }
            *sums.entry(num + num2).or_default() += 1;
        }
        buffer.push_back(num);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    match part1(BufReader::new(file), 25) {
        Err(PuzzleError::InvalidSum(num)) => println!("{}", num),
        unexpected => panic!("{:?}", unexpected),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1() {
        let input = "
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
"
        .trim();
        let preamble_length = 5;

        assert_eq!(Err(PuzzleError::InvalidSum(127)), part1(BufReader::new(input.as_bytes()), preamble_length));
    }
}
