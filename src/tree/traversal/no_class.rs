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

        let root_idx = inorder.iter().position(|&x| x == root_val).unwrap();
        let (inorder_left, inorder_right) = inorder.split_at(root_idx);

        let inorder_left_len = inorder_left.len();
        let (preorder_left, preorder_right) = preorder.split_at(inorder_left_len + 1);

        root.left = build(&preorder_left[1..], inorder_left);
        root.right = build(preorder_right, &inorder_right[1..]);
        Some(Rc::new(RefCell::new(root)))
    }
    build(&preorder, &inorder)
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
}
