use rayon::prelude::*;

pub fn a(input: &str) -> i64 {
  let mut positions: Vec<i64> = input
    .lines()
    .next()
    .unwrap()
    .split_terminator(',')
    .map(|s| s.parse().unwrap())
    .collect();
    
  positions.sort_unstable();
  let mid = positions[positions.len() / 2];
  positions.iter().map(|pos| (pos - mid).abs()).sum()
}

pub fn b(input: &str) -> i64 {
  let positions: Vec<i64> = input
    .lines()
    .next()
    .unwrap()
    .split_terminator(',')
    .map(|s| s.parse().unwrap())
    .collect();

  let highest = positions.iter().max().unwrap();
  (0..=*highest).into_par_iter()
    .map(|target_pos| 
      positions.par_iter().map(|pos| {
        let distance = (target_pos - pos).abs();
        (distance + 1) * distance / 2
    })
    .sum())
  .min().unwrap()
}