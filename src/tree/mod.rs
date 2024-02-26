//! 树相关题目
//! 
//! 主要是三种遍历, 加 dfs/bfs

pub mod traversal;
pub mod dfs;

#[cfg(test)]
mod tests {
    #[test]
    fn test_macro() {
        let t = macros::tree!(tree!{val: 16, left: {val: 8, left: {val: 1, right: {val: 2, right: {val: 7}}}, right: {val:12, left: {val: 9}}}, right: {val: 18, right: {val: 20}}});
        dbg!(t);
    }
}