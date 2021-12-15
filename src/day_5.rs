use std::collections::{HashMap};
use std::str::FromStr;
use std::cmp;

struct Line {
  from: (i32, i32),
  to: (i32, i32)
}

impl FromStr for Line {
  type Err = String;

  fn from_str(string_format: &str) -> Result<Self, Self::Err> {
    let points: Vec<i32> = string_format
      .replace("->", ",")
      .split_terminator(',')
      .map(|str| str
        .chars()
        .filter(|c|c.is_digit(10))
        .collect::<String>()
      )
      .map(|point|point.parse().unwrap())
      .collect();
    
    Ok(Line {
      from: (points[0], points[1]),
      to: (points[2], points[3])
    })
  }
}

pub fn a(input: &str) -> usize {
  let lines: HashMap<(i32, i32), i32> = input
    .lines()
    .map(|s| Line::from_str(s).unwrap())
    .filter(|line| line.from.0 == line.to.0 || line.from.1 == line.to.1)
    .fold(HashMap::new(), |mut state, line| {
      if line.from.0 == line.to.0 {
        for i in cmp::min(line.from.1,line.to.1)..=cmp::max(line.from.1,line.to.1) {
          state.entry((line.from.0, i)).and_modify(|count| *count+= 1).or_insert(1);
        }
      } else if line.from.1 == line.to.1 {
        for i in cmp::min(line.from.0,line.to.0)..=cmp::max(line.from.0,line.to.0) {
          state.entry((i, line.to.1)).and_modify(|count| *count+= 1).or_insert(1);
        }
      }
      state
    });

  lines.iter()
    .filter(|(_, count)| **count > 1)
    .count()
}

pub fn b(input: &str) -> usize {
  let lines: HashMap<(i32, i32), i32> = input
    .lines()
    .map(|s| Line::from_str(s).unwrap())
    .fold(HashMap::new(), |mut state, line| {
      if line.from.0 == line.to.0 {
        for i in cmp::min(line.from.1,line.to.1)..=cmp::max(line.from.1,line.to.1) {
          state.entry((line.from.0, i)).and_modify(|count| *count+= 1).or_insert(1);
        }
      } else if line.from.1 == line.to.1 {
        for i in cmp::min(line.from.0,line.to.0)..=cmp::max(line.from.0,line.to.0) {
          state.entry((i, line.to.1)).and_modify(|count| *count+= 1).or_insert(1);
        }
      } else {
        let (x, y) = (line.from.0 - line.to.0, line.from.1 - line.to.1);
        if (x > 1 && y > 1) || (x < 0 && y < 0) {
          let mut start = if line.from.0 < line.to.0 {line.from} else {line.to};
          let end = if start == line.from {line.to} else {line.from};
          
          while start != end {
            state.entry((start.0, start.1)).and_modify(|count| *count+= 1).or_insert(1);
            start = (start.0 + 1, start.1 + 1);
          }
          state.entry((start.0, start.1)).and_modify(|count| *count+= 1).or_insert(1);
        } else if (x > 1 && y < 0) || (x < 0 && y > 1) {
          let mut start = if line.from.0 < line.to.0 {line.from} else {line.to};
          let end = if start == line.from {line.to} else {line.from};
          
          while start != end {
            state.entry((start.0, start.1)).and_modify(|count| *count+= 1).or_insert(1);
            start = (start.0 + 1, start.1 - 1);
          }
          state.entry((start.0, start.1)).and_modify(|count| *count+= 1).or_insert(1);
        } else {
          dbg!("should not happen");
        }
      }
      state
    });

  lines.iter()
    .filter(|(_, count)| **count > 1)
    .count()
}