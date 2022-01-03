use std::{fs, env};

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;

type SolutionFunc = fn(&str) -> i64;

fn main() {
  let args: Vec<String> = env::args().collect();
  let iterations: i32 = args.get(1)
    .and_then(|i|i.parse().ok())
    .unwrap_or(1);
  let mut days_to_run: Vec<usize> = args.iter()
    .skip(2)
    .filter_map(|day| day.parse().ok())
    .collect();

  let solutions: Vec<(SolutionFunc, SolutionFunc)> = vec![
    (day_1::a, day_1::b),
    (day_2::a, day_2::b),
    (day_3::a, day_3::b),
    (day_4::a, day_4::b),
    (day_5::a, day_5::b),
    (day_6::a, day_6::b),
    (day_7::a, day_7::b),
    (day_8::a, day_8::b),
    (day_9::a, day_9::b),
    (day_10::a, day_10::b),
    (day_11::a, day_11::b),
    (day_12::a, day_12::b),
    (day_13::a, day_13::b),
    (day_14::a, day_14::b),
    (day_15::a, day_15::b),
  ];

  if days_to_run.is_empty() {
    (1..solutions.len() + 1).for_each(|i| days_to_run.push(i));
  }

  for _ in 0..iterations {
    println!("Solving");
    days_to_run.iter()
      .map(|day|(day, solutions[day - 1]))
      .for_each(|(day, (p1, p2))| {
        let day_input = fs::read_to_string(format!("./inputs/{}.txt", day)).unwrap();
        println!("{}a:{}", day, p1(&day_input));
        println!("{}b:{}", day, p2(&day_input));
    });
    println!("Done");
  }
}
