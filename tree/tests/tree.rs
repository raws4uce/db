#[cfg(test)]
mod tests {
    use std::vec;

    use anyhow::{Ok, Result};
    use tree::tree::TreeNode;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn root_init() -> Result<()> {
        let root = TreeNode::new_root(5);
        assert_eq!(root.data[0], 5);
        Ok(())
    }
    #[test]
    fn node_init() -> Result<()> {
        let mut root = TreeNode::new_root(2);
        let n0 = root.node_innit(vec![0, 1, 2, 3]);
        let n1 = root.node_innit(vec![5, 6, 7, 8]);
        let n2 = root.node_innit(vec![10, 11, 12, 13]);
        let n3 = root.node_innit(vec![15, 16, 17, 18]);
        let n4 = root.node_innit(vec![20, 21, 22, 23]);
        //root characteristics
        let d = Some(vec![4, 9, 14, 19]);
        root.data = d.expect("root node cant hold Option<Vec<4* Option<usize>>>");
        root.children = Some(vec![
            Box::new(n0),
            Box::new(n1),
            Box::new(n2),
            Box::new(n3),
            Box::new(n4),
        ]);
        Ok(())
    }
    #[test]
    fn searchin() -> Result<()> {
        let mut root = TreeNode::new_root(2);
        let n0 = root.node_innit(vec![0, 1, 2, 3]);
        let n1 = root.node_innit(vec![5, 6, 7, 8]);
        let n2 = root.node_innit(vec![10, 11, 12, 13]);
        let n3 = root.node_innit(vec![15, 16, 17, 18]);
        let n4 = root.node_innit(vec![20, 21, 22, 23]);
        //root characteristics
        let d = Some(vec![4, 9, 14, 19]);
        root.data = d.expect("root node cant hold Option<Vec<4* Option<usize>>>");
        root.children = Some(vec![
            Box::new(n0),
            Box::new(n1),
            Box::new(n2),
            Box::new(n3),
            Box::new(n4),
        ]);
        assert_eq!(root.look_for(19), Some(vec![4, 9, 14, 19]));
        assert_eq!(root.look_for(17), Some(vec![15, 16, 17, 18]));
        assert_eq!(root.look_for(3434), None);
        Ok(())
    }
    #[test]
    fn insert_overflow_root() -> Result<()> {
        let mut root = TreeNode::new_root(2);
        let n0 = root.node_innit(vec![0, 1, 2, 3]);
        let n1 = root.node_innit(vec![5, 6, 7, 8]);
        let n2 = root.node_innit(vec![10, 11, 12, 13]);
        let n3 = root.node_innit(vec![15, 16, 17, 18]);
        let n4 = root.node_innit(vec![20, 21, 22, 23]);
        //root characteristics
        let d = Some(vec![4, 9, 14, 19]);
        root.data = d.expect("root node cant hold Option<Vec<4* Option<usize>>>");
        root.children = Some(vec![
            Box::new(n0),
            Box::new(n1),
            Box::new(n2),
            Box::new(n3),
            Box::new(n4),
        ]);
        root.stick_in(24)?;
        assert_eq!(vec![14], root.data);
        Ok(())
    }
}
