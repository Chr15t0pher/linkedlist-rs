use std::ptr;

pub struct List<T> {
  head: Link<T>,
  tail: *mut Node<T>
}

pub type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
  elem: T,
  next: Link<T>
}

impl<T> List<T> {
  pub fn new() -> Self {
    Self { head: None, tail: ptr::null_mut() }
  }

  // new_tail 和 raw_tail 的 borrow stack 发生了重叠
  pub fn push(&mut self, elem: T) {
    let mut new_tail = Box::new(Node { elem, next: None }); // new_tail 借用 begins here
    let raw_tail: *mut _ = &mut *new_tail; // raw_tail 借用 begins here

    if !self.tail.is_null() {
      unsafe { (*self.tail).next = Some(new_tail) };
    } else {
      self.head = Some(new_tail); // new_tail 借用 ends here
    }
    self.tail = raw_tail; // raw_tail 借用 ends here
  }

  pub fn pop(&mut self) -> Option<T> {
    self.head.take().map(|old_head| {
      self.head = old_head.next;
      if self.head.is_none() {
        self.tail = ptr::null_mut();
      }
      old_head.elem
    })
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
    assert_eq!(list.pop(), Some(3));

    // 对同一指针的 copy，调用顺序变化不会导致 borrow stack 错误的问题
    unsafe {
      let mut data = [0; 10];
      let ref1_at_0 = &mut data[0];            // Reference to 0th element
      let ptr2_at_0 = ref1_at_0 as *mut i32;   // Ptr to 0th element
      let ptr3_at_0 = ptr2_at_0;               // Ptr to 0th element
      let ptr4_at_0 = ptr2_at_0.add(0);        // Ptr to 0th element
      let ptr5_at_0 = ptr3_at_0.add(1).sub(1); // Ptr to 0th element


      *ptr3_at_0 += 3;
      *ptr2_at_0 += 2;
      *ptr4_at_0 += 4;
      *ptr5_at_0 += 5;
      *ptr3_at_0 += 3;
      *ptr2_at_0 += 2;
      *ref1_at_0 += 1;

      // Should be [20, 0, 0, ...]
      println!("{:?}", &data[..]);
    }
  }
}