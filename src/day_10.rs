pub fn a(input: &str) -> usize {
  input.lines().filter_map(|line| {
    let mut stack: Vec<char> = Vec::new();
    
    for c in line.chars() {
      if ['>', ')', ']', '}'].contains(&c) {
        let start_tag = stack.pop().unwrap_or(' ');
        if match c {
          '>' => '<' != start_tag,
          ')' => '(' != start_tag,
          ']' => '[' != start_tag,
          '}' => '{' != start_tag,
          _ => panic!("waaa")
        } {
          return Some(c);
        }
      } else {
        stack.push(c);
      }
    }
    None
  }).map(|sign|
    match sign {
      ')' => 3,
      ']' => 57,
      '}' => 1197,
      '>' => 25137,
      _ => 1000
    }
  ).sum() 
}

pub fn b(input: &str) -> usize {
  let mut scores: Vec<usize> = input.lines().filter_map(|line| {
    let mut stack: Vec<char> = Vec::new();
    
    for c in line.chars() {
      if ['>', ')', ']', '}'].contains(&c) {
        let start_tag = stack.pop().unwrap_or(' ');
        if match c {
          '>' => '<' != start_tag,
          ')' => '(' != start_tag,
          ']' => '[' != start_tag,
          '}' => '{' != start_tag,
          _ => panic!("waaa")
        } {
          return None;
        }
      } else {
        stack.push(c);
      }
    }
    Some(stack)
  })
  .map(|stack| {
    stack.into_iter().rev().map(|c|{
      match c {
        '<' => '>',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => panic!("wwaaa")
      }
    }).collect::<Vec<char>>()
  })
  .map(|rest|
    rest.into_iter().fold(0, |sum, c|{
      5 * sum + match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0
      }
    })
  ).collect();

  scores.sort_unstable();
  scores[scores.len() / 2]

}
