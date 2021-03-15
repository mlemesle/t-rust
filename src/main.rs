use t_rust::tree::binary_tree::*;

fn main() {
    let a = BinaryTree::new(4);
    println!("{}", a.get_value());

    let b = BinaryTree::new("TOTO");
    println!("{}", b.get_value());

    let c = BinaryTree::new(3.14);
    println!("{}", c.get_value());
}
