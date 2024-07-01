use crate::problems::{Direction, Point, ProblemError};
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use tracing::{debug, error, info, trace};

// Shared types
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Operator {
  ShiftLeft,
  ShiftRight,
  ShiftUp,
  ShiftDown,
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Warp,
  Eq,
  NotEq,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
enum OperatorArity {
  Unary,
  Binary,
  Quad,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Parameter {
  A,
  B,
}

impl Display for Parameter {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Parameter::A => 'A',
        Parameter::B => 'B',
      }
    )
  }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum CellValues {
  Op(Operator),
  Param(Parameter),
  Val(i32),
  EndState,
  Empty,
}

impl Default for CellValues {
  fn default() -> Self {
    CellValues::Empty
  }
}

#[derive(Copy, Clone)]
struct Cell {
  x: i32,
  y: i32,
  value: CellValues,
}

impl Cell {
  fn point(&self) -> Point {
    Point::at(self.x, self.y)
  }
}

impl Debug for Cell {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_tuple("Cell").field(&self.value).finish()
  }
}

// End

#[derive(Clone, Debug)]
struct StateChanges {
  time: Option<usize>,
  results: HashMap<Point, CellValues>,
}

// Helpers
impl Operator {
  fn args(&self) -> Vec<Direction> {
    use Direction::*;
    match self {
      Operator::ShiftLeft => vec![Right],
      Operator::ShiftRight => vec![Left],
      Operator::ShiftUp => vec![Down],
      Operator::ShiftDown => vec![Up],
      Operator::Add
      | Operator::Sub
      | Operator::Mul
      | Operator::Div
      | Operator::Eq
      | Operator::NotEq
      | Operator::Mod => vec![Up, Left],
      Operator::Warp => vec![Left, Right, Up, Down],
    }
  }

