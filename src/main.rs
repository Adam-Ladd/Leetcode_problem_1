use std::collections::HashMap;

fn main() {
    assert_eq!( solution(vec![2,7,11,15], 9), vec![0, 1]);
    assert_eq!( solution(vec![3,2,4], 6), vec![1, 2]);
    assert_eq!( solution(vec![3,3], 6), vec![0, 1]);
}

/// efficient O(n)  algorithm to check find the indices of the two numbers in `nums` which add up to `target`
fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_indices: HashMap<i32, Vec<usize>> = HashMap::with_capacity(nums.len());

    for (index, num) in (&nums).into_iter().enumerate() {
        match num_indices.get_mut(&num) {
            Some(indices) => {
                indices.push(index)
            },
            None => {
                num_indices.insert(*num, vec![index]);
            },
        };
    }

    for (index, num) in nums.into_iter().enumerate() {
        if num_indices.contains_key(&(target - num)) {
            if let Some(num_2_indices) = num_indices.get_mut(&(target - num)) {
                if target == (num * 2) {
                    if num_2_indices.len() == 1 { continue }
                    else {
                        for val in num_2_indices.iter() {
                            if *val != index { return vec![index as i32, *val as i32]}
                        }
                    }
                }
                else {
                    return vec![index as i32, num_2_indices.pop().unwrap() as i32]
                }
            }
        }
    }
    panic!("should have only one solution, solution not found")
}