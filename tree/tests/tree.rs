#[cfg(test)]
mod tests {
    use tree::tree::TreeNode;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn std_insertion() {
        let root = TreeNode::new_root(5);
        println!("{:?}", root.look_for(5));
        assert!(
            format!("{:?}", Some([Some(5)])),
            format!("{:?}", root.look_for(5))
        );
    }
}