  fn apply(
    &self,
    pos: Point,
    args: HashMap<Direction, Cell>,
  ) -> Option<StateChanges> {
    use Direction::*;

    let changes = match self {
      Operator::ShiftLeft => {
        let value = args.get(&Right).unwrap().value;
        StateChanges {
          time: None,
          results: HashMap::from([(pos + Left, value)]),
        }
      }
      Operator::ShiftRight => {
        let value = args.get(&Left).unwrap().value;
        StateChanges {
          time: None,
          results: HashMap::from([(pos + Right, value)]),
        }
      }
      Operator::ShiftUp => {
        let value = args.get(&Down).unwrap().value;
        StateChanges {
          time: None,
          results: HashMap::from([(pos + Up, value)]),
        }
      }
      Operator::ShiftDown => {
        let value = args.get(&Up).unwrap().value;
        StateChanges {
          time: None,
          results: HashMap::from([(pos + Down, value)]),
        }
      }
      Operator::Add => {
        let left = args.get(&Left).unwrap().value;
        let up = args.get(&Up).unwrap().value;

        let value = CellValues::Val(match (left, up) {
          (CellValues::Val(l), CellValues::Val(u)) => l + u,
          _ => {
            trace!(op = ?self, "incorrect arg, skipping");
            return None;
          }
        });

        StateChanges {
          time: None,
          results: HashMap::from([(pos + Down, value), (pos + Right, value)]),
        }
      }
      Operator::Sub => {
        let left = args.get(&Left).unwrap().value;
        let up = args.get(&Up).unwrap().value;

        let value = CellValues::Val(match (left, up) {
          (CellValues::Val(l), CellValues::Val(u)) => l - u,
          _ => {
            trace!(op = ?self, "incorrect arg, skipping");

            return None;
          }
        });

        StateChanges {
          time: None,
          results: HashMap::from([(pos + Down, value), (pos + Right, value)]),
        }
      }
      Operator::Mul => {
        let left = args.get(&Left).unwrap().value;
        let up = args.get(&Up).unwrap().value;

        let value = CellValues::Val(match (left, up) {
          (CellValues::Val(l), CellValues::Val(u)) => l * u,
          _ => {
            trace!(op = ?self, "incorrect arg, skipping");

            return None;
          }
        });

        StateChanges {
          time: None,
          results: HashMap::from([(pos + Down, value), (pos + Right, value)]),
        }
      }
      Operator::Div => {
        let left = args.get(&Left).unwrap().value;
        let up = args.get(&Up).unwrap().value;

        let value = CellValues::Val(match (left, up) {
          (CellValues::Val(l), CellValues::Val(u)) => l / u,
          _ => {
            trace!(op = ?self, "incorrect arg, skipping");

            return None;
          }
        });

        StateChanges {
          time: None,
          results: HashMap::from([(pos + Down, value), (pos + Right, value)]),
        }
      }
      Operator::Mod => {
        let left = args.get(&Left).unwrap().value;
        let up = args.get(&Up).unwrap().value;

        let value = CellValues::Val(match (left, up) {
          (CellValues::Val(l), CellValues::Val(u)) => l % u,
          _ => {
            trace!(op = ?self, "incorrect arg, skipping");
            return None;
          }
        });

        StateChanges {
          time: None,
          results: HashMap::from([(pos + Down, value), (pos + Right, value)]),
        }
      }
      Operator::Eq => {
        let left = args.get(&Left).unwrap().value;
        let up = args.get(&Up).unwrap().value;

        let value = match (left, up) {
          (l @ CellValues::Val(_), u @ CellValues::Val(_))
          | (l @ CellValues::Op(_), u @ CellValues::Op(_))
            if l == u =>
          {
            trace!(left = ?l, up = ?u, "equal!");
            l
          }
          _ => {
            trace!(op = ?self, "incorrect arg, skipping");
            return None;
          }
        };

        StateChanges {
          time: None,
          results: HashMap::from([(pos + Down, value), (pos + Right, value)]),
        }
      }
      Operator::NotEq => {
        let left = args.get(&Left).unwrap().value;
        let up = args.get(&Up).unwrap().value;

        let (left, up) = match (left, up) {
          (l @ CellValues::Val(_), u @ CellValues::Val(_))
          | (l @ CellValues::Op(_), u @ CellValues::Op(_))
          | (l @ CellValues::Val(_), u @ CellValues::Op(_))
          | (l @ CellValues::Op(_), u @ CellValues::Val(_))
            if l != u =>
          {
            (l, u)
          }
          _ => {
            trace!(op = ?self, "incorrect arg, skipping");
            return None;
          }
        };

        StateChanges {
          time: None,
          results: HashMap::from([(pos + Down, left), (pos + Right, up)]),
        }
      }
      Operator::Warp => {
        let CellValues::Val(dx) = args.get(&Left).unwrap().value else {
          trace!(op = ?self, arg = ?Left, "incorrect arg, skipping");
          return None;
        };
        let CellValues::Val(dy) = args.get(&Right).unwrap().value else {
          trace!(op = ?self, arg = ?Right, "incorrect arg, skipping");
          return None;
        };
        let CellValues::Val(value) = args.get(&Up).unwrap().value else {
          trace!(op = ?self, arg = ?Up, "incorrect arg, skipping");
          return None;
        };
        let CellValues::Val(dt) = args.get(&Down).unwrap().value else {
          trace!(op = ?self, arg = ?Down, "incorrect arg, skipping");
          return None;
        };

        trace!(dx, dy, value, dt, ?pos, "Warping!");
        StateChanges {
          time: Some(dt as usize),
          results: HashMap::from([(pos + (-dx, -dy), CellValues::Val(value))]),
        }
      }
    };

    Some(changes)
  }

  fn _arity(&self) -> OperatorArity {
    match self {
      Operator::ShiftLeft => OperatorArity::Unary,
      Operator::ShiftRight => OperatorArity::Unary,
      Operator::ShiftUp => OperatorArity::Unary,
      Operator::ShiftDown => OperatorArity::Unary,
      Operator::Add => OperatorArity::Binary,
      Operator::Sub => OperatorArity::Binary,
      Operator::Mul => OperatorArity::Binary,
      Operator::Div => OperatorArity::Binary,
      Operator::Mod => OperatorArity::Binary,
      Operator::Warp => OperatorArity::Quad,
      Operator::Eq => OperatorArity::Binary,
      Operator::NotEq => OperatorArity::Binary,
    }
  }
}

