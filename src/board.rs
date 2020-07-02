use crate::kilobot;
use std::fmt;

pub struct Board
{
    width: u8,
    height: u8,
    bots: Vec<kilobot::Kilobot>,
}

impl Board
{
    pub fn init(&mut self, width: u8, height: u8) -> &mut Board
    {
        self.width = width;
        self.height = heigth;
        bots.clear;
        self
    }
}

impl fmt::Display for Board
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(width:{}, height:{}, number of bots:{})"
               , self.width
               , self.height
               , self.bots.len())
    }
}

pub fn new_board(width: u8, height: u8) -> Board
{
    init(width, height)
}