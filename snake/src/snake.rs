use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.1, 0.5, 0.3, 1.0];

#[derive(Copy, Clone, PartialEq)]

pub enum Direction{
    Up,
    Down,
    Left,
    Right,
}


// Below is an implementation for the above enumeration. 
// Enumeration defines what the direction values could be (four possible ones)
// Implementation of the enum ensures 
impl Direction{
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Debug)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction, // Direction that the snake is travelling in
    body: LinkedList<Block>, // body of the snake, a (growable) list of blocks
    tail: Option<Block>, // tail of the snake, which doesnt get deleted when you eat an apple and the snake moves forward
}

impl Snake{

    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        // We have chosen a LinkedList to store the blocks of the Snake
        // This is a LIFO data structure, so we use the push_back() method to add blocks to it
        // The snake has to be 3 blocks long, and moving to the right, so x is iterated across the blocks
    
        
        body.push_back(Block { // push_back appends an element to the back of a list
            x: x+2,
            y,
        });
        body.push_back(Block {
            x: x+1,
            y,
        });
        
        body.push_back(Block {
            x,
            y,
        });

        Snake {
            direction: Direction::Right, // This sets the movement direction of the snake
            body,
            tail: None,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x,block.y, con, g); // we're accessing and drawing each of the blocks in the Snake LinkedList
        }
            
    }
    

    pub fn head_position(&self) -> (i32, i32) { // return type is a tuple with two ints, one for each in the cardinal directions
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)  // returns the x, y coordinates of the head position
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d, // Captures any change of direction when moving forward
            None => (), // Returns nothing when there is no direction input
        }
        
        // To move forward, we capture the last known positions, and then iterate from there

        let (last_x, last_y) : (i32, i32) = self.head_position(); // function above will return the head block positions

        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1, // Note: Since x = 0 is the line at the top of the window, we have to decrement to move up!
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

        // Here we manage the blocks that become part of the snake when it eats the apple
        // New blocks are pushed to the front of the snake
        // We remove the last block

        self.body.push_front(new_block); //push the new block to the front of the snake
        let removed_block = self.body.pop_back().unwrap(); //pop the back part of the linked list, and create a removed block
        self.tail = Some(removed_block); // Set up the tail of the snake with the popped block
        

    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y) : (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {},
        }

        // This match allows the direction to be translated into relative coordinates

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }


    }

    //We are creating a mutable version of the tail, and pushing back the

    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    //Returns whether there is an overlap of any block with the tail by checking x and y coords

    pub fn overlap_tail(&self, x: i32, y:i32) -> bool {
        let mut ch = 0;
        
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            ch += 1;
            if ch == self.body.len() -1 {
                break;
            }
        }
        return false;

    }


}