fn yolo(data:&mut Vec<i64>, position:usize, add:i64) -> i64 {
  let prev_value = data[position];
  data[position] = add;
  if position == 0 {
    return prev_value;
  }
  yolo(data, position - 1, prev_value)
}

pub fn a(input: &str) -> i64 {
  let line = input
    .lines()
    .next().unwrap();
  
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

pub fn b(input: &str) -> i64 {
  let line = input
    .lines()
    .next().unwrap();
  
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