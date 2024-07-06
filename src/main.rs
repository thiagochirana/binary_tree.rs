mod binary_tree;

use binary_tree::tree::BinaryTree;

fn main() {
    let mut tree = BinaryTree::new();

    tree.insert(10);
    tree.insert(5);
    tree.insert(15);
    tree.insert(3);
    tree.insert(7);
    tree.insert(12);
    tree.insert(18);

    println!("Árvore Binária: {:?}", tree);
    println!("Pesquisar 7: {}", tree.search(7));
    println!("Pesquisar 9: {}", tree.search(9));
}
