CMSC388Z Homework #2
===

## Read Before You Start
1. This assignment is due on **September 24th, 2021 at noon**.
2. Please submit your `src/lib.rs` onto [**gradescope**](https://www.gradescope.com/courses/291105) electronically following the [instructions](https://help.gradescope.com/article/ccbpppziu9-student-submit-work).
3. Please make sure you are using the latest version of Rust.
    ```bash
    $ rustc --version
    rustc 1.54.0 (a178d0322 2021-07-26)
    ```
4. Please make sure your program doesn't contain any warning or error when submitting.
5. Public tests are provided. Make sure your program is passing these before you submit.
6. Please feel free to refer to any *appropriate* online resource. If you are not sure, you can email Dongze (dhe17 *at* umd *dot* edu) or Chase (ckanipe *at* terpmail *dot* umd *dot* edu) for clarification.
7. This is an individual project, please do not discuss any code related questions with anyone.
8. [This feedback survey](https://forms.gle/kon3fKNB8qyXf2AB9) will be open throughout the semester, if you have any comments or suggestions for the course, please feel free to report them to us *anonymously*. 
9. We highly recommend using VS code + [rust_analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extention. But it is OK if you have your preferred code editor.

## Introduction

In this assignment, you will implement a variation of the snake game using knowledge covered in class. 

## 1. Game
Please implement the following functions in `src/game.rs`

### `update_food_expired`

The food on the map decays over time. The remaning life for each food is track using the `time` paramater of the `Food` struct. This function should remove all food which has a time value of less than 0.1.

### `update_food_life`

This function decay's the food life during each update. It should iterate over the food vector and subtract `FOOD_DECAY_SPEED` from the `time` parameter of food. 

### `check_eating`

When the snake head overlaps with food it should remove this food from the map, and increase the length of the snake. You may find the functions `head_position()` and `increase_length()` in snake useful.

### `check_snake_alive`

This function should check if the snake is alive. It should do this by
 - Getting the snake head position, and calculating the next head position using the current head direction. If this overlaps with the body, the snake is dead.
 - Checking if the next head position is within the bounds of the map. If not, the snake is dead.

The function should return a `bool` that indicates whether the snake is alive.

### `update_food`

This function should add food to the map while there are less than `NUM_FOODS` currently avaliable. The location can be randomly generated using `thread_rng`, which is included with the project. The food should be initialized with a life value of `INIT_FOOD_LIFE`. 

## 2. Snake

Please implement the following functions in `src/snake.rs`

### `head_position`

Returns the snake's current head position.

### `head_direction`

Returns the snake's current head direction.

### `increase_length`

Increase the snake's current length by one. Use the `last_removed_block` member of `Snake`. 

### `get_body`

Return the snake's body as a linked list.

## Check your program

You **MUST** run `cargo clippy`, `cargo test`, `cargo fmt`, and `cargo build` before submission. You may also run `cargo test`, to test your program against the public tests. 

## Submission

Please submit your `src/game.rs` and `src/snake.rs` onto [**gradescope**](https://www.gradescope.com/courses/291105) electronically.


