mod bot_map;
mod signal_map;

use crate::kilobot ::*;
use std::{fmt, mem};
use crate::board::LocationError::OutOfBounds;
use crate::board::bot_map::BotMap;
use crate::board::signal_map::SignalMap;

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
    pub(crate) x: u8,
    pub(crate) y: u8,
}

pub struct Board
{
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
    pub fn new(width: u8, height: u8) -> Board
    {
        Board{ bot_map: BotMap::new(width, height), signal_map: SignalMap::new(width, height) }
    }
}

impl fmt::Display for Board
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut num_bots: u16 = 0;
        for index in 0..self.locations.len()
        {
            if self.bot_map.locations.get(index).unwrap().is_some()
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
