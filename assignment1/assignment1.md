CMSC388Z Homework #1
Rust Basics
===

## Read Before You Start
1. This assignment is due on **September 10th, 2021 at noon**.
2. Please submit your code onto [**gradescope**](https://www.gradescope.com/courses/291105) electronically following the [instructions](https://help.gradescope.com/article/ccbpppziu9-student-submit-work).
3. Please make sure you are using the latest version of Rust.
    ```bash
    $ rustc --version
    rustc 1.54.0 (a178d0322 2021-07-26)
    ```
4. Please make sure your program doesn't contain any warning or error when submitting.
5. Please provide a [unit test](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) for each function you implemented.
6. Please provide a short description for each function you implemented. 
7. Please feel free to refer to any *appropriate* online resource. If you are not sure, you can email Dongze (dhe17 *at* umd *dot* edu) or Chase (ckanipe *at* terpmail *dot* umd *dot* edu) for clarification.
8. This is an individual project, please do not discuss any code related questions with anyone.
9. [This feedback survey](https://forms.gle/kon3fKNB8qyXf2AB9) will be open throughout the semester, if you have any comments or suggestions for the course, please feel free to report them to us *anonymously*. 
10. We highly recommend using VS code + [rust_analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extention. But it is OK if you have your preferred code editor.
## Introduction

In this assignment, you will create a rust project and implement functions using the concepts we covered during class. For every function that you implement, you **MUST** provide a unit test, and a short description to explain what you did for each function. For example,

```rust
/// In this function, the input car will undergo transformation. This funtion will return a transformer.
pub fn transformation(car) -> transformer {
    // Transformation!
    umimplemented!()
}

/// Describe `fun_2`
pub fn func_2() {
    umimplemented!()
}

// All implemented functions should be placed outside of the tests module `mod tests{}`

#[cfg(test)] // This attribute tells Rust that the follow module `tests` is used for testing.
mod tests { // All unit tests should be included in this module.
    use super::*; // This tells Rust that we will use the functions implemented above.
    #[test] // This attribute specifies that the following function is a unit test.
    fn test_transformation() -> {
        assert_eq!(transformation(beetle), bumblebee)
    }
    
    #[test]
    fn test_func_2() {
        // test something
    }
    
    ...
}
```

The keyword `pub` tells Rust that the functions are public, we can call them from other modules.


## 1. Create a rust project

Please create a new rust _library_ project using `cargo` named `rust_hw1`. 

```rust
cargo new rust_hw1 --lib
```

## 2. Implementing functions
Please implement the following functions in `rust_hw1/src/lib.rs`.  

### 2.1 Doubling a number
Implement the function that doubles an integer in three different ways:
1. A function that takes an `i32` integer as the input and returns an `i32` integer. A [unit test](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) that tests the correctness of your `double_v1()`.
    ```rust=
    /// A function that doubles a `i32` integer. The returned value is an `i32` integer.
    pub fn double_v1(n: i32) -> i32 {
        unimplemented!()
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_double_v1() {
            assert_eq!(double_v1(2), 4);
            assert_eq!(double_v1(-3), -6);    
        }
    }
    ```
2. A function that takes the reference of an `i32` integer as the input and returns an `i64` integer. In this function, I want you to create a new `i32` to take the doubled value, and then return its `i64` version. A unit test that test the correctness of your `double_v2()`.
    ```rust=
    ///
    pub fn double_v2(n: &i32) -> i64 {
        unimplemented!()
    }
    ```
3. A function that double an `i32` integer in place. A unit test that test the correctness of your `double_v2()`.
    ```rust=
    ///
    pub fn double_v3(n: &mut i32) {
        unimplemented!()
    }
    ```

### 2.2 Integer square root


Implement the integer square root function`sqrt(n)`. In this function I want you to try some values and then return the largest value, $m$, such that $m * m <= n$. For a 'harder' version, try to do it more efficiently than trying every possibility. Remember to write a unit test here (and on all future functions).

```rust
pub fn sqrt(n: usize) -> usize {
    unimplemented!()
}
```

### 2.3 [Fibonacci number](https://en.wikipedia.org/wiki/Fibonacci_number)


Given starting fibonacci numbers n1 and n2, compute an array of length `OUTSIZE` where `v[i]` is the i-th fibonacci number after n2. (Do not forget to write a unit test!)

```rust=
const OUTSIZE: usize = 5;
pub fn fibonacci(ns: (i32, i32)) -> [i32; OUTSIZE] {
    unimplemented!()
}
```

### 2.4 Array

#### Slice an array

In this function, I want you to first check the query tuple `(start, end)` is valid (i.e., taking `&arr[range.0..range.1]` will never make the program crash. For example, not out-of-boundary), then return the array slice if the query is valid, or an [error type](https://learning-rust.github.io/docs/e3.option_and_result.html#ok-err-for-Result-types). In this function, you should use Right-exclusive range literal `..`, but you [should also know](https://doc.rust-lang.org/book/appendix-02-operators.html) what does `..=` mean.

```rust=
pub fn slice(arr: &[i32], range: (usize, usize)) -> Result<&[i32], &'static str> {
    // check whether the query is valid (not out-of-boundary)
    // if it is valid, get the slice
    // if it is invalid, return an error
    // return Err("OOB!")
}

```

#### Binary search

In this function, you need to find a specific value in a sorted array and return the position of this value or nothing.

This function will take the reference of a sorted array, returns the position of the query if it is in the array or nothing if it is not in the array. The return type will be `Option<usize>`. Please write a unit test!

```rust=
pub fn binary_search(arr: &[i32], query: i32) -> Option<usize> {
    unimplemented!()
}
```

## Check your program

You **MUST** run `cargo clippy`, `cargo test`, `cargo fmt`, and `cargo build` before submission.

## Submission

Please submit your code onto [**gradescope**](https://www.gradescope.com/courses/291105) electronically following the [instructions](https://help.gradescope.com/article/ccbpppziu9-student-submit-work).

