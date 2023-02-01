# linkedlist-rs

使用 Rust，按照 [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/index.html) 的描述实现链表（实现链表是 Rust 实实在在的痛）。

## 实现

### stack_bad

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

- 避免分配 junk tail。

### stack_ok

- 使用 Option 来替代之前的 Enum 类型。Option 中有各种 Rust 特色的类型转换工具，如 as_deref, as_ref, as_deref_mut, and so on。

### persistent_stack

- 共享 ownership 节点的栈，更具有实际应用价值。
- 无 ownership 和内部可变性的 Rc 无法在 Iter 和 IterMut 很方便地解出 &T 和 &mut T，当然如果 T 类型实现了 Clone trait，事情就没那么复杂了。

### deque

不怎么 ok 的双端队列

- 引入了内部可变性，链表的结构类似于 Rc<RefCell< T>> 类型。从它中解出带 ownership 的类型 T 数据的过程，很冗长。
- 在实现 Iter<'a, T> 的过程中，受 Ref<'a, T> 智能指针的 lifetime 'a 约束，无法像下面这种方式返回一个 &T 类型，此时 &T 的生命周期不能比Ref<'a, T> 更长，所以设计上需要一点折中，iter 函数不能直接返回 &T，而是直接返回 Ref<'a, T>。

  ```rust
  &cell.borrow().inner // cannot return value referencing temporary value
  ```

### unsafe_deque_ok

- *mut 不同于 Box，它是 nullable 的，意味着它无法受益于空指针优化 Option< Box< T>>，换句话说，Option 对裸指针不是很友好，可以使用 null 来代替 None，可以通过 std::ptr::null_mut 函数获取一个 null，当然，还可以用 0 as *mut _。
- 使用 cargo miri test 来检查常见的 UB。包括：
  
  - 内存越界检查和内存释放后使用(use-after-free)。
  - 使用未初始化的数据
  - 数据竞争
  - 内存对齐问题

- 栈借用 borrow stack

  - 什么是 Pointer aliasing？Pointer aliasing 指的是两个或以上的指针指向同一份数据，而不安全的来源就在于同时存在多个指针并且其中之一可变。safe 代码中，不可能同时存在独占和共享两种引用，这种规则完全规避了：两个指针指向同一块内存区域，并且其中一个指针可变。

    ```c
    int i = 0;
    int *a = &i;
    int *b = &i;

    // 上述代码会造成什么问题呢？
    int foo(int* a, int* b) {
      *a = 5;
      *b = 6;
      return *a + *b // 不一定是11
    }
    // 上述代码中，a 和 b 都指向同一数据，*b = 6导致了 *a = 6，返回值也会是 12
    ```

  - rust 中的 NLL 机制，使得 reborrow 在 safe 代码的情况下依旧能够正常运行。借用者的借用范围并不是看作用域，而是看最后一次的使用位置。所有的 reborrow 都很清晰地嵌套，因此第一段代码中的 reborrow 不会和其他的发生冲突。如何来表达这种嵌套关系？答案是使用栈，而这就是栈借用。
  - 当使用 unsafe 指针时，借用检查就无法再正常工作了。

    ```rust
    let mut data = 10;
    let ref1 = &mut data;
    let ref2 = &mut *ref1;

    *ref2 += 2;
    *ref1 += 1;

    println!("{}", data);

    // 上述代码正确运行，但是将 ref1 和 ref2 的赋值位置交换，就会 error
    let mut data = 10;
    let ref1 = &mut data;
    let ref2 = &mut *ref1;

    // ORDER SWAPPED!
    *ref1 += 1;
    *ref2 += 2;

    println!("{}", data);
    ```
  
  - 在借用栈中，一个不可变引用，它上面的所有引用（在它之后被推入借用栈的引用）都只能拥有只读权限

    ```rust
    fn opaque_read(val: &i32) {
      println!("{}", val);
    } 

    unsafe {
        let mut data = 10;
        let mref1 = &mut data;
        let ptr2 = mref1 as *mut i32;
        let sref3 = &*mref1;
        let ptr4 = sref3 as *mut i32;

        *ptr4 += 4;
        opaque_read(sref3);
        *ptr2 += 2;
        *mref1 += 1;

        opaque_read(&data);
    }

    ```

  - 使用 raw pointer 时，应该遵循一个准则：一旦开始使用裸指针，就要尝试只使用它。
