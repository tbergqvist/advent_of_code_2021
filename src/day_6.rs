use std::fs::File;
use std::io::{self, BufRead};

fn yolo(data:&mut Vec<i64>, position:usize, add:i64) -> i64 {
  let prev_value = data[position];
  data[position] = add;
  if position == 0 {
    return prev_value;
  }
  yolo(data, position - 1, prev_value)
}

pub fn a() -> i64 {
  let file = io::BufReader::new(
    File::open("./inputs/6.txt").unwrap()
  );
  let line = file
    .lines()
    .next().unwrap().unwrap();
  
  let mut data = vec![0; 9];
  line.split_terminator(',')
    .map(|line|line.parse::<usize>().unwrap())
    .for_each(|num| data[num] += 1);

  for _ in 0..80 {
    let bla2 = data[0];
    yolo(&mut data, 8, bla2);
    data[6] += bla2;
  }

  data.into_iter().sum()
}

pub fn b() -> i64 {
  let file = io::BufReader::new(
    File::open("./inputs/6.txt").unwrap()
  );
  let line = file
    .lines()
    .next().unwrap().unwrap();
  
  let mut data = vec![0; 9];
  line.split_terminator(',')
    .map(|line|line.parse::<usize>().unwrap())
    .for_each(|num| data[num] += 1);

  for _ in 0..256 {
    let bla2 = data[0];
    yolo(&mut data, 8, bla2);
    data[6] += bla2;
  }

  data.into_iter().sum()
}