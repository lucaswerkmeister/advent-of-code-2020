use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Square {
    Space,
    Tree,
}

impl TryFrom<char> for Square {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Square::Space),
            '#' => Ok(Square::Tree),
            _ => Err(ParseError {}),
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Square::Space => '.',
            Square::Tree => '#',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    squares: Vec<Square>,
    width: usize,
    height: usize,
}

impl Map {
    fn square_at(&self, x: usize, y: usize) -> Option<&Square> {
        if y < self.height {
            Some(&self.squares[y * self.width + x % self.width])
        } else {
            None
        }
    }
}

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut squares: Vec<Square> = vec![];
        let mut width = 0;
        let mut height = 0;

        let mut lines = s.lines();
        let first_line = lines.next().ok_or(ParseError {})?;
        for c in first_line.chars() {
            squares.push(Square::try_from(c)?);
            width += 1;
        }
        height += 1;

        for line in lines {
            if line.len() != width {
                return Err(ParseError {});
            }
            for c in line.chars() {
                squares.push(Square::try_from(c)?);
            }
            height += 1;
        }

        Ok(Map {
            squares,
            width,
            height,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let map = Map::from_str(&fs::read_to_string("input")?)?;
    let mut x = 0;
    let mut trees = 0;
    for y in 0..map.height {
        let square = map.square_at(x, y).unwrap();
        if square == &Square::Tree {
            trees += 1;
        }
        x += 3;
    }
    println!("{}", trees);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_space() {
        assert_eq!(Ok(Square::Space), Square::try_from('.'));
    }

    #[test]
    fn parse_tree() {
        assert_eq!(Ok(Square::Tree), Square::try_from('#'));
    }

    #[test]
    fn parse_map() {
        let map = Map::from_str(
            r#"
.#.#.#
......
######
##..##

"#
            .trim(),
        )
        .unwrap();
        assert_eq!(6, map.width);
        assert_eq!(4, map.height);
    }

    #[test]
    fn square_at() {
        let map = Map::from_str(
            r#"
.#.#.#
......
######
##..##

"#
            .trim(),
        )
        .unwrap();

        assert_eq!(&Square::Space, map.square_at(0, 0).unwrap());
        assert_eq!(&Square::Tree, map.square_at(1, 0).unwrap());
        assert_eq!(&Square::Space, map.square_at(6, 0).unwrap());
        assert_eq!(&Square::Tree, map.square_at(7, 0).unwrap());
        assert_eq!(&Square::Space, map.square_at(0, 1).unwrap());
        assert_eq!(&Square::Tree, map.square_at(0, 2).unwrap());
        assert_eq!(&Square::Tree, map.square_at(5, 3).unwrap());
        assert_eq!(&Square::Space, map.square_at(8, 3).unwrap());
        assert_eq!(None, map.square_at(0, 4));
    }
}
