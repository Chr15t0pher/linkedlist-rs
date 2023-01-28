# rs-linkedlist

使用 Rust，按照 [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/index.html) 的描述实现链表。

## stack_bad

- 使用 Enum 实现链表的时候，使用 Box 包裹递归的 Enum 类型使其值封装到堆上，然后使用栈上的定长指针来指向堆上的不定长的值，不然会产生不定长类型。
- 内存对齐与 null 指针优化。

  ```rust
  struct A (i64, i8);
  struct B (i64, i8, bool);
  fn main() {
    dbg!(std::mem::size_of::<A>());
    dbg!(std::mem::size_of::<Option<A>>());
    dbg!(std::mem::size_of::<B>());
    dbg!(std::mem::size_of::<Option<B>>());
  }
  ```

- 避免分配*junk tail*。

## stack_ok

- 使用 Option 来替代之前的 Enum 类型。Option 中有各种 Rust 特色的类型转换工具，如 as_deref, as_ref, as_deref_mut, and so on。

## persistent_stack

- 更具有实际应用价值的节点的共享 ownership 下的栈。
- 无 ownership 和内部可变性的 Rc 无法实现 Iter 和 IterMut。

## deque

双端队列

- 引入了内部可变性，链表的结构类似于 Rc<RefCell< T>> 类型。从它中解出带 ownership 的 T 类型的过程，像答辩。
- 在实现 Iter<'a, T> 的过程中，受 Ref<'a, T> 智能指针的 lifetime 'a 约束，无法像下面这种方式返回一个 &T 类型，此时 &T 的生命周期不能比Ref<'a, T> 更长，所以设计上需要一点折中，iter 函数不能直接返回 &T，而是直接返回 Ref<'a, T>。

  ```rust
  &cell.borrow().inner // cannot return value referencing temporary value
  ```
