fn bla(values: Vec<Vec<u32>>, position: usize, invert: bool) -> Vec<u32> {
  if values.len() == 1 {
    return values[0].clone();
  }

  let sum: u32 = values.iter()
    .map(|v| v[position])
    .sum();

  let bit = if sum as f64 >= (values.len() as f64 / 2.0) {1} else {0};
  let bit = if invert {bit ^ 0x1} else {bit};

  bla(values.into_iter().filter(|v| v[position] == bit).collect(), position + 1, invert)
}

fn to_decimal(vec: Vec<u32>) -> usize {
  let binary_string = vec.iter()
  .map(|num| num.to_string().chars().next().unwrap())
  .collect::<String>();
  
  usize::from_str_radix(&binary_string, 2).unwrap()
}

pub fn a(input: &str) -> usize {
  let lines: Vec<&str> = input
    .lines()
    .collect();
    
  let count = lines.len() as u32;
  let mut res = lines.into_iter()
    .map(|s| { 
      s.chars().map(|c| c.to_digit(10).unwrap()).collect()
    })
    .reduce(|mut result, row: Vec<u32>| {
        for i in 0..result.len() {
          result[i] += row[i];
        }
        result
    }).unwrap();
  
  let limit: u32 = count / 2;
  res.iter_mut().for_each(|num| *num /= limit);

  let decimal = to_decimal(res);
  decimal * (decimal ^ 0xFFF)
}

pub fn b(input: &str) -> usize {
  let res: Vec<Vec<u32>> = input
    .lines()
    .map(|s| { 
      s.chars().map(|c| c.to_digit(10).unwrap()).collect()
    })
    .collect();

  let oxygen = bla(res.clone(), 0, false);
  let co2 = bla(res, 0, true);
  to_decimal(oxygen) * to_decimal(co2)
}

#[cfg(test)]
mod tests {
  use crate::day_3::bla;
    #[test]
    fn it_works() {
        let res = bla(vec![
          vec![0,0,1,0,0],
          vec![1,1,1,1,0],
          vec![1,0,1,1,0],
          vec![1,0,1,1,1],
          vec![1,0,1,0,1],
          vec![0,1,1,1,1],
          vec![0,0,1,1,1],
          vec![1,1,1,0,0],
          vec![1,0,0,0,0],
          vec![1,1,0,0,1],
          vec![0,0,0,1,0],
          vec![0,1,0,1,0],
        ], 0, false);
        assert_eq!(res, vec![1,0,1,1,1]);
    }
    
    #[test]
    fn it_also_works() {
      let res = bla(vec![
        vec![0,0,1,0,0],
        vec![1,1,1,1,0],
        vec![1,0,1,1,0],
        vec![1,0,1,1,1],
        vec![1,0,1,0,1],
        vec![0,1,1,1,1],
        vec![0,0,1,1,1],
        vec![1,1,1,0,0],
        vec![1,0,0,0,0],
        vec![1,1,0,0,1],
        vec![0,0,0,1,0],
        vec![0,1,0,1,0],
      ], 0, true);
      assert_eq!(res, vec![0,1,0,1,0]);
  }
}