impl Display for Operator {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Operator::ShiftLeft => '<',
        Operator::ShiftRight => '>',
        Operator::ShiftUp => '^',
        Operator::ShiftDown => 'v',
        Operator::Add => '+',
        Operator::Sub => '-',
        Operator::Mul => '*',
        Operator::Div => '/',
        Operator::Mod => '%',
        Operator::Warp => '@',
        Operator::Eq => '=',
        Operator::NotEq => '#',
      }
    )
  }
}
// End

// Read from file
// End

// Compiler
// End

// Evaluator
fn evaluate(
  mut grid: HashMap<Point, Cell>,
  args: HashMap<Parameter, i32>,
  iterations: usize,
) {
  let mut states = Vec::new();
  states.push(grid.clone()); // t0

  // replace params
  grid.values_mut().for_each(|v| {
    if let CellValues::Param(ref p) = v.value {
      v.value = args
        .get(p)
        .map(|v| CellValues::Val(*v))
        .unwrap_or(CellValues::Empty);
    }
  });

  states.push(grid.clone()); // t1

  'ticker: for time in 1..=iterations {
    let mut new_time: Option<usize> = None;

    let grid = states[time].clone();

    println!("======= T{time} =======");
    // print_grid(&grid);

    let operators: Vec<(Point, Operator)> = grid
      .values()
      .copied()
      .filter_map(|cell| {
        if let CellValues::Op(op) = cell.value {
          Some((Point::at(cell.x, cell.y), op))
        } else {
          None
        }
      })
      .collect();

    debug!(operator_count = operators.len(), "Found Operators");

    let mut map = grid.clone();
    let mut consumed = HashSet::new();
    let mut written = HashSet::new();

    let mut warps: Vec<StateChanges> = Vec::new();

    for (p, op) in &operators {
      debug!(point = ?p, ?op, "processing operator");
      let arg_len = op.args().len();
      let args: HashMap<Direction, _> = op
        .args()
        .iter()
        .filter_map(|dir| match grid.get(&(*p + *dir)) {
          Some(cell) if cell.value != CellValues::Empty => Some((*dir, *cell)),
          _ => None,
        })
        .collect();

      trace!(args = ?args, ?op, "collected args");
      if args.len() == arg_len {
        for (_, cell) in &args {
          consumed.insert(cell.point());
        }

        if let Some(changes) = op.apply(*p, args) {
          debug!(?changes, "operator has changes");
          for (p, v) in &changes.results {
            let new_cell = Cell {
              x: p.x,
              y: p.y,
              value: *v,
            };

            if !written.contains(p) {
              written.insert(*p);
            } else {
              error!(pos = ?p, "write conflict: double write!!");
              return;
            }

            debug!(new = ?*v, pos=?p, "update");
            if let Some(previous) = (&mut map).insert(*p, new_cell) {
              if matches!(previous.value, CellValues::EndState) {
                info!(value = ?v, "END STATE SET!");
                print_grid(&map);

                println!("Value: {v:?}");

                states.push(map.clone());
                break 'ticker;
              } else if !matches!(previous.value, CellValues::Empty) {
              }
            }

            // TODO: Enforce conflict writes

            if let Some(time_delta) = changes.time {
              let proposal = time - time_delta;
              trace!(to = proposal, "requesting warp");
              new_time = match new_time {
                None => Some(proposal),
                Some(proposed_time) if proposed_time == proposal => Some(proposal),
                _ => panic!("Multiple time warps"),
              };

              warps.push(changes.clone());
            }
          }
        }
      }

      trace!("processed operator")
    }

    let mut warp_writes = HashSet::new();

    if let Some(new_time) = new_time {
      map = states[new_time].clone();
      for changes in warps {
        for (p, cell) in changes.results {
          warp_writes.insert(p);
          map.insert(
            p,
            Cell {
              x: p.x,
              y: p.y,
              value: cell,
            },
          );
        }
      }
    } else {
      for consumed_point in &consumed {
        if !written.contains(consumed_point) {
          map.insert(
            *consumed_point,
            Cell {
              x: consumed_point.x,
              y: consumed_point.y,
              value: CellValues::Empty,
            },
          );
        }
      }
    };

    print_pretty_grid(&grid, &consumed, &written, &warp_writes);

    if map
      .values()
      .find(|v| matches!(v.value, CellValues::EndState))
      .is_none()
    {
      println!("END STATE GONE!");
      print_grid(&map);
      states.push(map);
      break;
    } else {
      states.push(map);
    }
  }
}
// End

