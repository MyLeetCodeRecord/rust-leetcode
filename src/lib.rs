//!
//! # 集中训练
//!
//! ## 参考
//! 1. [代码随想录](https://programmercarl.com)
//! 2. [labuladong 的算法小抄](https://labuladong.gitee.io/algo/)
//!

pub mod array;
pub mod backtrack;
pub mod dp;
pub mod mhash;
pub mod moreandmore;
pub mod sstr;
pub mod tree;
pub mod list;
pub mod graph_search;

#[macro_export]
macro_rules! vec2 {
    () => (
        Vec::new()
    );
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x.to_vec());
            )*
            temp_vec
        }
    };
}