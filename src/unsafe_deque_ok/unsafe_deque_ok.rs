pub struct List<T> {
  head: Link<T>,
  tail: Link<T>
}

pub type Link<T> = *mut Node<T>;

pub struct Node<T> {
  elem: T,
  next: Link<T>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    List { head: std::ptr::null_mut(), tail: std::ptr::null_mut() }
  }

  pub fn push(&mut self, elem: T) {
    let new_tail = Box::into_raw(Box::new(Node { elem, next: std::ptr::null_mut() }));

    if !self.head.is_null() {
      unsafe { (*self.tail).next = new_tail };
    } else {
      self.head = new_tail;
    }

    self.tail = new_tail;
  }

  pub fn pop(&mut self) -> Option<T> {
    if self.head.is_null() {
      None
    } else {
      let old_head = unsafe { Box::from_raw(self.head) };
      self.head = old_head.next;

      if self.head.is_null() {
        self.tail = std::ptr::null_mut();
      }
      Some(old_head.elem)
    }
  }

  pub fn peek(&self) -> Option<&T> {
    // if self.head.is_null() {
    //   None
    // } else {
    //   Some(unsafe { &(*self.head).elem })
    // }
    unsafe { self.head.as_ref().map(|node| &node.elem) }
  }

  pub fn peek_mut(&mut self) -> Option<&mut T> {
    unsafe { self.head.as_mut().map(|node| &mut node.elem) }
  }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}

pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    self.next.take().map(|old_head| {
      self.next = unsafe { old_head.next.as_ref() };
      // NOTE: 这个地方的 &old_head.elem 的返回值生命周期是什么？
      &old_head.elem
    })
  }
}

pub struct IterMut<'a, T> {
  next: Option<&'a mut Node<T>>
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    self.next.take().map(|old_head| {
      self.next = unsafe { old_head.next.as_mut() };
      &mut old_head.elem
    })
  }
}

impl<T> List<T> {
  pub fn into_iter(self) -> IntoIter<T> {
    IntoIter(self)
  }

  pub fn iter(&mut self) -> Iter<T> {
    Iter { next: unsafe { self.head.as_ref() } }
  }

  pub fn iter_mut(&mut self) -> IterMut<T> {
    IterMut { next: unsafe { self.head.as_mut() } }
  }
}

impl<T> IntoIter<T> {
  
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    while let Some(_) = self.pop() {}
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn basic() {
    let mut list = List::new();
    assert_eq!(list.pop(), None);

    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), Some(2));
  }

  #[test]
  fn iter() {
    let mut list = List::new();

    list.push(1);
    
    list.push(2);
    list.push(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));

    let mut itermut = list.iter_mut();
    assert_eq!(itermut.next(), Some(&mut 1));
    assert_eq!(itermut.next(), Some(&mut 2));
    assert_eq!(itermut.next(), Some(&mut 3));

    let mut intoiter = list.into_iter();
    assert_eq!(intoiter.next(), Some(1));
    assert_eq!(intoiter.next(), Some(2));
    assert_eq!(intoiter.next(), Some(3));
  }
}