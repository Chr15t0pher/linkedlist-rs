#[allow(dead_code)]

// 使用 Option 替代之前的 enum
// pub enum Link {
//   Empty,
//   More(Box<Node>),
// }
type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct Node<T> {
  elem: T,
  next: Link<T>
}

#[derive(Debug)]
pub struct List<T> {
  head: Link<T>
}

impl<T> List<T> {
  pub fn new() -> Self {
    List {
      head: Link::None
    }
  }

  pub fn push(&mut self, elem: T) {
    let new_node = Box::new(Node {
      elem,
      next: self.head.take(),
    });
    self.head = Link::Some(new_node);
  }

  pub fn pop(&mut self) -> Option<T> {
    self.head.take().map(|node| {
      self.head = node.next;
      node.elem
    })
  }

  pub fn peek(&self) -> Option<&T> {
    self.head.as_ref().map(|node| {
      &node.elem
    })
  }

  pub fn peek_mut(&mut self) -> Option<&mut T> {
    self.head.as_mut().map(|node| {
      &mut node.elem
    })
  }

  pub fn into_iter(self) -> IntoIter<T> {
    IntoIter(self)
  }

  pub fn iter(&self) -> Iter<T> {
    // Iter{ next: self.head.as_deref() }
    Iter { next: self.head.as_ref().map(|node| { &**node })}
  }

  pub fn iter_mut(&mut self) -> IterMut<T> {
    IterMut { next: self.head.as_deref_mut() }
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    let mut cur_link = self.head.take();
    while let Link::Some(mut node) = cur_link {
      cur_link = node.next.take();
    }
  }
}

// impl iterator for List
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // self.next = node.next.map(|node| &*node); error,此时 node 中值的所有权被传递到 map 中，这个时候 &*node 返回的话，引用指向一个局部作用域
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
  next: Option<&'a mut Node<T>>
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    self.next.take().map(|node| {
      self.next = node.next.as_deref_mut();
      &mut node.elem
    })
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
    let mut list = List::new();
    for i in 0..1000000 {
      list.push(i);
    }
    drop(list);
  }

  #[test]
  fn peek() {
    let mut list = List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.push(1);
    list.push(2);
    list.push(3);
    
    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));
    // list.peek_mut().map(|&mut elem| {
    //   elem = 42
    // }); // error: cannot assign twice to immutable variable `elem` label: first assignment to `elem`
    // 实际上 &mut elem 是一个模式匹配，它用 &mut elem 模式去匹配一个可变的引用，此时匹配出来的 elem 显然是一个值，而不是可变引用，因为只有完整的形式才是可变引用！

    list.peek_mut().map(|elem| *elem = 42);
    assert_eq!(list.peek(), Some(&42));
  }

  #[test]
  fn into_iter_fn() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
  }

  #[test]
  fn iter_fn() {
    let mut list = List::new();
    list.push(1);
    list.push(2);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
  }

  #[test]
  fn iter_mut_fn() {
    let mut list = List::new();
    list.push(1);
    list.push(2);

    let mut iter_mut = list.iter_mut();
    assert_eq!(iter_mut.next(), Some(&mut 2));
    assert_eq!(iter_mut.next(), Some(&mut 1));
  }
}