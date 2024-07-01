use crate::evaluator::Environment;
use crate::expressions::decoding::{Decode, DeferredDecode};
use miette::{miette, Result};
use std::fmt::{Debug, Display, Formatter};
use std::str::SplitWhitespace;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tracing::warn;

pub type NatType = usize;

pub type ExprRef = Box<ICFPExpr>;

static LAMBDA_ID: AtomicUsize = AtomicUsize::new(0);

#[allow(dead_code)]
/// ICFP Alien Language
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ICFPExpr {
  Boolean(bool),
  Integer(DeferredDecode<IntType>),
  String(DeferredDecode<String>),
  UnaryOp(UnOp, ExprRef),
  BinaryOp(BinOp, ExprRef, ExprRef),
  /// ? B> I# I$ S9%3 S./
  If(ExprRef, ExprRef, ExprRef),
  /// B$ B$ L# L$ v# B. SB%,,/ S}Q/2,$_ IK
  /// ((\v2 -> \v3 -> v2) ("Hello" . " World!")) 42
  Lambda(usize, Var, ExprRef),
  VarRef(Var),
  Closure {
    id: usize,
    arg: Var,
    body: ExprRef,
    env: Environment,
  },
  Thunk(Arc<crate::evaluator::LazyExpr>),
  /// The above set of language constructs are all that researchers have discovered,
  /// and it is conjectured that the Cult will never use anything else in their
  /// communication towards Earth. However, it is unknown whether more language constructs exist.
  Unknown {
    indicator: char,
    body: String,
  },
}

#[allow(dead_code)]
impl ICFPExpr {
  pub fn const_true() -> Self {
    ICFPExpr::Boolean(true)
  }

  pub fn const_false() -> Self {
    ICFPExpr::Boolean(false)
  }

  pub fn str<S: Into<String>>(str: S) -> Self {
    ICFPExpr::String(DeferredDecode::Lit(str.into()))
  }

  pub fn if_<C, T, F>(
    cond: C,
    if_true: T,
    if_false: F,
  ) -> Self
  where
    C: Into<ICFPExpr>,
    T: Into<ICFPExpr>,
    F: Into<ICFPExpr>,
  {
    ICFPExpr::If(
      Box::new(cond.into()),
      Box::new(if_true.into()),
      Box::new(if_false.into()),
    )
  }

  pub fn bin_op<L: Into<ICFPExpr>, R: Into<ICFPExpr>>(
    left: L,
    op: BinOp,
    right: R,
  ) -> Self {
    ICFPExpr::BinaryOp(op, Box::new(left.into()), Box::new(right.into()))
  }

  pub fn lambda(
    id: usize,
    arg: Var,
    body: ICFPExpr,
  ) -> Self {
    ICFPExpr::Lambda(id, arg, Box::new(body))
  }

  pub fn int(i: impl Into<IntType>) -> Self {
    ICFPExpr::Integer(DeferredDecode::Lit(i.into()))
  }

  pub fn var(v: usize) -> Self {
    Self::VarRef(Var(v))
  }

  pub fn call<L: Into<ICFPExpr>, A: Into<ICFPExpr>>(
    lambda: L,
    arg: A,
  ) -> Self {
    Self::BinaryOp(
      BinOp::ApplyLambda,
      Box::new(lambda.into()),
      Box::new(arg.into()),
    )
  }
}

impl From<&str> for ICFPExpr {
  fn from(value: &str) -> Self {
    Self::str(value)
  }
}

impl From<String> for ICFPExpr {
  fn from(value: String) -> Self {
    Self::str(value)
  }
}

impl From<i64> for ICFPExpr {
  fn from(value: i64) -> Self {
    Self::Integer(DeferredDecode::Lit(value.into()))
  }
}

impl From<i32> for ICFPExpr {
  fn from(value: i32) -> Self {
    Self::Integer(DeferredDecode::Lit(value.into()))
  }
}

impl From<usize> for ICFPExpr {
  fn from(value: usize) -> Self {
    Self::Integer(DeferredDecode::Lit(value.into()))
  }
}

impl From<Var> for ICFPExpr {
  fn from(value: Var) -> Self {
    Self::VarRef(value)
  }
}

impl From<bool> for ICFPExpr {
  fn from(value: bool) -> Self {
    Self::Boolean(value)
  }
}

