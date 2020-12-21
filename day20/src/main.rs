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

    fn turn(&self) -> Self {
        let mut turned = Vec::with_capacity(100);
        for y in 0..10 {
            for x in 0..10 {
                turned.push(self.data[10 * (9 - x) + y]);
            }
        }
        Self {
            id: self.id,
            data: turned.try_into().unwrap(),
        }
    }

    fn flip(&self) -> Self {
        let mut flipped = Vec::with_capacity(100);
        for y in 0..10 {
            for x in 0..10 {
                flipped.push(self.data[10 * x + y]);
            }
        }
        Self {
            id: self.id,
            data: flipped.try_into().unwrap(),
        }
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
    tile_ids_by_edge: HashMap<u16, HashSet<u64>>,
    corner_tile_ids: [u64; 4],
}

impl Input {
    fn new(tiles: Vec<Tile>) -> Self {
        let tiles: HashMap<u64, Tile> = tiles.into_iter().map(|tile| (tile.id, tile)).collect();

        let mut tile_ids_by_edge = HashMap::<u16, HashSet<u64>>::with_capacity(tiles.len() * 4);
        for &tile in tiles.values() {
            for &edge in tile.edges().iter() {
                tile_ids_by_edge.entry(edge).or_default().insert(tile.id);
            }
            for &edge in tile.flipped_edges().iter() {
                tile_ids_by_edge.entry(edge).or_default().insert(tile.id);
            }
        }

        let mut shared_edges_per_tile_id = HashMap::<u64, usize>::with_capacity(tiles.len());
        for tile_ids in tile_ids_by_edge.values() {
            if tile_ids.len() > 1 {
                for tile_id in tile_ids {
                    *shared_edges_per_tile_id.entry(*tile_id).or_default() += 1;
                }
            }
        }

        let corner_tile_ids: [u64; 4] = shared_edges_per_tile_id
            .iter()
            .filter(|(&_tile_id, &shared_edges)| shared_edges == 4)
            .map(|(tile_id, _shared_edges)| *tile_id)
            .collect::<Vec<_>>()
            .try_into()
            .expect("Corner tile IDs should be unambiguous");

        Self {
            tiles,
            tile_ids_by_edge,
            corner_tile_ids,
        }
    }
}

impl FromStr for Input {
    type Err = ParseTileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input::new(
            s.trim()
                .split("\n\n")
                .map(|tile| tile.parse::<Tile>())
                .collect::<Result<Vec<_>, _>>()?,
        ))
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

struct Image {
    data: Vec<bool>,
    width: usize,
    height: usize,
}

impl Image {
    fn new(input: &Input) -> Self {
        let mut initial_tile = input.tiles[&input.corner_tile_ids[0]];
        while input.tile_ids_by_edge[&initial_tile.edges()[0]].len() > 1
            || input.tile_ids_by_edge[&initial_tile.edges()[3]].len() > 1
        {
            initial_tile = initial_tile.turn();
        }
        let mut tiles = Vec::with_capacity(input.tiles.len());
        tiles.push(initial_tile);
        let mut current_row = Vec::with_capacity((input.tiles.len() as f64).sqrt() as usize);
        current_row.push(initial_tile);

        while tiles.len() < input.tiles.len() {
            let last_tile = current_row.last().unwrap();
            let right_edge = last_tile.edges()[1];
            if let Some(next_tile_id) = input.tile_ids_by_edge[&right_edge]
                .iter()
                .find(|&id| *id != last_tile.id)
            {
                let next_tile = input.tiles[next_tile_id];
                let next_tile = match next_tile
                    .edges()
                    .iter()
                    .position(|&edge| edge == right_edge)
                {
                    Some(0) => next_tile.flip(),
                    Some(1) => next_tile.flip().turn(),
                    Some(2) => next_tile.turn(),
                    Some(3) => next_tile,
                    _ => match next_tile
                        .flipped_edges()
                        .iter()
                        .position(|&edge| edge == right_edge)
                    {
                        Some(0) => next_tile.turn().turn().turn(),
                        Some(1) => next_tile.turn().turn(),
                        Some(2) => next_tile.flip().turn().turn(),
                        Some(3) => next_tile.flip().turn().turn().turn(),
                        _ => panic!(
                            "Tile {} does not go to the right of tile {} in any rotation",
                            next_tile.id, last_tile.id
                        ),
                    },
                };
                assert_eq!(next_tile.edges()[3], right_edge);
                tiles.push(next_tile);
                current_row.push(next_tile);
            } else {
                let last_tile = current_row.first().unwrap();
                let bottom_edge = last_tile.edges()[2];
                let next_tile_id = input.tile_ids_by_edge[&bottom_edge]
                    .iter()
                    .find(|&id| *id != last_tile.id)
                    .unwrap();
                let next_tile = input.tiles[next_tile_id];
                let next_tile = match next_tile
                    .edges()
                    .iter()
                    .position(|&edge| edge == bottom_edge)
                {
                    Some(0) => next_tile,
                    Some(1) => next_tile.turn().turn().turn(),
                    Some(2) => next_tile.flip().turn().turn().turn(),
                    Some(3) => next_tile.flip(),
                    _ => match next_tile
                        .flipped_edges()
                        .iter()
                        .position(|&edge| edge == bottom_edge)
                    {
                        Some(0) => next_tile.flip().turn(),
                        Some(1) => next_tile.flip().turn().turn(),
                        Some(2) => next_tile.turn().turn(),
                        Some(3) => next_tile.turn(),
                        _ => panic!(
                            "Tile {} does go below tile {} in any rotation",
                            next_tile.id, last_tile.id
                        ),
                    },
                };
                assert_eq!(next_tile.edges()[0], bottom_edge);
                tiles.push(next_tile);
                current_row.clear();
                current_row.push(next_tile);
            }
        }

        let width_in_tiles = (tiles.len() as f64).sqrt() as usize;
        let height_in_tiles = width_in_tiles;
        let width = width_in_tiles * 8;
        let height = height_in_tiles * 8;
        let mut data = vec![false; width * height];

        for y in 0..height {
            let tile_y = y / 8;
            for x in 0..width {
                let tile_x = x / 8;
                let tile_index = tile_y * width_in_tiles + tile_x;
                let index_in_tile = ((y % 8) + 1) * 10 + (x % 8) + 1;
                data[y * width + x] = tiles[tile_index].data[index_in_tile];
            }
        }

        Self {
            data,
            width,
            height,
        }
    }

