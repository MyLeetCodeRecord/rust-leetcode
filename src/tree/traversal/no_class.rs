//! 多种遍历方式混合的

use datastructure::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [105. 从前序与中序遍历序列构造二叉树](https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-inorder-traversal/)
///
/// ## 思路
/// 1. 通过前序知道根节点，然后分割中序得到左右子树
///    - 通过中序找到根节点的位置, 然后通过左右子树长度分割前序
/// 2. 递归构建左右子树
///     - 递归结束条件是前序为空
/// 3. 挂载左右子树到根, 然后返回根节点
pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn build(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        let root_val = preorder[0];
        let mut root = TreeNode::new(root_val);

        // 题目保证值不同, 因此可以直接遍历寻找
        let root_idx = inorder.iter().position(|&x| x == root_val).unwrap();
        let (inorder_left, inorder_right) = inorder.split_at(root_idx);

        root.left = build(&preorder[1..1+inorder_left.len()], inorder_left);
        root.right = build(&preorder[1+inorder_left.len()..], &inorder_right[1..]);
        Some(Rc::new(RefCell::new(root)))
    }
    build(&preorder, &inorder)
}

/// [106. 从中序与后序遍历序列构造二叉树](https://leetcode.cn/problems/construct-binary-tree-from-inorder-and-postorder-traversal/)
///
/// ## 思路
/// 1. 与[105. 从前序与中序遍历序列构造二叉树](build_tree)类似, 通过中序做分割, 然后递归左右子树
/// 2. 区别是后序遍历的根节点在最后
pub fn build_tree2(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn build(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if postorder.is_empty() {
            return None;
        }

        let root_val = postorder.last().unwrap();
        let mut root = TreeNode::new(*root_val);

        // 题目保证值不同, 因此可以直接遍历寻找
        let root_idx = inorder.iter().position(|&x| x == *root_val).unwrap();
        let (inorder_left, inorder_right) = inorder.split_at(root_idx);

        root.left = build(inorder_left, &postorder[..inorder_left.len()]);
        root.right = build(
            &inorder_right[1..],
            &postorder[inorder_left.len()..postorder.len() - 1],
        );
        Some(Rc::new(RefCell::new(root)))
    }

    build(&inorder, &postorder)
}

#[cfg(test)]
mod tests {
    use macros::tree;

    use super::*;

    #[test]
    fn test_build_tree() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let root = build_tree(preorder, inorder);
        assert_eq!(
            root,
            tree!({3, left: {9}, right:{20, left: {15}, right:{7} }})
        );
    }

    #[test]
    fn test_build_tree2() {
        let inorder = vec![9, 3, 15, 20, 7];
        let postorder = vec![9, 15, 7, 20, 3];
        let root = build_tree2(inorder, postorder);
        assert_eq!(
            root,
            tree!({3, left: {9}, right:{20, left: {15}, right:{7} }})
        );
    }
}
