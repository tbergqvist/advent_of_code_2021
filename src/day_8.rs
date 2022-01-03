pub fn a(input: &str) -> i64 {
  input
    .lines()
    .map(|row| {
      row[61..]
        .split_whitespace()
        .filter(|num|num.len() == 2 || num.len() == 3 || num.len() == 4 || num.len() == 7)
        .count() as i64
    })
    .sum()
}

pub fn b(input: &str) -> i64 {
  input
  .lines()
  .map(|row| {
    let times: Vec<&str> = row
      .split_whitespace()
      .take(10)
      .collect();

    let positions = times.iter()
      .flat_map(|s|s.as_bytes())
      .fold(vec![0;7], |mut res, char| {
        res[(*char - b'a') as usize] += 1;
        res
      });
    
    let mut a = 'a';
    let mut b = 'a';
    let mut c = 'a';
    let mut d = 'a';
    let mut e = 'a';
    let mut f = 'a';
    let mut g = 'a';

    let num_one = times.iter().find(|s|s.len() == 2).unwrap();
    let num_four = times.iter().find(|s|s.len() == 4).unwrap();

    for (i, count) in positions.iter().enumerate() {
      if *count == 6 {
        b = (b'a' + i as u8) as char;
      }
      if *count == 8 {
        let letter = (b'a' + i as u8) as char;
        if num_one.contains(letter as char) {
          c = letter;
        } else {
          a = letter;
        }
      }
      if *count == 4 {
        e = (b'a' + i as u8) as char;
      }
      if *count == 7 {
        let letter = (b'a' + i as u8) as char;
        if num_four.contains(letter as char) {
          d = letter;
        } else {
          g = letter;
        }
      }
      if *count == 9 {
        f = (b'a' + i as u8) as char;
      }
    }

    let coolio: Vec<Vec<char>> = vec![
      vec![a, b, c, e, f, g],
      vec![c, f],
      vec![a, c, d, e, g],
      vec![a, c, d, f, g],
      vec![b, c, d, f],
      vec![a, b, d, f, g],
      vec![a, b, d, e, f, g],
      vec![a, c, f],
      vec![a, b, c, d, e, f, g],
      vec![a, b, c, d, f, g],
      vec![a, b, c, e, f, g],
    ];

    let clock_str: String = row[61..]
      .split_whitespace()
      .map(|str| {
        coolio.iter()
          .enumerate()
          .find(|(_, v)| 
            v.len() == str.len() && str.chars().all(|c|v.contains(&c))
          )
          .map(|(i, _)|(b'0' + i as u8) as char)
          .unwrap()
      })
      .collect();

      clock_str.parse::<i64>().unwrap()
  })
  .sum()
}