fn read_bits(bytes: &[u8]) -> u32 {
  bytes.iter().rev().enumerate().fold(0, |sum, (i, bit)| {
    sum + ((*bit as u32) << i)
  })
}

fn read_package(bytes: &[u8]) -> (u32, usize) {
  println!("reading package size: {}", bytes.len());

  if bytes.len() <= 3 {
    return (0, 0);
  }
  let version = read_bits(&bytes[0..3]);
  let type_id = read_bits(&bytes[3..6]);
  if type_id == 4 {
    let mut current_pos = 0;
    while (read_bits(&bytes[(6 + current_pos)..(11 + current_pos)]) & 16) > 0 {
      current_pos += 5;
    }
    return (version, 11 + current_pos + current_pos % 4);
  } else {
    let length_type_id = read_bits(&bytes[6..7]);
    if length_type_id == 0 {
      let length = read_bits(&bytes[7..22]) as usize;
      let mut total_version = 0;
      let mut current_pos: usize = 0;
      while dbg!(length - current_pos) > 0 {
        let (version, pos) = read_package(&bytes[(22 + current_pos)..]);
        total_version += version;
        current_pos += pos;
      }
      
      return (version + total_version, 22 + length);
    } else {
      let no_of_sub_packages = read_bits(&bytes[7..18]) as usize;
      return (0..no_of_sub_packages)
        .into_iter()
        .fold((version, 18), |(version, pos), _| {
          let (v1, pos1) = read_package(&bytes[pos..]);
          (version + v1, pos + pos1)
        });
    }
  }
}

pub fn a(input: &str) -> u32 {
  let bytes: Vec<u8> = input
    .lines()
    .flat_map(|line| line.chars())
    .flat_map(|c| {
      let digit = c.to_digit(16).unwrap() as u8;
      vec![(digit & 8) >> 3, (digit & 4) >> 2, (digit & 2) >> 1, digit & 1].into_iter()
    })
    .collect();
  
  read_package(&bytes).0
}

pub fn b(input: &str) -> i32 {
  0
}