const COLUMN_WIDTH: usize = 3;

fn grid_line(line: &[CellValues]) -> String {
  line
    .iter()
    .map(|cell| {
      let symbol = match cell {
        CellValues::Op(o) => o.to_string(),
        CellValues::Param(p) => p.to_string(),
        CellValues::Val(v) => v.to_string(),
        CellValues::EndState => "S".to_string(),
        CellValues::Empty => ".".to_string(),
      };
      format!("{:^COLUMN_WIDTH$}", symbol)
    })
    .collect::<String>()
}

fn pretty_grid_line(
  col: usize,
  line: &[CellValues],
  consumed: &HashSet<Point>,
  writes: &HashSet<Point>,
  warp_writes: &HashSet<Point>,
) -> String {
  use owo_colors::{OwoColorize, Style};

  let warp_write = Style::new().bright_black().on_bright_green().blink();
  let write = Style::new().bright_black().on_bright_red().bold();
  let read = Style::new().black().on_bright_blue().underline();
  let norm = Style::new().white().on_black();

  line
    .iter()
    .enumerate()
    .map(|(x, cell)| {
      let symbol = match cell {
        CellValues::Op(o) => o.to_string(),
        CellValues::Param(p) => p.to_string(),
        CellValues::Val(v) => v.to_string(),
        CellValues::EndState => "S".to_string(),
        CellValues::Empty => "•".to_string(),
      };

      let point = Point::at(x as i32, col as i32);
      let sym = if warp_writes.contains(&point) {
        symbol.style(warp_write)
      } else if writes.contains(&point) {
        symbol.style(write)
      } else if consumed.contains(&point) {
        symbol.style(read)
      } else {
        symbol.style(norm)
      };

      format!("{:^COLUMN_WIDTH$}", sym)
    })
    .collect::<String>()
}

fn print_compact(grid: &HashMap<Point, Cell>) -> String {
  let xs = grid.keys().map(|p| p.x).collect::<Vec<_>>();
  let ys = grid.keys().map(|p| p.y).collect::<Vec<_>>();

  let min_x = *xs.iter().min().unwrap();
  let max_x = *xs.iter().max().unwrap() + 1;

  let min_y = *ys.iter().min().unwrap();
  let max_y = *ys.iter().max().unwrap() + 1;

  let cols = (max_x - min_x) as usize;
  let rows = (max_y - min_y) as usize;

  let mut map = vec![vec![CellValues::default(); cols]; rows];

  grid.iter().for_each(|(k, v)| {
    map[(k.y - min_y) as usize][(k.x - min_x) as usize] = v.value;
  });

  map
    .iter()
    .map(|line| grid_line(&line))
    .collect::<Vec<_>>()
    .join("\n")
}

fn print_pretty_grid(
  grid: &HashMap<Point, Cell>,
  consumed: &HashSet<Point>,
  writes: &HashSet<Point>,
  warp_writes: &HashSet<Point>,
) {
  let xs = grid.keys().map(|p| p.x).collect::<Vec<_>>();
  let ys = grid.keys().map(|p| p.y).collect::<Vec<_>>();

  let min_x = min(*xs.iter().min().unwrap(), 0);
  let max_x = *xs.iter().max().unwrap() + 1;

  let min_y = min(*ys.iter().min().unwrap(), 0);
  let max_y = *ys.iter().max().unwrap() + 1;

  let cols = (max_x - min_x) as usize;
  let rows = (max_y - min_y) as usize;

  println!("{}", "=".repeat(COLUMN_WIDTH * cols + 10));
  println!(
    "   | {}",
    (0..cols)
      .map(|c| {
        if c < 10 || c % 2 == 0 {
          format!("{c:^COLUMN_WIDTH$}")
        } else {
          format!("{:^COLUMN_WIDTH$}", "")
        }
      })
      .collect::<String>()
  );
  println!("{}", "_".repeat(COLUMN_WIDTH * cols + 10));
  let mut map = vec![vec![CellValues::default(); cols]; rows];

  grid.iter().for_each(|(k, v)| {
    map[(k.y - min_y) as usize][(k.x - min_x) as usize] = v.value;
  });

  map.iter().enumerate().for_each(|(no, line)| {
    println!(
      "{no:^3}| {}",
      pretty_grid_line(no, &line, consumed, writes, warp_writes)
    )
  });
  println!("{}", "=".repeat(COLUMN_WIDTH * cols + 10));
}

