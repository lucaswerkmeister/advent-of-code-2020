#![feature(linked_list_cursors)]
#![feature(option_result_contains)]
use std::collections::LinkedList;
use std::error::Error;
use std::fmt;
use std::fs;
use std::mem;
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
    cups: LinkedList<u32>,
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
            cups: vec![c1, c2, c3, c4, c5, c6, c7, c8, c9].into_iter().collect(),
        }
    }

    fn extend_to_one_million(&mut self) {
        self.cups.extend((self.cups.len() as u32 + 1)..=1_000_000);
    }

    fn do_move(&mut self) {
        // pop current cup from front
        let current_cup = self.cups.pop_front().unwrap();
        // pick three cups after current cup
        let remaining_cups = self.cups.split_off(3);
        let picked_cups = mem::replace(&mut self.cups, remaining_cups);
        // find destination cup
        let mut destination_cup = current_cup - 1;
        if destination_cup == 0 {
            destination_cup = (self.cups.len()
                + 1 // current_cup
                + 3) as u32; // picked_cups
        }
        while picked_cups.contains(&destination_cup) {
            destination_cup -= 1;
            if destination_cup == 0 {
                destination_cup = (self.cups.len()
                    + 1 // current_cup
                    + 3) as u32; // picked_cups
            }
        }
        let mut cursor = self.cups.cursor_front_mut();
        while !cursor.current().contains(&&destination_cup) {
            cursor.move_next();
        }
        // insert picked cups after destination cup
        cursor.splice_after(picked_cups);
        // push current cup to end
        self.cups.push_back(current_cup);
    }

    fn rotate_to_1(&mut self) {
        let mut cursor = self.cups.cursor_front_mut();
        while !cursor.current().contains(&&1) {
            cursor.move_next();
        }
        let mut cups_before_1 = cursor.split_before();
        self.cups.append(&mut cups_before_1);
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
            .collect::<Result<LinkedList<_>, _>>()?;
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
    let mut cursor = state.cups.cursor_front();
    while !cursor.current().contains(&&1) {
        cursor.move_next();
    }
    cursor.move_next();
    while cursor.current().is_none() {
        cursor.move_next();
    }
    let cup_1 = *cursor.current().unwrap();
    cursor.move_next();
    while cursor.current().is_none() {
        cursor.move_next();
    }
    let cup_2 = *cursor.current().unwrap();
    cup_1 as u64 * cup_2 as u64
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
        let iter = state.cups.iter();
        let mut iter = iter.skip(9);
        assert_eq!(Some(&10), iter.next());
        let mut iter = iter.skip(989);
        assert_eq!(Some(&1_000), iter.next());
        assert_eq!(Some(&1_000_000), state.cups.back());
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
