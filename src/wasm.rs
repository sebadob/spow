use crate::BitValue;
use nom::IResult;
use sha2::{Digest, Sha256};
use std::fmt::Write;

/// Solve the given Proof of Work
pub fn pow_work(input: &str) -> Result<String, String> {
    if input.len() < 5 {
        return Err("Invalid input length".to_string());
    }

    let version = &input[..1];
    if version != "1" {
        return Err("Unknown version".to_string());
    }

    let difficulty = input[2..4].parse::<u8>().map_err(|err| err.to_string())?;
    let mut counter: u64 = 0;
    let mut buf = String::with_capacity(8);

    loop {
        write!(buf, "{}", counter).unwrap();
        let hash = Sha256::new()
            .chain_update(input.as_bytes())
            .chain_update(buf.as_bytes())
            .finalize();

        if has_leading_zeros(hash.as_slice(), difficulty) {
            return Ok(format!("{}{}", input, counter));
        }

        counter += 1;
        buf.clear();
    }
}

#[inline(always)]
fn has_leading_zeros(input: &[u8], count: u8) -> bool {
    parse_bits((input, 0), count).is_ok()
}

#[inline(always)]
fn parse_bits(input: BitValue, count: u8) -> IResult<BitValue, u64> {
    nom::bits::complete::tag(0, count)(input)
}
