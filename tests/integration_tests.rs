use binary_tree::binary_tree::tree::BinaryTree;

#[test]
fn test_insert_and_search() {
    let mut tree = BinaryTree::new();

    tree.insert(10);
    tree.insert(5);
    tree.insert(15);

    assert!(tree.search(10));
    assert!(tree.search(5));
    assert!(tree.search(15));
    assert!(!tree.search(20));
}
