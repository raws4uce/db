use anyhow::{anyhow, Result};
use dump::util;
pub struct TreeNode {
    id: usize,
    data: Vec<Option<usize>>,             // Data stored in this node
    children: Option<Vec<Box<TreeNode>>>, // Children nodes
    parent: Option<*mut TreeNode>,        // Raw pointer to avoid lifetime conflicts
}

impl TreeNode {
    pub fn new_root(data: usize) -> TreeNode {
        TreeNode {
            id: util::id_init(),
            data: vec![Some(data)],
            children: None,
            parent: None,
        }
    }
    pub fn node_innit(&mut self, data: usize) -> TreeNode {
        TreeNode {
            id: util::id_init(),
            data: vec![Some(data)],       // Directly initialise with the data
            children: None,               // No children for a new node
            parent: Some(self as *mut _), // Use raw pointer to self for parent
        }
    }

    pub fn look_for(&self, data: usize) -> Option<Vec<Option<usize>>> {
        if let Ok(_) = self.data.binary_search(&Some(data)) {
            return Some(self.data.clone());
        }
        let index = self
            .data
            .iter()
            .enumerate()
            .find(|(_, dat)| dat.as_ref().map_or(false, |d| &data < d))
            .map(|(i, _)| i)
            .unwrap_or_else(|| self.data.len());
        if let Some(children) = self.children.as_ref() {
            if let Some(child) = children.get(index) {
                child.look_for(data);
            }
        }
        None
    }

    pub fn stick_in(&mut self, data: usize) -> Result<()> {
        let index = self
            .data
            .iter()
            .enumerate()
            .find(|(_, dat)| dat.as_ref().map_or(false, |d| &data < d))
            .map(|(i, _)| i)
            .unwrap_or_else(|| self.data.len());

        let new_child = self.node_innit(data); //satisfies compiler...

        if let Some(children) = self.children.as_mut() {
            if let Some(child) = children.get_mut(index) {
                child.stick_in(data)?;
            } else {
                children.insert(index, Box::new(new_child));
            }
        } else {
            if self.data.len() == 4 {
                self.data.push(Some(data));
                self.split()?;
            } else {
                self.data.push(Some(data));
                self.data.sort_by(|a, b| a.cmp(b));
            }
        }
        Ok(())
    }
    pub fn delete(&mut self, data: usize) -> Result<()> {
        if let Ok(i) = self.data.binary_search(&Some(data)) {
            self.data.remove(i);
        } else {
            let index = self
                .data
                .iter()
                .enumerate()
                .find(|(_, dat)| dat.as_ref().map_or(false, |d| &data < d))
                .map(|(i, _)| i)
                .unwrap_or_else(|| self.data.len());
            if let Some(children) = self.children.as_mut() {
                if let Some(child) = children.get_mut(index) {
                    child.delete(data)?;
                }
            }
        }
        Err(anyhow!("THIS IS AN ERROR"))
    }
    pub fn update() {}
    pub fn traverse() {}
    pub fn split(&mut self) -> Result<()> {
        let mid_index = self.data.len() / 2;
        let middle = self.data[mid_index].take().unwrap();

        let left_data: Vec<Option<usize>> = self.data.drain(..mid_index).collect();
        let right_data: Vec<Option<usize>> = self.data.drain(mid_index..).collect();

        let left_child: Option<Vec<Box<TreeNode>>> = self
            .children
            .as_mut()
            .map(|c| c.drain(..mid_index + 1).collect());
        let right_child: Option<Vec<Box<TreeNode>>> = self
            .children
            .as_mut()
            .map(|c| c.drain(mid_index + 1..).collect());
        if self.parent.is_none() {
            let left_node = Box::new(TreeNode {
                id: util::id_init(),
                data: left_data,
                children: left_child,
                parent: None,
            });
            let right_node = Box::new(TreeNode {
                id: util::id_init(),

                data: right_data,
                children: right_child,
                parent: None,
            });
            let new_root = TreeNode {
                id: util::id_init(),
                data: vec![Some(middle)],
                children: Some(vec![left_node, right_node]),
                parent: None,
            };
            *self = new_root;
            Ok(())
        } else {
            let parent = self
                .parent
                .as_mut()
                .ok_or(anyhow!("parent is None"))?
                .clone();
            let left_node = Box::new(TreeNode {
                id: util::id_init(),
                data: left_data,
                children: left_child,
                parent: Some(parent),
            });
            let right_node = Box::new(TreeNode {
                id: util::id_init(),
                data: right_data,
                children: right_child,
                parent: Some(parent),
            });

            unsafe {
                (*parent).data.push(Some(middle));
                (*parent).data.sort();
                if let Some(children) = (*parent).children.as_mut() {
                    let index = children.iter().position(|c| c.id == self.id).unwrap_or(1);
                    children.remove(index); // Remove the current node
                    children.insert(index, left_node);
                    children.insert(index + 1, right_node);
                } else {
                    return Err(anyhow!("parent has NO child Vec"));
                }
            }

            Ok(())
        }
    }
}
