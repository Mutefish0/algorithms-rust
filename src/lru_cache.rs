use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

struct Node {
    val: i32,
    key: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(k: i32, v: i32) -> Self {
        Node {
            val: v,
            key: k,
            next: None, 
            prev: None
        }
    }
}

struct LRUCache {
    capacity: i32,
    tail: Option<Rc<RefCell<Node>>>,
    head: Option<Rc<RefCell<Node>>>,
    node_map: HashMap<i32, Rc<RefCell<Node>>>
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity,
            tail: None,
            head: None,
            node_map: HashMap::new()
        }
    }

    fn get(&mut self, k: i32) -> i32 {
        if let Some(v) = self.node_map.get(&k) {
            // 如果是头部，则直接返回
            if let Some(ref head) = self.head {
                if head.borrow().key == k {
                    return head.borrow().val;
                }
            }

            // 把左邻节点指向右邻节点
            if let Some(ref prev) = v.borrow().prev {
                if let Some(ref next) = v.borrow().next {
                    prev.borrow_mut().next = Some(Rc::clone(next));
                }
            }

            // 把右邻节点指向左邻节点
            if let Some(ref next) = v.borrow().next {
                if let Some(ref prev) = v.borrow().prev {
                    next.borrow_mut().prev = Some(Rc::clone(prev));
                } else {
                    next.borrow_mut().prev = None;
                }
            }
            
            // 把当前节点的prev指向head，把head的next指向当前节点
            if let Some(ref head) = self.head {
                v.borrow_mut().prev = Some(Rc::clone(head));
                head.borrow_mut().next = Some(Rc::clone(v));
            }


            // 如果当前节点是尾节点，则需要移动尾节点到当前节点的右邻节点
            let mut next_tail = None;
            if let Some(ref tail) = self.tail {
                if Rc::ptr_eq(tail, v) {
                    if let Some(ref next) = v.borrow().next {
                        next_tail = Some(Rc::clone(next));
                    }
                }
            }
            if next_tail.is_some() {
                self.tail = next_tail;
            }

            // 把当前节点的next置空
            v.borrow_mut().next = None;
            // 把head指向当前节点
            self.head = Some(Rc::clone(v));

            return v.borrow().val;
        }
        -1
    }

    fn put(&mut self, k: i32, v: i32) {
        if self.capacity > 0 {
          // 1. 判断是否存在 2. 如果存在则激活该节点
          let exists = self.get(k) > -1;
          // 如果存在则直接设置
          if exists {
            if let Some(ref head) = self.head {
              head.borrow_mut().val = v;
            }
          } else {
            // 超出，移除尾部节点
            if self.node_map.len() as i32 == self.capacity {
              let mut next_tail: Option<Rc<RefCell<Node>>> = None;
              if let Some(ref tail) = self.tail {
                self.node_map.remove(&tail.borrow().key);
                if let Some(ref next) = tail.borrow().next {
                    next.borrow_mut().prev = None;
                    next_tail = Some(Rc::clone(next));
                }
                tail.borrow_mut().next = None;
              }
              self.tail = next_tail;
            }

            let node = Rc::new(RefCell::new(Node::new(k, v)));
            self.node_map.insert(k, Rc::clone(&node));
            if let Some(ref head) = self.head {
                head.borrow_mut().next = Some(Rc::clone(&node));
                node.borrow_mut().prev = Some(Rc::clone(head));
            }
            if self.tail.is_none() {
                self.tail = Some(Rc::clone(&node));
            }
            self.head = Some(node);
          }
        }
    }
}


#[cfg(test)]
mod tests {
  use crate::lru_cache::LRUCache;

  #[test]
  fn case0() {
    let mut cache = LRUCache::new(0);
    cache.put(1, 1);
    assert_eq!(cache.get(1), -1);
  }

  #[test]
  fn case1() {
    let mut cache = LRUCache::new(1);
    cache.put(1, 1);
    assert_eq!(cache.get(1), 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(2), 2);
  }

  #[test]
  fn case2() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);
    cache.put(3, 3);
    assert_eq!(cache.get(2), -1);
    cache.put(4, 4);
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.get(4), 4);
  }

  #[test]
  fn case3() {
    let mut cache = LRUCache::new(2);
    cache.put(2, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(2), 2);
    cache.put(1, 1);
    cache.put(4, 1);
    assert_eq!(cache.get(2), -1);
  }
}