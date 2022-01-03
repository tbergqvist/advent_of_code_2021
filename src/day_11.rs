type Position = (i8, i8);

fn add_energy(octopuses: &mut Vec<Vec<i8>>, (x, y): Position) -> i64 {
  if x < 0 || y < 0 || x >= 10 || y >= 10 {
    return 0;
  }

  let energy = &mut octopuses[y as usize][x as usize];
  *energy += 1;

  if *energy == 10 {
    1 +
    add_energy(octopuses, (x + 1, y - 1)) + 
    add_energy(octopuses, (x + 1, y)) +
    add_energy(octopuses, (x + 1, y + 1)) +
    add_energy(octopuses, (x - 1, y - 1)) +
    add_energy(octopuses, (x - 1, y)) +
    add_energy(octopuses, (x - 1, y + 1)) +
    add_energy(octopuses, (x, y + 1)) +
    add_energy(octopuses, (x, y - 1))
  } else {
    0
  }
}

pub fn a(input: &str) -> i64 {
  let mut octopuses:Vec<Vec<i8>> = input
    .lines()
    .map(|row| {
      row.bytes()
        .map(|c| c as i8 - 48)
        .collect()
    })
    .collect();

  let mut flashes = 0;
  for _ in 0..100 {
    for y in 0..10 {
      for x in 0..10 {
        flashes += add_energy(&mut octopuses, (x, y));
      }
    }

    octopuses.iter_mut()
      .flat_map(|row|row.iter_mut())
      .filter(|i| **i >= 10)
      .for_each(|i|*i = 0);
  }
  flashes
}

pub fn b(input: &str) -> i64 {
  let mut octopuses:Vec<Vec<i8>> = input
  .lines()
  .map(|row| {
    row.bytes()
      .map(|c| c as i8 - 48)
      .collect()
  })
  .collect();

  for round in 1.. {
    let mut flashes = 0;
    for y in 0..10 {
      for x in 0..10 {
        flashes += add_energy(&mut octopuses, (x, y));
      }
    }
    if flashes >= 100 {
      return round;
    }

    octopuses.iter_mut()
      .flat_map(|row|row.iter_mut())
      .filter(|i| **i >= 10)
      .for_each(|i|*i = 0);
  }

  0
}
