use my_data_structure::*;

fn main() {
    let mut bst = bst::BST::new(10);
    bst.insert(9);
    bst.delete(9);
    println!("{:?}", bst.inorder());
}
