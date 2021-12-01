use std::fs::File;
use std::io::{self, BufRead};

pub fn a() -> i32 {
  let file = File::open("./inputs/1.txt").unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|s| s.unwrap())
        .map(|s| s.parse::<i32>().unwrap())
        .fold((0, None), |(count, prev_value), value| {
          (
            if prev_value.is_some() && value > prev_value.unwrap() {
              count + 1
            } else {
              count
            }, 
            Some(value)
          )
        }).0
}

pub fn b() -> i32 {
  let file = File::open("./inputs/1.txt").unwrap();
  let rows: Vec<i32> = io::BufReader::new(file)
        .lines()
        .map(|s| s.unwrap())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

  rows.windows(3)
    .map(|slice| slice.iter().sum())
    .fold((0, None), |(count, prev_value), value: i32| {
      (
        if prev_value.is_some() && value > prev_value.unwrap() {
          count + 1
        } else {
          count
        }, 
        Some(value)
      )
    }).0
}