    fn turn(&self) -> Self {
        let mut turned = Vec::with_capacity(self.width * self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                turned.push(self.data[self.width * (self.height - 1 - y) + x]);
            }
        }
        Self {
            data: turned,
            width: self.height,
            height: self.width,
        }
    }

    fn flip(&self) -> Self {
        let mut flipped = Vec::with_capacity(self.width * self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                flipped.push(self.data[self.width * y + x]);
            }
        }
        Self {
            data: flipped,
            width: self.height,
            height: self.width,
        }
    }

    fn pixel(&self, x: usize, y: usize) -> bool {
        self.data[self.width * y + x]
    }

    fn count_sea_monsters(&self) -> usize {
        let mut monsters = 0;
        for y in 0..(self.height - 2) {
            for x in 0..(self.width - 19) {
                if !self.pixel(x, y + 1) {
                    continue;
                }
                if !self.pixel(x + 1, y + 2) {
                    continue;
                }
                if !self.pixel(x + 4, y + 2) {
                    continue;
                }
                if !self.pixel(x + 5, y + 1) {
                    continue;
                }
                if !self.pixel(x + 6, y + 1) {
                    continue;
                }
                if !self.pixel(x + 7, y + 2) {
                    continue;
                }
                if !self.pixel(x + 10, y + 2) {
                    continue;
                }
                if !self.pixel(x + 11, y + 1) {
                    continue;
                }
                if !self.pixel(x + 12, y + 1) {
                    continue;
                }
                if !self.pixel(x + 13, y + 2) {
                    continue;
                }
                if !self.pixel(x + 16, y + 2) {
                    continue;
                }
                if !self.pixel(x + 17, y + 1) {
                    continue;
                }
                if !self.pixel(x + 18, y) {
                    continue;
                }
                if !self.pixel(x + 18, y + 1) {
                    continue;
                }
                if !self.pixel(x + 19, y + 1) {
                    continue;
                }
                monsters += 1;
            }
        }
        monsters
    }

    fn count_set_pixels(&self) -> usize {
        self.data.iter().filter(|&pixel| *pixel).count()
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    if self.data[y * self.width + x] {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1(input: &Input) -> u64 {
    input.corner_tile_ids.iter().product()
}

fn part2(input: &Input) -> usize {
    let mut image = Image::new(input);
    let mut sea_monsters = image.count_sea_monsters();
    for _ in 0..3 {
        if sea_monsters > 0 {
            break;
        }
        image = image.turn();
        sea_monsters = image.count_sea_monsters();
    }
    if sea_monsters == 0 {
        image = image.flip();
        sea_monsters = image.count_sea_monsters();
        for _ in 0..3 {
            if sea_monsters > 0 {
                break;
            }
            image = image.turn();
            sea_monsters = image.count_sea_monsters();
        }
    }
    assert!(sea_monsters > 0);
    image.count_set_pixels() - sea_monsters * 15
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: Input = fs::read_to_string("input")?.parse()?;
    println!("{}", part1(&input));
    println!("{}", part2(&input));
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

    #[test]
    fn test_tile_turn() {
        assert_eq!(
            Tile {
                id: 2311,
                #[rustfmt::skip]
                data: [
                    false, true, false, false, true, true, true, true, true, false,
                    false, true, false, true, true, true, true, false, true, false,
                    true, true, true, false, false, false, true, false, false, true,
                    true, false, false, true, false, true, true, false, false, true,
                    true, false, false, false, false, true, false, true, true, false,
                    false, false, false, true, true, false, true, true, false, true,
                    false, true, false, false, false, true, false, false, false, false,
                    true, false, true, false, true, true, false, false, false, false,
                    true, true, false, true, true, true, false, true, false, true,
                    true, false, false, true, true, false, true, false, false, false,
                ],
            },
            sample_tile().turn()
        );
    }

    #[test]
    fn test_tile_flip() {
        assert_eq!(
            Tile {
                id: 2311,
                #[rustfmt::skip]
                data: [
                    false, true, true, true, true, true, false, false, true, false,
                    false, true, false, true, true, true, true, false, true, false,
                    true, false, false, true, false, false, false, true, true, true,
                    true, false, false, true, true, false, true, false, false, true,
                    false, true, true, false, true, false, false, false, false, true,
                    true, false, true, true, false, true, true, false, false, false,
                    false, false, false, false, true, false, false, false, true, false,
                    false, false, false, false, true, true, false, true, false, true,
                    true, false, true, false, true, true, true, false, true, true,
                    false, false, false, true, false, true, true, false, false, true,
                ],
            },
            sample_tile().flip()
        );
    }
}
