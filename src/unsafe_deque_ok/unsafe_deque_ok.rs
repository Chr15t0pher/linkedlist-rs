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
}