impl<R: Into<ICFPExpr>> core::ops::Add<R> for ICFPExpr {
  type Output = ICFPExpr;

  fn add(
    self,
    rhs: R,
  ) -> Self::Output {
    ICFPExpr::bin_op(self, BinOp::Add, rhs.into())
  }
}

impl<R: Into<ICFPExpr>> core::ops::Sub<R> for ICFPExpr {
  type Output = ICFPExpr;

  fn sub(
    self,
    rhs: R,
  ) -> Self::Output {
    ICFPExpr::bin_op(self, BinOp::Sub, rhs.into())
  }
}

impl Display for ICFPExpr {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      ICFPExpr::Boolean(t) => write!(f, "{}", t),
      ICFPExpr::Integer(i) => write!(f, "{}", i),
      ICFPExpr::String(s) => write!(f, "{}", s),
      ICFPExpr::UnaryOp(op, operand) => match op {
        UnOp::Negate => write!(f, " -{} ", operand),
        UnOp::Not => write!(f, " !{} ", operand),
        UnOp::StrToInt => write!(f, " {:?}({}) ", op, operand),
        UnOp::IntToStr => write!(f, " {:?}({}) ", op, operand),
      },
      ICFPExpr::BinaryOp(op, left, right) => match op {
        BinOp::Add => write!(f, "({} + {})", left, right),
        BinOp::Sub => write!(f, "({} - {})", left, right),
        BinOp::Mul => write!(f, "({} * {})", left, right),
        BinOp::Div => write!(f, "({} / {})", left, right),
        BinOp::Mod => write!(f, "({} % {})", left, right),
        BinOp::LessThan => write!(f, "({} < {})", left, right),
        BinOp::GreaterThan => write!(f, "({} > {})", left, right),
        BinOp::Equals => write!(f, "({} == {})", left, right),
        BinOp::Or => write!(f, "({} || {})", left, right),
        BinOp::And => write!(f, "({} && {})", left, right),
        BinOp::Concat => write!(f, "({}).concat({})", left, right),
        BinOp::TakeChars => write!(f, "take({}, {})", left, right),
        BinOp::SkipChars => write!(f, "skip({}, {})", left, right),
        BinOp::ApplyLambda => write!(f, "{}({})", left, right),
      },
      ICFPExpr::If(cond, if_true, if_false) => {
        write!(
          f,
          "if ({}) {{ return {} }} else {{ return {} }}",
          cond, if_true, if_false
        )
      }
      ICFPExpr::Lambda(id, var, body) => {
        if matches!(**body, ICFPExpr::If(_, _, _)) {
          write!(f, "(function lam_{id}({}){{ {} }})", var, body)
        } else {
          write!(f, "(function lam_{id}({}){{ return {} }})", var, body)
        }
      }
      ICFPExpr::VarRef(var) => write!(f, "{}", var),
      ICFPExpr::Closure { id, arg, body, env } => {
        write!(
          f,
          "Closure({id}) {:?} ({}) => {{ return {} }}",
          env, arg, body
        )
      }
      ICFPExpr::Thunk(thunk) => f.debug_tuple("thunk").field(thunk).finish(),
      ICFPExpr::Unknown { indicator, body } => f
        .debug_struct("Unknown")
        .field("indicator", indicator)
        .field("body", body)
        .finish(),
    }
  }
}

/// As communication with Earth is complicated,
/// the Cult seems to have put some restrictions on their Macroware Insight software.
/// Specifically, message processing is aborted when exceeding 10_000_000 beta reductions.
/// Built-in operators are strict (except for B$, of course) and do not count
/// towards the limit of beta reductions. Contestants' messages therefore must stay within these limits.
///
/// For example, the following term, which evaluates to 16, uses 109 beta reductions during evaluation:
///
/// ```
/// B$ B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L" L# ? B= v# I! I" B$ L$ B+ B$ v" v$ B$ v" v$ B- v# I" I%
/// ```
///
/// Researchers expect that the limit on the amount beta reductions is the only limit that contestants may run into,
/// but there seem to also be some (unknown) limits on memory usage and total runtime.
const _FUNCTION_CALL_LIMIT: usize = 1000;

pub type IntType = malachite::Integer;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Var(pub usize);

impl Display for Var {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "var_{}", self.0)
  }
}

