use im::{vector, Vector};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::fs;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ParseError {
    BadLength(usize),
    BadCharacter(char),
    BadWidth(usize, usize), // expected, actual
    WidthTooLarge,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl FromStr for Cell {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 1 {
            return Err(ParseError::BadLength(s.len()));
        }
        match s.chars().next() {
            Some('.') => Ok(Cell::Floor),
            Some('L') => Ok(Cell::EmptySeat),
            Some('#') => Ok(Cell::OccupiedSeat),
            Some(c) => Err(ParseError::BadCharacter(c)),
            None => Err(ParseError::BadLength(0)),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Floor => write!(f, "."),
            Cell::EmptySeat => write!(f, "L"),
            Cell::OccupiedSeat => write!(f, "#"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Grid {
    // width and height can’t actually be negative,
    // but we want to be able to index -1 easily
    width: isize,
    height: isize,
    cells: Vector<Cell>,
}

impl Grid {
    fn cell(&self, x: isize, y: isize) -> Option<Cell> {
        if 0 <= x && x < self.width && 0 <= y && y < self.height {
            Some(self.cells[usize::try_from(y * self.width + x).unwrap()])
        } else {
            None
        }
    }

    fn count_eq(&self, cell: Cell) -> usize {
        self.cells.iter().filter(|&c| *c == cell).count()
    }

    fn count_eq_neighbors(&self, cell: Cell, x: isize, y: isize) -> usize {
        #[rustfmt::skip]
        let deltas = vec![
            (-1, -1), (0, -1), (1, -1),
            (-1, 0), /*(0, 0),*/ (1, 0),
            (-1, 1), (0, 1), (1, 1),
        ];
        deltas
            .iter()
            .map(|&(dx, dy)| self.cell(x + dx, y + dy))
            .filter(|&neighbor| neighbor == Some(cell))
            .count()
    }

    fn count_eq_visible(&self, cell: Cell, x: isize, y: isize) -> usize {
        #[rustfmt::skip]
        let deltas = vec![
            (-1, -1), (0, -1), (1, -1),
            (-1, 0), /*(0, 0),*/ (1, 0),
            (-1, 1), (0, 1), (1, 1),
        ];
        deltas
            .iter()
            .map(|&(dx, dy)| {
                for i in 1.. {
                    match self.cell(x + i * dx, y + i * dy) {
                        Some(Cell::Floor) => continue,
                        Some(visible) => return visible,
                        None => return Cell::Floor,
                    }
                }
                Cell::Floor
            })
            .filter(|&neighbor| neighbor == cell)
            .count()
    }

    fn round_part1(&self) -> Grid {
        let mut new_cells = self.cells.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.cell(x, y).unwrap();
                if cell == Cell::Floor {
                    continue;
                }
                let occupied = self.count_eq_neighbors(Cell::OccupiedSeat, x, y);
                if cell == Cell::EmptySeat && occupied == 0 {
                    new_cells[usize::try_from(y * self.width + x).unwrap()] = Cell::OccupiedSeat;
                } else if cell == Cell::OccupiedSeat && occupied >= 4 {
                    new_cells[usize::try_from(y * self.width + x).unwrap()] = Cell::EmptySeat;
                }
            }
        }
        Grid {
            width: self.width,
            height: self.height,
            cells: new_cells,
        }
    }

    fn fixpoint_part1(self) -> Grid {
        let mut grid = self;
        let mut next_grid = grid.round_part1();
        while grid != next_grid {
            grid = next_grid;
            next_grid = grid.round_part1();
        }
        grid
    }

    fn round_part2(&self) -> Grid {
        let mut new_cells = self.cells.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.cell(x, y).unwrap();
                if cell == Cell::Floor {
                    continue;
                }
                let occupied = self.count_eq_visible(Cell::OccupiedSeat, x, y);
                if cell == Cell::EmptySeat && occupied == 0 {
                    new_cells[usize::try_from(y * self.width + x).unwrap()] = Cell::OccupiedSeat;
                } else if cell == Cell::OccupiedSeat && occupied >= 5 {
                    new_cells[usize::try_from(y * self.width + x).unwrap()] = Cell::EmptySeat;
                }
            }
        }
        Grid {
            width: self.width,
            height: self.height,
            cells: new_cells,
        }
    }

    fn fixpoint_part2(self) -> Grid {
        let mut grid = self;
        let mut next_grid = grid.round_part2();
        while grid != next_grid {
            grid = next_grid;
            next_grid = grid.round_part2();
        }
        grid
    }
}

