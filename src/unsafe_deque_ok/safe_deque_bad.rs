// pub struct List<T> {
//   head: Link<T>,
//   tail: Link<T>
// }

// pub type Link<T> = Option<Box<Node<T>>>;

// pub struct Node<T> {
//   elem: T,
//   next: Link<T>
// }

// impl<T> List<T> {
//   pub fn new() -> Self {
//     Self { head: None, tail: None }
//   }

//   pub fn push(&mut self, elem: T) {
//     let new_tail = Box::new(Node { elem, next: None });
//     let old_tail = std::mem::replace(&mut self.tail, Some(new_tail));
//     match old_tail {
//       Some(mut old_tail) => {
//         old_tail.next = Some(new_tail); // error: uew of moved value: `new_tail` label: value used here after move, 因为 Box 类型并未实现 Copy trait
//       },
//       None => {
//         self.head = Some(new_tail);
//       }
//     }
//   }
// }

// Box 没有 Copy 特征，因此不能再两个地方赋值，好在，可以使用没有所有权的引用类型
pub struct List<'a, T> {
  head: Link<T>,
  tail: Option<&'a mut Node<T>>
}

pub type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
  elem: T,
  next: Link<T>
}

impl<'a, T> List<'a, T> {
  pub fn new() -> Self {
    Self { head: None, tail: None }
  }

  // pub fn push(&mut self, elem: T) {
  //   let new_tail = Box::new(Node{ elem, next: None });
  //   let new_tail = match self.tail.take() {
  //     Some(old_tail) => {
  //       old_tail.next = Some(new_tail);
  //       old_tail.next.as_deref_mut()
  //     },
  //     None => {
  //       self.head = Some(new_tail);
  //       // 恼人的错误，`cannot infer an appropriate lifetime for autoref due to conflicting requirements`, push 函数中入参的 &mut self 的匿名生命周期，无法推断出来和 'a 的关系，所以需要手动标注 &mut self 的生命周期
  //       self.head.as_deref_mut()
  //     }
  //   };
  //   self.tail = new_tail
  // }

  pub fn push(&'a mut self, elem: T) {
    let new_tail = Box::new(Node{ elem, next: None });
    let new_tail = match self.tail.take() {
      Some(old_tail) => {
        old_tail.next = Some(new_tail);
        old_tail.next.as_deref_mut()
      },
      None => {
        self.head = Some(new_tail);
        self.head.as_deref_mut()
      }
    };
    self.tail = new_tail
  }

  pub fn pop(&'a mut self) -> Option<T> {
    self.head.take().map(|old_head| {
      self.head = old_head.next;
      if self.head.is_none() {
        self.tail = None
      }
      old_head.elem
    })
  }
}

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn basic() {
//     let mut list = List::new();
//     // 这里的错误来源于，pop 和 push 中的 &'a mut self 标注的 可变引用生命周期和外部结构体一样长，导致了只有 list 销毁才能再借出可变借用，'a 的生命周期实在太长了，不得不回到 RefCell 的老路上。
//     // error: cannot borrow `list` as mutable more than once at a time, first mutable borrow occurs here
//     assert_eq!(list.pop(), None);

//     // cannot borrow `list` as mutable more than once at a time
//     list.push(1);
//     list.push(2);
//     list.push(3);

//     assert_eq!(list.pop(), Some(1));
//     assert_eq!(list.pop(), Some(2));
//     assert_eq!(list.pop(), Some(3));
//   }
// }