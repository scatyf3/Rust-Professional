/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        if self.root.is_none(){
            let new_node = Box::new(TreeNode::new(value));
            self.root = Some(new_node);
            return;
        }else{
            self.root.as_mut().unwrap().insert_node(value);
        }
    }
    

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        if self.root.is_none(){
            return false;
        }
        let mut cur_ptr = self.root.as_ref().unwrap();
        loop{
            if cur_ptr.value > value {
                if let Some(ref right_ptr) = cur_ptr.right{ 
                    cur_ptr = right_ptr;
                }else{
                    return false;
                }
            }else if cur_ptr.value < value{
                if let Some(ref left_ptr) = cur_ptr.left{ //正确使用 &mut Box aka ref mut
                    cur_ptr = left_ptr;
                }else{
                    let new_node = Some(Box::new(TreeNode::new(value)));
                    return false;
                }
            }else{
                return true;
            }
            
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert_node(&mut self, value: T) {
        if self.value > value {
            if let Some(ref mut right_ptr) = self.right{ //正确使用 &mut Box aka ref mut
                right_ptr.insert_node(value);
            }else{
                let new_node = Some(Box::new(TreeNode::new(value)));
                self.right = new_node;
            }
        }else if  self.value < value{
            if let Some(ref mut left_ptr) = self.left{ //正确使用 &mut Box aka ref mut
                left_ptr.insert_node(value);
            }else{
                let new_node = Some(Box::new(TreeNode::new(value)));
                self.left = new_node;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