fn print_grid(grid: &HashMap<Point, Cell>) {
  let xs = grid.keys().map(|p| p.x).collect::<Vec<_>>();
  let ys = grid.keys().map(|p| p.y).collect::<Vec<_>>();

  let min_x = min(*xs.iter().min().unwrap(), 0);
  let max_x = *xs.iter().max().unwrap() + 1;

  let min_y = min(*ys.iter().min().unwrap(), 0);
  let max_y = *ys.iter().max().unwrap() + 1;

  let cols = (max_x - min_x) as usize;
  let rows = (max_y - min_y) as usize;

  println!("{}", "=".repeat(COLUMN_WIDTH * cols + 10));
  println!(
    "   | {}",
    (0..cols)
      .map(|c| {
        if c < 10 || c % 2 == 0 {
          format!("{c:^COLUMN_WIDTH$}")
        } else {
          format!("{:^COLUMN_WIDTH$}", "")
        }
      })
      .collect::<String>()
  );
  println!("{}", "_".repeat(COLUMN_WIDTH * cols + 10));
  let mut map = vec![vec![CellValues::default(); cols]; rows];

  grid.iter().for_each(|(k, v)| {
    map[(k.y - min_y) as usize][(k.x - min_x) as usize] = v.value;
  });

  map
    .iter()
    .enumerate()
    .for_each(|(no, line)| println!("{no:^3}| {}", grid_line(&line)));
  println!("{}", "=".repeat(COLUMN_WIDTH * cols + 10));
}

// Visualizer
fn parse_grid(grid: String) -> HashMap<Point, Cell> {
  let lines: Vec<Vec<_>> = grid
    .lines()
    .map(|l| l.trim())
    .filter(|l| !l.is_empty() && !l.starts_with(";"))
    .map(|l| l.split_whitespace().collect::<Vec<_>>())
    .collect();

  let mut map = HashMap::new();

  for (row, line) in lines.iter().enumerate() {
    for (col, cell) in line.iter().enumerate() {
      let point = Point::at(col as i32, row as i32);

      map.insert(
        point,
        Cell {
          x: point.x,
          y: point.y,
          value: match *cell {
            "+" => CellValues::Op(Operator::Add),
            "-" => CellValues::Op(Operator::Sub),
            "<" => CellValues::Op(Operator::ShiftLeft),
            ">" => CellValues::Op(Operator::ShiftRight),
            "^" => CellValues::Op(Operator::ShiftUp),
            "v" | "V" => CellValues::Op(Operator::ShiftDown),
            "*" => CellValues::Op(Operator::Mul),
            "/" => CellValues::Op(Operator::Div),
            "%" => CellValues::Op(Operator::Mod),
            "=" => CellValues::Op(Operator::Eq),
            "#" => CellValues::Op(Operator::NotEq),
            "@" => CellValues::Op(Operator::Warp),
            "A" => CellValues::Param(Parameter::A),
            "B" => CellValues::Param(Parameter::B),
            "S" => CellValues::EndState,
            a if a.chars().all(|c| c == '-' || c.is_ascii_digit()) => {
              CellValues::Val(a.parse().unwrap())
            }
            "." => continue,
            c => panic!("I don't know this char: {c}"),
          },
        },
      );
    }
  }

  map
}

// End

pub fn solve(
  _id: usize,
  input: String,
) -> miette::Result<String, ProblemError> {
  let map = parse_grid(input);
  print_grid(&map);

  let string = print_compact(&map);

  println!("{}", string);

  Ok(string)
}

pub(crate) fn simulate(
  iterations: usize,
  input: String,
  args: Vec<String>,
) -> miette::Result<String, ProblemError> {
  let map = parse_grid(input);
  print_grid(&map);

  println!("Starting simulation");

  let arg_map = args
    .iter()
    .zip([Parameter::A, Parameter::B])
    .map(|(arg, param)| {
      (
        param,
        arg
          .parse()
          .unwrap_or_else(|_| panic!("invalid digit: {arg})")),
      )
    })
    .collect();

  evaluate(map, arg_map, iterations);

  Ok("Good".to_string())
}
