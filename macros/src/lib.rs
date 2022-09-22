//! 树/链表题, 除了题目自身难受, 构建测试用例也是难事
//! `Rc` `RefCell` `Box` 反复写, 人都麻了.
//!
//! 所以正这个 proc_macro, 自己定义一套 dsl, 至少方便一下测试用例.
//!
use proc_macro::TokenStream;

mod tree_impl;
mod list_impl;

/// tree!(tree-define)
/// tree-define: {val: int, [left: tree-define], [right: tree-define] }
#[proc_macro]
pub fn tree(input: TokenStream) -> TokenStream {
    tree_impl::tree(input.into()).into()
}

/// list!(1,2,3,4)
/// list!([1,2,3,4])
#[proc_macro]
pub fn list(input: TokenStream) -> TokenStream{
    list_impl::list(input.into()).into()
}