use crate::BitValue;
use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::Engine as _;
use chrono::{Duration, Utc};
use nom::IResult;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::Display;
use std::num::ParseIntError;
use std::ops::Add;
use std::sync::OnceLock;
use thiserror::Error;

// 12 bytes plain
const SALT_LEN_B64: usize = 16;
const CHALLENGE_LEN_B64: usize = 43;

// shortest possible string:
// 1:20:1702682422:Rhs5wflYb9mpiDQX:F+CSBSpalGG6FvfSUYjN8zw95z/LYd7jnnu+lYhA3wI:0
const MIN_LEN_VERIFY: usize = 1 + 1 + 2 + 1 + 10 + 1 + SALT_LEN_B64 + 1 + CHALLENGE_LEN_B64 + 1 + 1;

const DEFAULT_DIFFICULTY: u8 = 18;

static SECRET: OnceLock<String> = OnceLock::new();

#[derive(Debug, Error)]
pub enum PowError {
    #[error("Pow::init() must be called once before using it")]
    Init,
    #[error("{0}")]
    Difficulty(&'static str),
    #[error("Cannot generate a secure random value")]
    Randomize,
    #[error("Verification failed: {0}")]
    Verify(&'static str),
}

impl From<ParseIntError> for PowError {
    fn from(_: ParseIntError) -> Self {
        Self::Verify("Cannot parse the input")
    }
}

// const COUNTER_ALPHABET_LEN: u8 = 26;
// const COUNTER_ALPHABET_WITH_UPPER_LEN: u8 = 52;
//
// #[derive(Debug)]
// pub struct CharsCounter {
//     values: Vec<u8>,
// }
//
// impl CharsCounter {
//     fn new() -> Self {
//         let mut values = Vec::with_capacity(4);
//         values.push(0);
//         Self { values }
//     }
//
//     #[inline(always)]
//     fn inc(&mut self) {
//         let curr = self.values.first_mut().unwrap();
//
//         if *curr == COUNTER_ALPHABET_WITH_UPPER_LEN - 1 {
//             // this value is at its max
//
//             // reset current value
//             *curr = 0;
//
//             // increase the following values until success or append a new one at the end
//             let mut idx = 1;
//             while let Some(next) = self.values.get_mut(idx) {
//                 if *next == COUNTER_ALPHABET_WITH_UPPER_LEN - 1 {
//                     *next = 0;
//                     idx += 1;
//                 } else {
//                     *next += 1;
//                     return;
//                 }
//             }
//
//             // if we get here, we increased all values before to its max -> append a new one
//             self.values.push(0);
//         } else {
//             *curr += 1;
//         }
//     }
//
//     #[inline(always)]
//     fn as_slice(&self) -> &[u8] {
//         self.values.as_slice()
//     }
//
//     #[inline(always)]
//     fn to_string(&self) -> String {
//         let mut s = String::with_capacity(self.values.len());
//         for i in 0..self.values.len() {
//             let value = self.values.get(i).expect("pos counter to be correct");
//             let c = if value >= &COUNTER_ALPHABET_LEN {
//                 char::from(*value + 65 - COUNTER_ALPHABET_LEN)
//             } else {
//                 char::from(*value + 97)
//             };
//             s.push(c);
//         }
//         s
//     }
// }

/// A Proof of Work which is compute-heavy to solve.
///
/// A higher difficulty will increase the calculation time for solving the Proof of Work to grow exponentially.
/// The validation on the server side will always be `O(1)`, no matter how high the difficulty has been set.
///
/// The PoW itself is a modified version of the very popular Hashcat algorithm.
///
/// You must call either `Pow::init()` or `Pow::init_random()` once at application startup.
#[derive(Debug, Serialize, Deserialize)]
pub struct Pow {
    version: u8,
    difficulty: u8,
    salt: String,
    challenge: String,
    expires: i64,
}

impl Display for Pow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}:{}:",
            self.version, self.difficulty, self.expires, self.salt, self.challenge
        )
    }
}

impl Pow {
    /// Build the challenge for the client from this PoW. This will lead to the same result as just calling
    /// `.to_string()` on a PoW. You can use either one.
    pub fn build_challenge(&self) -> String {
        self.to_string()
    }

    /// Static initializer for the secret used on the server-side. This must be called once at application start up
    /// before you can create any PoW's.
    pub fn init(secret: String) {
        let _ = SECRET.set(secret);
    }

