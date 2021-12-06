pub fn a(input: &str) -> i32 {
  let (x, y) = input
    .lines()
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

pub fn b(input: &str) -> i32 {
  let (x, y, _) = input
    .lines()
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