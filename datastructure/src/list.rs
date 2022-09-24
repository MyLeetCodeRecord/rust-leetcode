#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// impl TryFrom<&[i32]> for ListNode {
//   type Error = &'static str;

//   fn try_from(s: &[i32]) -> Result<Self, Self::Error> {
        
//         if s.is_empty() {
//             return Err("empty");
//         }

//         let head = ListNode {
//             val: s.first().copied().unwrap(),
//             next: Self::try_from(&s[1..]).ok(),
//         };
//         Ok(head)
//   }
  
// }

