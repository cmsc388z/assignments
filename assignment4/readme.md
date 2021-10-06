# CMSC388Z Assignment 4

## Read Before You Start
1. This assignment is due on **October 22nd, 2021 at noon**.
2. Please submit your `src/main.rs` and `src/lib.rs` onto [**GradeScope**](https://www.gradescope.com/courses/291105) electronically following the [instructions](https://help.gradescope.com/article/ccbpppziu9-student-submit-work).
3. Please make sure you are using the latest version of Rust.
    ```bash
    $ rustup update
    $ rustc --version
    rustc 1.55.0 (c8dfcfe04 2021-09-06)
    ```
4. Please make sure your program doesn't contain any warning or error when submitting.
5. Please feel free to refer to any *appropriate* online resource. If you are not sure, you can email Dongze (dhe17 *at* umd *dot* edu) or Chase (ckanipe *at* terpmail *dot* umd *dot* edu) for clarification.
6. This is an individual project, please do not discuss any code-related questions with anyone.
7. [This feedback survey](https://forms.gle/kon3fKNB8qyXf2AB9) will be open throughout the semester, if you have any comments or suggestions for the course, please feel free to report them to us *anonymously*.
8. We highly recommend using VS code + [rust_analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension. But it is OK if you have your preferred code editor.
9. **Compile your code frequently!** As the function signatures contain the expected input and output, I recommend you to write a unit testing before you implement each function, and test and run your code frequently when you are writing code. If you don't compile your code frequently, it is possible that you make some mistakes in the very beginning but you cannot realize it until you compile your program in the very end.
10. In this assignment you may use the `match` keyword when handling errors. To be specific, when handling a `Result` value, I want you to check what the value is, and continue if it is a `Ok()` or write a customized error message if it is a `Err()`. This is why I define the return type of some functions as `Result<T, &'static str>`. To be specific, I only require you to process the `T` and `E` of `Result` differently. You don't need to worry about differnt variants of `E`. However, I want to emphasize that even all modules have their own error types. For example, `std::io::Error`, `clap::Error`, etc. If you want to handle the variants of a `E` explicitly **in the future** (you don't need to do this in this assignment), pleaes remember that the cases in `match e` correspond to possible variants of `e` as determined by its type. Notice that you can also use the `is_err()` method of `Result` and `is_none()` method of `Option` when handling the errors.

## Please use Piazza to ask questions!

## Introduction
For this assignment you will implement a variation of a linked list data structure, to get familiar pattern matching, `Box`, and implementing traits. 

In the boilerplate `lib.rs` file, you should find a linked list data structure.

```rust
pub struct LinkedList<T> {
    head: Option<Node<T>>,
    len: i32
}
```

And a node data structure.
```rust
pub struct Node<T> {
    pub next: Option<Box<Node<T>>>,
    pub data: T
}
```

You are required to implement the following methods on the linked list data structure.

### new

```rust
pub fn new() -> LinkedList<T>
```

This *function* will return an empty linked list.

### len

```rust
pub fn len(&mut self) -> i32
```

This method will return the length of the linked list.

### push

```rust
pub fn push(&mut self, item: T)
```

This method will push to the end of the linked list.

### push_unique

```rust
pub fn push_unique(&mut self, item: T) -> Result<(), String>
```

This method will only push the data to the end of the linked list if it does not already exist in the list. If the data is sucessfully pushed, it will return `Ok()`, otherwise, it will return an `Err(String)` where the string is a useful description of the error. 

### pop

```rust
pub fn pop(&mut self) -> Option<T>
```

This method will pop the last element off the linked list and return the value if there is such an element. Otherwise, it will return `None`. 

---

### Display

Finally, in order to make the list printable, you are required to implement the `fmt::Display` trait for the linked list so that if the user prints it some useful data will be printed to stdout. It is up to you what information you would like to print about the linked list. 

## Other notes

It will likely be useful to you to implement versions of all these functions above for the `Node` struct so they can be written recursivley.

---

## Testing

All tests for this project are included in the `tests/` directory as integration tests. 

## Submission

Please run the following cargo commands before submitting:
```bash
$ cargo clippy
$ cargo check
$ cargo fmt
```

Please submit your`src/lib.rs` onto [GradeScope](https://www.gradescope.com/courses/291105). If you cannot access the GradeScope course page, please let us know.


