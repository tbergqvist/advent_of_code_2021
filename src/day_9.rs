fn get_point(height_map: &[Vec<i32>], x: i32, y: i32) -> i32 {
  if x >= 100 || y >= 100 || x < 0 || y < 0 {
    10
  } else {
    height_map[y as usize][x as usize]
  }
}

pub fn a(input: &str) -> i32 {
  let height_map: Vec<Vec<i32>> = input
    .lines()
    .map(|line| {
      line.bytes().map(|b|(b - b'0') as i32).collect()
    })
    .collect();

  height_map.iter().enumerate().map(|(y, row)| {
    row.iter().enumerate().filter(|(x, point)| {
      get_point(&height_map, *x as i32 - 1, y as i32) > **point &&
        get_point(&height_map, *x as i32 + 1, y as i32) > **point &&
        get_point(&height_map, *x as i32, y  as i32 - 1) > **point &&
        get_point(&height_map, *x as i32, y as i32 + 1) > **point
    })
    .map(|(_, point)| (point + 1) as i32)
    .sum::<i32>()
  }).sum()
}

pub fn fill(height_map: &mut Vec<Vec<i32>>, (x, y): (i32, i32)) -> usize {
  if x < 0 || y < 0 || x >= 100 || y >= 100 || height_map[y as usize][x as usize] >= 9 {
    return 0;
  }

  height_map[y as usize][x as usize] += 10;
  1 + 
    fill(height_map, (x - 1, y)) + 
    fill(height_map, (x + 1, y)) +
    fill(height_map, (x, y - 1)) +
    fill(height_map, (x, y + 1))
}

pub fn b(input: &str) -> usize {
  let mut height_map: Vec<Vec<i32>> = input
    .lines()
    .map(|line| {
      line.bytes().map(|b|(b - b'0') as i32).collect()
    })
    .collect();
  
  let low_points: Vec<(i32, i32)> = height_map
    .iter()
    .enumerate()
    .flat_map(|(y, row)| {
      let height_map = &height_map;

      row.iter()
        .enumerate()
        .filter(move |(x, point)| {
          get_point(height_map, *x as i32 - 1, y as i32) > **point &&
          get_point(height_map, *x as i32 + 1, y as i32) > **point &&
          get_point(height_map, *x as i32, y as i32 - 1) > **point &&
          get_point(height_map, *x as i32, y as i32 + 1) > **point
        })
        .map(move |(x, _)| (x as i32, y as i32))
  })
  .collect();

  let mut basins: Vec<usize> = low_points.into_iter()
    .map(|low_point| fill(&mut height_map, low_point))
    .collect();

  basins.sort_unstable_by(|a, b| b.cmp(a));

  basins[0] * basins[1] * basins[2]
}