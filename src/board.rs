use crate::kilobot;
use std::fmt;

pub struct Board
{
    width: u8,
    height: u8,
    bots: Vec<BotLocation>,
}

//TODO: REWORK print_board ASAP!
impl Board
{
    //Add new bot to the board
    //TODO: Sort bots
    pub fn add_bot(&mut self, bot: kilobot::Kilobot, x: u8, y: u8)
    {
        let mut space_available: bool = true;
        if x >= self.width || y >= self.height
        {
            space_available = false;
        }
        for i in 0..self.bots.len()
        {
            if self.bots.get(i).unwrap().x == x && self.bots.get(i).unwrap().y == y
            {
                space_available = false;
                break;
            }
        }
        if space_available
        {
            self.bots.push(BotLocation {x,y,bot})
        }

    }

    //Print left to right, top to bottom
    pub fn print_board(&self)
    {
        for i in 0..self.height
        {
            for j in 0..self.width
            {
                let mut bot_here = false;
                for k in 0..self.bots.len()
                {

                    if self.bots.get(k).unwrap().x == j && self.bots.get(k).unwrap().y == i
                    {
                        bot_here = true;
                        break;
                    }
                }

                if bot_here
                {
                    print!("O ");
                }
                else
                {
                    print!("* ");
                }
            }
            print!("\n");
        }
    }
}

struct BotLocation
{
    pub x: u8,
    pub y: u8,
    pub bot: kilobot::Kilobot,
}

impl BotLocation
{

}

pub fn new_board(width: u8, height: u8) -> Board
{
    Board {width, height, bots: vec![] }
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