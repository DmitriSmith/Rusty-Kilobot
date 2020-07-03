use crate::kilobot ::*;
use std::fmt;
use std::borrow::BorrowMut;

pub const NORTH: u16 = 0;
pub const EAST: u16 = 90;
pub const SOUTH: u16 = 180;
pub const WEST: u16 = 270;

enum LocationError {
    AlreadyOccupied,
    NotOccupied,
}

pub struct Board
{
    width: u8,
    height: u8,
    bots: Vec<Option<BotLocation>>,         //2D array packed into a Vector
}

impl Board
{
    /// Add new bot to the board
    /// # Arguements
    /// 'bot' - Kilobot to add to the board
    /// 'x' - X coordinate to place the bot
    /// 'y' - Y coordinate to place the bot
    /// 'facing' - Direction the bot is initially facing, in degrees clockwise from north
    pub fn add_bot(&mut self, bot: Kilobot, x: u8, y: u8, facing: u16)
    {
        if x < self.width && y < self.height
        {
            let desired_index = self.get_index_from_coord(x, y);
            let mut desired_position = self.bots.get(desired_index).unwrap();
            //Super hacky way to add an element to a specific index in a vector
            if desired_position.is_none() {
                let new_bot = Some(BotLocation {bot: Box::new(bot), facing});
                self.bots.push(new_bot);
                self.bots.swap_remove(desired_index);
            }
        }
    }

    pub fn get_index_from_coord(&self, x: u8, y: u8) -> usize
    {
        (x + (y * self.width)) as usize
    }

    /// Print left to right, top to bottom
    pub fn print_board(&self) {
        for j in 0..self.height {
            for i in 0..self.width {
                let mut bot_here = false;

                let this_space = self.bots.get(self.get_index_from_coord(i,j)).unwrap();

                match this_space {
                    Some(loc) => print!("  {}   ", loc.bot.get_uid()),
                    None => print!("({},{}) ", i, j),
                }
            }
            print!("\n\n");
        }
    }
}

struct BotLocation {
    bot: Box<Kilobot>,
    facing: u16,            //Represents the current angle of the bot, where 0 is north
}

impl BotLocation {
    /// Return the bot in the location
    /// Allows accessing the bot functions, but should not be used to remove the bot
    /// # Returns
    /// * Reference to the pointer to the bot in the Location
    pub fn get_bot(&mut self) -> &Box<Kilobot> {
        let ref b = self.bot;
        return b;
    }

    /// Return the facing of the bot in the location
    /// # Returns
    /// * The rotation of the bot in degrees clockwise away from north
    pub fn get_facing(&self) -> u16 {
        self.facing
    }

    /// Sets the facing of the bot in the location
    /// # Arguements
    /// * 'new_facing' - The new facing of the bot, in degrees clockwise from north
    pub fn set_facing(&mut self, new_facing: u16) {
        self.facing = new_facing
    }
}

/// Create a new instance of Board and fill it with empty Locations
/// #Arguements
/// * 'width' - How wide the board should be
/// * 'height' - How tall the board should be
///
/// For example, new_board(4,3) would create a 4x3 board that looks like this
///             * * * *
///             * * * *
///             * * * *
///         where '*' represents "None"
pub fn new_board(width: u8, height: u8) -> Board
{
    let mut board = Board {width, height, bots: Vec::with_capacity((width * height).into())};
    for i in 0..width * height {
        board.bots.push(None);
    }
    return board;
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