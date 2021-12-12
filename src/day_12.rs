use std::collections::HashMap;

fn loop_di_loop(caves: &HashMap<&str, Vec<String>>, cave_name: &str, current_path: String) -> Vec<String> {
  if cave_name.to_lowercase() == cave_name && current_path.contains(cave_name) {
    return vec![];
  }
  let neighbors = &caves[cave_name];
  if neighbors.is_empty() || cave_name == "end" {
    return vec![current_path + "->" + cave_name];
  }

  return neighbors.iter().flat_map(|n| loop_di_loop(caves, n, current_path.clone() + "->" + cave_name)).collect();
}

pub fn a(input: &str) -> usize {
  let mut caves: HashMap<&str, Vec<String>> = HashMap::new();
  input.lines()
    .for_each(|line| {
      let bla = line.split_terminator('-').collect::<Vec<&str>>();
      let parent = bla[0];
      let child = bla[1];
      {
        let woho = caves.entry(parent).or_default();
        woho.push(child.to_string());
      }
      let woho2 = caves.entry(child).or_default();
      woho2.push(parent.to_string());
    }
  );

  loop_di_loop(&caves, "start", "".to_string()).len()
}


fn loop_di_loop2(caves: &HashMap<&str, Vec<String>>, cave_name: &str, current_path: String, mut small_cave_visited_twice: bool) -> Vec<String> {
  if cave_name.to_lowercase() == cave_name {
    let visit_count = current_path.split_terminator("->").filter(|s| *s == cave_name).count();
    if visit_count >= 2 || ((cave_name == "start" || small_cave_visited_twice) && visit_count >= 1) {
      return vec![];
    }
    small_cave_visited_twice = small_cave_visited_twice || visit_count >= 1;
  }


  let neighbors = &caves[cave_name];
  if neighbors.is_empty() || cave_name == "end" {
    return vec![current_path + "->" + cave_name];
  }

  return neighbors.iter().flat_map(|n| loop_di_loop2(caves, n, current_path.clone() + "->" + cave_name, small_cave_visited_twice)).collect();
}


pub fn b(input: &str) -> usize {
  let mut caves: HashMap<&str, Vec<String>> = HashMap::new();
  input.lines()
    .for_each(|line| {
      let bla = line.split_terminator('-').collect::<Vec<&str>>();
      let parent = bla[0];
      let child = bla[1];
      {
        let woho = caves.entry(parent).or_default();
        woho.push(child.to_string());
      }
      let woho2 = caves.entry(child).or_default();
      woho2.push(parent.to_string());
    }
  );

  loop_di_loop2(&caves, "start", "".to_string(), false).len()
}
