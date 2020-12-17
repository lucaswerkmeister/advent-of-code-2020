use std::cmp::{max, min};
use std::collections::HashSet;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::fs;
use std::num::TryFromIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ParseStateError {
    BadCharacter(char),
    TooHigh(TryFromIntError),
    TooWide(TryFromIntError),
}

impl fmt::Display for ParseStateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseStateError {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State3d {
    active_cells: HashSet<(i64, i64, i64)>,
    cycles: u64,
    xs: RangeInclusive<i64>,
    ys: RangeInclusive<i64>,
    zs: RangeInclusive<i64>,
}

impl State3d {
    fn cycle(&self) -> Self {
        let mut potentially_active_cells = HashSet::with_capacity(self.active_cells.len() * 3);
        for (x, y, z) in &self.active_cells {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        potentially_active_cells.insert((x + dx, y + dy, z + dz));
                    }
                }
            }
        }
        let mut active_cells = HashSet::with_capacity(self.active_cells.len());
        let mut min_x = i64::MAX;
        let mut max_x = i64::MIN;
        let mut min_y = i64::MAX;
        let mut max_y = i64::MIN;
        let mut min_z = i64::MAX;
        let mut max_z = i64::MIN;
        for (x, y, z) in potentially_active_cells {
            let mut currently_active = false;
            let mut currently_active_neighbors = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        if self.active_cells.contains(&(x + dx, y + dy, z + dz)) {
                            if dx == 0 && dy == 0 && dz == 0 {
                                currently_active = true;
                            } else {
                                currently_active_neighbors += 1;
                            }
                        }
                    }
                }
            }
            if (currently_active
                && (currently_active_neighbors == 2 || currently_active_neighbors == 3))
                || (!currently_active && currently_active_neighbors == 3)
            {
                active_cells.insert((x, y, z));
                min_x = min(min_x, x);
                max_x = max(max_x, x);
                min_y = min(min_y, y);
                max_y = max(max_y, y);
                min_z = min(min_z, z);
                max_z = max(max_z, z);
            }
        }
        Self {
            cycles: self.cycles + 1,
            xs: min_x..=max_x,
            ys: min_y..=max_y,
            zs: min_z..=max_z,
            active_cells,
        }
    }
}

impl FromStr for State3d {
    type Err = ParseStateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut active_cells = HashSet::with_capacity(s.len());
        let mut max_x = 0;
        let mut max_y = 0;
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        let x = x.try_into().map_err(|e| ParseStateError::TooWide(e))?;
                        let y = y.try_into().map_err(|e| ParseStateError::TooHigh(e))?;
                        active_cells.insert((x, y, 0));
                    }
                    '.' => (),
                    _ => return Err(ParseStateError::BadCharacter(c)),
                }
                max_x = x;
            }
            max_y = y;
        }
        Ok(Self {
            active_cells,
            cycles: 0,
            xs: 0..=max_x
                .try_into()
                .expect("match already tested that each x fits in i64"),
            ys: 0..=max_y
                .try_into()
                .expect("match already tested that each y fits in i64"),
            zs: 0..=0,
        })
    }
}

impl fmt::Display for State3d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for z in self.zs.clone() {
            write!(f, "z={}\n", z)?;
            for y in self.ys.clone() {
                for x in self.xs.clone() {
                    write!(
                        f,
                        "{}",
                        if self.active_cells.contains(&(x, y, z)) {
                            '#'
                        } else {
                            '.'
                        }
                    )?;
                }
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State4d {
    active_cells: HashSet<(i64, i64, i64, i64)>,
    cycles: u64,
    xs: RangeInclusive<i64>,
    ys: RangeInclusive<i64>,
    zs: RangeInclusive<i64>,
    ws: RangeInclusive<i64>,
}

impl State4d {
    fn cycle(&self) -> Self {
        let mut potentially_active_cells = HashSet::with_capacity(self.active_cells.len() * 4);
        for (x, y, z, w) in &self.active_cells {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        for dw in -1..=1 {
                            potentially_active_cells.insert((x + dx, y + dy, z + dz, w + dw));
                        }
                    }
                }
            }
        }
        let mut active_cells = HashSet::with_capacity(self.active_cells.len());
        let mut min_x = i64::MAX;
        let mut max_x = i64::MIN;
        let mut min_y = i64::MAX;
        let mut max_y = i64::MIN;
        let mut min_z = i64::MAX;
        let mut max_z = i64::MIN;
        let mut min_w = i64::MAX;
        let mut max_w = i64::MIN;
        for (x, y, z, w) in potentially_active_cells {
            let mut currently_active = false;
            let mut currently_active_neighbors = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        for dw in -1..=1 {
                            if self
                                .active_cells
                                .contains(&(x + dx, y + dy, z + dz, w + dw))
                            {
                                if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                                    currently_active = true;
                                } else {
                                    currently_active_neighbors += 1;
                                }
                            }
                        }
                    }
                }
            }
            if (currently_active
                && (currently_active_neighbors == 2 || currently_active_neighbors == 3))
                || (!currently_active && currently_active_neighbors == 3)
            {
                active_cells.insert((x, y, z, w));
                min_x = min(min_x, x);
                max_x = max(max_x, x);
                min_y = min(min_y, y);
                max_y = max(max_y, y);
                min_z = min(min_z, z);
                max_z = max(max_z, z);
                min_w = min(min_w, w);
                max_w = max(max_w, w);
            }
        }
        Self {
            cycles: self.cycles + 1,
            xs: min_x..=max_x,
            ys: min_y..=max_y,
            zs: min_z..=max_z,
            ws: min_w..=max_w,
            active_cells,
        }
    }
}

