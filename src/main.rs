
const MAX_VALUE: isize = isize::MAX / 2;
const MIN_VALUE: isize = isize::MIN / 2;
const MAX_INPUT_SIZE: usize = usize::MAX / 2 - 1;

fn main() -> Result<(), &'static str> {
    let input: Vec<isize> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut segmemt_tree = SegmentTree::new(&input)?;
    // Input values are now the leaves in the segment tree
    drop(input);

    // Original input values / 36
    let result = segmemt_tree.query(0, 7)?;
    println!("Query result: {}", result);

    // Original input values / 14
    let result = segmemt_tree.query(1, 4)?;
    println!("Query result: {}", result);

    // [1, 2, 3, 10, 5, 6, 7, 8] / 20
    // Change idx 3 to 10
    segmemt_tree.update(3, 10)?;
    let result = segmemt_tree.query(1, 4)?;
    println!("Query result: {}", result);

    // [2, 2, 3, 10, 5, 6, 7, 8] / 43
    // Change idx 0 to 2
    segmemt_tree.update(0, 2)?;
    let result = segmemt_tree.query(0, 7)?;

    println!("Query result: {}", result);

    Ok(())
}

#[derive(Debug, Clone)]
pub struct Node {
    pub value: isize,
    pub start: usize, // Start index of the range, in leaves
    pub end: usize, // end index of the range, in leaves
    pub left: Option<usize>, // Index of left child
    pub right: Option<usize>, // Index of right child
}

struct SegmentTree {
    nodes: Vec<Node>,
    leaf_len: usize,
    tree_len: usize,
    leaf_indices: Vec<usize>,
}

impl SegmentTree {
    pub fn new(input: &Vec<isize>) -> Result<SegmentTree, &'static str> {
        SegmentTree::validate_input(&input)?;
        let leaf_len = input.len();
        let tree_len = SegmentTree::get_segment_tree_size(leaf_len);
        let mut nodes = SegmentTree::reserve_nodes(tree_len);
        let mut leaf_indices = vec![0; leaf_len];
        SegmentTree::build_nodes(&mut nodes, &mut leaf_indices, 0, 0, leaf_len - 1, &input);

        Ok(SegmentTree {
            nodes,
            leaf_len,
            tree_len,
            leaf_indices,
        })
    }

    fn validate_input(input: &Vec<isize>) -> Result<(), &'static str> {
        if input.len() == 0 {
            return Err("Input is empty");
        }

        if input.len() > MAX_INPUT_SIZE {
            return Err("Input size exceeded maximum value");
        }

        if input.len() % 2 != 0 {
            return Err("Input length is not an even number");
        }

        for i in 0..input.len() {
            if input[i] < MIN_VALUE {
                return Err("Input value exceeded maximum value");
            }
            if input[i] > MAX_VALUE {
                return Err("Input value exceeded minimum value");
            }
        }

        Ok(())
    }

    fn get_segment_tree_size(vec_len: usize) -> usize {
        if vec_len == 0 {
            return 0;
        }

        let height = (vec_len as f64).log2().ceil() as u32;
        let next_pow2 = 1 << height;

        2 * next_pow2 - 1
    }

    fn reserve_nodes(tree_size: usize) -> Vec<Node> {
        vec![
            Node {
                value: 0,
                start: 0,
                end: 0,
                left: None,
                right: None,
            }
            ; tree_size
        ]
    }

    fn build_nodes(nodes: &mut Vec<Node>, leaf_indices: &mut Vec<usize>, node: usize, start: usize, end: usize, input: &[isize]) -> isize {
        if start == end {
            // Leaf node
            nodes[node].value = input[start];
            nodes[node].start = start;
            nodes[node].end = end;
            leaf_indices[start] = node;
            return input[start];
        }

        let mid = (start + end) / 2;
        let left = 2 * node + 1;
        let right = 2 * node + 2;

        nodes[node].left = Some(left);
        nodes[node].right = Some(right);
        nodes[node].start = start;
        nodes[node].end = end;

        let left_sum = SegmentTree::build_nodes(nodes, leaf_indices, left, start, mid, input);
        let right_sum = SegmentTree::build_nodes(nodes, leaf_indices, right, mid + 1, end, input);

        nodes[node].value = left_sum + right_sum;
        nodes[node].value
    }

    fn validate_public_query(&self, start: usize, end: usize) -> Result<(), &'static str> {
        if self.leaf_len == 0 {
            return Err("Segment tree is empty")
        }

        if start > end {
            return Err("Start index is greater than end index")
        }

        if start < 0 || start >= self.leaf_len - 1 {
            return Err("Start index is out of bounds")
        }

        if end > self.leaf_len - 1 {
            return Err("End index is out of bounds")
        }

        Ok(())
    }

    pub fn query(&self, start: usize, end: usize) -> Result<isize, &'static str> {
        self.validate_public_query(start, end)?;
        Ok(self.internal_query_recursive(0, start, end))
    }

    fn internal_query_recursive(&self, node_idx: usize, start: usize, end: usize) -> isize {
        if start <= self.nodes[node_idx].start && end >= self.nodes[node_idx].end {
            return self.nodes[node_idx].value;
        }

        if end < self.nodes[node_idx].start || start > self.nodes[node_idx].end {
            return 0;
        }

        let left_sum = self.internal_query_recursive(self.nodes[node_idx].left.unwrap(), start, end);
        let right_sum = self.internal_query_recursive(self.nodes[node_idx].right.unwrap(), start, end);

        left_sum + right_sum
    }

    fn validate_public_update(&self, index: usize, new_value: isize) -> Result<(), &'static str> {
        if self.leaf_len == 0 {
            return Err("Segment tree is empty");
        }

        if index >= self.leaf_len {
            return Err("Update index is out of bounds");
        }

        if new_value > MAX_VALUE || new_value < MIN_VALUE {
            return Err("New value is out of valid range");
        }

        Ok(())
    }

    pub fn update(&mut self, index: usize, new_value: isize) -> Result<(), &'static str> {
        self.validate_public_update(index, new_value)?;

        let leaf_node = self.leaf_indices[index];
        self.nodes[leaf_node].value = new_value;

        self.update_ancestors(leaf_node);
        Ok(())
    }

    fn update_ancestors(&mut self, mut node_idx: usize) {
        while node_idx > 0 {
            let parent = (node_idx - 1) / 2;
            let left_child = self.nodes[parent].left.unwrap();
            let right_child = self.nodes[parent].right.unwrap();

            self.nodes[parent].value = self.nodes[left_child].value + self.nodes[right_child].value;

            node_idx = parent;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_segment_tree() {
        let input = vec![1, 2, 3, 4];
        let result = SegmentTree::new(&input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_input() {
        let input = vec![1, 2, 3]; // Not power of 2
        let result = SegmentTree::new(&input);
        assert!(result.is_err());
    }
}
