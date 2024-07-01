use crate::expressions::decoding::{DeferredDecode, ALIEN_ASCII};
use crate::expressions::numbers;
use crate::expressions::parser::{BinOp, ICFPExpr, IntType, NatType, UnOp, Var};
use std::collections::VecDeque;
use tracing::{trace, trace_span};

pub const MIN_CHAR: char = '!'; // ASCII 33
pub const _MAX_CHAR: char = '~'; // ASCII 126
pub const NUM_BASE: NatType = 94;

pub trait Encode {
  fn encode(&self) -> String;
}

impl Encode for ICFPExpr {
  fn encode(&self) -> String {
    let _encoding_span = trace_span!("expression-encode").entered();
    use std::fmt::Write;
    let mut result = String::new();

    let mut stack = VecDeque::new();

    stack.push_back(self);

    let mut expression_count = 1;

    while let Some(e) = stack.pop_back() {
      if result.len() > 0 {
        result.push(' ');
      }

      if expression_count % 1000 == 0 {
        trace!(cnt = expression_count, stack_len = stack.len(), "encoding");
      }

      expression_count += 1;

      match e {
        ICFPExpr::Boolean(b) => write!(&mut result, "{}", b.encode()).unwrap(),
        ICFPExpr::Integer(i) => write!(&mut result, "I{}", i.encode()).unwrap(),
        ICFPExpr::String(s) => write!(&mut result, "S{}", s.encode()).unwrap(),
        ICFPExpr::UnaryOp(op, expr) => {
          stack.push_back(&expr);
          write!(&mut result, "U{}", op.encode()).unwrap()
        }
        ICFPExpr::BinaryOp(op, left, right) => {
          stack.push_back(&right);
          stack.push_back(&left);
          write!(&mut result, "B{}", op.encode()).unwrap()
        }
        ICFPExpr::If(c, t, f) => {
          stack.push_back(&f);
          stack.push_back(&t);
          stack.push_back(&c);

          write!(&mut result, "?").unwrap()
        }
        ICFPExpr::Lambda(_id, arg, body) => {
          stack.push_back(&body);
          write!(&mut result, "L{}", arg.encode()).unwrap();
        }
        ICFPExpr::VarRef(var) => write!(&mut result, "v{}", var.encode()).unwrap(),
        ICFPExpr::Unknown {
          indicator: _indicator,
          body: _body,
        } => unimplemented!(),
        ICFPExpr::Closure { .. } => {
          unreachable!("You can't encode a closure")
        }
        ICFPExpr::Thunk(_thunk) => {
          unreachable!("No way thanks can be encoded")
        }
      }
    }

    trace!(expression_count, "encoding done");

    result
  }
}

impl Encode for bool {
  fn encode(&self) -> String {
    if *self {
      "T".to_string()
    } else {
      "F".to_string()
    }
  }
}

impl Encode for DeferredDecode<String> {
  fn encode(&self) -> String {
    match self {
      DeferredDecode::Deferred { coded, .. } => format!("S{coded}"),
      DeferredDecode::Lit(s) => s.encode(),
    }
  }
}

impl Encode for DeferredDecode<IntType> {
  fn encode(&self) -> String {
    match self {
      DeferredDecode::Deferred { coded, .. } => format!("I{coded}"),
      DeferredDecode::Lit(s) => s.encode(),
    }
  }
}

impl Encode for IntType {
  fn encode(&self) -> String {
    numbers::base94_encode_number(self.clone())
  }
}

impl Encode for i64 {
  fn encode(&self) -> String {
    numbers::base94_encode_number((*self).into())
  }
}

impl Encode for Var {
  fn encode(&self) -> String {
    numbers::base94_encode_number(self.0.into())
  }
}

impl Encode for String {
  fn encode(&self) -> String {
    self
      .chars()
      .map(|c| {
        let c_base = MIN_CHAR as usize;
        let encoded_idx = ALIEN_ASCII
          .char_indices()
          .find_map(|(idx, x)| if x == c { Some(idx) } else { None })
          .unwrap();
        (c_base + encoded_idx) as u8 as char
      })
      .collect::<String>()
  }
}

impl Encode for UnOp {
  fn encode(&self) -> String {
    let char = match self {
      UnOp::Negate => '-',
      UnOp::Not => '!',
      UnOp::StrToInt => '#',
      UnOp::IntToStr => '$',
    };
    format!("{}", char)
  }
}

impl Encode for BinOp {
  fn encode(&self) -> String {
    match self {
      BinOp::Add => "+".to_string(),
      BinOp::Sub => "-".to_string(),
      BinOp::Mul => "*".to_string(),
      BinOp::Div => "/".to_string(),
      BinOp::Mod => "%".to_string(),
      BinOp::LessThan => "<".to_string(),
      BinOp::GreaterThan => ">".to_string(),
      BinOp::Equals => "=".to_string(),
      BinOp::Or => "|".to_string(),
      BinOp::And => "&".to_string(),
      BinOp::Concat => ".".to_string(),
      BinOp::TakeChars => "T".to_string(),
      BinOp::SkipChars => "D".to_string(),
      BinOp::ApplyLambda => "$".to_string(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Encode;
  use crate::expressions::parser::{ICFPExpr, Var};

  #[test]
  fn encode_string() {
    let input = "Hello World!";
    let string = input.to_string().encode();

    let expected = "B%,,/}Q/2,$_";

    assert_eq!(string, expected);
  }

  #[test]
  fn encode_bools() {
    assert_eq!(true.encode(), "T");
    assert_eq!(false.encode(), "F");
  }

  #[test]
  fn encode_if() {
    let expr = ICFPExpr::if_(ICFPExpr::const_true(), ICFPExpr::int(3), ICFPExpr::int(4));
    assert_eq!(expr.encode(), "? T I$ I%");
  }

  #[test]
  fn encode_simple_lambda() {
    let expr = ICFPExpr::lambda(0, Var(1), ICFPExpr::int(3));
    assert_eq!(expr.encode(), "L\" I$");
  }
}
