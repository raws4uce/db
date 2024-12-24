use anyhow::{anyhow, Result};
use dump::util;
pub struct TreeNode {
    id: usize,
    pub data: Vec<usize>,                     // Data stored in this node
    pub children: Option<Vec<Box<TreeNode>>>, // Children nodes
    pub parent: Option<*mut TreeNode>,        // Raw pointer to avoid lifetime conflicts
}

impl TreeNode {
    pub fn new_root(data: usize) -> TreeNode {
        TreeNode {
            id: util::id_init(),
            data: vec![data],
            children: None,
            parent: None,
        }
    }
    pub fn node_innit(&mut self, data: Vec<usize>) -> TreeNode {
        TreeNode {
            id: util::id_init(),
            data,                         // Directly initialise with the data
            children: None,               // No children for a new node
            parent: Some(self as *mut _), // Use raw pointer to self for parent
        }
    }

    pub fn look_for(&self, data: usize) -> Option<Vec<usize>> {
        if let Ok(_) = self.data.binary_search(&data) {
            return Some(self.data.clone());
        }

        let index = self
            .data
            .iter()
            .enumerate()
            .find(|(_, dat)| Some(dat).map_or(false, |d| &data < d))
            .map(|(i, _)| i)
            .unwrap_or_else(|| self.data.len());
        if let Some(children) = self.children.as_ref() {
            if let Some(child) = children.get(index) {
                return child.look_for(data);
            }
        }
        None
    }

    pub fn stick_in(&mut self, data: usize) -> Result<()> {
        let index = self
            .data
            .iter()
            .enumerate()
            .find(|(_, dat)| Some(dat).map_or(false, |d| &data < d))
            .map(|(i, _)| i)
            .unwrap_or_else(|| self.data.len());

        //a little inneficient but its like 4 u64||u32s, no biggie
        let mut v = self.data.clone();
        v.push(data);
        let new_child = self.node_innit(v); //satisfies compiler...

        if let Some(children) = self.children.as_mut() {
            if let Some(child) = children.get_mut(index) {
                child.stick_in(data)?;
            } else {
                children.insert(index, Box::new(new_child));
            }
        } else {
            if self.data.len() == 4 {
                self.data.push(data);
                self.split()?;
            } else {
                self.data.push(data);
                self.data.sort_by(|a, b| a.cmp(b));
            }
        }
        Ok(())
    }

    pub fn delete(&mut self, data: usize) -> Result<()> {
        if let Ok(i) = self.data.binary_search(&data) {
            if self.parent.is_none() {
                //leaf me
                self.data.remove(i);
            } else {
                //replace entry with predecessor
                let pre = self.data[i - 1];
                self.data[i] = pre;
                self.children.as_mut().unwrap()[i].delete(pre)?;
            }
        } else {
            let index = self
                .data
                .iter()
                .enumerate()
                .find(|(_, dat)| Some(dat).map_or(false, |d| &data < d))
                .map(|(i, _)| i)
                .unwrap_or_else(|| self.data.len());

            if let Some(children) = self.children.as_mut() {
                if let Some(child) = children.get_mut(index) {
                    child.delete(data)?;
                    if child.data.len() < 2 {
                        self.underflow(index)?;
                    }
                }
            }
        }
        Err(anyhow!("THIS IS AN ERROR"))
    }
    fn underflow(&mut self, index: usize) -> Result<()> {
        Ok(())
    }
    pub fn update() {}
    pub fn traverse() {}
    pub fn split(&mut self) -> Result<()> {
        let mid_index = self.data.len() / 2;
        let middle = self.data[mid_index];

        let left_data: Vec<usize> = self.data.drain(..mid_index).collect();
        let right_data: Vec<usize> = self.data.drain(mid_index..).collect();

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
                data: vec![middle],
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
                (*parent).data.push(middle);
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

            unsafe {
                if (*parent).data.len() > 4 {
                    (*parent).split()?;
                }
            }

            Ok(())
        }
    }
}
