use std::collections::HashMap;

fn run(input: &str, rounds: u32) -> usize {
  let first_line = input
    .lines()
    .next()
    .unwrap();

  let mut counts: HashMap<&[u8], usize> = first_line
    .as_bytes()
    .windows(2)
    .fold(HashMap::new(), |mut map: HashMap<&[u8], usize>, key| {
      map
        .entry(key)
        .and_modify(|count| *count += 1)
        .or_insert(1);
      map
    });

  let pairs_created: HashMap<&[u8], Vec<String>> = input
    .lines()
    .skip(2)
    .map(|line| {
      let val = line.split_once(" -> ").unwrap();
      let (a, b) = val.0.split_at(1);
      (val.0.as_bytes(), vec![a.to_string() + val.1, val.1.to_string() + b])
    })
    .collect();

  for _ in 0..rounds {
    let pairs_to_create: Vec<(&String, usize)> = counts
      .iter()
      .flat_map(|(key, count)| pairs_created
        .get(key)
        .unwrap()
        .iter()
        .map(|s|(s, *count)))
      .collect();

    counts
      .values_mut()
      .for_each(|val|*val = 0);

    pairs_to_create
      .into_iter()
      .for_each(|(pair, current_count)| {
        counts
          .entry(pair.as_bytes())
          .and_modify(|count| *count += current_count)
          .or_insert(current_count);
      });
  }

  let mut map: HashMap<&u8, usize> = counts
    .into_iter()
    .map(|(key, value)|(&key[1], value))
    .fold(HashMap::new(), |mut map: HashMap<&u8, usize>, (c, current_count)| {
      map
        .entry(c)
        .and_modify(|count| *count += current_count)
        .or_insert(current_count);
      map
    });

  let start_val = map.get_mut(&first_line.as_bytes()[0]).unwrap();
  *start_val += 1;

  map.values().max().unwrap() - map.values().min().unwrap()
}

pub fn a(input: & str) -> usize {
  run(input, 10)
}

pub fn b(input: &str) -> usize {
  run(input, 40)
}
