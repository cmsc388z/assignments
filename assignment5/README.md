# CMSC388Z Assignment 5

## Read Before You Start
1. This assignment is due on **November 5th, 2021 at noon**.
2. Please submit your `src/lib.rs` onto [**GradeScope**](https://www.gradescope.com/courses/291105) electronically following the [instructions](https://help.gradescope.com/article/ccbpppziu9-student-submit-work).
3. Please make sure you are using the latest version of Rust.
    ```bash
    $ rustup update
    $ rustc --version
    rustc 1.56.0 (09c42c458 2021-10-18)    
    ```
4. Please make sure your program doesn't contain any warning or error when submitting.
5. Please feel free to refer to any *appropriate* online resource. If you are not sure, you can email Dongze (dhe17 *at* umd *dot* edu) or Chase (ckanipe *at* terpmail *dot* umd *dot* edu) for clarification.
6. This is an individual project, please do not discuss any code-related questions with anyone.
7. [This feedback survey](https://forms.gle/kon3fKNB8qyXf2AB9) will be open throughout the semester, if you have any comments or suggestions for the course, please feel free to report them to us *anonymously*.
8. We highly recommend using VS code + [rust_analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension. But it is OK if you have your preferred code editor.
9. **Compile your code frequently!** As the function signatures contain the expected input and output, I recommend you to write a unit testing before you implement each function, and test and run your code frequently when you are writing code. If you don't compile your code frequently, it is possible that you make some mistakes in the very beginning but you cannot realize it until you compile your program in the very end.

## Please use Piazza to ask questions!

## Introduction

For this assignment you will upgrade the linked list data structure you implemented in HW#4 to a doubly linked list data structure (DLL), to get familiar with

1. OOP in rust
2. Smart pointers
3. Trait

In the boilerplate `lib.rs` file, you should find a DLL data structure.

```rust
type Link = Option<Rc<RefCell<Box<dyn Node>>>>;

pub struct List {
    head: Link,
    tail: Link,
}
```

The `Link` type is very interesting. As we discussed in previous lectures, a `Rc` smart pointer allows a value to have multiple owners, however the data inside it must be immutable.
On the contrary, a `RefCell` can have only one owner, but it is able to modify its inner data without being a mutable smart pointer.
This involves the *interior mutability* concept that we will talk about it in our future lectures.
As you can see, if we combine these two types of smart pointers to create a `Rc<RefCell<>>` type, the innder data can have multiple owners **and** be modified! I know what you want to ask, but don't worry, the `RefCell` will first lock the inner data before modifying its value, which means nobody else can modify the data at the same time. So, the `Link` type corresponds to a chunk of memory in the heap that can have multiple owners, and can be modified by all its owners (definitely not at the same time).

You can also find two node data structures that implement the `Node` trait.

```rust
trait Node {
    fn data(&mut self) -> &mut u32;
    fn next(&mut self) -> &mut Link;
    fn prev(&mut self) -> &mut Link;
}

struct SmallNode {
    data: u32,
    next: Link,
    prev: Link,
}

struct BigNode {
    data: u32,
    next: Link,
    prev: Link,
}
```

You may notice that the DLL only accepts `u32` as the data. This is because implementing a generic type DLL is too hard to be a HW assignment. Moreover, to make your life easier, we have implemented all the basic stuff for you, so that you can focus on the concepts that we want to emphasize.

You are required to implement the `rand_typ_node()` function of `List` that returns one of the two node data structures. The purpose of implementing this function is to practice instantiating of smart pointers. In this function, you need to use the `thread_rng()` of `rand` crate to instantiate a random number generator to generate a random boolean. If the random generated boolean is `true`, you create an isolated `BigNode`, which means its `prev` and `next` are None, else you create an isolated `SmallNode`. You can find the function signature in the `src/lib.rs`

You are also required to implement four basic operations on the a DLL:

1. push front: This function uses the given data to generate a random type node using `rand_typ_node()`, and pushs the node to the head of the DLL.
2. pop front: This function pops out the first node from a DLL.
3. push back: This function uses the given data to generate a random type node using `rand_typ_node()`,and push the node to the tail of the DLL.
4. pop back: This function pops out the last node from a DLL.

---

## Testing

All tests for this project are included in the `tests/` directory as integration tests. We will test `pop_front()` and `push_front()` together, and `pop_back()` and `push_back()` together. You are welcome to write unit testing for each function.

## Submission

Please run the following cargo commands before submitting:
```bash
$ cargo test
$ cargo clippy
$ cargo check
$ cargo fmt
```

Please submit your`src/lib.rs` onto [GradeScope](https://www.gradescope.com/courses/291105). If you cannot access the GradeScope course page, please let us know.

