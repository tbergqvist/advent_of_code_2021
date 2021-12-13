use rayon::prelude::*;
use std::collections::HashMap;

fn loop_di_loop<'a>(caves: &'a HashMap<&str, Vec<&str>>, cave_name: &'a str, visited_small_caves: &Vec<&'a str>) -> usize {
  let small_cave = cave_name.as_bytes()[0].is_ascii_lowercase();
  if small_cave && visited_small_caves.contains(&cave_name) {
    return 0;
  }
  let neighbors = &caves[cave_name];
  if cave_name == "end" {
    return 1;
  }

  if small_cave {
    let mut visited_small_caves = visited_small_caves.clone();
    visited_small_caves.push(cave_name);
  
    return neighbors.par_iter().map(|n| loop_di_loop(caves, n, &visited_small_caves)).sum();  
  } else {
    return neighbors.par_iter().map(|n| loop_di_loop(caves, n, visited_small_caves)).sum();  
  }
}

pub fn a(input: &str) -> usize {
  let mut caves: HashMap<&str, Vec<&str>> = HashMap::new();
  input.lines()
    .for_each(|line| {
      let bla = line.split('-').collect::<Vec<&str>>();
      let parent = bla[0];
      let child = bla[1];
      {
        let woho = caves.entry(parent).or_default();
        woho.push(child);
      }
      let woho2 = caves.entry(child).or_default();
      woho2.push(parent);
    }
  );

  loop_di_loop(&caves, "start", &Vec::new())
}

fn loop_di_loop2<'a>(caves: &'a HashMap<&str, Vec<&str>>, cave_name: &'a str, visited_small_caves: &Vec<&'a str>, mut small_cave_visited_twice: bool) -> usize {
  let small_cave = cave_name.as_bytes()[0].is_ascii_lowercase();
  if small_cave {
    let visit_count = visited_small_caves.iter().filter(|s| **s == cave_name).count();
    if visit_count >= 2 || ((cave_name == "start" || small_cave_visited_twice) && visit_count >= 1) {
      return 0;
    }
    small_cave_visited_twice = small_cave_visited_twice || visit_count >= 1;
  }

  let neighbors = &caves[cave_name];
  if cave_name == "end" {
    return 1;
  }

  if small_cave {
    let mut visited_small_caves = visited_small_caves.clone();
    visited_small_caves.push(cave_name);
  
    return neighbors.par_iter().map(|n| loop_di_loop2(caves, n, &visited_small_caves, small_cave_visited_twice)).sum();  
  } else {
    return neighbors.par_iter().map(|n| loop_di_loop2(caves, n, visited_small_caves, small_cave_visited_twice)).sum();  
  }
}

/*
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
 */

pub fn b(input: &str) -> usize {
  let mut caves: HashMap<&str, Vec<&str>> = HashMap::new();
  input.lines()
    .for_each(|line| {
      let bla = line.split_terminator('-').collect::<Vec<&str>>();
      let parent = bla[0];
      let child = bla[1];
      {
        let woho = caves.entry(parent).or_default();
        woho.push(child);
      }
      let woho2 = caves.entry(child).or_default();
      woho2.push(parent);
    }
  );

  loop_di_loop2(&caves, "start", &Vec::new(), false)
}
