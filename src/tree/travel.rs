use super::*;

/// 层序遍历
#[allow(dead_code)]
mod level {
    use super::TreeNode;
    use std::{cell::RefCell, rc::Rc};
    use std::collections::VecDeque;

    /// [662. 二叉树最大宽度](https://leetcode.cn/problems/maximum-width-of-binary-tree/)
    /// 
    /// 其实就是问一个[完全二叉树](https://zh.wikipedia.org/zh-sg/%E4%BA%8C%E5%8F%89%E6%A0%91#%E5%AE%8C%E5%85%A8%E4%BA%8C%E5%8F%89%E6%A0%91)顺序存储时每层占用的最大空间
    /// 
    /// 父子节点的关系: node.left ==> nodes[2*i]; node.right ==> nodes[2*i + 1]
    /// 
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none(){
            return 0;
        }
        let mut queue = VecDeque::new();
        queue.push_back((root.unwrap(), 1usize));

        let mut width = 0;

        while !queue.is_empty(){
            let sz = queue.len();
            width = width.max(
                queue.back().unwrap().1 - queue.front().unwrap().1 + 1
            );

            for _ in 0..sz{
                let (node, label) = queue.pop_front().unwrap();
                if let Some(ref left) = node.clone().borrow().left{
                    queue.push_back((left.clone(), label*2));
                }
                if let Some(ref right) = node.clone().borrow().right{
                    queue.push_back((right.clone(), label*2 + 1));
                }
            }
        }
        width as i32
    }
}
