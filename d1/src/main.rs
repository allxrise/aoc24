use std::{fs::File, io::read_to_string};

fn main() {
  let input = read_to_string(File::open("input.txt").unwrap()).unwrap();

  let mut left_list = Vec::with_capacity(1000);
  let mut right_list = Vec::with_capacity(1000);

  input.lines().for_each(|line| {
    let mut split = line.split("   ");

    let l: i32 = split.next().unwrap().parse().unwrap();
    left_list.push(l);

    let r: i32 = split.next().unwrap().parse().unwrap();
    right_list.push(r);
  });

  part_one(&left_list, &right_list);

  part_two(&left_list, &right_list);
}

fn part_one(left_list: &Vec<i32>, right_list: &Vec<i32>) {
  let mut total_distance = 0;

  let mut left_list = left_list.clone();
  let mut right_list = right_list.clone();

  left_list.sort();
  right_list.sort();

  (0..1000).for_each(|i| {
    let distance = (left_list[i] - right_list[i]).abs();
    total_distance += distance;
  });

  println!("Total Distance: {total_distance}");
}

fn part_two(left_list: &Vec<i32>, right_list: &Vec<i32>) {
  let mut total_similarity = 0;

  left_list.iter().for_each(|l| {
    let mut times = 0;

    right_list.iter().for_each(|r| {
      if *l == *r {
        times += 1;
      }
    });

    total_similarity += l * times;
  });

  println!("Total Similarity: {total_similarity}");
}