impl Debug for Var {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "{}", self)
  }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum UnOp {
  /// `-`	Integer negation	U- I$ -> -3
  Negate,
  /// `!`	Boolean not	U! T -> false
  Not,
  /// `#`	string-to-int: interpret a string as a base-94 number	U# S4%34 -> 15818151
  StrToInt,
  /// `$`	int-to-string: inverse of the above	U$ I4%34 -> test
  IntToStr,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum BinOp {
  /// `+`	Integer addition	B+ I# I$ -> 5
  Add,
  /// `-`	Integer subtraction	B- I$ I# -> 1
  Sub,
  /// `*`	Integer multiplication	B* I$ I# -> 6
  Mul,
  /// `/`	Integer division (truncated towards zero)	B/ U- I( I# -> -3
  Div,
  /// `%`	Integer modulo	B% U- I( I# -> -1
  Mod,
  /// `<`	Integer comparison	B< I$ I# -> false
  LessThan,
  /// `>`	Integer comparison	B> I$ I# -> true
  GreaterThan,
  /// `=`	Equality comparison, works for int, bool and string	B= I$ I# -> false
  Equals,
  /// `|`	Boolean or	B| T F -> true
  Or,
  /// `&`	Boolean and	B& T F -> false
  And,
  /// `.`	String concatenation	B. S4% S34 -> "test"
  Concat,
  /// `T`	Take first x chars of string y	BT I$ S4%34 -> "tes"
  TakeChars,
  /// `D`	Drop first x chars of string y	BD I$ S4%34 -> "t"
  SkipChars,
  /// `$` Apply term x to y (see ...)
  ApplyLambda,
}

pub trait Parsable: Sized {
  fn parse(input: &str) -> Result<Self> {
    Self::parse_impl(&mut input.split_whitespace())
  }

  fn parse_impl(input: &mut SplitWhitespace) -> Result<Self>;
}

impl Parsable for ICFPExpr {
  fn parse_impl(expressions: &mut SplitWhitespace) -> Result<Self> {
    let Some(exp) = expressions.next() else {
      return Err(miette!("Not enough expressions in input"));
    };

    let indicator = exp[0..1].chars().next().unwrap();
    let body = &exp[1..];

    let expr = match indicator {
      'S' => {
        let result = String::decode(body)?;
        ICFPExpr::str(result)
      }
      'I' => ICFPExpr::Integer(DeferredDecode::deferred(body)),
      'v' => ICFPExpr::VarRef(Var::decode(body)?),
      'T' => ICFPExpr::Boolean(true),
      'F' => ICFPExpr::Boolean(false),
      'U' => ICFPExpr::UnaryOp(
        UnOp::decode(body)?,
        Box::new(ICFPExpr::parse_impl(expressions)?),
      ),
      'B' => ICFPExpr::BinaryOp(
        BinOp::decode(body)?,
        Box::new(ICFPExpr::parse_impl(expressions)?),
        Box::new(ICFPExpr::parse_impl(expressions)?),
      ),
      '?' => ICFPExpr::If(
        Box::new(ICFPExpr::parse_impl(expressions)?),
        Box::new(ICFPExpr::parse_impl(expressions)?),
        Box::new(ICFPExpr::parse_impl(expressions)?),
      ),
      'L' => {
        let arg_name = Var::decode(body)?;
        ICFPExpr::Lambda(
          LAMBDA_ID.fetch_add(1, Ordering::SeqCst),
          arg_name,
          Box::new(ICFPExpr::parse_impl(expressions)?),
        )
      }
      indicator => {
        warn!(?indicator, expr = exp, "Unknown expression");
        ICFPExpr::Unknown {
          indicator,
          body: body.to_string(),
        }
      }
    };

    Ok(expr)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_if() -> Result<()> {
    let input = "? T I$ I%";
    let expected = ICFPExpr::if_(ICFPExpr::const_true(), ICFPExpr::int(3), ICFPExpr::int(4));

    let result = ICFPExpr::parse(input)?;
    assert_eq!(result, expected);

    Ok(())
  }

  #[test]
  fn parse_simple_lambda() -> Result<()> {
    let input = "L% I$";
    let expected = ICFPExpr::lambda(0, Var(4), ICFPExpr::int(3));
    let result = ICFPExpr::parse(input)?;
    assert_eq!(result, expected);

    Ok(())
  }
}
