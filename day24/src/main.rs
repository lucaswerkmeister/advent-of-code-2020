use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Floor {
    tiles: HashSet<(i64, i64)>,
}

impl Floor {
    fn new_from_flip_directions(directions: &str) -> Self {
        let mut tiles: HashMap<(i64, i64), bool> = HashMap::new();
        for line in directions.lines() {
            let mut x = 0;
            let mut y = 0;
            let mut chars = line.chars();
            while let Some(c) = chars.next() {
                match c {
                    'e' => x += 1,
                    'w' => x -= 1,
                    'n' => match chars.next().expect("must have character after 'n'") {
                        'e' => y += 1,
                        'w' => {
                            y += 1;
                            x -= 1;
                        }
                        c2 => panic!("unexpected character {} after 'n'", c2),
                    },
                    's' => match chars.next().expect("must have character after 'n'") {
                        'e' => {
                            y -= 1;
                            x += 1;
                        }
                        'w' => y -= 1,
                        c2 => panic!("unexpected character {} after 'n'", c2),
                    },
                    _ => panic!("unexpected character {}", c),
                }
            }
            *tiles.entry((x, y)).or_default() ^= true;
        }
        Self {
            tiles: tiles
                .into_iter()
                .filter(|&(_key, value)| value)
                .map(|(key, _value)| key)
                .collect(),
        }
    }

    fn neighbor_coordinates(&self, x: i64, y: i64) -> Vec<(i64, i64)> {
        vec![
            (x + 1, y),
            (x + 1, y - 1),
            (x, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y + 1),
        ]
    }

    fn neighbors(&self, x: i64, y: i64) -> usize {
        self.neighbor_coordinates(x, y)
            .into_iter()
            .filter(|&(x, y)| self.tiles.contains(&(x, y)))
            .count()
    }

    fn next_day(&self) -> Self {
        let potential_tiles: HashSet<(i64, i64)> = self
            .tiles
            .iter()
            .flat_map(|&(x, y)| self.neighbor_coordinates(x, y).into_iter())
            .collect();
        let mut tiles = HashSet::with_capacity(self.tiles.len());
        for (x, y) in potential_tiles {
            if self.tiles.contains(&(x, y)) {
                let neighbors = self.neighbors(x, y);
                if neighbors == 1 || neighbors == 2 {
                    tiles.insert((x, y));
                }
            } else {
                if self.neighbors(x, y) == 2 {
                    tiles.insert((x, y));
                }
            }
        }
        Self { tiles }
    }
}

fn part1(floor: &Floor) -> usize {
    floor.tiles.len()
}

fn part2(mut floor: Floor) -> usize {
    for _day in 0..100 {
        floor = floor.next_day();
    }
    floor.tiles.len()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let floor = Floor::new_from_flip_directions(&input);
    println!("{}", part1(&floor));
    println!("{}", part2(floor));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1() {
        let input = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
";
        let floor = Floor::new_from_flip_directions(&input);
        assert_eq!(10, part1(&floor));
    }

    #[test]
    fn test_part2() {
        let input = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
";
        let floor = Floor::new_from_flip_directions(&input);
        assert_eq!(2208, part2(floor));
    }
}
