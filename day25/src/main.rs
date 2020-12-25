use std::error::Error;
use std::fmt;
use std::fs;
use std::num::ParseIntError;

#[derive(Clone, Debug, Eq, PartialEq)]
enum ParseSystemError {
    NoFirstLine,
    BadFirstLine(ParseIntError),
    NoSecondLine,
    BadSecondLine(ParseIntError),
    ExtraLine,
}

impl fmt::Display for ParseSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseSystemError {}

fn mod_pow(base: u32, exponent: u32, modulus: u32) -> u32 {
    let base = base as u64;
    let exponent = exponent as u64;
    let modulus = modulus as u64;
    let mut res = 1;
    for _ in 0..exponent {
        res = (res * base) % modulus;
    }
    res as u32
}

#[derive(Clone, Copy, Debug)]
struct System {
    private_key_1: Option<u32>,
    public_key_1: u32,
    private_key_2: Option<u32>,
    public_key_2: u32,
    shared_key: Option<u32>,
    modulus: u32,
}

impl System {
    fn new_from_private_keys(
        private_key_1: u32,
        private_key_2: u32,
        generator: u32,
        modulus: u32,
    ) -> Self {
        let public_key_1 = mod_pow(generator, private_key_1, modulus);
        let public_key_2 = mod_pow(generator, private_key_2, modulus);
        let shared_key = mod_pow(public_key_1, private_key_2, modulus);
        debug_assert_eq!(shared_key, mod_pow(public_key_2, private_key_1, modulus));
        Self {
            private_key_1: Some(private_key_1),
            public_key_1,
            private_key_2: Some(private_key_2),
            public_key_2,
            shared_key: Some(shared_key),
            modulus,
        }
    }

    fn new_from_private_and_public_key(
        private_key_1: u32,
        public_key_2: u32,
        generator: u32,
        modulus: u32,
    ) -> Self {
        Self {
            private_key_1: Some(private_key_1),
            public_key_1: mod_pow(generator, private_key_1, modulus),
            private_key_2: None,
            public_key_2,
            shared_key: Some(mod_pow(public_key_2, private_key_1, modulus)),
            modulus,
        }
    }

    fn new_from_public_keys(public_key_1: u32, public_key_2: u32, modulus: u32) -> Self {
        Self {
            private_key_1: None,
            public_key_1,
            private_key_2: None,
            public_key_2,
            shared_key: None,
            modulus,
        }
    }

    fn parse_from_public_keys(s: &str, modulus: u32) -> Result<Self, ParseSystemError> {
        let mut lines = s.lines();
        let public_key_1: u32 = lines
            .next()
            .ok_or(ParseSystemError::NoFirstLine)?
            .parse()
            .map_err(|e| ParseSystemError::BadFirstLine(e))?;
        let public_key_2: u32 = lines
            .next()
            .ok_or(ParseSystemError::NoSecondLine)?
            .parse()
            .map_err(|e| ParseSystemError::BadSecondLine(e))?;
        if lines.next().is_some() {
            return Err(ParseSystemError::ExtraLine);
        }
        Ok(Self::new_from_public_keys(
            public_key_1,
            public_key_2,
            modulus,
        ))
    }

    fn crack_shared_key(&self, generator: u32) -> Self {
        if self.shared_key.is_some() {
            return *self;
        }
        let generator = generator as u64;
        let modulus = self.modulus as u64;
        let mut public_key_1 = 1;
        for private_key_1 in 1.. {
            public_key_1 = (public_key_1 * generator) % modulus;
            if public_key_1 as u32 == self.public_key_1 {
                let private_key_1 = private_key_1 as u32;
                return Self {
                    private_key_1: Some(private_key_1),
                    public_key_1: self.public_key_1,
                    private_key_2: None,
                    public_key_2: self.public_key_2,
                    shared_key: Some(mod_pow(self.public_key_2, private_key_1, self.modulus)),
                    modulus: self.modulus,
                };
            }
        }
        panic!("could not find any matching private key");
    }
}

impl PartialEq for System {
    fn eq(&self, other: &Self) -> bool {
        self.public_key_1 == other.public_key_1
            && self.public_key_2 == other.public_key_2
            && self.shared_key == other.shared_key
            && self.modulus == other.modulus
    }
}
impl Eq for System {}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let system = System::parse_from_public_keys(&input, 20201227)?;
    let system = system.crack_shared_key(7);
    let part1 = system.shared_key.expect("shared key should be cracked");
    println!("{}", part1);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const GENERATOR: u32 = 7;
    const MODULUS: u32 = 20201227;
    const PRIVATE_KEY_1: u32 = 8;
    const PUBLIC_KEY_1: u32 = 5764801;
    const PRIVATE_KEY_2: u32 = 11;
    const PUBLIC_KEY_2: u32 = 17807724;

    #[test]
    fn test_mod_pow() {
        assert_eq!(1, mod_pow(2, 3, 7));
        assert_eq!(23, mod_pow(5, 6, 29));
        assert_eq!(PUBLIC_KEY_1, mod_pow(GENERATOR, PRIVATE_KEY_1, MODULUS));
        assert_eq!(PUBLIC_KEY_2, mod_pow(GENERATOR, PRIVATE_KEY_2, MODULUS));
    }

    #[test]
    fn test_complete_constructors() {
        let from_private_keys =
            System::new_from_private_keys(PRIVATE_KEY_1, PRIVATE_KEY_2, GENERATOR, MODULUS);
        let from_private_and_public_key = System::new_from_private_and_public_key(
            PRIVATE_KEY_1,
            PUBLIC_KEY_2,
            GENERATOR,
            MODULUS,
        );
        assert_eq!(from_private_keys, from_private_and_public_key);
        // swap key pairs 1 and 2
        let from_private_keys =
            System::new_from_private_keys(PRIVATE_KEY_2, PRIVATE_KEY_1, GENERATOR, MODULUS);
        let from_private_and_public_key = System::new_from_private_and_public_key(
            PRIVATE_KEY_2,
            PUBLIC_KEY_1,
            GENERATOR,
            MODULUS,
        );
        assert_eq!(from_private_keys, from_private_and_public_key);
    }

    #[test]
    fn test_partial_constructor() {
        let expected = System {
            private_key_1: None,
            public_key_1: PUBLIC_KEY_1,
            private_key_2: None,
            public_key_2: PUBLIC_KEY_2,
            shared_key: None,
            modulus: MODULUS,
        };
        let actual = System::new_from_public_keys(PUBLIC_KEY_1, PUBLIC_KEY_2, MODULUS);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_crack_shared_key() {
        let input = System::new_from_public_keys(PUBLIC_KEY_1, PUBLIC_KEY_2, MODULUS);
        let cracked = input.crack_shared_key(GENERATOR);
        let expected = System::new_from_private_and_public_key(
            PRIVATE_KEY_1,
            PUBLIC_KEY_2,
            GENERATOR,
            MODULUS,
        );
        assert_eq!(expected, cracked);
    }
}
