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
    fn lit(num: u64) -> Self {
        Self::Lit(num)
    }

    fn add(left: Expr, right: Expr) -> Self {
        Self::Add(Box::new(left), Box::new(right))
    }

    fn mul(left: Expr, right: Expr) -> Self {
        Self::Mul(Box::new(left), Box::new(right))
    }

    fn par(expr: Expr) -> Self {
        Self::Par(Box::new(expr))
    }

    fn eval(&self) -> u64 {
        match self {
            Self::Lit(num) => *num,
            Self::Add(left, right) => left.eval() + right.eval(),
            Self::Mul(left, right) => left.eval() * right.eval(),
            Self::Par(expr) => expr.eval(),
        }
    }

    fn parse_lit(s: &str) -> Result<(Self, Option<&str>), ParseExprError> {
        match s.find(|c: char| !c.is_ascii_digit()) {
            Some(i) => Ok((Self::Lit(s[..i].parse()?), Some(&s[i..].trim()))),
            None => Ok((Self::Lit(s.parse()?), None)),
        }
    }

    fn parse_par(s: &str) -> Result<(Self, Option<&str>), ParseExprError> {
        if s.starts_with("(") {
            let (expr, rest) = Self::parse_expr(&s[1..])?;
            match rest {
                Some(rest) if rest.starts_with(")") => {
                    let rest = rest[1..].trim();
                    let rest = if rest.is_empty() { None } else { Some(rest) };
                    Ok((Self::Par(Box::new(expr)), rest))
                }
                _ => Err(ParseExprError::NoCloseParen),
            }
        } else {
            panic!("parse_par() called with non-par string: {}", s);
        }
    }

    fn parse_atom(s: &str) -> Result<(Self, Option<&str>), ParseExprError> {
        if s.starts_with("(") {
            Self::parse_par(s)
        } else {
            Self::parse_lit(s)
        }
    }

    fn parse_factor(s: &str) -> Result<(Self, Option<&str>), ParseExprError> {
        let mut cur = Self::parse_atom(s)?;
        while let Some(rest) = cur.1 {
            if rest.starts_with(")") || rest.starts_with("*") {
                break;
            }
            if rest.starts_with("+") {
                let (atom, rest) = Self::parse_atom(&rest[1..].trim())?;
                cur = (Self::Add(Box::new(cur.0), Box::new(atom)), rest);
            } else {
                return Err(ParseExprError::UnknownOperator(
                    rest.chars().next().unwrap(),
                ));
            }
        }
        Ok(cur)
    }

    fn parse_expr(s: &str) -> Result<(Self, Option<&str>), ParseExprError> {
        let mut cur = Self::parse_factor(s)?;
        while let Some(rest) = cur.1 {
            if rest.starts_with(")") {
                break;
            }
            if rest.starts_with("*") {
                let (factor, rest) = Self::parse_factor(&rest[1..].trim())?;
                cur = (Self::Mul(Box::new(cur.0), Box::new(factor)), rest);
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
        let (expr, rest) = Self::parse_expr(s)?;
        if rest.is_some() {
            return Err(ParseExprError::TrailingGarbage);
        }
        Ok(expr)
    }
}

fn part2(exprs: &[Expr]) -> u64 {
    exprs.iter().map(Expr::eval).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let exprs: Vec<Expr> = fs::read_to_string("input")?
        .lines()
        .map(|s| s.parse::<Expr>())
        .collect::<Result<_, _>>()?;
    println!("{}", part2(&exprs));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_expr() {
        assert_eq!(
            Ok(Expr::mul(
                Expr::mul(
                    Expr::add(Expr::lit(1), Expr::lit(2)),
                    Expr::add(Expr::lit(3), Expr::lit(4))
                ),
                Expr::add(Expr::lit(5), Expr::lit(6))
            )),
            "1 + 2 * 3 + 4 * 5 + 6".parse()
        );

        assert_eq!(
            Ok(Expr::add(
                Expr::add(
                    Expr::lit(1),
                    Expr::par(Expr::mul(Expr::lit(2), Expr::lit(3)))
                ),
                Expr::par(Expr::mul(
                    Expr::lit(4),
                    Expr::par(Expr::add(Expr::lit(5), Expr::lit(6)))
                ))
            )),
            "1 + (2 * 3) + (4 * (5 + 6))".parse()
        );
    }

    #[test]
    fn test_eval_expr() {
        assert_eq!(231, "1 + 2 * 3 + 4 * 5 + 6".parse::<Expr>().unwrap().eval());
        assert_eq!(
            51,
            "1 + (2 * 3) + (4 * (5 + 6))"
                .parse::<Expr>()
                .unwrap()
                .eval()
        );
        assert_eq!(46, "2 * 3 + (4 * 5)".parse::<Expr>().unwrap().eval());
        assert_eq!(
            1445,
            "5 + (8 * 3 + 9 + 3 * 4 * 3)"
                .parse::<Expr>()
                .unwrap()
                .eval()
        );
        assert_eq!(
            669060,
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
                .parse::<Expr>()
                .unwrap()
                .eval()
        );
        assert_eq!(
            23340,
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
                .parse::<Expr>()
                .unwrap()
                .eval()
        );
    }
}
