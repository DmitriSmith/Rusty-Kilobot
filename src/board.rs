pub(crate) mod board_map;
pub mod bot_map;
pub(crate) mod signal_map;

use std::fmt;
use crate::board::bot_map::{BotMap, BotLocation};
use crate::board::signal_map::SignalMap;
use crate::kilobot::Kilobot;
use crate::board::board_map::BoardMap;

pub const NORTH: u16 = 0;
pub const EAST: u16 = 90;
pub const SOUTH: u16 = 180;
pub const WEST: u16 = 270;

/// Basic error tpe that encompasses errors that can occur related to the board.
/// Doesn't carry any sort of message
pub enum LocationError {
    AlreadyOccupied,
    NotOccupied,
    OutOfBounds,
}

/// Struct representing an (x,y) coordinate on a 2D plane
pub struct CoordinatePair
{
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl CoordinatePair
{
    /// Create a new CoordinatePair
    pub fn new(x: usize, y: usize) -> CoordinatePair
    {
        CoordinatePair {x, y}
    }
    /// Return the Coordinate Pair as a tuple of usize
    pub fn as_usize_tuple(&self) -> (usize, usize)
    {
        (self.x, self.y)
    }

    /// Return the CoordinatePair as a tuple of u8
    pub fn as_u8_tuple(&self) -> (u8, u8)
    {
        (self.x as u8, self.y as u8)
    }

    /// Return the CoordinatePair as a tuple of type f64
    pub fn as_f64_tuple(&self) -> (f64, f64)
    {
        (self.x as f64, self.y as f64)
    }

    /// Clone the coordinate pair - just copies the x and y values
    pub fn clone(&self) -> CoordinatePair
    {
        CoordinatePair::new(self.x, self.y)
    }
}

pub struct Board
{
    width: usize,
    height: usize,
    pub bot_map: BotMap,
    pub signal_map: SignalMap,
}

impl Board
{
    /// Create a new instance of Board and fill it with empty Locations
    /// # Arguments
    /// * 'width' - How wide the board should be
    /// * 'height' - How tall the board should be
    ///
    /// For example, new(4,3) would create a 4x3 board that looks like this
    ///             * * * *
    ///             * * * *
    ///             * * * *
    ///         where '*' represents "None"
    pub fn new(width: usize, height: usize) -> Board
    {
        Board{width, height, bot_map: BotMap::new(width, height), signal_map: SignalMap::new(width, height) }
    }

    /// Returns the length of the Vector representing the board
    pub fn len(&self) -> usize
    {
        (self.width * self.height) as usize
    }

    /// Add new bot to the board at the given index
    /// # Arguments
    /// 'bot' - Kilobot to add to the board
    /// 'index' - Index in vector to place the bot
    /// 'facing' - Direction the bot is initially facing, in degrees clockwise from north
    /// # Returns
    /// None - Insert successful
    /// LocationError if out of bounds or coordinates already occupied
    pub fn add_new_bot_at_index(&mut self, bot: Kilobot, index: usize, facing: u16) -> Option<LocationError>
    {
        self.bot_map.add_new_bot_at_index(bot, index, facing)
    }

    /// Adds an existing BotLocation to the given index
    /// # Arguments
    /// * 'bot_loc' - Existing BotLocation object
    /// * 'index' - Index to insert into
    /// # Returns
    /// Option<LocationError> if coordinates are out of bounds, or there is already a bot at the coordinates
    pub fn add_bot_location_at_index(&mut self, bot_loc: BotLocation, index: usize) -> Option<LocationError>
    {
        self.bot_map.add_bot_location_at_index(bot_loc, index)
    }

    /// Removes the BotLocation at the specified index if a bot is present there and replaces it with None
    /// # Arguments
    /// * 'index' - Index of BotLocation to remove
    /// # Returns
    /// * Ok - Box<BotLocation> Pointer to removed BotLocation
    /// * Err(LocationError) if index is out of bounds or there is no bot in the coordinate
    pub fn remove_bot_location_at_index(&mut self, index: usize) -> Result<Box<BotLocation>,LocationError>
    {
        self.bot_map.remove_bot_location_at_index(index)
    }

    /// Returns an immutable reference to the bot at given coordinates, or LocationError if none
    /// # Arguments
    /// * 'index' - Index of location in board array
    /// # Returns
    /// * Ok - Reference to a Kilobot
    /// * Err - LocationError if no bot is found, or out of bounds
    pub fn get_bot_at_index(&self, index: usize) -> Result<&Kilobot, LocationError>
    {
        self.bot_map.get_bot_at_index(index)
    }

    /// Gets the BotLocation at the given index
    /// # Arguments
    /// * 'index' - Index to get BotLocation from
    /// # Returns
    /// * Ok - Reference to BotLocation at given index
    /// LocationError if None or Out of Bounds
    pub fn get_bot_location_at_index(&self, index: usize) -> Result<&BotLocation, LocationError>
    {
        self.bot_map.get_bot_location_at_index(index)
    }

    /// Returns whether the given index is occupied by a kilobot
    /// # Arguments
    /// * 'index' - Vector index to check
    /// # Returns
    /// * true if the location has a kilobot inside it
    /// * LocationError if the index is out of bounds
    pub fn index_has_bot(&self, index: usize) -> Result<bool, LocationError>
    {
        self.bot_map.index_is_occupied(index)
    }

    //pub fn get_signals_at_index(&self, index: usize) -> Result<Vec<Signal>>


}

impl BoardMap for Board
{
    /// Get the width of the board
    /// # Returns
    /// * Width of the board
    fn get_width(&self) -> usize
    {
        self.width
    }

    /// Gets the height of the board
    /// # Returns
    /// * Height of the board
    fn get_height(&self) -> usize
    {
        self.height
    }
}

impl fmt::Display for Board
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut num_bots: u16 = 0;
        for index in 0..self.len()
        {
            if self.bot_map.bots.get(index).unwrap().is_some()
            {
                num_bots += 1;
            }
        }
        write!(f, "(width:{}, height:{}, number of bots:{})"
               , self.width
               , self.height
               , num_bots)
    }
}
