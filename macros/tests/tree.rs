use macros::tree;


fn main() {
    let t = tree!({1, right:{2, left: 3}});
    dbg!(t);
}
