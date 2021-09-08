use std::collections::LinkedList;

use piston_window::types::Color;
use piston_window::Context;
use piston_window::G2d;

use utils::draw_block;

const SNAKE_COLOR: Color = [1.0, 0.82, 0.0, 1.0];

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub x: i32,
    pub y: i32,
}

pub struct Snake {
    moving_direction: Direction,
    body: LinkedList<Block>,
    last_removed_block: Option<Block>,
}

impl Snake {
    pub fn new(init_x: i32, init_y: i32) -> Snake {
        let mut init_body: LinkedList<Block> = LinkedList::new();

        init_body.push_back(Block {
            x: init_x + 2,
            y: init_y,
        });
        init_body.push_back(Block {
            x: init_x + 1,
            y: init_y,
        });
        init_body.push_back(Block {
            x: init_x,
            y: init_y,
        });

        Snake {
            moving_direction: Direction::Right,
            body: init_body,
            last_removed_block: None,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        if let Some(d) = dir {
            self.moving_direction = d;
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.moving_direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };

        self.body.push_front(new_block);
        let removed_blk = self.body.pop_back().unwrap();
        self.last_removed_block = Some(removed_blk);
    }

    /// Get the head position
    pub fn head_position(&self) -> (i32, i32) {
        (0, 0)
    }

    /// Get the head direction
    pub fn head_direction(&self) -> Direction {
        Direction::Right
    }

    /// Increase the snake length
    pub fn increase_length(&mut self) {
        unimplemented!();
    }

    /// Return snake body as a linked list
    pub fn get_body(&self) -> LinkedList<Block> {
        unimplemented!();
    }
}
