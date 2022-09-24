use proc_macro2::{TokenStream, TokenTree};
use std::str::FromStr;


pub fn tree(input: TokenStream) -> TokenStream {
    if input.is_empty(){
        TokenStream::from_str("None").unwrap()
    } else {
        let tree = TreeNode::from(input);
        TokenStream::from_str(format!("Some(::std::rc::Rc::new(::std::cell::RefCell::new({})))", tree).as_str()).unwrap()
    }
    
}

#[derive(Debug)]
struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl std::fmt::Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "datastructure::TreeNode{{val: {}, ", self.val)?;

        if let Some(ref left) = self.left {
            write!(
                f,
                "left: Some(::std::rc::Rc::new(::std::cell::RefCell::new({}))), ",
                left
            )?;
        } else {
            write!(f, "left: None, ")?;
        }

        if let Some(ref right) = self.right {
            write!(
                f,
                "right: Some(::std::rc::Rc::new(::std::cell::RefCell::new({}))), ",
                right
            )?;
        } else {
            write!(f, "right: None, ")?;
        }
        write!(f, "}}")
    }
}

impl Default for TreeNode {
    fn default() -> Self {
        TreeNode {
            val: 0,
            left: None,
            right: None,
        }
    }
}

enum WhichNode {
    S,
    L,
    R,
}

impl From<TokenStream> for TreeNode {
    fn from(stream: TokenStream) -> Self {
        let mut tree_node = TreeNode::default();
        let mut which_node = WhichNode::S;

        for tree in stream.into_iter() {
            match tree {
                TokenTree::Group(group) => match which_node {
                    WhichNode::S => {
                        tree_node = TreeNode::from(group.stream());
                    }
                    WhichNode::L => {
                        tree_node.left = Some(Box::new(TreeNode::from(group.stream())));
                    }
                    WhichNode::R => {
                        tree_node.right = Some(Box::new(TreeNode::from(group.stream())));
                    }
                },
                TokenTree::Ident(ident) => {
                    let id = ident.to_string();
                    if id.eq_ignore_ascii_case("left") {
                        which_node = WhichNode::L;
                    } else if id.eq_ignore_ascii_case("right") {
                        which_node = WhichNode::R;
                    }
                }
                TokenTree::Punct(_punct) => {}
                TokenTree::Literal(literal) => {
                    let lit = literal.to_string().parse().unwrap();
                    tree_node.val = lit;
                }
            }
        }
        tree_node
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_x() {
        let ts = TokenStream::from_str("{1, right:{2, left: {3}}}").unwrap();
        let tree = tree(ts);
        dbg!(tree);
    }

    #[test]
    fn print_tree() {
        let t = TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })),
            right: None,
        };

        println!("{}", t);
    }
}
