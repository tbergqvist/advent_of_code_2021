use pathfinding::prelude::{astar};

pub fn run(input: &str, map_size: i32, tiled: i32) -> i32 {
  let map: Vec<Vec<u8>> = input
    .lines()
    .map(|line| 
      line.as_bytes()
        .iter()
        .map(|c|*c - 48)
        .collect())
    .collect();

  let goal = (map_size * tiled - 1, map_size * tiled - 1);

  astar(&(0, 0), |&(x, y)| {
    vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
      .into_iter()
      .filter(|&(x, y)| x >= 0 && y >= 0 && x <= goal.0 && y <= goal.1)
      .map(|(x, y)| {
        let map_x = x % map_size;
        let map_y = y % map_size;
        let risk_mod = x / map_size + y / map_size;
        let risk = (map[map_y as usize][map_x as usize] + risk_mod as u8 - 1) % 9 + 1;
        ((x, y), risk as i32)
      })
    },
    |&(x, y)| {
      goal.0 - x + goal.1 - y
    }, |pos| {
      pos == &goal
    }
  ).unwrap().1
}

pub fn a(input: &str) -> i32 {
  run(input, 100, 1)
}

pub fn b(input: &str) -> i32 {
  run(input, 100, 5)
}