impl From<State3d> for State4d {
    fn from(state: State3d) -> Self {
        Self {
            active_cells: state
                .active_cells
                .into_iter()
                .map(|(x, y, z)| (x, y, z, 0))
                .collect(),
            cycles: state.cycles,
            xs: state.xs,
            ys: state.ys,
            zs: state.zs,
            ws: 0..=0,
        }
    }
}

impl FromStr for State4d {
    type Err = ParseStateError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state3d: State3d = s.parse()?;
        Ok(state3d.into())
    }
}

impl fmt::Display for State4d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for w in self.ws.clone() {
            for z in self.zs.clone() {
                write!(f, "z={}, w={}\n", z, w)?;
                for y in self.ys.clone() {
                    for x in self.xs.clone() {
                        write!(
                            f,
                            "{}",
                            if self.active_cells.contains(&(x, y, z, w)) {
                                '#'
                            } else {
                                '.'
                            }
                        )?;
                    }
                    writeln!(f, "")?;
                }
            }
        }
        Ok(())
    }
}

fn part1(input: State3d) -> usize {
    let mut state = input;
    for _i in 0..6 {
        state = state.cycle();
    }
    state.active_cells.len()
}

fn part2(input: State4d) -> usize {
    let mut state = input;
    for _i in 0..6 {
        state = state.cycle();
    }
    state.active_cells.len()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: State3d = fs::read_to_string("input")?.parse()?;
    println!("{}", part1(input.clone()));
    println!("{}", part2(input.into()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_state3d_cycle() {
        #[rustfmt::skip]
        let input = State3d {
            active_cells: vec![
                (1, 0, 0),
                (2, 1, 0),
                (0, 2, 0),
                (1, 2, 0),
                (2, 2, 0),
            ].into_iter().collect(),
            cycles: 0,
            xs: 0..=2,
            ys: 0..=2,
            zs: 0..=0,
        };
        let output = input.cycle();
        #[rustfmt::skip]
        assert_eq!(State3d {
            active_cells: vec![
                (0, 1, -1),
                (2, 2, -1),
                (1, 3, -1),
                (0, 1, 0),
                (2, 1, 0),
                (1, 2, 0),
                (2, 2, 0),
                (1, 3, 0),
                (0, 1, 1),
                (2, 2, 1),
                (1, 3, 1),
            ].into_iter().collect(),
            cycles: 1,
            xs: 0..=2,
            ys: 1..=3,
            zs: -1..=1,
        }, output);
    }

    #[test]
    fn test_parse_state3d() {
        let input = "\
.#.
..#
###
";
        #[rustfmt::skip]
        assert_eq!(Ok(State3d {
            active_cells: vec![
                (1, 0, 0),
                (2, 1, 0),
                (0, 2, 0),
                (1, 2, 0),
                (2, 2, 0),
            ].into_iter().collect(),
            cycles: 0,
            xs: 0..=2,
            ys: 0..=2,
            zs: 0..=0,
        }), input.parse());
    }

    #[test]
    fn test_display_state3d() {
        #[rustfmt::skip]
        let state = State3d {
            active_cells: vec![
                (1, 0, 0),
                (2, 1, 0),
                (0, 2, 0),
                (1, 2, 0),
                (2, 2, 0),
            ].into_iter().collect(),
            cycles: 0,
            xs: 0..=2,
            ys: 0..=2,
            zs: 0..=0,
        };
        assert_eq!(
            "\
z=0
.#.
..#
###
",
            state.to_string()
        );
    }
}
