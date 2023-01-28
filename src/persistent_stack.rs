// 在 single linked list 中的链表是一个单所有权、可变的链表
// 实际使用中，共享所有权是更实用的方式

// 考虑如下的使用场景, list1, list2, list3 都拥有对 B 节点的所有权
// list1 = A -> B -> C -> D
// list2 = tail(list1) = B -> C -> D
// list3 = push(list, X) = X -> B -> C -> D

// list1 ->  A
//           |
//           v
// list2 ->  B -> C -> D
//           ^
//           |
// list3 ->  X

use std::rc::Rc;

pub struct Node<T> {
  elem: T,
  next: Link<T>
}

pub type Link<T> = Option<Rc<Node<T>>>;

pub struct List<T> {
  head: Link<T>
}

impl<T> List<T> {
  pub fn new() -> List<T> {
    List { head: None }
  }

  pub fn prepend(&self, elem: T) -> List<T> {
    List {
      head: Some(Rc::new(Node {
        elem, 
        next: self.head.clone()
      }))
    }
  }

  pub fn tail(&self) -> List<T> {
    // List { head: self.head.as_ref().map(|node| node.next.unwrap().clone() )}
    List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
  }

  pub fn head(&self) -> Option<&T> {
    self.head.as_ref().map(|node| &node.elem)
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    let mut head = self.head.take();
    while let Some(node) = head {
      if let Ok(mut inner) = Rc::try_unwrap(node) {
        head = inner.next.take();
      } else {
        break
      }
    }
  }
}

// impl<T:Clone> Iterator for List<T> {
//   type Item = T;

//   fn next(&mut self) -> Option<Self::Item> {
//     self.head.take().map(|node| {
//       self.head = node.next.clone();
//       node.elem.clone()
//     })
//   }
// }

pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
      self.next = node.next.as_deref();
      &node.elem
    })
  }
}

impl<T> List<T> {
  pub fn iter(&self) -> Iter<'_, T> {
    Iter {
      next: self.head.as_deref()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn basic() {
    let list = List::new();
    assert_eq!(list.head(), None);

    let list = list.prepend(1).prepend(2).prepend(3);
    assert_eq!(list.head(), Some(&3));

    let list = list.tail();
    assert_eq!(list.head(), Some(&2));

    let list = list.tail();
    assert_eq!(list.head(), Some(&1));

    let list = list.tail();
    assert_eq!(list.head(), None);
  }

  #[test]
  fn iter_fn() {
    let list = List::new();
    let list = list.prepend(1).prepend(2).prepend(3);
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
  }
}