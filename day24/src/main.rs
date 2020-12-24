use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn part1(input: &str) -> usize {
    let mut tiles: HashMap<(i64, i64), bool> = HashMap::new();
    for line in input.lines() {
        let mut x = 0;
        let mut y = 0;
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            match c {
                'e' => x += 1,
                'w' => x -= 1,
                'n' => match chars.next().expect("must have character after 'n'") {
                    'e' => y += 1,
                    'w' => { y += 1; x -= 1; },
                    c2 => panic!("unexpected character {} after 'n'", c2),
                },
                's' => match chars.next().expect("must have character after 'n'") {
                    'e' => { y -= 1; x += 1; },
                    'w' => y -= 1,
                    c2 => panic!("unexpected character {} after 'n'", c2),
                },
                _ => panic!("unexpected character {}", c),
            }
        }
        *tiles.entry((x, y)).or_default() ^= true;
    }
    tiles.values().filter(|&tile| *tile).count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    println!("{}", part1(&input));
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
        assert_eq!(10, part1(&input));
    }
}
