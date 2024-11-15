
use segment_tree::SegmentTree;

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
