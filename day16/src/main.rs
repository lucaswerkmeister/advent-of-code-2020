use std::error::Error;
use std::fmt;
use std::fs;
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
enum ParseFieldError {
    NoColon,
    NoOr,
    R1FromError(ParseIntError),
    R1NoHyphen,
    R1ToError(ParseIntError),
    R2FromError(ParseIntError),
    R2NoHyphen,
    R2ToError(ParseIntError),
}

impl fmt::Display for ParseFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseFieldError {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Field {
    r1: RangeInclusive<u64>,
    r2: RangeInclusive<u64>,
    name: String,
}

impl Field {
    fn accepts(&self, n: u64) -> bool {
        self.r1.contains(&n) || self.r2.contains(&n)
    }
}

impl FromStr for Field {
    type Err = ParseFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s_split_by_colon = s.splitn(2, ':');
        let name = s_split_by_colon
            .next()
            .expect("splitn returns at least one result");
        let rest = s_split_by_colon
            .next()
            .ok_or(ParseFieldError::NoColon)?
            .trim();
        let mut rest_split_by_or = rest.splitn(2, " or ");
        let r1 = rest_split_by_or
            .next()
            .expect("splitn returns at least one result");
        let r2 = rest_split_by_or.next().ok_or(ParseFieldError::NoOr)?;
        let mut r1_split_by_hyphen = r1.splitn(2, '-');
        let r1_from = r1_split_by_hyphen
            .next()
            .expect("splitn returns at least one result")
            .parse()
            .map_err(|e| ParseFieldError::R1FromError(e))?;
        let r1_to = r1_split_by_hyphen
            .next()
            .ok_or(ParseFieldError::R1NoHyphen)?
            .parse()
            .map_err(|e| ParseFieldError::R1ToError(e))?;
        let mut r2_split_by_hyphen = r2.splitn(2, '-');
        let r2_from = r2_split_by_hyphen
            .next()
            .expect("splitn returns at least one result")
            .parse()
            .map_err(|e| ParseFieldError::R2FromError(e))?;
        let r2_to = r2_split_by_hyphen
            .next()
            .ok_or(ParseFieldError::R2NoHyphen)?
            .parse()
            .map_err(|e| ParseFieldError::R2ToError(e))?;
        Ok(Field {
            r1: r1_from..=r1_to,
            r2: r2_from..=r2_to,
            name: name.to_owned(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Ticket {
    values: Vec<u64>,
}

impl Ticket {
    fn invalid_values(&self, fields: &[Field]) -> Vec<u64> {
        self.values
            .iter()
            .filter(|&value| !fields.iter().any(|field| field.accepts(*value)))
            .copied()
            .collect()
    }
}

impl FromStr for Ticket {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ticket {
            values: s.split(',').map(str::parse).collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ParseInputError {
    ParseFieldError(ParseFieldError),
    NoYourTicketHeader,
    NoYourTicket,
    ParseYourTicketError(ParseIntError),
    NoNearbyTicketsBlankLine,
    NoNearbyTicketsHeader,
    ParseNearbyTicketError(ParseIntError),
}

impl fmt::Display for ParseInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseInputError {}

impl From<ParseFieldError> for ParseInputError {
    fn from(e: ParseFieldError) -> Self {
        ParseInputError::ParseFieldError(e)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Input {
    fields: Vec<Field>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Input {
    fn part1(&self) -> u64 {
        self.nearby_tickets
            .iter()
            .map(|ticket| ticket.invalid_values(&self.fields))
            .flatten()
            .sum()
    }

    fn validate(&self) -> Self {
        Self {
            fields: self.fields.clone(),
            your_ticket: self.your_ticket.clone(),
            nearby_tickets: self.nearby_tickets
                .iter()
                .filter(|&ticket| ticket.invalid_values(&self.fields).is_empty())
                .cloned()
                .collect(),
        }
    }

    fn part2(&self) -> u64 {
        let mut possible_fields = vec![];
        for _i in 0..self.your_ticket.values.len() {
            possible_fields.push(self.fields.clone());
        }
        for ticket in &self.nearby_tickets {
            for (i, &value) in ticket.values.iter().enumerate() {
                possible_fields[i].retain(|field| field.accepts(value));
            }
        }
        let mut unambiguous_fields = vec![];
        loop {
            let mut had_ambiguous_fields = false;
            for possible_field in possible_fields.iter_mut() {
                if possible_field.len() == 1 {
                    unambiguous_fields.push(possible_field[0].clone());
                } else {
                    had_ambiguous_fields = true;
                    possible_field.retain(|field| !unambiguous_fields.contains(&field));
                }
            }
            if !had_ambiguous_fields {
                break;
            }
        }
        possible_fields
            .iter()
            .enumerate()
            .map(|(i, possible_field)| {
                if possible_field.len() != 1 {
                    panic!("Field {} not unambiguous ({} options)", i, possible_field.len());
                }
                &possible_field[0]
            })
            .zip(&self.your_ticket.values)
            .filter(|(possible_field, _value)| possible_field.name.starts_with("departure"))
            .map(|(_possible_field, value)| value)
            .product()
    }
}

impl FromStr for Input {
    type Err = ParseInputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let fields = lines
            .by_ref()
            .take_while(|&line| !line.is_empty())
            .map(|line| line.parse())
            .collect::<Result<_, _>>()?;
        let your_ticket_header = lines.next();
        if your_ticket_header != Some("your ticket:") {
            return Err(ParseInputError::NoYourTicketHeader);
        }
        let your_ticket = lines
            .next()
            .ok_or(ParseInputError::NoYourTicket)
            ?.parse()
            .map_err(|e| ParseInputError::ParseYourTicketError(e))?;
        let nearby_tickets_blank_line = lines.next();
        if nearby_tickets_blank_line != Some("") {
            return Err(ParseInputError::NoNearbyTicketsBlankLine);
        }
        let nearby_tickets_header = lines.next();
        if nearby_tickets_header != Some("nearby tickets:") {
            return Err(ParseInputError::NoNearbyTicketsHeader);
        }
        let nearby_tickets = lines
            .map(|line| line.parse())
            .collect::<Result<_, _>>()
            .map_err(|e| ParseInputError::ParseNearbyTicketError(e))?;

        Ok(Input {
            fields,
            your_ticket,
            nearby_tickets,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: Input = fs::read_to_string("input")?.parse()?;
    println!("{:?}", input.part1());
    println!("{:?}", input.validate().part2());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_field_accepts() {
        let field = Field {
            r1: 1..=3,
            r2: 5..=7,
            name: "class".to_owned(),
        };
        assert!(field.accepts(3));
        assert!(field.accepts(5));
        assert!(!field.accepts(4));
    }

    #[test]
    fn test_parse_field() {
        assert_eq!(
            Ok(Field {
                r1: 1..=3,
                r2: 5..=7,
                name: "class".to_owned()
            }),
            "class: 1-3 or 5-7".parse()
        );
        assert_eq!(
            Ok(Field {
                r1: 6..=11,
                r2: 33..=44,
                name: "row".to_owned()
            }),
            "row: 6-11 or 33-44".parse()
        );
        assert_eq!(
            Ok(Field {
                r1: 13..=40,
                r2: 45..=50,
                name: "seat".to_owned()
            }),
            "seat: 13-40 or 45-50".parse()
        );
    }

    #[test]
    fn test_ticket_invalid_values() {
        let fields: Vec<Field> = vec![
            "class: 1-3 or 5-7".parse().unwrap(),
            "row: 6-11 or 33-44".parse().unwrap(),
            "seat: 13-40 or 45-50".parse().unwrap(),
        ];
        assert_eq!(vec![4], Ticket { values: vec![40, 4, 50] }.invalid_values(&fields));
        assert_eq!(vec![55], Ticket { values: vec![55, 2, 20] }.invalid_values(&fields));
        assert_eq!(vec![12], Ticket { values: vec![38, 6, 12] }.invalid_values(&fields));
    }

    #[test]
    fn test_parse_ticket() {
        assert_eq!(
            Ok(Ticket {
                values: vec![7, 1, 14],
            }),
            "7,1,14".parse()
        );
        assert_eq!(
            Ok(Ticket {
                values: vec![7, 3, 47],
            }),
            "7,3,47".parse()
        );
        assert_eq!(
            Ok(Ticket {
                values: vec![40, 4, 50],
            }),
            "40,4,50".parse()
        );
        assert_eq!(
            Ok(Ticket {
                values: vec![55, 2, 20],
            }),
            "55,2,20".parse()
        );
        assert_eq!(
            Ok(Ticket {
                values: vec![38, 6, 12],
            }),
            "38,6,12".parse()
        );
    }

    #[test]
    fn test_part1() {
        let sample_input: Input = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
"
                .parse().unwrap();
        assert_eq!(71, sample_input.part1());
    }

    #[test]
    fn test_validate_input() {
        let sample_input: Input = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
"
                .parse().unwrap();
        assert_eq!(
            Input {
                fields: vec![
                    Field {
                        r1: 1..=3,
                        r2: 5..=7,
                        name: "class".to_owned(),
                    },
                    Field {
                        r1: 6..=11,
                        r2: 33..=44,
                        name: "row".to_owned(),
                    },
                    Field {
                        r1: 13..=40,
                        r2: 45..=50,
                        name: "seat".to_owned(),
                    },
                ],
                your_ticket: Ticket { values: vec![7, 1, 14] },
                nearby_tickets: vec![
                    Ticket { values: vec![7, 3, 47] },
                ],
            },
            sample_input.validate()
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            Ok(Input {
                fields: vec![
                    Field {
                        r1: 1..=3,
                        r2: 5..=7,
                        name: "class".to_owned(),
                    },
                    Field {
                        r1: 6..=11,
                        r2: 33..=44,
                        name: "row".to_owned(),
                    },
                    Field {
                        r1: 13..=40,
                        r2: 45..=50,
                        name: "seat".to_owned(),
                    },
                ],
                your_ticket: Ticket { values: vec![7, 1, 14] },
                nearby_tickets: vec![
                    Ticket { values: vec![7, 3, 47] },
                    Ticket { values: vec![40, 4, 50] },
                    Ticket { values: vec![55, 2, 20] },
                    Ticket { values: vec![38, 6, 12] },
                ],
            }),
            "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
"
                .parse()
        );
    }
}
