# Segment Tree
Implements a basic sum segment tree that is safe on any system bit-width. ie 16, 32, 64

## State
Overall, the code is organized and safe.

This code would need some cleanup for any production use
- Code documentation and comments
- Results as `Result<T, CustomError>` rather than `Result<T, &'static str>`
- `Sync` and `Send` implementation for use with async

# Segment Tree

A segment tree is a data structure designed to efficiently perform range queries and updates
on an array of values. It is particularly useful when you need to perform multiple range
operations on a static array.

## Properties
- Height: O(log n) where n is the number of elements
- Space complexity: O(n)
- Query time complexity: O(log n)
- Update time complexity: O(log n)

## Common Use Cases
- Range sum queries
- Range minimum/maximum queries
- Range GCD queries
- Finding the smallest number greater than a given value in a given range

## Structure
The segment tree is structured as a binary tree where:
- Each leaf node represents a single element from the input array
- Each internal node represents a merger of its children (in this implementation, the sum)
- The root node represents the entire range

## Example
For an array [1, 2, 3, 4], the segment tree might look like:
```text
                10 (sum of 0-3)
               /              \
          3 (sum of 0-1)    7 (sum of 2-3)
          /        \        /        \
         1         2      3          4
```

## Operations
1. Query: Find the sum of elements from index i to j
2. Update: Modify the value at a given index and update affected ranges

## Implementation Details
This implementation uses:
- A vector to store the tree nodes
- Each node contains the sum of its range and range boundaries
- Leaf indices are stored separately for efficient updates

Note: This implementation specifically handles range sum queries, but the concept
can be extended to other range operations (min, max, GCD, etc.) by modifying
the merge operation between nodes.
