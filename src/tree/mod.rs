//! 树相关题目
//! 
//! 主要是三种遍历, 加 dfs/bfs

pub mod traversal;
pub mod dfs;

#[cfg(test)]
mod tests {
    #[test]
    fn test_macro() {
        let t = macros::tree!({val: 1, left: {2, right: {3}}});
        dbg!(t);
    }
}