use std::rc::Rc;
use std::cell::{ RefCell, Ref, RefMut };

pub struct List<T> {
  head: Link<T>,
  tail: Link<T>,
}

pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Node<T> {
  elem: T,
  prev: Link<T>,
  next: Link<T>
}

impl<T> Node<T> {
  pub fn new(elem: T) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Node {
      elem,
      prev: None,
      next: None
    }))
  }
}

impl<T> List<T> {
  pub fn new() -> Self {
    Self {
      head: None,
      tail: None,
    }
  }

  pub fn push_front(&mut self, elem: T) {
    let new_node = Node::new(elem);
    match self.head.take() {
      Some(old_head) => {
        old_head.borrow_mut().prev = Some(new_node.clone());
        new_node.borrow_mut().next = Some(old_head.clone());
        self.head = Some(new_node);
      },
      None => {
        self.head = Some(new_node.clone());
        self.tail = Some(new_node.clone());
      }
    }
  }

  pub fn push_back(&mut self, elem: T) {
    let new_node = Node::new(elem);
    match self.tail.take() {
      Some(old_tail) => {
        old_tail.borrow_mut().next = Some(new_node.clone());
        new_node.borrow_mut().prev = Some(new_node.clone());
        self.tail = Some(new_node);
      },
      None => {
        self.head = Some(new_node.clone());
        self.tail = Some(new_node.clone());
      }
    }
  }

  pub fn pop_front(&mut self) -> Option<T> {
    self.head.take().map(|old_head| {
      match old_head.borrow_mut().next.take() {
        Some(new_head) => {
          new_head.borrow_mut().prev.take();
          self.head = Some(new_head.clone());
        },
        None => {
          // old_head 为当前列表中最后一个元素
          self.tail.take();
        }
      }

      // can not move out of dereference RefMut
      // old_head.borrow_mut().elem

      // Result<T, E> 中 T 要实现 Debug trait 才能使用 unwrap
      // Rc::try_unwrap(old_head).unwrap().into_inner().elem

      Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
    })
  }

  pub fn pop_back(&mut self) -> Option<T> {
    self.tail.take().map(|old_tail| {
      match old_tail.borrow_mut().prev.take() {
        Some(new_tail) => {
          new_tail.borrow_mut().next.take();
          self.tail = Some(new_tail.clone());
        },
        None => {
          self.head.take();
        }
      }
      Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
    })
  }

  pub fn peek_front(&self) -> Option<Ref<T>> {
    self.head.as_ref().map(|node|
      // cannot return value referencing temporary value
      // &node.borrow().elem
      // --------------
      // |             
      // temporary value created here
      // borrow 方法返回值包了一层 Ref<'a, T>，导致了内部的引用生命周期发生了变化，这里并不能从 Ref<'a, T> 中 得到生命周期更长的 &T

      // node.borrow()

      Ref::map(node.borrow(), |node| &node.elem)
    )
  }

  pub fn peek_back(&self) -> Option<Ref<T>> {
    self.tail.as_ref().map(|node| Ref::map(node.borrow(), |node| &node.elem))
  }

  pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
    self.head.as_ref().map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
  }

  pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
    self.tail.as_ref().map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
  }

  pub fn into_iter(self) -> IntoIter<T> {
    IntoIter(self)
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    while self.pop_front().is_some() {}
  }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop_front()
  }
}

pub struct Iter<'a, T>(Option<Ref<'a, Node<T>>>);

impl<T> List<T> {
  pub fn iter(&self) -> Iter<'_, T> {
    Iter(self.head.as_ref().map(|node| node.borrow()))
  }
}

// impl<'a, T> Iterator for Iter<'a, T> {
//   type Item = Ref<'a, T>;

  // 这种方法有两处错误，一个是 escape from closure，在 self.0 的赋值处，另一处在 Ref::map 处的 move out
  // fn next(&mut self) -> Option<Self::Item> {
    // ------------
    //           | `self` is declared here, outside of the closure body
    // self.0.take().map(|old_head| {
      // self.0 = old_head.next.as_ref().map(|new_head| new_head.borrow());
      // |        -------- borrow is only valid in the closure body
      // |
      // reference to `old_head` escapes the closure body here, old_head 活的不够久，从 new_head 中 borrow 出来的 Ref 只允许和 old_head 活的一样久
      // Ref::map(old_head, |node| &node.elem) // old_head.next.as_ref() 和 Ref::map(old_head, 两处都使用了 old_head 并对其 map，相当于对其做了一次分割, old_head 在此处活的不够久


      // self.0 = Some(Ref::map(old_head, |node| &*node.next.unwrap().into_inner()));
    // })
  // }

  // 因为上述问题，所以考虑能不能对 old_head map 一次，分割出它里面的两个 filed
  // fn next(&mut self) -> Option<Self::Item> {
  //   self.0.take().map(|old_head| {
  //     let (new_head, elem) = Ref::map_split(old_head, |inner_old_head| (&inner_old_head.next, &inner_old_head.elem));
  //     self.0 = new_head.as_ref().map(|new_head_inner| new_head_inner.borrow());
  //     elem
  //   })
  // }

  // fn next(&mut self) -> Option<Self::Item> {
  //   self.0.take().map(|old_head| {
  //     let (new_head, elem) = Ref::map_split(old_head, |inner_old_head| (&inner_old_head.next, &inner_old_head.elem));
  //     self.0 = if new_head.is_some() {
  //       Some(Ref::map(new_head, |inner| {
  //         &(*inner.as_ref().unwrap()).into_inner()
  //       }))
  //     } else {
  //       None
  //     };
  //     elem
  //   })
  // }
// }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    let mut list = List::new();
    assert_eq!(list.pop_front(), None);

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(1));

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(3));
  }

  #[test]
  fn peek() {
    let mut list = List::new();
    list.push_front(1);
    list.push_front(2);

    // Ref 不能被直接比较，所以要解出来
    assert_eq!(&*list.peek_front().unwrap(), &2);

    assert_eq!(&*list.peek_back().unwrap(), &1);
    
    let front = list.peek_front_mut();
    *front.unwrap() += 1;
    assert_eq!(&*list.peek_front().unwrap(), &3);

    let back = list.peek_back_mut();
    *back.unwrap() += 1;
    assert_eq!(&*list.peek_back().unwrap(), &2);
  }
}