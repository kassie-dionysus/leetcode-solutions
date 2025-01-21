"""
Time Complexity: O(N)
Space Complexity: O(N)
"""
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        // Create a hash map to store number-to-index mapping
        let mut num_map = HashMap::with_capacity(nums.len());
        
        // Iterate through the array with enumeration to get both index and value
        for (index, &num) in nums.iter().enumerate() {

            // Calculate the complement
            let complement = target - num;
            
            // Check if the complement exists in the map
            if let Some(&complement_index) = num_map.get(&complement) {

                // Return the indices if complement is found
                return vec![index as i32, complement_index as i32];
            }
            
            // Insert the current number and its index into the map
            num_map.insert(num, index);
        }

        // This should never happen given the problem constraints
        unreachable!()
    }
}
