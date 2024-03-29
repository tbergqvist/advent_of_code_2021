pub fn a(input: &str) -> i64 {
  input
    .lines()
    .map(|s| s.parse::<i64>().unwrap())
    .fold((0, None), |(count, prev_value), value| {
      (
        if prev_value.is_some() && value > prev_value.unwrap() {
          count + 1
        } else {
          count
        }, 
        Some(value)
      )
    }).0
}

pub fn b(input: &str) -> i64 {
  let rows: Vec<i64> = input
    .lines()
    .map(|s| s.parse().unwrap())
    .collect();

  rows.windows(3)
    .map(|slice| slice.iter().sum())
    .fold((0, None), |(count, prev_value), value: i64| {
      (
        if prev_value.is_some() && value > prev_value.unwrap() {
          count + 1
        } else {
          count
        }, 
        Some(value)
      )
    }).0
}