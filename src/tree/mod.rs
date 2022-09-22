pub mod traversal;


#[cfg(test)]
mod tests {
    #[test]
    fn test_macro() {
        let t = macros::tree!({val: 1, left: {2, right: {3}}});
        dbg!(t);
    }
}