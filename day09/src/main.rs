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
    NoBadSum,
    NoConsecutiveSum,
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

fn parse_input<T: BufRead>(input: T) -> Result<Vec<u64>, PuzzleError> {
    input.lines().map(|line| line?.parse().map_err(PuzzleError::from)).collect()
}

fn find_bad_sum(nums: &[u64], preamble_length: usize) -> Option<u64> {
    let mut buffer = VecDeque::<u64>::with_capacity(preamble_length);
    let mut sums = HashMap::<u64, u64>::with_capacity(preamble_length * preamble_length);
    let mut nums = nums.iter();
    for num in nums.by_ref().take(preamble_length) {
        for &num2 in &buffer {
            *sums.entry(num + num2).or_default() += 1;
        }
        buffer.push_back(*num);
    };
    for num in nums {
        if !sums.contains_key(num) {
            return Some(*num);
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
        buffer.push_back(*num);
    }
    None
}

fn part1<T: BufRead>(input: T, preamble_length: usize) -> Result<u64, PuzzleError> {
    let input = parse_input(input)?;
    find_bad_sum(&input, preamble_length).ok_or(PuzzleError::NoBadSum)
}

fn part2<T: BufRead>(input: T, preamble_length: usize) -> Result<u64, PuzzleError> {
    let input = parse_input(input)?;
    let bad_sum = find_bad_sum(&input, preamble_length).ok_or(PuzzleError::NoBadSum)?;
    for upper_limit in 1..input.len() {
        for lower_limit in 0..upper_limit {
            let range = &input[lower_limit..upper_limit+1];
            if bad_sum == range.iter().sum() {
                return Ok(range.iter().min().expect("empty iterator") + range.iter().max().expect("empty iterator"));
            }
        }
    }
    Err(PuzzleError::NoConsecutiveSum)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", part1(BufReader::new(File::open("input")?), 25)?);
    println!("{}", part2(BufReader::new(File::open("input")?), 25)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const SAMPLE_INPUT: &str = "\
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
";
    const SAMPLE_PREAMBLE_LENGTH: usize = 5;

    #[test]
    fn test_part1() {
        assert_eq!(Ok(127), part1(BufReader::new(SAMPLE_INPUT.as_bytes()), SAMPLE_PREAMBLE_LENGTH));
    }

    #[test]
    fn test_part2() {
        assert_eq!(Ok(62), part2(BufReader::new(SAMPLE_INPUT.as_bytes()), SAMPLE_PREAMBLE_LENGTH));
    }
}
