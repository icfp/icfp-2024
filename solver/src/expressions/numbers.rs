use crate::expressions::encoding::{MIN_CHAR, NUM_BASE};
use crate::expressions::parser::{IntType, NatType};
use malachite::num::arithmetic::traits::{Mod, Pow};
use malachite::num::basic::traits::Zero;
use miette::{miette, LabeledSpan};
use std::ops::DivAssign;
use tracing::{trace, trace_span};

pub fn base94_decode(encoded: &str) -> miette::Result<IntType> {
  let _encode_span = trace_span!("base94-decode").entered();

  let ascii_offset = 33; // '!' is ASCII 33
  let mut num: malachite::Integer = malachite::Integer::ZERO;

  for (i, char) in encoded.chars().rev().enumerate() {
    let value = (char as NatType).checked_sub(ascii_offset);
    if let Some(digit) = value {
      let digit = malachite::Integer::from(digit);
      let base = IntType::from(NUM_BASE);
      if digit < base {
        trace!(i, "^Power");
        let pow = base.pow(i as u64);
        trace!(?digit, ?pow, "digit*pow");
        num += digit * pow;
        //.ok_or(miette!("Encoded Number is too big: {encoded}"))?;
      } else {
        return Err(miette!(
          labels = vec![LabeledSpan::at(i..i + 1, "invalid"),],
          "Invalid character '{}' in input",
          char
        ));
      }
    } else {
      return Err(miette!(
        labels = vec![LabeledSpan::at(i..i + 1, "invalid"),],
        "Invalid character '{}' in input",
        char
      ));
    }
  }

  trace!("num decode done");

  Ok(num as IntType)
}

// How the fuck do you do negatives?
pub fn base94_encode_number(mut num: IntType) -> String {
  let _encode_span = trace_span!("base94-encode").entered();
  trace!("encoding_number");
  let ascii_offset = 33; // '!' is ASCII 33
  let mut encoded = String::new();

  if num == 0 {
    return format!("{}", MIN_CHAR);
  }

  while num > 0 {
    let base: IntType = NUM_BASE.into();
    let rem: IntType = num.clone().mod_op(&base);
    let remainder: u8 = i64::try_from(&rem).unwrap() as u8;
    num.div_assign(base);
    encoded.push((remainder + ascii_offset) as char);
  }

  encoded.chars().rev().collect() // Reverse the encoded string
}

#[cfg(test)]
mod tests {
  use super::super::encoding::Encode;
  use crate::expressions::decoding::Decode;
  use miette::Result;

  #[test]
  fn encode_num() {
    let string = 1337.encode();

    let expected = "/6";

    assert_eq!(string, expected);
  }

  #[test]
  fn decode_num() -> Result<()> {
    let input = "B%,,/}Q/2,$_";
    let expected = "Hello World!";

    let result = String::decode(input)?;
    assert_eq!(result, expected);

    Ok(())
  }
}
