# LeetCode

[![Build Status](https://github.com/MyLeetCodeRecord/rust-leetcode/actions/workflows/CICD.yml/badge.svg?branch=master)](https://github.com/MyLeetCodeRecord/rust-leetcode/actions/workflows/CICD.yml)
[![codecov](https://codecov.io/gh/MyLeetCodeRecord/rust-leetcode/branch/master/graph/badge.svg?token=XLG5PJFZ13)](https://codecov.io/gh/MyLeetCodeRecord/rust-leetcode)

## 介绍

起初只是为了面试刷刷题, 做做总结.

后来发现顺路写写Rust也是ok的.

纯个人笔记向, 所以一些地方的注释说明看着会有些乱, 有问题可以随时提issue.

### 环境

- OJ: 
  - [LeetCode][leetcode]
- 语言: [Rust][rust]
- IDE: [我的 IDE 配置][我的ide 配置]

### 内容

- 题目的*解法&说明*都在代码文档中标注了. 可以执行`cargo doc`查看.
- 利用 Rust 的单测, 对题目说明中的基础测试用例和修复过程中的 badcase 都做了校验, 可以通过`cargo test`查看.
- 覆盖率使用了[grcov](https://github.com/mozilla/grcov) 和 [codecov](https://about.codecov.io/), 可参考前面的[IDE 配置][我的ide 配置]

### 其他

- 一些题目, [LeetCode][leetcode]不支持[Rust][rust]
- 链表, 树的题目用 [Rust] 写也确实费劲

## 参考

- [labuladong 的算法小抄](https://labuladong.gitee.io/algo/)
- [代码随想录](https://programmercarl.com/)
- [OI Wiki](https://oi-wiki.org/)
- 以及各个题目下面的题解

## 协议

MIT

---

[leetcode]: https://leetcode-cn.com/problemset/all/
[rust]: https://www.rust-lang.org/
[我的ide 配置]: https://www.wolai.com/4NmTTcVoLjSGPLxpKMYJRp
