use std::error::Error;
use std::fmt;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
enum ParseExprError {
    BadNum(ParseIntError),
    NoCloseParen,
    UnknownOperator(char),
    TrailingGarbage,
}

impl fmt::Display for ParseExprError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseExprError {}

impl From<ParseIntError> for ParseExprError {
    fn from(e: ParseIntError) -> Self {
        Self::BadNum(e)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Expr {
    Lit(u64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Par(Box<Expr>),
}

impl Expr {
    fn eval(&self) -> u64 {
        match self {
            Self::Lit(num) => *num,
            Self::Add(left, right) => left.eval() + right.eval(),
            Self::Mul(left, right) => left.eval() * right.eval(),
            Self::Par(expr) => expr.eval(),
        }
    }

    fn lit(s: &str) -> Result<(Self, Option<&str>), ParseExprError> {
        match s.find(|c: char| !c.is_ascii_digit()) {
            Some(i) => Ok((Self::Lit(s[..i].parse()?), Some(&s[i..].trim()))),
            None => Ok((Self::Lit(s.parse()?), None)),
        }
    }

    fn par(s: &str) -> Result<(Self, Option<&str>), ParseExprError> {
        if s.starts_with("(") {
            let (expr, rest) = Self::expr(&s[1..])?;
            match rest {
                Some(rest) if rest.starts_with(")") => {
                    let rest = rest[1..].trim();
                    let rest = if rest.is_empty() { None } else { Some(rest) };
                    Ok((Self::Par(Box::new(expr)), rest))
                }
                _ => Err(ParseExprError::NoCloseParen),
            }
        } else {
            panic!("par() called with non-par string: {}", s);
        }
    }

    fn atom(s: &str) -> Result<(Self, Option<&str>), ParseExprError> {
        if s.starts_with("(") {
            Self::par(s)
        } else {
            Self::lit(s)
        }
    }

    fn expr(s: &str) -> Result<(Self, Option<&str>), ParseExprError> {
        let mut cur = Self::atom(s)?;
        while let Some(rest) = cur.1 {
            if rest.starts_with(")") {
                break;
            }
            if rest.starts_with("+") {
                let (atom, rest) = Self::atom(&rest[1..].trim())?;
                cur = (Self::Add(Box::new(cur.0), Box::new(atom)), rest);
            } else if rest.starts_with("*") {
                let (atom, rest) = Self::atom(&rest[1..].trim())?;
                cur = (Self::Mul(Box::new(cur.0), Box::new(atom)), rest);
            } else {
                return Err(ParseExprError::UnknownOperator(
                    rest.chars().next().unwrap(),
                ));
            }
        }
        Ok(cur)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lit(num) => write!(f, "{}", num),
            Self::Add(left, right) => write!(f, "{} + {}", left, right),
            Self::Mul(left, right) => write!(f, "{} * {}", left, right),
            Self::Par(expr) => write!(f, "({})", expr),
        }
    }
}

impl From<u64> for Expr {
    fn from(num: u64) -> Self {
        Self::Lit(num)
    }
}

impl FromStr for Expr {
    type Err = ParseExprError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (expr, rest) = Self::expr(s)?;
        if rest.is_some() {
            return Err(ParseExprError::TrailingGarbage);
        }
        Ok(expr)
    }
}

fn part1(exprs: &[Expr]) -> u64 {
    exprs.iter().map(Expr::eval).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let exprs: Vec<Expr> = fs::read_to_string("input")?
        .lines()
        .map(|s| s.parse::<Expr>())
        .collect::<Result<_, _>>()?;
    println!("{}", part1(&exprs));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_expr() {
        assert_eq!(
            Ok(Expr::Add(
                Box::new(Expr::Mul(
                    Box::new(Expr::Add(
                        Box::new(Expr::Mul(
                            Box::new(Expr::Add(Box::new(Expr::Lit(1)), Box::new(Expr::Lit(2)))),
                            Box::new(Expr::Lit(3))
                        )),
                        Box::new(Expr::Lit(4))
                    )),
                    Box::new(Expr::Lit(5))
                )),
                Box::new(Expr::Lit(6))
            )),
            "1 + 2 * 3 + 4 * 5 + 6".parse()
        );

        assert_eq!(
            Ok(Expr::Add(
                Box::new(Expr::Add(
                    Box::new(Expr::Lit(1)),
                    Box::new(Expr::Par(Box::new(Expr::Mul(
                        Box::new(Expr::Lit(2)),
                        Box::new(Expr::Lit(3))
                    ))))
                )),
                Box::new(Expr::Par(Box::new(Expr::Mul(
                    Box::new(Expr::Lit(4)),
                    Box::new(Expr::Par(Box::new(Expr::Add(
                        Box::new(Expr::Lit(5)),
                        Box::new(Expr::Lit(6))
                    ))))
                ))))
            )),
            "1 + (2 * 3) + (4 * (5 + 6))".parse()
        );
    }

    #[test]
    fn test_eval_expr() {
        assert_eq!(71, "1 + 2 * 3 + 4 * 5 + 6".parse::<Expr>().unwrap().eval());
        assert_eq!(
            51,
            "1 + (2 * 3) + (4 * (5 + 6))"
                .parse::<Expr>()
                .unwrap()
                .eval()
        );
        assert_eq!(26, "2 * 3 + (4 * 5)".parse::<Expr>().unwrap().eval());
        assert_eq!(
            437,
            "5 + (8 * 3 + 9 + 3 * 4 * 3)"
                .parse::<Expr>()
                .unwrap()
                .eval()
        );
        assert_eq!(
            12240,
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
                .parse::<Expr>()
                .unwrap()
                .eval()
        );
        assert_eq!(
            13632,
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
                .parse::<Expr>()
                .unwrap()
                .eval()
        );
    }
}
