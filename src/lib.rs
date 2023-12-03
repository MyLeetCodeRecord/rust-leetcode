#![warn(missing_docs)]
#![warn(dead_code)]
#![deny(unsafe_code)]

//! # 集中训练
//! 日常做点题, 适当的脑训练, 防止老年痴呆
//! 
//! # 说明
//! ## 题目来源
//! - [leetcode](https://leetcode-cn.com/)
//! - [codewars](https://www.codewars.com/)
//! 
//! ## 组织方式
//! 根据题目类型做了简单的模块化分, 每个模块的注释中会给出题目解法的跳转地址和难度(如果有)
//! 题目的地址和思路都会在相应的注释中标出
//! 
//! 同时利用rust-test来做单元测试
//! 单元测试的用例, 一般是相应OJ上给出的示例, 并且会把提交过程中出现的边界用例也加入到单元测试中
//! 
//! ## 费劲的题
//! 题不费劲, 但是rust对于一些数据结构的支持不太友好, 比如链表, 树. 
//! 为了便于构建树, 链表, 以及测试, 独立出了 datastructure 和 macros 两个模块
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
pub mod simulation;

/// 生成二维数组
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

fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    // assert!((u32::MAX as f64) < f64::MAX );
	let mut div = vec![];
    for i in 2..(integer as f64).sqrt() as u32 + 1{
        if integer%i == 0{
            div.push(i);
            div.push(integer/i);
        }
    }
    if div.is_empty(){
        return Err(format!("{} is prime", integer));
    }
    div.sort_unstable();
    Ok(div)
}