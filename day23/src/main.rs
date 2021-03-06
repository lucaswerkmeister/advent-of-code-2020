use std::error::Error;
use std::fmt;
use std::fs;
use std::iter;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ParseStateError {
    BadLength(usize),
    BadCup(char),
    DuplicateCup,
}

impl fmt::Display for ParseStateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseStateError {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    cups: Vec<u32>,
}

impl State {
    fn new(
        c1: u32,
        c2: u32,
        c3: u32,
        c4: u32,
        c5: u32,
        c6: u32,
        c7: u32,
        c8: u32,
        c9: u32,
    ) -> Self {
        Self {
            cups: vec![c1, c2, c3, c4, c5, c6, c7, c8, c9],
        }
    }

    fn extend_to_one_million(&mut self) {
        self.cups.extend((self.cups.len() as u32 + 1)..=1_000_000);
    }

    fn do_move(&mut self) {
        // pop current cup from front
        let current_cup = self.cups.remove(0);
        // pick three cups after current cup
        let picked_cups = self.cups.splice(0..3, iter::empty()).collect::<Vec<_>>();
        // find destination cup
        let mut destination_cup = current_cup;
        let mut destination_cup_position = None;
        while destination_cup_position.is_none() {
            destination_cup -= 1;
            if destination_cup == 0 {
                destination_cup = (self.cups.len()
                    + 1 // current_cup
                    + 3) as u32; // picked_cups
            }
            destination_cup_position = self.cups.iter().position(|&cup| cup == destination_cup);
        }
        // insert picked cups after destination cup
        let picked_cups_position = destination_cup_position.unwrap() + 1;
        self.cups.splice(
            picked_cups_position..picked_cups_position,
            picked_cups.into_iter(),
        );
        // push current cup to end
        self.cups.push(current_cup);
    }

    fn rotate_to_1(&mut self) {
        let cup_1_position = self
            .cups
            .iter()
            .position(|&cup| cup == 1)
            .expect("cup 1 must exist");
        if cup_1_position != 0 {
            let cups_before_1 = self
                .cups
                .splice(0..cup_1_position, iter::empty())
                .collect::<Vec<_>>();
            let append_index = self.cups.len();
            self.cups
                .splice(append_index..append_index, cups_before_1.into_iter());
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cup in &self.cups {
            write!(f, "{}", cup)?;
        }
        Ok(())
    }
}

impl FromStr for State {
    type Err = ParseStateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 9 {
            return Err(ParseStateError::BadLength(s.len()));
        }
        let cups = s
            .chars()
            .map(|cup| cup.to_digit(10).ok_or_else(|| ParseStateError::BadCup(cup)))
            .collect::<Result<Vec<_>, _>>()?;
        if !(1..=9).all(|cup| cups.contains(&cup)) {
            return Err(ParseStateError::DuplicateCup);
        }
        Ok(Self { cups })
    }
}

fn part1(mut state: State) -> String {
    for _ in 0..100 {
        state.do_move();
    }
    state.rotate_to_1();
    state.to_string().chars().skip(1).collect()
}

fn part2(mut state: State) -> u64 {
    state.extend_to_one_million();
    for _ in 0..10_000_000 {
        state.do_move();
    }
    let cup_1_position = state
        .cups
        .iter()
        .position(|&cup| cup == 1)
        .expect("cup 1 must exist");
    let star_1_position = (cup_1_position + 1) % state.cups.len();
    let star_2_position = (cup_1_position + 2) % state.cups.len();
    state.cups[star_1_position] as u64 * state.cups[star_2_position] as u64
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: State = fs::read_to_string("input")?.trim().parse()?;
    println!("{}", part1(input.clone()));
    println!("{}", part2(input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_state() {
        assert_eq!(
            Ok(State::new(1, 2, 3, 4, 5, 6, 7, 8, 9)),
            "123456789".parse()
        );
        assert_eq!(
            Ok(State::new(3, 8, 9, 1, 2, 5, 4, 6, 7)),
            "389125467".parse()
        );

        assert_eq!(Err(ParseStateError::BadLength(5)), "32415".parse::<State>());
        assert_eq!(
            Err(ParseStateError::BadCup('-')),
            "-12345678".parse::<State>()
        );
        assert_eq!(
            Err(ParseStateError::DuplicateCup),
            "111111111".parse::<State>()
        );
    }

    #[test]
    fn test_display_state() {
        assert_eq!(
            "123456789",
            State::new(1, 2, 3, 4, 5, 6, 7, 8, 9).to_string()
        );
        assert_eq!(
            "389125467",
            State::new(3, 8, 9, 1, 2, 5, 4, 6, 7).to_string()
        );
    }

    #[test]
    fn test_do_move() {
        let mut state = State::new(3, 8, 9, 1, 2, 5, 4, 6, 7);
        state.do_move();
        assert_eq!("289154673", state.to_string());
        state.do_move();
        assert_eq!("546789132", state.to_string());
        state.do_move();
        assert_eq!("891346725", state.to_string());
        state.do_move();
        assert_eq!("467913258", state.to_string());
        state.do_move();
        assert_eq!("136792584", state.to_string());
        state.do_move();
        assert_eq!("936725841", state.to_string());
        state.do_move();
        assert_eq!("258367419", state.to_string());
        state.do_move();
        assert_eq!("674158392", state.to_string());
        state.do_move();
        assert_eq!("574183926", state.to_string());
        state.do_move();
        assert_eq!("837419265", state.to_string());
    }

    #[test]
    fn test_rotate_to_1() {
        let mut state = State::new(8, 3, 7, 4, 1, 9, 2, 6, 5);
        state.rotate_to_1();
        assert_eq!("192658374", state.to_string());
    }

    #[test]
    fn test_extend_to_one_million() {
        let mut state = State::new(3, 8, 9, 1, 2, 5, 4, 6, 7);
        state.extend_to_one_million();
        assert_eq!(1_000_000, state.cups.len());
        assert_eq!(10, state.cups[9]);
        assert_eq!(1_000, state.cups[999]);
        assert_eq!(1_000_000, state.cups[999_999]);
    }

    #[test]
    fn test_part1() {
        assert_eq!("67384529", part1("389125467".parse().unwrap()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(149245887792, part2("389125467".parse().unwrap()));
    }
}
