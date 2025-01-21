"""
Time Complexity: O(N) – In the worst case, the for-loop iterates through all N elements, performing O(1) operations (lookup and insertion) per iteration on average.

Space Complexity: O(N) – In the worst case, the hash map stores all N elements of nums before a valid pair is found.
"""
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:

        # Create a hash map to store number-to-index mapping
        hashmap = dict()

        # Iterate through the array with enumeration to get both index and value
        for index, num in enumerate(nums):

            # Calculate the complement
            complement = target - num
            
            # Check if the complement exists in the map
            if complement in hashmap:

                # Return the indices if complement is found
                return [index, hashmap[complement]]

            # Insert the current number and its index into the map
            hashmap[num] = index
