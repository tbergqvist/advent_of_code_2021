use std::fs;

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

fn main() {
  println!("Solving");
  let day1_input = fs::read_to_string("./inputs/1.txt").unwrap();
  println!("1a:{}", day_1::a(&day1_input));
  println!("1b:{}", day_1::b(&day1_input));
  
  let day2_input = fs::read_to_string("./inputs/2.txt").unwrap();
  println!("2a:{}", day_2::a(&day2_input));
  println!("2b:{}", day_2::b(&day2_input));
  
  let day3_input = fs::read_to_string("./inputs/3.txt").unwrap();
  println!("3a:{}", day_3::a(&day3_input));
  println!("3b:{}", day_3::b(&day3_input));
  
  let day4_input = fs::read_to_string("./inputs/4.txt").unwrap();
  println!("4a:{}", day_4::a(&day4_input));
  println!("4b:{}", day_4::b(&day4_input));
  
  let day5_input = fs::read_to_string("./inputs/5.txt").unwrap();
  println!("5a:{}", day_5::a(&day5_input));
  println!("5b:{}", day_5::b(&day5_input));
  
  let day6_input = fs::read_to_string("./inputs/6.txt").unwrap();
  println!("6a:{}", day_6::a(&day6_input));
  println!("6b:{}", day_6::b(&day6_input));
  
  let day7_input = fs::read_to_string("./inputs/7.txt").unwrap();
  println!("7a:{}", day_7::a(&day7_input));
  println!("7b:{}", day_7::b(&day7_input));
  
  let day8_input = fs::read_to_string("./inputs/8.txt").unwrap();
  println!("8a:{}", day_8::a(&day8_input));
  println!("8b:{}", day_8::b(&day8_input));

  let day9_input = fs::read_to_string("./inputs/9.txt").unwrap();
  println!("9a:{}", day_9::a(&day9_input));
  println!("9b:{}", day_9::b(&day9_input));

  let day10_input = fs::read_to_string("./inputs/10.txt").unwrap();
  println!("10a:{}", day_10::a(&day10_input));
  println!("10b:{}", day_10::b(&day10_input));

  let day11_input = fs::read_to_string("./inputs/11.txt").unwrap();
  println!("11a:{}", day_11::a(&day11_input));
  println!("11b:{}", day_11::b(&day11_input));
  println!("Done");
}
