use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::fs;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ParseTileError {
    NoHeader,
    BadHeaderStart,
    BadHeaderEnd,
    BadHeaderId,
    BadLineLength(usize),
    BadChar(char),
    BadDataLength,
}

impl fmt::Display for ParseTileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseTileError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Tile {
    id: u64,
    data: [bool; 100],
}

impl Tile {
    fn edges(&self) -> [u16; 4] {
        [
            self.data
                .iter()
                .take(10)
                .fold(0, |acc, &bit| acc << 1 | bit as u16),
            self.data
                .iter()
                .skip(9)
                .step_by(10)
                .fold(0, |acc, &bit| acc << 1 | bit as u16),
            self.data
                .iter()
                .skip(90)
                .fold(0, |acc, &bit| acc << 1 | bit as u16),
            self.data
                .iter()
                .step_by(10)
                .fold(0, |acc, &bit| acc << 1 | bit as u16),
        ]
    }

    fn flipped_edges(&self) -> [u16; 4] {
        [
            self.data
                .iter()
                .take(10)
                .rev()
                .fold(0, |acc, &bit| acc << 1 | bit as u16),
            self.data
                .iter()
                .skip(9)
                .step_by(10)
                .rev()
                .fold(0, |acc, &bit| acc << 1 | bit as u16),
            self.data
                .iter()
                .skip(90)
                .rev()
                .fold(0, |acc, &bit| acc << 1 | bit as u16),
            self.data
                .iter()
                .step_by(10)
                .rev()
                .fold(0, |acc, &bit| acc << 1 | bit as u16),
        ]
    }
}

impl FromStr for Tile {
    type Err = ParseTileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let header = lines.next().ok_or(ParseTileError::NoHeader)?;
        if !header.starts_with("Tile ") {
            return Err(ParseTileError::BadHeaderStart);
        }
        let header = &header[5..];
        if !header.ends_with(':') {
            return Err(ParseTileError::BadHeaderEnd);
        }
        let header = &header[..header.len() - 1];
        let id: u64 = header.parse().map_err(|_| ParseTileError::BadHeaderId)?;
        let data: Vec<bool> = lines
            .map(|line| {
                if line.len() != 10 {
                    return Err(ParseTileError::BadLineLength(line.len()));
                }
                line.chars()
                    .map(|c| match c {
                        '.' => Ok(false),
                        '#' => Ok(true),
                        _ => Err(ParseTileError::BadChar(c)),
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?
            .concat();
        Ok(Tile {
            id,
            data: data.try_into().map_err(|_| ParseTileError::BadDataLength)?,
        })
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tile {}:", self.id)?;
        for y in 0..10 {
            for x in 0..10 {
                write!(f, "{}", if self.data[y * 10 + x] { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Input {
    tiles: HashMap<u64, Tile>,
}

impl FromStr for Input {
    type Err = ParseTileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input {
            tiles: s
                .trim()
                .split("\n\n")
                .map(|tile| tile.parse::<Tile>())
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|tile| (tile.id, tile))
                .collect(),
        })
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for &tile in self.tiles.values() {
            if first {
                first = false;
            } else {
                writeln!(f)?;
            }
            writeln!(f, "{}", tile)?;
        }
        Ok(())
    }
}

fn part1(input: &Input) -> u64 {
    let mut tiles_by_edge = HashMap::<u16, HashSet<u64>>::with_capacity(input.tiles.len() * 4);
    for &tile in input.tiles.values() {
        for &edge in tile.edges().iter() {
            tiles_by_edge.entry(edge).or_default().insert(tile.id);
        }
        for &edge in tile.flipped_edges().iter() {
            tiles_by_edge.entry(edge).or_default().insert(tile.id);
        }
    }
    let mut shared_edges_per_tile = HashMap::<u64, usize>::with_capacity(input.tiles.len());
    for tile_ids in tiles_by_edge.values() {
        if tile_ids.len() > 1 {
            for tile_id in tile_ids {
                *shared_edges_per_tile.entry(*tile_id).or_default() += 1;
            }
        }
    }
    shared_edges_per_tile
        .iter()
        .filter(|(&_tile_id, &shared_edges)| shared_edges == 4)
        .map(|(tile_id, _shared_edges)| tile_id)
        .product()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: Input = fs::read_to_string("input")?.parse()?;
    println!("{}", part1(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const SAMPLE_TILE: &str = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###
";

    fn sample_tile() -> Tile {
        Tile {
            id: 2311,
            #[rustfmt::skip]
            data: [
                false, false, true, true, false, true, false, false, true, false,
                true, true, false, false, true, false, false, false, false, false,
                true, false, false, false, true, true, false, false, true, false,
                true, true, true, true, false, true, false, false, false, true,
                true, true, false, true, true, false, true, true, true, false,
                true, true, false, false, false, true, false, true, true, true,
                false, true, false, true, false, true, false, false, true, true,
                false, false, true, false, false, false, false, true, false, false,
                true, true, true, false, false, false, true, false, true, false,
                false, false, true, true, true, false, false, true, true, true,
            ],
        }
    }

    #[test]
    fn test_parse_tile() {
        assert_eq!(Ok(sample_tile()), SAMPLE_TILE.parse());
    }

    #[test]
    fn test_display_tile() {
        assert_eq!(SAMPLE_TILE, sample_tile().to_string());
    }

    #[test]
    fn test_tile_edges() {
        assert_eq!(
            [0b0011010010, 0b0001011001, 0b0011100111, 0b0111110010,],
            sample_tile().edges()
        );
    }

    #[test]
    fn test_tile_flipped_edges() {
        assert_eq!(
            [0b0100101100, 0b1001101000, 0b1110011100, 0b0100111110,],
            sample_tile().flipped_edges()
        );
    }
}
