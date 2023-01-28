// first version
// #[derive(Debug)]
// pub enum List {
//   Cons(i32, Box<List>),
//   Nil,
// }
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Link {
  Empty,
  More(Box<Node>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
  elem: i32,
  next: Link
}

#[derive(Debug)]
pub struct List {
  head: Link
}

impl List {
  pub fn new() -> Self {
    List {
      head: Link::Empty
    }
  }

  pub fn push(&mut self, elem: i32) {
    let new_node = Box::new(Node {
      elem,
      next: std::mem::replace(&mut self.head, Link::Empty),
    });
    self.head = Link::More(new_node);
  }

  pub fn pop(&mut self) -> Option<i32> {
    match std::mem::replace(&mut self.head, Link::Empty) {
      Link::Empty => return None,
      Link::More(node) => {
        self.head = node.next;
        Some(node.elem)
      }
    }
  }
}

impl Drop for List {
  fn drop(&mut self) {
    let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);
    while let Link::More(mut node) = cur_link {
      cur_link = std::mem::replace(&mut node.next, Link::Empty);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn basic() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(1));
  }
  
  #[test]
  fn long_list() {
    // let mut list = List::new();
    // for i in 0..100000000 {
    //   list.push(i);
    // }
    // drop(list);
  }
}