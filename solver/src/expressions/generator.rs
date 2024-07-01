use crate::expressions::parser::{ICFPExpr, Var};

pub fn let_in<F>(
  var: Var,
  expr: ICFPExpr,
  in_expr: F,
) -> ICFPExpr
where
  F: FnOnce(Var) -> ICFPExpr,
{
  ICFPExpr::call(ICFPExpr::lambda(0, var, in_expr(var)), expr)
}
