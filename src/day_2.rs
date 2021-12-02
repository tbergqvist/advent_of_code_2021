use std::fs::File;
use std::io::{self, BufRead};

pub fn a() -> i32 {
  let file = File::open("./inputs/2.txt").unwrap();
    let (x, y) = io::BufReader::new(file)
        .lines()
        .map(|s| s.unwrap())
        .map(|s| { 
          let split: Vec<&str> = s.split_whitespace().collect();
          (split[0].to_string(), split[1].parse::<i32>().unwrap())
        })
        .fold((0, 0), |(x, y), (direction, amount)| {
            match direction.as_str() {
              "forward" => (x + amount, y),
              "down" => (x, y + amount),
              "up" => (x, y - amount),
              _ => panic!("waa")
            }
        });
        x * y
}

pub fn b() -> i32 {
  let file = File::open("./inputs/2.txt").unwrap();
  let (x, y, _) = io::BufReader::new(file)
    .lines()
    .map(|s| s.unwrap())
    .map(|s| { 
      let split: Vec<&str> = s.split_whitespace().collect();
      (split[0].to_string(), split[1].parse::<i32>().unwrap())
    })
    .fold((0, 0, 0), |(x, y, aim), (direction, amount)| {
        match direction.as_str() {
          "forward" => (x + amount, y + amount * aim, aim),
          "down" => (x, y, aim + amount),
          "up" => (x, y, aim - amount),
          _ => panic!("waa")
        }
    });
  x * y
}