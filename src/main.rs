mod tree;
use tree::BST;

fn main() {
    let mut tree: BST<i32> = BST::new();

    for &node in [10, 12, 5, 4, 20, 8, 7, 15, 13].iter() {
        tree.insert(node);
    }

    tree.to_graphviz("graph");
}
