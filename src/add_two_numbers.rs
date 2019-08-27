use crate::solution::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  val: i32,
  next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      if l1.is_some() && l2.is_some() {
        let mut sum = 0;
        let mut nums = vec![];
        let mut result = None;
        let mut p1 = &l1;
        let mut p2 = &l2;
        while p1.is_some() || p2.is_some() || sum > 0 {
          if let Some(n) = p1 {
            sum += n.val;
            p1 = &n.next;
          }
          if let Some(n) = p2 {
            sum += n.val;
            p2 = &n.next;
          }
          if sum > 9 {
            nums.push(sum - 10);
            sum = 1;
          } else {
            nums.push(sum);
            sum = 0;
          }
        }
        while let Some(x) = nums.pop() {
          let mut node = Box::new(ListNode::new(x));
          node.next = result;
          result = Some(node);
        }
        result
      } else if l1.is_some() {
        l1
      } else if l2.is_some() {
        l2
      } else {
        None
      }
    }
}

#[cfg(test)]
mod tests {
  use crate::solution::Solution;
  use crate::add_two_numbers::ListNode;

  fn vec_to_list_node(mut v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    loop {
      if let Some(x) = v.pop() {
        let mut node = Box::new(ListNode::new(x));
        node.next = head;
        head = Some(node);
      } else {
        break;
      }
    }
    head
  }

  #[test]
  fn test_vec_to_list_node() {
    let ln = vec_to_list_node(vec![1, 2, 3]);
    let mut p = ln.unwrap();
    assert_eq!(p.val, 1);
    p = p.next.unwrap();
    assert_eq!(p.val, 2);
    p = p.next.unwrap();
    assert_eq!(p.val, 3);
  }

  #[test]
  fn test_solution() {
    assert_eq!(
      Solution::add_two_numbers(
        vec_to_list_node(vec![3, 2, 1]), 
        vec_to_list_node(vec![2, 3, 4])
      ),
      vec_to_list_node(vec![5, 5, 5])
    );

    assert_eq!(
      Solution::add_two_numbers(
        vec_to_list_node(vec![2, 4, 3]), 
        vec_to_list_node(vec![5, 6, 4])
      ),
      vec_to_list_node(vec![7, 0, 8])
    );

    assert_eq!(
      Solution::add_two_numbers(
        vec_to_list_node(vec![0]), 
        vec_to_list_node(vec![5, 6, 4])
      ),
      vec_to_list_node(vec![5, 6, 4])
    );

    assert_eq!(
      Solution::add_two_numbers(
        vec_to_list_node(vec![2, 4, 6]), 
        vec_to_list_node(vec![5, 6, 4])
      ),
      vec_to_list_node(vec![7, 0, 1, 1])
    );
  }
}