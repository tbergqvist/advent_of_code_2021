type Position = (usize, usize);

fn fold_x((x, y): Position, fold_pos: usize) -> Position {
  if x > fold_pos {
    (fold_pos * 2 - x, y)
  } else {
    (x, y)
  }
}

fn fold_y((x, y): Position, fold_pos: usize) -> Position {
  if y > fold_pos {
    (x, fold_pos * 2 - y)
  } else {
    (x, y)
  }
}

#[derive(Debug)]
struct Fold {
  fold_x: bool,
  position: usize,
}

impl Fold {
  fn fold(&self, position: Position) -> Position {
    if self.fold_x { 
      fold_x(position, self.position)
    } else {
      fold_y(position, self.position)
    }
  }
}

pub fn a(input: &str) -> usize {
  let folds: Vec<Fold> = input
    .lines()
    .skip(1023)
    .map(|line| Fold{ 
      fold_x: line.as_bytes().get(11).unwrap() == &b'x', 
      position: line.split_at(13).1.parse().unwrap()
    })
    .collect();

  let mut dots: Vec<Position> = input
    .lines()
    .take(1022)
    .map(|line|line.split_once(',').unwrap())
    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
    .map(|position| {
      folds.iter()
        .take(1)
        .fold(position, |current_pos, fold| fold.fold(current_pos))
    })
    .collect();

  dots.sort_unstable();
  dots.dedup();
  dots.len()
}

pub fn b(input: &str) -> usize {
  let folds: Vec<Fold> = input
    .lines()
    .skip(1023)
    .map(|line| Fold{ 
      fold_x: line.as_bytes().get(11).unwrap() == &b'x', 
      position: line.split_at(13).1.parse().unwrap()
    })
    .collect();

  let mut image: Vec<Vec<char>> = vec![vec![' ';40]; 6];
  input
    .lines()
    .take(1022)
    .map(|line|line.split_once(',').unwrap())
    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
    .map(|position| {
      folds.iter()
        .fold(position, |current_pos, fold| fold.fold(current_pos))
    } )
    .for_each(|(x, y)| {
      image[y][x] = '#';
    });

  for row in image {
    for c in row {
      print!("{}", c);
    }
    println!();
  }
  0
}