impl FromStr for Grid {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let mut height = 0;
        let mut cells = vector![];
        for line in s.lines() {
            if width == 0 {
                width = line.len();
            } else if width != line.len() {
                return Err(ParseError::BadWidth(width, line.len()));
            }
            height += 1;
            for c in line.chars() {
                cells.push_back(c.to_string().parse()?);
            }
        }
        Ok(Grid {
            width: isize::try_from(width).map_err(|_| ParseError::WidthTooLarge)?,
            height,
            cells,
        })
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.cell(x, y).unwrap())?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn part1(input: &str) -> Result<usize, ParseError> {
    let grid: Grid = input.parse()?;
    let fix = grid.fixpoint_part1();
    Ok(fix.count_eq(Cell::OccupiedSeat))
}

fn part2(input: &str) -> Result<usize, ParseError> {
    let grid: Grid = input.parse()?;
    let fix = grid.fixpoint_part2();
    Ok(fix.count_eq(Cell::OccupiedSeat))
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_cell() {
        assert_eq!(Ok(Cell::Floor), ".".parse());
        assert_eq!(Ok(Cell::EmptySeat), "L".parse());
        assert_eq!(Ok(Cell::OccupiedSeat), "#".parse());
        assert_eq!(Err(ParseError::BadCharacter('X')), "X".parse::<Cell>());
        assert_eq!(Err(ParseError::BadLength(0)), "".parse::<Cell>());
        assert_eq!(Err(ParseError::BadLength(2)), "..".parse::<Cell>());
    }

    #[test]
    fn test_display_cell() {
        assert_eq!(".", Cell::Floor.to_string());
        assert_eq!("L", Cell::EmptySeat.to_string());
        assert_eq!("#", Cell::OccupiedSeat.to_string());
    }

    #[test]
    fn test_parse_grid() {
        let input = "\
#.#L
#LLL
L.#.
#L##
";
        let grid = Grid {
            width: 4,
            height: 4,
            #[rustfmt::skip]
            cells: vector![
                Cell::OccupiedSeat, Cell::Floor, Cell::OccupiedSeat, Cell::EmptySeat,
                Cell::OccupiedSeat, Cell::EmptySeat, Cell::EmptySeat, Cell::EmptySeat,
                Cell::EmptySeat, Cell::Floor, Cell::OccupiedSeat, Cell::Floor,
                Cell::OccupiedSeat, Cell::EmptySeat, Cell::OccupiedSeat, Cell::OccupiedSeat,
            ],
        };
        assert_eq!(Ok(grid), input.parse());
    }

    #[test]
    fn test_display_grid() {
        let grid = Grid {
            width: 4,
            height: 4,
            #[rustfmt::skip]
            cells: vector![
                Cell::OccupiedSeat, Cell::Floor, Cell::OccupiedSeat, Cell::EmptySeat,
                Cell::OccupiedSeat, Cell::EmptySeat, Cell::EmptySeat, Cell::EmptySeat,
                Cell::EmptySeat, Cell::Floor, Cell::OccupiedSeat, Cell::Floor,
                Cell::OccupiedSeat, Cell::EmptySeat, Cell::OccupiedSeat, Cell::OccupiedSeat,
            ],
        };
        let string = "\
#.#L
#LLL
L.#.
#L##
";
        assert_eq!(string, grid.to_string());
    }

    #[test]
    fn test_grid_cell() {
        let grid = "#.\nL#".parse::<Grid>().unwrap();
        assert_eq!(Some(Cell::OccupiedSeat), grid.cell(0, 0));
        assert_eq!(Some(Cell::Floor), grid.cell(1, 0));
        assert_eq!(Some(Cell::EmptySeat), grid.cell(0, 1));
        assert_eq!(Some(Cell::OccupiedSeat), grid.cell(1, 1));
        assert_eq!(None, grid.cell(2, 0));
        assert_eq!(None, grid.cell(-1, 0));
        assert_eq!(None, grid.cell(0, 2));
        assert_eq!(None, grid.cell(0, -1));
    }

    #[test]
    fn test_grid_count_eq() {
        let grid: Grid = "\
#.#L
#LLL
L.#.
#L##
"
        .parse()
        .unwrap();
        assert_eq!(3, grid.count_eq(Cell::Floor));
        assert_eq!(6, grid.count_eq(Cell::EmptySeat));
        assert_eq!(7, grid.count_eq(Cell::OccupiedSeat));
    }

    #[test]
    fn test_grid_count_eq_neighbors() {
        let grid: Grid = "\
#.#L
#LLL
L.#.
#L##
"
        .parse()
        .unwrap();
        assert_eq!(1, grid.count_eq_neighbors(Cell::OccupiedSeat, 0, 0));
        assert_eq!(4, grid.count_eq_neighbors(Cell::OccupiedSeat, 1, 1));
        assert_eq!(3, grid.count_eq_neighbors(Cell::OccupiedSeat, 1, 3));
    }

    #[test]
    fn test_grid_count_eq_visible() {
        let grid_1: Grid = "\
.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....
"
        .parse()
        .unwrap();
        assert_eq!(8, grid_1.count_eq_visible(Cell::OccupiedSeat, 3, 4));

        let grid_2: Grid = "\
.............
.L.L.#.#.#.#.
.............
"
        .parse()
        .unwrap();
        assert_eq!(1, grid_2.count_eq_visible(Cell::EmptySeat, 1, 1));
        assert_eq!(0, grid_2.count_eq_visible(Cell::OccupiedSeat, 1, 1));

        let grid_3: Grid = "\
.##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.
"
        .parse()
        .unwrap();
        assert_eq!(0, grid_3.count_eq_visible(Cell::OccupiedSeat, 3, 3));
    }

    #[test]
    fn test_grid_round_part1() {
        let round_0 = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";
        let round_1 = "\
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
";
        let round_2 = "\
#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##
";
        let round_3 = "\
#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##
";
        let round_4 = "\
#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##
";
        let round_5 = "\
#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##
";

        let mut grid: Grid = round_0.parse().unwrap();
        grid = grid.round_part1();
        assert_eq!(round_1, grid.to_string());
        grid = grid.round_part1();
        assert_eq!(round_2, grid.to_string());
        grid = grid.round_part1();
        assert_eq!(round_3, grid.to_string());
        grid = grid.round_part1();
        assert_eq!(round_4, grid.to_string());
        grid = grid.round_part1();
        assert_eq!(round_5, grid.to_string());
        grid = grid.round_part1();
        assert_eq!(round_5, grid.to_string());
    }

    #[test]
    fn test_grid_fixpoint_part1() {
        let initial: Grid = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"
        .parse()
        .unwrap();
        let fix: Grid = "\
#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##
"
        .parse()
        .unwrap();
        assert_eq!(fix, initial.fixpoint_part1());
    }

    #[test]
    fn test_part1() {
        let input = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";
        assert_eq!(37, part1(&input).unwrap());
    }

    #[test]
    fn test_grid_round_part2() {
        let round_0 = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";
        let round_1 = "\
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
";
        let round_2 = "\
#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#
";
        let round_3 = "\
#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#
";
        let round_4 = "\
#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#
";
        let round_5 = "\
#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#
";
        let round_6 = "\
#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#
";

        let mut grid: Grid = round_0.parse().unwrap();
        grid = grid.round_part2();
        assert_eq!(round_1, grid.to_string());
        grid = grid.round_part2();
        assert_eq!(round_2, grid.to_string());
        grid = grid.round_part2();
        assert_eq!(round_3, grid.to_string());
        grid = grid.round_part2();
        assert_eq!(round_4, grid.to_string());
        grid = grid.round_part2();
        assert_eq!(round_5, grid.to_string());
        grid = grid.round_part2();
        assert_eq!(round_6, grid.to_string());
        grid = grid.round_part2();
        assert_eq!(round_6, grid.to_string());
    }

    #[test]
    fn test_grid_fixpoint_part2() {
        let initial: Grid = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"
        .parse()
        .unwrap();
        let fix: Grid = "\
#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#
"
        .parse()
        .unwrap();
        assert_eq!(fix, initial.fixpoint_part2());
    }

    #[test]
    fn test_part2() {
        let input = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";
        assert_eq!(26, part2(&input).unwrap());
    }
}
