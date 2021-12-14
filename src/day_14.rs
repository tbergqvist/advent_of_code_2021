use std::collections::HashMap;

type Bla = char;

type Value = ([Bla;2], Bla);

fn run(input: &str, rounds: u32) -> usize {
  let mut start: Vec<Bla> = input
    .lines()
    .next()
    .map(|line| line.chars())
    .unwrap()
    .collect();

  let rules: Vec<Value> = input
    .lines()
    .skip(2)
    .map(|line| {
      let val = line.split_once(" -> ").unwrap();
      let (a, b) = val.0.split_at(1);
      ([a.parse().unwrap(), b.parse().unwrap()], val.1.parse().unwrap())
    })
    .collect();
  
  for round in 0..rounds {
    let positions: Vec<(usize, Bla)> = start
      .windows(2)
      .enumerate()
      .filter_map(|(i, window)|
        rules.iter().find(|(target, _)| target == window)
        .map(|(_, c)|(i + 1, *c))
      )
      .collect();

    positions
      .iter()
      .enumerate()
      .for_each(|(i, (position, c))| {
        start.insert(position + i, *c);
      });
  }

  let map = start.into_iter().fold(HashMap::new(), |mut map: HashMap<Bla, usize>, c| {
    map.entry(c).and_modify(|sum| *sum += 1).or_insert(1);
    map
  });

  map.values().max().unwrap() - map.values().min().unwrap()
}

pub fn a(input: & str) -> usize {
  run(input, 10)
}

pub fn b(input: &str) -> usize {
  run(input, 0)
}
