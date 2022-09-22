use proc_macro2::{TokenStream, TokenTree};
use std::str::FromStr;


pub fn list(stream: TokenStream) -> TokenStream {
    let mut vals : Vec<i32> = vec![];
    for tree in stream.into_iter(){
        match tree{
            TokenTree::Group(group) => return list(group.stream()),
            TokenTree::Literal(literal) => {
                let lit = literal.to_string().parse().unwrap();
                vals.push(lit);
            },
            _ => {}
        }
    }

    fn res(vals: &[i32]) -> String{
        if vals.is_empty(){
            return "None".to_string();
        }
        let (left, right) = vals.split_at(1);
        return format!("Some(Box::new(datastructure::ListNode{{val: {}, next: {}}}))", left[0], res(right));
    }

    TokenStream::from_str(res(&vals).as_str()).unwrap()
}
