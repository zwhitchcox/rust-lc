use std::collections::HashMap;
/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
       let mut hashmap = HashMap::with_capacity(nums.len());

       for (idx, &n) in nums.iter().enumerate() {
           let y = target - n;
           if let Some(&i) = hashmap.get(&y) {
               return vec![idx as i32, i as i32]
           } else {
               hashmap.insert(n, idx);
           }
       }
       vec![]
    }
}

