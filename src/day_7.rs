pub fn a(input: &str) -> i32 {
  let positions: Vec<i32> = input
    .lines()
    .next()
    .unwrap()
    .split_terminator(',')
    .map(|s| s.parse().unwrap())
    .collect();

    let highest = positions.iter().max().unwrap();

    (0..=*highest).into_iter()
      .map(|target_pos| positions.iter().map(|pos| (target_pos - pos).abs()).sum())
      .min().unwrap()
}

pub fn b(input: &str) -> i32 {
  let positions: Vec<i32> = input
  .lines()
  .next()
  .unwrap()
  .split_terminator(',')
  .map(|s| s.parse().unwrap())
  .collect();

  let highest = positions.iter().max().unwrap();

  (0..=*highest).into_iter()
    .map(|target_pos| positions.iter().map(|pos| {
      let distance = (target_pos - pos).abs();
      (1..=distance).into_iter().sum::<i32>()
    }).sum())
    .min().unwrap()
}