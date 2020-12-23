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
    cups: Vec<u8>,
}

impl State {
    fn new(c1: u8, c2: u8, c3: u8, c4: u8, c5: u8, c6: u8, c7: u8, c8: u8, c9: u8) -> Self {
        let state = Self {
            cups: vec![c1, c2, c3, c4, c5, c6, c7, c8, c9],
        };
        assert!(state.valid());
        state
    }

    fn valid(&self) -> bool {
        self.cups.len() == 9 && (1..=9).all(|cup| self.cups.contains(&cup))
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
                destination_cup = 9;
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
        // check that nothing went wrong (only in debug mode)
        debug_assert!(self.valid());
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
        // check that nothing went wrong (only in debug mode)
        debug_assert!(self.valid());
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
            .map(|cup| {
                cup.to_digit(10)
                    .map(|c| c as u8)
                    .ok_or_else(|| ParseStateError::BadCup(cup))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let state = Self { cups };
        if state.valid() {
            Ok(state)
        } else {
            Err(ParseStateError::DuplicateCup) // only error possible at this point
        }
    }
}

fn part1(mut state: State) -> String {
    for _ in 0..100 {
        state.do_move();
    }
    state.rotate_to_1();
    state.to_string().chars().skip(1).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: State = fs::read_to_string("input")?.trim().parse()?;
    println!("{}", part1(input.clone()));
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
    fn test_part1() {
        assert_eq!("67384529", part1("389125467".parse().unwrap()));
    }
}
