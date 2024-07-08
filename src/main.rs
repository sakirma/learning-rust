use std::collections::HashMap;

pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
  let mut drinked = num_bottles;
  let mut remaining = num_bottles;

  while remaining >= num_exchange {
    let exchange = remaining / num_exchange;
    drinked += exchange;
    remaining = exchange + (remaining % num_exchange);
  }


  return drinked
}

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
  // let vector: Vec<i32> = two_sum(vec![8, 10, 2], 12);

  let water_bottles = num_water_bottles(9,3);

  println!("{:?}", water_bottles)
}


