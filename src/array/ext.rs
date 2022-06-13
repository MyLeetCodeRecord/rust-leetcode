/// # 二维数组旋转
///
/// ## 题目
/// * 简单
///     * [剑指 Offer 58 - II. 左旋转字符串](reverse_left_words)
/// * 中等
///     * [48. 旋转图像](rotate)
///     * [54. 螺旋矩阵](spiral_order)
///     * [59. 螺旋矩阵 II](generate_matrix)
/// * 困难
pub mod rotate;

/// # 随机
///
/// 特点: 概率; 随机
///
/// 对应的测试用例不好写, 也不知OJ是如何判定概率的
/// 这里只校验 仅一个选择时的情况, 即100%的结果
pub mod random;

/// # 快排相关
/// 
/// 特点: 每论能确定一个元素的最终位置
/// 
/// ## 题目
/// * 简单
/// * 中等
///     * [912. 排序数组](sort_array)
///     * [56. 合并区间](merge)
pub mod quick;


pub mod n_sum;

pub mod kmp;

/// 数学相关题目
pub mod math;
/// 区间相关题目
pub mod rng;
/// 栈相关题目
pub mod stack;
// 拓扑排序
// pub mod topological;