    /// The same as `init()`, but initializes a random secret each time.
    pub fn init_random() -> Result<(), PowError> {
        let mut buf = [0u8; 24];
        getrandom::getrandom(&mut buf).map_err(|_| PowError::Randomize)?;
        let _ = SECRET.set(STANDARD_NO_PAD.encode(buf));
        Ok(())
    }

    #[inline(always)]
    fn get_secret() -> Result<&'static String, PowError> {
        SECRET.get().ok_or(PowError::Init)
    }

    /// Create a new PoW which will be valid for the given amount of seconds.
    ///
    /// Choose a validity as short as possible and as long as needed to prevent PoW re-use without the need for a local
    /// cache with recently used PoW's.
    ///
    /// The default difficulty of `18` will be chosen, which is reasonable if you use the wasm client in the browser.
    /// If you however must use JS, which you should only do for a very good reason, you need a way lower difficulty
    /// to not harm the UX.
    pub fn new(valid_seconds: u32) -> Result<Self, PowError> {
        let salt = Self::salt()?;
        let expires = Utc::now()
            .add(Duration::seconds(valid_seconds as i64))
            .timestamp();
        let challenge = Self::challenge(1u8, DEFAULT_DIFFICULTY, expires, &salt)?;

        Ok(Self {
            version: 1,
            difficulty: DEFAULT_DIFFICULTY,
            salt,
            challenge,
            expires,
        })
    }

    /// Create a new PoW with a custom difficulty.
    pub fn with_difficulty(difficulty: u8, valid_seconds: u32) -> Result<Self, PowError> {
        Self::validate_difficulty(difficulty)?;

        let salt = Self::salt()?;
        let expires = Utc::now()
            .add(Duration::seconds(valid_seconds as i64))
            .timestamp();
        let challenge = Self::challenge(1u8, difficulty, expires, &salt)?;

        Ok(Self {
            version: 1,
            difficulty,
            salt,
            challenge,
            expires,
        })
    }

    #[inline(always)]
    fn salt() -> Result<String, PowError> {
        let mut buf = [0u8; 12];
        getrandom::getrandom(&mut buf).map_err(|_| PowError::Randomize)?;
        Ok(STANDARD_NO_PAD.encode(buf))
    }

    #[inline(always)]
    fn challenge(
        version: u8,
        difficulty: u8,
        expires: i64,
        salt: &str,
    ) -> Result<String, PowError> {
        let plain = format!(
            "{}{}{}{}{}",
            version,
            difficulty,
            expires,
            salt,
            Self::get_secret()?,
        );
        let hash = Sha256::digest(plain.as_bytes());
        let b64 = STANDARD_NO_PAD.encode(hash.as_slice());
        Ok(b64)
    }

    #[inline(always)]
    fn challenge_verify(
        version: u8,
        difficulty: u8,
        expires: i64,
        salt: &str,
        challenge: &str,
    ) -> Result<(), PowError> {
        let b64 = Self::challenge(version, difficulty, expires, salt)?;
        if challenge == b64.as_str() {
            Ok(())
        } else {
            Err(PowError::Verify("Challenge cannot be verified"))
        }
    }

    /// Solve the given challenge.
    pub fn work(input: &str) -> Result<String, PowError> {
        use std::fmt::Write;

        if input.len() < 5 {
            return Err(PowError::Verify("Invalid input length"));
        }

        let version = &input[..1];
        if version != "1" {
            return Err(PowError::Verify("Unknown version"));
        }

        let difficulty = input[2..4].parse::<u8>()?;
        let mut counter: u64 = 0;
        let mut buf = String::with_capacity(8);

        loop {
            write!(buf, "{}", counter).unwrap();
            let hash = Sha256::new()
                .chain_update(input.as_bytes())
                .chain_update(buf.as_bytes())
                .finalize();

            if Self::has_leading_zeros(hash.as_slice(), difficulty) {
                return Ok(format!("{}{}", input, counter));
            }

            counter += 1;
            buf.clear();
        }
    }

    #[inline(always)]
    fn has_leading_zeros(input: &[u8], count: u8) -> bool {
        Self::parse_bits((input, 0), count).is_ok()
    }

    #[inline(always)]
    fn parse_bits(input: BitValue, count: u8) -> IResult<BitValue, u64> {
        nom::bits::complete::tag(0, count)(input)
    }

    /// Validate a solved PoW
    pub fn validate(input: &str) -> Result<(), PowError> {
        if input.len() < MIN_LEN_VERIFY {
            return Err(PowError::Verify("Invalid input length"));
        }

        // 1:20:1702684559:OyYuEP70pUiTa3NK:KLk3tEG+Kn79ROObaRMLptwWzYs4OFSfm0FACdAgQ9g:79715

        let version = input[..1].parse::<u8>()?;
        if version != 1 {
            return Err(PowError::Verify("Invalid version"));
        }

        let difficulty = input[2..4].parse::<u8>()?;
        if Self::validate_difficulty(difficulty).is_err() {
            return Err(PowError::Verify("Invalid difficulty"));
        }

        let expires = input[5..15].parse::<i64>()?;
        let now = Utc::now().timestamp();
        if now > expires {
            return Err(PowError::Verify("Pow has expired"));
        }

        let salt_end = 16 + SALT_LEN_B64;
        let salt = &input[16..salt_end];
        let end_challenge = salt_end + CHALLENGE_LEN_B64 + 1;
        let challenge = &input[(salt_end + 1)..end_challenge];

        Self::challenge_verify(version, difficulty, expires, salt, challenge)?;

        let counter = &input[(end_challenge + 1)..];
        if counter.is_empty() {
            return Err(PowError::Verify("Counter too short"));
        }

        // finally, verify the leading zero's
        let hash = Sha256::digest(input.as_bytes());
        let bytes = hash.as_slice();
        if Self::has_leading_zeros(bytes, difficulty) {
            Ok(())
        } else {
            Err(PowError::Verify("Invalid PoW"))
        }
    }

    #[inline(always)]
    fn validate_difficulty(difficulty: u8) -> Result<(), PowError> {
        if !(10u8..99).contains(&difficulty) {
            Err(PowError::Difficulty("Difficulty must be between 10 and 99"))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pow::{Pow, MIN_LEN_VERIFY};
    use chrono::Utc;

    const SECRET: &str = "MySecureTestSecret1337";

    // #[rstest]
    // #[case(0, "a")]
    // #[case(1, "b")]
    // #[case(2, "c")]
    // #[case(25, "z")]
    // #[case(26, "A")]
    // #[case(51, "Z")]
    // #[case(52, "aa")]
    // #[case(53, "ba")]
    // #[case(54, "ca")]
    // #[case(77, "za")]
    // #[case(78, "Aa")]
    // #[case(103, "Za")]
    // #[case(104, "ab")]
    // #[case(105, "bb")]
    // #[case(106, "cb")]
    // fn test_chars_counter(#[case] it: usize, #[case] s: &str) {
    //     let mut counter = CharsCounter::new();
    //
    //     for _ in 0..it {
    //         counter.inc();
    //     }
    //     println!("{:?}", counter);
    //     println!("{:?}", counter.as_slice());
    //     println!("{}", counter.to_string());
    //
    //     assert_eq!(counter.to_string().as_str(), s);
    // }

    #[test]
    fn test_challenge_verify() {
        Pow::init(SECRET.to_string());
        let ts = Utc::now().timestamp();
        let salt = Pow::salt().unwrap();
        let challenge = Pow::challenge(1, 20, ts, &salt).unwrap();
        assert!(Pow::challenge_verify(1, 20, ts, &salt, &challenge).is_ok());
    }

    #[test]
    fn test_parse_bits() {
        assert!(Pow::parse_bits((&[0b0001_1111], 0), 1).is_ok());
        assert!(Pow::parse_bits((&[0b0001_1111], 0), 2).is_ok());
        assert!(Pow::parse_bits((&[0b0001_1111], 0), 3).is_ok());

        assert!(Pow::parse_bits((&[0b0001_1111], 0), 4).is_err());
        assert!(Pow::parse_bits((&[0b0001_1111], 0), 5).is_err());
        assert!(Pow::parse_bits((&[0b0001_1111], 0), 6).is_err());
        assert!(Pow::parse_bits((&[0b0001_1111], 0), 7).is_err());
        assert!(Pow::parse_bits((&[0b0001_1111], 0), 8).is_err());
    }

    #[test]
    fn test_hash() {
        // for _ in 0..20 {
        Pow::init(SECRET.to_string());
        let pow = Pow::new(60).unwrap();

        let pow_challenge = pow.to_string();
        println!("challenge:\n{}", pow_challenge);

        // work and find a matching counter
        let res = Pow::work(&pow_challenge).unwrap();
        println!("res:\n{}", res);
        assert!(res.len() > MIN_LEN_VERIFY);

        // verify the result
        assert!(Pow::validate(&res).is_ok());
    }
}
