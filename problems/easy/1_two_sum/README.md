# [1. Two Sum](https://leetcode.com/problems/two-sum/)

## Description

Given an array of integers `nums` and an integer `target`, return indices of the two numbers in the array such that they add to `target`.
You may assume that each input would have exactly one solution, and you may not use the same element twice. 
You can return the answer in any order.

## Approaches

### Naive Solution: Brute-Force

The naïve solution is a brute-force approach where every pair of numbers in the array are added together to check if they add to `target`. Since a nested loop is required to evaluate each pair, the time complexity of this solution is `O(n²)`.

### Optimal Solution: Hash Map

The optimal solution uses a hash map to store previously seen numbers together with their indices and checks whether each number's complement, `target` minus the number, already exists in the hash map. If it does exist, the indices of the number and its complement can be returned in constant time so the time complexity of this solution is `O(n)`. 

### Key Insight

The key insight is that finding two numbers that sum to `target` reduces to checking whether `target` minus the current number has already been encountered. Consequently, rather than looking forward, only the numbers seen so far need to be considered.

## Algorithm

1. **Initialize** an empty hash map to store mappings of numbers to their indices.
2. **Iterate through `nums`:**
    - Calculate its complement by subtracting the current number from the target.
    - If the current number's complement exists in the hash map, return the indices of the current number and its complement.
    - Otherwise, add a mapping from the current number to its index to the hash map.

## Analysis

Let `n` be the length of array `nums`.

### Time Complexity: `O(n)`

- A single pass is made through the `nums` array.
- Hash map operations (lookup and insert) are O(1) on average.

### Space Complexity: `O(n)`

- The hash map will only hold at most n-1 mappings of numbers to their indices.
- No additional space is needed beyond the hash map.

