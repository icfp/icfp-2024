use crate::problems::lambdaman::Direction::Down;
use crate::problems::{Direction, ProblemError, DIRS};
use std::collections::VecDeque;
use std::fmt::Display;

impl Display for Direction {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Up => "U",
        Down => "D",
        Left => "L",
        Right => "R",
      }
    )
  }
}

impl Direction {
  fn backtrack(self) -> Self {
    match self {
      Up => Down,
      Down => Up,
      Left => Right,
      Right => Left,
    }
  }

  fn move_pos(
    self,
    (row, col): (usize, usize),
  ) -> (usize, usize) {
    match self {
      Up => (row - 1, col),
      Down => (row + 1, col),
      Left => (row, col - 1),
      Right => (row, col + 1),
    }
  }
}

use crate::expressions::generator::let_in;
use crate::expressions::parser::BinOp::Concat;
use crate::expressions::parser::ICFPExpr::VarRef;
use crate::expressions::parser::{BinOp, ICFPExpr, Var};
use crate::problems::lambdaman::GridState::{Candy, Visited};
use Direction::*;

#[derive(Debug)]
struct State {
  pos: (usize, usize),          // current position (row, col)
  direction: Option<Direction>, // direction taken to reach current position
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum GridState {
  Candy,
  Visited,
}

// DFS function to find the path with direction tracking without recursion
fn dfs_without_recursion(
  grid: Vec<Vec<char>>,
  start: (usize, usize),
) -> Option<Vec<Direction>> {
  let rows = grid.len();
  let cols = grid[0].len();

  let mut state_grid = vec![vec![GridState::Candy; cols]; rows];
  let mut path = Vec::new();
  let mut to_visit_stack = VecDeque::new();

  let mut actual_walked_path = Vec::new();

  // Initialize the start point
  to_visit_stack.push_back(State {
    pos: start,
    direction: None,
  });

  state_grid[start.0][start.1] = Visited;
  let mut map = grid.clone();
  map[start.0][start.1] = 'S';

  // Function to check if a cell is within bounds and not a wall
  fn is_valid(
    grid: &[Vec<char>],
    row: usize,
    col: usize,
    grid_state: &[Vec<GridState>],
  ) -> bool {
    row < grid.len()
      && col < grid[0].len()
      && grid[row][col] != '#'
      && grid_state[row][col] != Visited
  }

  let mut is_backtracking = false;

  while let Some(State { pos, direction }) = to_visit_stack.pop_back() {
    let (row, col) = pos;

    if map[row][col] == '.' {
      map[row][col] = 'a';
    } else {
      map[row][col] = (map[row][col] as u8 + 1) as char;
    }

    // Push the current position and direction to the path
    if let Some(dir) = direction {
      actual_walked_path.push(dir);
      state_grid[pos.0][pos.1] = Visited;
      if !is_backtracking {
        path.push((pos, dir));
      }
    }

    // println!("path: {:?}", path.last());
    //
    // print_grid(&map);
    // let debug_path = actual_walked_path
    //   .iter()
    //   .map(|dir| dir.to_string())
    //   .collect::<String>();
    // println!("{}", debug_path);
    // Explore all directions
    let mut found_next = false;
    for &((dr, dc), dir_name) in &DIRS {
      if Some(dir_name.backtrack()) == direction {
        continue;
      }

      let new_row = (row as isize + dr) as isize;
      let new_col = (col as isize + dc) as isize;

      if new_row >= 0
        && new_col >= 0
        && is_valid(&grid, new_row as usize, new_col as usize, &state_grid)
      {
        let new_pos = (new_row as usize, new_col as usize);

        // Move to the new position
        to_visit_stack.push_back(State {
          pos: new_pos,
          direction: Some(dir_name),
        });

        found_next = true;
      }
    }

    // If no valid move found, backtrack by popping from path
    if !found_next {
      map[pos.0][pos.1] = '!';
      if has_candy(&state_grid) && !path.is_empty() {
        let (back_track, dir): ((usize, usize), Direction) = path.pop().unwrap();

        let backwards = dir.backtrack();
        // actual_walked_path.push(backwards);

        let state = State {
          pos: backwards.move_pos(back_track),
          direction: Some(backwards),
        };

        // println!("Backtrack: {:?}", state);

        to_visit_stack.push_back(state);
      }

      is_backtracking = true;
    } else {
      is_backtracking = false;
    }

    // if iter > 80 {
    //   break;
    // }
    // iter += 1;
  }

  Some(actual_walked_path)
}

fn has_candy(grid: &[Vec<GridState>]) -> bool {
  grid.iter().any(|row| row.iter().any(|c| *c == Candy))
}

fn print_grid(grid: &[Vec<char>]) {
  println!("============");
  grid
    .iter()
    .for_each(|line| println!("|{}|", line.iter().collect::<String>()));
  println!("============");
}

pub fn solve(
  _id: usize,
  input: String,
) -> miette::Result<String, ProblemError> {
  let _x = input.is_empty();

  let grid = input
    .lines()
    .map(|row| row.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let (start, _): ((usize, usize), &char) = grid
    .iter()
    .enumerate()
    .flat_map(|(idx, row): (usize, _)| {
      row
        .iter()
        .enumerate()
        .map(move |(colidx, col)| ((idx, colidx).clone(), col))
    })
    .find(|(_, char)| **char == 'L')
    .unwrap();

  // print_grid(&grid);

  match dfs_without_recursion(grid, start) {
    Some(path) => {
      let solution = path.iter().map(|dir| dir.to_string()).collect::<String>();
      println!("Path: {}", solution);
      println!("Length: {}", solution.len());

      Ok(solution)
    }
    None => Err(ProblemError::BadSolution {
      reason: "Not Solved",
    }),
  }
}

pub fn compress(solution: &str) -> ICFPExpr {
  let self_call = Var(1);
  let func = Var(2);

  let recurse = ICFPExpr::lambda(
    1,
    func,
    ICFPExpr::call(
      ICFPExpr::lambda(
        2,
        self_call,
        ICFPExpr::call(func, ICFPExpr::call(self_call, self_call)),
      ),
      ICFPExpr::lambda(
        3,
        self_call,
        ICFPExpr::call(func, ICFPExpr::call(self_call, self_call)),
      ),
    ),
  );

  let char_var = Var(10);
  let n_var = Var(12);

  let gen_char_loop_f_char_self_n = ICFPExpr::lambda(
    'c' as usize,
    char_var,
    ICFPExpr::lambda(
      4,
      self_call,
      ICFPExpr::lambda(
        5,
        n_var,
        ICFPExpr::if_(
          ICFPExpr::bin_op(n_var, BinOp::Equals, 1),
          char_var,
          ICFPExpr::bin_op(
            char_var,
            Concat,
            ICFPExpr::call(self_call, VarRef(n_var) - 1),
          ),
        ),
      ),
    ),
  );

  const RLE_CUTOFF: usize = 2;

  let y_comb = Var(0);
  let prog = let_in(y_comb, recurse, |y| {
    let loop_gen = Var(1);
    let_in(loop_gen, gen_char_loop_f_char_self_n, |loop_gen| {
      let gen_n_rs = Var(2);
      let gen_n_r = ICFPExpr::call(y, ICFPExpr::call(loop_gen, "R"));
      let_in(gen_n_rs, gen_n_r, |gen_n_rs| {
        let gen_n_ls = Var(3);
        let gen_n_l = ICFPExpr::call(y, ICFPExpr::call(loop_gen, "L"));
        let_in(gen_n_ls, gen_n_l, |gen_n_ls| {
          let gen_n_us = Var(4);
          let gen_n_u = ICFPExpr::call(y, ICFPExpr::call(loop_gen, "U"));
          let_in(gen_n_us, gen_n_u, |gen_n_us| {
            let gen_n_ds = Var(5);
            let gen_n_d = ICFPExpr::call(y, ICFPExpr::call(loop_gen, "D"));
            let_in(gen_n_ds, gen_n_d, |gen_n_ds| {
              let mut chars = solution.chars();
              let first_char = chars.next().unwrap();
              let init: (char, usize, Option<ICFPExpr>) = (first_char, 1, None);

              let (last_char, last_count, accum) =
                chars.fold(init, |(prev_char, count, acc), curr_char| {
                  if curr_char == prev_char {
                    (prev_char, count + 1, acc)
                  } else {
                    if count > RLE_CUTOFF {
                      let func = match prev_char {
                        'R' => ICFPExpr::call(gen_n_rs, count),
                        'L' => ICFPExpr::call(gen_n_ls, count),
                        'U' => ICFPExpr::call(gen_n_us, count),
                        'D' => ICFPExpr::call(gen_n_ds, count),
                        _ => panic!("Unexpected Char"),
                      };

                      if let Some(expr) = acc {
                        (curr_char, 1, Some(ICFPExpr::bin_op(expr, Concat, func)))
                      } else {
                        (curr_char, 1, Some(func))
                      }
                    } else {
                      if let Some(expr) = acc {
                        let repeated = ICFPExpr::str(prev_char.to_string().repeat(count));
                        (curr_char, 1, Some(ICFPExpr::bin_op(expr, Concat, repeated)))
                      } else {
                        let repeated = ICFPExpr::str(prev_char.to_string().repeat(count));
                        (curr_char, 1, Some(repeated))
                      }
                    }
                  }
                });

              let repeated = ICFPExpr::str(last_char.to_string().repeat(last_count));

              match accum {
                Some(acc) => ICFPExpr::bin_op(acc, Concat, repeated),
                None => {
                  if last_count > RLE_CUTOFF {
                    match last_char {
                      'R' => ICFPExpr::call(gen_n_rs, last_count),
                      'L' => ICFPExpr::call(gen_n_ls, last_count),
                      'U' => ICFPExpr::call(gen_n_us, last_count),
                      'D' => ICFPExpr::call(gen_n_ds, last_count),
                      _ => panic!("Unexpected Char"),
                    }
                  } else {
                    ICFPExpr::str(last_char.to_string().repeat(last_count))
                  }
                }
              }
            })
          })
        })
      })
    })
  });

  // fn callf_1<F: Fn(ICFPExpr) -> ICFPExpr>(target: &ICFPExpr) -> F {
  //   |a| ICFPExpr::call(target, a)
  // }
  //
  // callf_1()

  prog
}

#[cfg(test)]
mod test {
  use crate::expressions::evaluator::eval;
  use miette::Report;

  // #[test]
  fn prob9() -> Result<(), Report> {
    let result = super::solutions::problem_9()?;

    assert_eq!(eval(result)?, "..........".into());
    Ok(())
  }

  #[test]
  fn rle() -> Result<(), Report> {
    let result = super::compress("LLLDDDUUU");
    println!("RLE: {:?}", result);

    assert_eq!(eval(result)?, "LLLDDDUUU".into());
    Ok(())
  }

  #[test]
  fn rle_mixed() -> Result<(), Report> {
    let result = super::compress("LLLDDURRRR");
    println!("RLE: {:?}", result);

    assert_eq!(eval(result)?, "LLLDDURRRR".into());
    Ok(())
  }
}

pub mod solutions {
  use crate::expressions::parser::BinOp::Concat;
  use crate::expressions::parser::ICFPExpr::VarRef;
  use crate::expressions::parser::{BinOp, ICFPExpr, Var};
  use miette::Report;

  pub fn problem_9() -> Result<ICFPExpr, Report> {
    let func_var = Var(0);
    let self_var = Var(1);
    let self_call = Var(3);

    let recurse = ICFPExpr::lambda(
      1,
      func_var,
      ICFPExpr::call(
        ICFPExpr::lambda(
          2,
          self_var,
          ICFPExpr::call(func_var, ICFPExpr::call(self_var, self_var)),
        ),
        ICFPExpr::lambda(
          3,
          self_var,
          ICFPExpr::call(func_var, ICFPExpr::call(self_var, self_var)),
        ),
      ),
    );

    // ( 49 x R D 49 x L D ) x 25

    // loop(25) { // 1
    // (loop(49) { //10 "R".concat(loop(48)) }).concat("D")
    //  .concat(
    //        (loop(49) { // 20 "L".concat(loop(48)) }).concat("D")).(loop(24))

    // a + b
    // + b a

    let num_rs = 49;

    let n_var = Var(10);
    let gen_r_loop = ICFPExpr::lambda(
      4,
      self_call,
      ICFPExpr::lambda(
        5,
        n_var,
        ICFPExpr::if_(
          ICFPExpr::bin_op(n_var, BinOp::Equals, 1),
          "R",
          ICFPExpr::bin_op("R", Concat, ICFPExpr::call(self_call, VarRef(n_var) - 1)),
        ),
      ),
    );

    let generate_n_rs = ICFPExpr::call(
      ICFPExpr::call(recurse.clone(), gen_r_loop),
      ICFPExpr::int(num_rs),
    );

    let l_generator_func = ICFPExpr::lambda(
      6,
      self_call,
      ICFPExpr::lambda(
        7,
        n_var,
        ICFPExpr::if_(
          ICFPExpr::bin_op(n_var, BinOp::Equals, 1),
          "L",
          ICFPExpr::bin_op("L", Concat, ICFPExpr::call(self_call, VarRef(n_var) - 1)),
        ),
      ),
    );

    let generate_ls = ICFPExpr::call(ICFPExpr::call(recurse.clone(), l_generator_func), num_rs);

    let r_concat_d = ICFPExpr::bin_op(generate_n_rs, Concat, ICFPExpr::str("D"));
    let l_concat_d = ICFPExpr::bin_op(generate_ls, Concat, ICFPExpr::str("D"));

    let sol = ICFPExpr::bin_op(r_concat_d, Concat, l_concat_d);

    let prog = ICFPExpr::lambda(
      8,
      self_call,
      ICFPExpr::lambda(
        9,
        n_var,
        ICFPExpr::if_(
          ICFPExpr::bin_op(n_var, BinOp::Equals, 1),
          sol.clone(),
          ICFPExpr::bin_op(
            sol.clone(),
            Concat,
            ICFPExpr::call(self_call, VarRef(n_var) - 1),
          ),
        ),
      ),
    );

    let final_result = ICFPExpr::call(ICFPExpr::call(recurse.clone(), prog), 25);

    Ok(final_result)
  }
}
