use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut hm = HashMap::with_capacity(nums.len());
  for (i, &num) in nums.iter().enumerate() {
    match hm.get(&num) {
      Some(&j) => return vec![j as i32, i as i32],
      None => {
        hm.insert(target - num, i);
      }
    }
  }
  unreachable!()
}


fn main() {
  let vector: Vec<i32> = two_sum(vec![8, 10, 2], 12);

  println!("{:?}", vector)
}


