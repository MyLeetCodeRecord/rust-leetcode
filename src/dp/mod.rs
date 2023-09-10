//! DP
//!
//! DP的关键在于状态转移方程, 对应数学上, 也就是递推公式.
//!
//! 既然是递推, 也就是规模可以缩减. 只是思考的方向是 后一项能否用前面的状态推出.
//! 在确定了递推关系后, 还有一个难点就是初始状态.
//!
//! 其实从数据的角度看, 就是一个数列, 给出了前几项和递推关系, 然后让你求数列的第N项.
//!
//! 状态转移, 又有点记忆化搜索的味道, 不然就会出现大量的重复计算, 因此常见的就是维护一个dp数组, 维护出现过的状态.
//!
//! 这里面有一些是系列的题目, 放到了[ser]中, 其他在 [no_class] 中
//!
//! ## 题目
//! * 简单
//! * 中等
//!     * [926. 将字符串翻转到单调递增](min_flips_mono_incr)
//! * 困难
//!     * [473. 火柴拼正方形](makesquare)
//!

pub mod ser;
pub mod no_class;